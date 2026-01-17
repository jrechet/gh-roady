use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artifact {
    pub id: u64,
    pub node_id: String,
    pub name: String,
    pub size_in_bytes: u64,
    pub url: String,
    pub archive_download_url: String,
    pub expired: bool,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub workflow_run_id: Option<u64>,
    pub repository_name: String,
    pub repository_owner: String,
}

impl From<octocrab::models::workflows::WorkflowListArtifact> for Artifact {
    fn from(a: octocrab::models::workflows::WorkflowListArtifact) -> Self {
        Self {
            id: a.id.0,
            node_id: a.node_id,
            name: a.name,
            size_in_bytes: a.size_in_bytes as u64,
            url: a.url.to_string(),
            archive_download_url: a.archive_download_url.to_string(),
            expired: a.expired,
            created_at: a.created_at,
            expires_at: a.expires_at,
            workflow_run_id: None, // Summary might not have ID easily accessible if it's a different type
            repository_name: String::new(),
            repository_owner: String::new(),
        }
    }
}

impl Artifact {
    pub fn is_older_than(&self, days: i64) -> bool {
        let now = Utc::now();
        let duration = now - self.created_at;
        duration.num_days() >= days
    }
}
