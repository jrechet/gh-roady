use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Repository {
    pub id: u64,
    pub name: String,
    pub full_name: String,
    pub owner: String,
    pub description: Option<String>,
    pub private: bool,
    pub fork: bool,
    pub url: String,
    pub html_url: String,
    pub language: Option<String>,
    pub stargazers_count: u32,
    pub watchers_count: u32,
    pub forks_count: u32,
    pub open_issues_count: u32,
    pub default_branch: String,
    pub archived: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub pushed_at: Option<DateTime<Utc>>,
}

impl From<octocrab::models::Repository> for Repository {
    fn from(repo: octocrab::models::Repository) -> Self {
        Self {
            id: repo.id.0,
            name: repo.name,
            full_name: repo.full_name.unwrap_or_default(),
            owner: repo.owner.map(|o| o.login).unwrap_or_default(),
            description: repo.description,
            private: repo.private.unwrap_or(false),
            fork: repo.fork.unwrap_or(false),
            url: repo.url.to_string(),
            html_url: repo.html_url.map(|u| u.to_string()).unwrap_or_default(),
            language: repo.language.and_then(|v| v.as_str().map(|s| s.to_string())),
            stargazers_count: repo.stargazers_count.unwrap_or(0),
            watchers_count: repo.watchers_count.unwrap_or(0),
            forks_count: repo.forks_count.unwrap_or(0),
            open_issues_count: repo.open_issues_count.unwrap_or(0),
            default_branch: repo.default_branch.unwrap_or_else(|| "main".into()),
            archived: repo.archived.unwrap_or(false),
            created_at: repo.created_at.unwrap_or_else(Utc::now),
            updated_at: repo.updated_at.unwrap_or_else(Utc::now),
            pushed_at: repo.pushed_at,
        }
    }
}
impl Repository {
    pub fn display_name(&self) -> String {
        let visibility = if self.private { "🔒" } else { "📂" };
        format!("{} {}", visibility, self.name)
    }

    /// Get short description (truncated)
    pub fn short_description(&self, max_len: usize) -> String {
        match &self.description {
            Some(desc) if desc.len() > max_len => {
                format!("{}...", &desc[..max_len])
            }
            Some(desc) => desc.clone(),
            None => "No description".to_string(),
        }
    }
}
