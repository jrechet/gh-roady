use async_trait::async_trait;
use crate::domain::artifact::Artifact;
use crate::domain::repo::Repository;
use crate::domain::user::User;
use crate::domain::error::Result;

use crate::domain::storage::StorageUsageReport;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait GithubRepository: Send + Sync {
    async fn get_current_user(&self) -> Result<User>;
    async fn list_user_repos(&self) -> Result<Vec<Repository>>;
    async fn list_repos_for(&self, owner: &str) -> Result<Vec<Repository>>;
    async fn get_repo(&self, owner: &str, repo: &str) -> Result<Repository>;
    
    // Artifacts
    async fn list_artifacts_for_repo(&self, owner: &str, repo: &str) -> Result<Vec<Artifact>>;
    async fn list_all_artifacts(&self) -> Result<Vec<Artifact>>;
    async fn delete_artifact(&self, owner: &str, repo: &str, artifact_id: u64) -> Result<()>;
    async fn delete_cache(&self, owner: &str, repo: &str, cache_id: u64) -> Result<()>;

    // Billing
    async fn get_storage_usage(&self) -> Result<StorageUsageReport>;
}
