use std::sync::Arc;
use colored::*;
use crate::domain::error::Result;
use crate::infra::github::auth::AuthManager;
use crate::app::artifact_use_case::ArtifactUseCase;
use dialoguer::{MultiSelect, Select};
use crate::domain::storage::StorageItemType;

pub async fn handle_df() -> Result<()> {
    let auth = AuthManager::new()?;
    let client = Arc::new(auth.get_client()?);
    let use_case = ArtifactUseCase::new(client);

    println!("{}", "📊 Calculating storage usage...".cyan());

    let report = use_case.get_storage_usage().await?;
    let used = report.total_used;
    let max = report.max_allowed;

    let used_mo = used as f64 / 1_048_576.0;
    let max_mo = max as f64 / 1_048_576.0;
    let percentage = if max > 0 { (used as f64 / max as f64) * 100.0 } else { 0.0 };

    println!("\n{}", "Actions Storage Usage (Artifacts, Packages, Caches):".bold());
    
    let bar_width = 40;
    let filled_width = if max > 0 {
        ((percentage / 100.0) * bar_width as f64) as usize
    } else {
        0
    };
    let filled_width = std::cmp::min(filled_width, bar_width);
    
    let bar_color = if percentage > 90.0 {
        Color::Red
    } else if percentage > 70.0 {
        Color::Yellow
    } else {
        Color::Green
    };

    let bar = format!(
        "[{}{}]",
        "▆".repeat(filled_width).color(bar_color),
        " ".repeat(bar_width - filled_width)
    );

    println!("{}", bar);
    println!(
        "{:.2} Mo / {:.2} Mo ({:.1}%)",
        used_mo,
        max_mo,
        percentage
    );

    if percentage > 90.0 {
        println!("\n{}", "⚠️  Warning: You are almost out of storage space!".red().bold());
    }

    // Filter out items smaller than 1KB and sort by size descending
    let mut items: Vec<_> = report.items.into_iter()
        .filter(|i| i.size_in_bytes >= 1024)
        .collect();
    items.sort_by(|a, b| b.size_in_bytes.cmp(&a.size_in_bytes));

    if items.is_empty() {
        println!("\n{}", "No individual items (>1KB) found to manage.".yellow());
        return Ok(());
    }

    println!("\n{}", "Detailed Storage Usage:".bold());
    println!("{}", "Select items to delete (Space to toggle, Enter to confirm, Esc to skip):".cyan());

    let options: Vec<String> = items.iter().map(|i| {
        let size = format!("{:.2} MB", i.size_in_bytes as f64 / 1_048_576.0);
        let type_label = match i.item_type {
            StorageItemType::Artifact => "ARTIFACT".blue(),
            StorageItemType::Cache => "CACHE   ".magenta(),
        };
        format!("{:<10} {:<40} {:>10} [{}/{}]", type_label, i.name, size, i.owner, i.repo)
    }).collect();

    let selected = MultiSelect::new()
        .with_prompt("Select items to delete")
        .items(&options)
        .interact_opt()?;

    let selected_indices = match selected {
        Some(s) if !s.is_empty() => s,
        _ => {
            println!("{}", "No items selected for deletion.".yellow());
            return Ok(());
        }
    };

    println!("\n{}", format!("⚠️  Confirm deletion of {} items?", selected_indices.len()).red().bold());
    let confirm = Select::new()
        .items(&["No, cancel", "Yes, delete them"])
        .default(0)
        .interact()?;

    if confirm == 1 {
        for idx in selected_indices {
            let item = &items[idx];
            print!("Deleting {} from {}... ", item.name, item.repo);
            match item.item_type {
                StorageItemType::Artifact => {
                    use_case.delete_artifacts(vec![(item.owner.clone(), item.repo.clone(), item.id)]).await?;
                }
                StorageItemType::Cache => {
                    use_case.delete_cache(&item.owner, &item.repo, item.id).await?;
                }
            }
            println!("{}", "✓".green());
        }
        println!("\n{}", "Cleanup completed!".green().bold());
    } else {
        println!("{}", "Deletion cancelled.".yellow());
    }

    Ok(())
}
