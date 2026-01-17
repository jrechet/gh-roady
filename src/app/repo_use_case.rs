use std::sync::Arc;
use crate::domain::repo::Repository;
use crate::domain::github::GithubRepository;
use crate::domain::error::Result;

pub struct RepoUseCase {
    repo: Arc<dyn GithubRepository>,
}

impl RepoUseCase {
    pub fn new(repo: Arc<dyn GithubRepository>) -> Self {
        Self { repo }
    }

    pub async fn list_repos(
        &self,
        owner: Option<String>,
        filter_private: bool,
        filter_public: bool,
        all: bool,
    ) -> Result<Vec<Repository>> {
        let mut repos = match owner {
            Some(ref o) => self.repo.list_repos_for(o).await?,
            None => self.repo.list_user_repos().await?,
        };

        if !all {
            repos.retain(|r| !r.archived);
        }

        if filter_private {
            repos.retain(|r| r.private);
        }
        if filter_public {
            repos.retain(|r| !r.private);
        }

        Ok(repos)
    }

    pub async fn get_repo(&self, owner: &str, name: &str) -> Result<Repository> {
        self.repo.get_repo(owner, name).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::github::MockGithubRepository;

    #[tokio::test]
    async fn test_list_repos_filtering() {
        let mut mock = MockGithubRepository::new();
        mock.expect_list_user_repos()
            .times(1)
            .returning(|| {
                let mut r1 = Repository::default();
                r1.private = true;
                let mut r2 = Repository::default();
                r2.private = false;
                Ok(vec![r1, r2])
            });

        let use_case = RepoUseCase::new(Arc::new(mock));
        let private_only = use_case.list_repos(None, true, false, false).await.unwrap();
        assert_eq!(private_only.len(), 1);
        assert!(private_only[0].private);
    }
}
