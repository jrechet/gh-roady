use std::sync::Arc;
use colored::*;
use comfy_table::*;
use crate::presenter::cli::ArtifactCommands;
use crate::domain::error::Result;
use crate::domain::artifact::Artifact;
use crate::infra::github::auth::AuthManager;
use crate::app::artifact_use_case::ArtifactUseCase;

pub async fn handle_artifacts(command: ArtifactCommands) -> Result<()> {
    let auth = AuthManager::new()?;
    let client = Arc::new(auth.get_client()?);
    let use_case = ArtifactUseCase::new(client);

    match command {
        ArtifactCommands::List { owner, repo } => {
            println!("{}", "📚 Fetching artifacts...".cyan());
            let artifacts = if let (Some(o), Some(r)) = (owner, repo) {
                use_case.list_repo_artifacts(&o, &r).await?
            } else {
                use_case.list_all_artifacts().await?
            };

            if artifacts.is_empty() {
                println!("{}", "No artifacts found".yellow());
                return Ok(());
            }

            display_artifact_table(&artifacts);
        }
        ArtifactCommands::Delete { id, older_than, repo, owner } => {
            if let Some(days) = older_than {
                println!("{}", format!("🗑️ Deleting artifacts older than {} days...", days).red());
                let deleted = use_case.delete_older_than(days).await?;
                
                if deleted.is_empty() {
                    println!("{}", "No artifacts met the age criteria".yellow());
                } else {
                    println!("{}", format!("✓ Successfully deleted {} artifacts:", deleted.len()).green());
                    display_artifact_table(&deleted);
                }
            } else if let Some(artifact_id) = id {
                if let (Some(o), Some(r)) = (owner, repo) {
                    println!("{}", format!("🗑️ Deleting artifact {}...", artifact_id).red());
                    use_case.delete_artifacts(vec![(o, r, artifact_id)]).await?;
                    println!("{}", "✓ Deleted artifact".green());
                } else {
                    println!("{}", "Error: owner and repo are required to delete a specific artifact by ID".red());
                }
            } else {
                println!("{}", "Error: provide --id and --repo/--owner OR --older-than".red());
            }
        }
    }

    Ok(())
}

fn display_artifact_table(artifacts: &[Artifact]) {
    let mut table = Table::new();
    table
        .set_header(vec!["ID", "Name", "Repo", "Size", "Created At", "Expires At"])
        .load_preset(presets::UTF8_FULL)
        .apply_modifier(modifiers::UTF8_ROUND_CORNERS);

    for a in artifacts {
        let size = format!("{:.2} MB", a.size_in_bytes as f64 / 1_048_576.0);
        table.add_row(vec![
            a.id.to_string(),
            a.name.clone(),
            format!("{}/{}", a.repository_owner, a.repository_name),
            size,
            a.created_at.to_string(),
            a.expires_at.to_string(),
        ]);
    }

    println!("\n{}", table);
}
