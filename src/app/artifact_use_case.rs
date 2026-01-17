use std::sync::Arc;
use crate::domain::artifact::Artifact;
use crate::domain::github::GithubRepository;
use crate::domain::error::Result;

pub struct ArtifactUseCase {
    repo: Arc<dyn GithubRepository>,
}

impl ArtifactUseCase {
    pub fn new(repo: Arc<dyn GithubRepository>) -> Self {
        Self { repo }
    }

    pub async fn list_all_artifacts(&self) -> Result<Vec<Artifact>> {
        self.repo.list_all_artifacts().await
    }

    pub async fn list_repo_artifacts(&self, owner: &str, repo_name: &str) -> Result<Vec<Artifact>> {
        self.repo.list_artifacts_for_repo(owner, repo_name).await
    }

    pub async fn delete_artifacts(&self, artifacts: Vec<(String, String, u64)>) -> Result<Vec<u64>> {
        let mut deleted_ids = Vec::new();
        for (owner, repo, id) in artifacts {
            self.repo.delete_artifact(&owner, &repo, id).await?;
            deleted_ids.push(id);
        }
        Ok(deleted_ids)
    }

    pub async fn delete_older_than(&self, days: i64) -> Result<Vec<Artifact>> {
        let artifacts = self.repo.list_all_artifacts().await?;
        let to_delete: Vec<_> = artifacts
            .into_iter()
            .filter(|a| a.is_older_than(days))
            .collect();

        let mut deleted = Vec::new();
        for artifact in to_delete {
            self.repo.delete_artifact(&artifact.repository_owner, &artifact.repository_name, artifact.id).await?;
            deleted.push(artifact);
        }

        Ok(deleted)
    }

    pub async fn delete_cache(&self, owner: &str, repo: &str, cache_id: u64) -> Result<()> {
        self.repo.delete_cache(owner, repo, cache_id).await
    }

    pub async fn get_storage_usage(&self) -> Result<crate::domain::storage::StorageUsageReport> {
        self.repo.get_storage_usage().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::github::MockGithubRepository;
    use crate::domain::artifact::Artifact;
    use chrono::Utc;

    #[tokio::test]
    async fn test_list_all_artifacts() {
        let mut mock = MockGithubRepository::new();
        mock.expect_list_all_artifacts()
            .times(1)
            .returning(|| Ok(vec![]));

        let use_case = ArtifactUseCase::new(Arc::new(mock));
        let results = use_case.list_all_artifacts().await.unwrap();
        assert!(results.is_empty());
    }

    #[tokio::test]
    async fn test_delete_older_than() {
        let mut mock = MockGithubRepository::new();
        let now = Utc::now();
        let old_artifact = Artifact {
            id: 1,
            node_id: "node1".into(),
            name: "old".into(),
            size_in_bytes: 100,
            url: "url".into(),
            archive_download_url: "url".into(),
            expired: false,
            created_at: now - chrono::Duration::days(10),
            expires_at: now + chrono::Duration::days(10),
            workflow_run_id: None,
            repository_name: "repo".into(),
            repository_owner: "owner".into(),
        };

        mock.expect_list_all_artifacts()
            .times(1)
            .returning(move || Ok(vec![old_artifact.clone()]));

        mock.expect_delete_artifact()
            .with(mockall::predicate::eq("owner"), mockall::predicate::eq("repo"), mockall::predicate::eq(1))
            .times(1)
            .returning(|_, _, _| Ok(()));

        let use_case = ArtifactUseCase::new(Arc::new(mock));
        let deleted = use_case.delete_older_than(5).await.unwrap();
        assert_eq!(deleted.len(), 1);
        assert_eq!(deleted[0].id, 1);
    }
}
