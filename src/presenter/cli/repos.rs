use std::sync::Arc;
use crate::infra::github::auth::AuthManager;
use crate::domain::error::Result;
use crate::app::repo_use_case::RepoUseCase;
use colored::*;
use comfy_table::*;

pub async fn list_repos(
    owner: Option<String>,
    filter_private: bool,
    filter_public: bool,
    all: bool,
) -> Result<()> {
    let auth = AuthManager::new()?;
    let client = Arc::new(auth.get_client()?);
    let use_case = RepoUseCase::new(client);

    println!("{}", "📚 Fetching repositories...".cyan());

    let repos = use_case.list_repos(owner, filter_private, filter_public, all).await?;

    if repos.is_empty() {
        println!("{}", "No repositories found".yellow());
        return Ok(());
    }

    // Create table
    let mut table = Table::new();
    table
        .set_header(vec!["Name", "Visibility", "Language", "⭐", "Description"])
        .load_preset(presets::UTF8_FULL)
        .apply_modifier(modifiers::UTF8_ROUND_CORNERS);

    for repo in &repos {
        let visibility = if repo.private { "Private".red() } else { "Public".green() };
        let language = repo.language.as_deref().unwrap_or("-");
        let stars = repo.stargazers_count.to_string();
        let description = repo.short_description(50);

        table.add_row(vec![
            &repo.name,
            &visibility.to_string(),
            language,
            &stars,
            &description,
        ]);
    }

    println!("\n{}", table);
    println!("\n{} repositories found", repos.len().to_string().cyan());

    Ok(())
}
