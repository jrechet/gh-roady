use crate::infra::github::auth::AuthManager;
use crate::infra::github::client::GitHubClient;
use crate::domain::repo::Repository;
use crate::domain::artifact::Artifact;
use crate::domain::error::Result;
use crate::domain::github::GithubRepository;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum View {
    RepoList,
    RepoDetail,
    ArtifactList,
    AuthPrompt,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputMode {
    Normal,
    Editing,
}

pub struct App {
    pub should_quit: bool,
    pub current_view: View,
    pub input_mode: InputMode,
    pub repos: Vec<Repository>,
    pub artifacts: Vec<Artifact>,
    pub selected_index: usize,
    pub loading: bool,
    pub error_message: Option<String>,
    pub filter_text: String,
    pub client: Option<GitHubClient>,
}

impl App {
    pub async fn new() -> Result<Self> {
        let auth = AuthManager::new()?;
        let client = auth.get_client().ok();
        let current_view = if client.is_some() { View::RepoList } else { View::AuthPrompt };

        Ok(Self {
            should_quit: false,
            current_view,
            input_mode: InputMode::Normal,
            repos: Vec::new(),
            artifacts: Vec::new(),
            selected_index: 0,
            loading: false,
            error_message: None,
            filter_text: String::new(),
            client,
        })
    }

    pub async fn load_repos(&mut self) -> Result<()> {
        if let Some(ref client) = self.client {
            self.loading = true;
            self.error_message = None;

            match client.list_user_repos().await {
                Ok(repos) => {
                    self.repos = repos;
                    if self.current_view == View::RepoList {
                        self.selected_index = 0;
                    }
                }
                Err(e) => {
                    self.error_message = Some(format!("Error loading repos: {}", e));
                }
            }

            self.loading = false;
        } else {
            self.error_message = Some("Not authenticated".into());
        }
        Ok(())
    }

    pub async fn load_artifacts(&mut self, owner: &str, repo: &str) -> Result<()> {
        if let Some(ref client) = self.client {
            self.loading = true;
            self.error_message = None;

            match client.list_artifacts_for_repo(owner, repo).await {
                Ok(artifacts) => {
                    self.artifacts = artifacts;
                    self.selected_index = 0;
                }
                Err(e) => {
                    self.error_message = Some(format!("Error loading artifacts: {}", e));
                }
            }

            self.loading = false;
        }
        Ok(())
    }

    pub fn next(&mut self) {
        let len = match self.current_view {
            View::RepoList => self.filtered_repos().len(),
            View::ArtifactList => self.artifacts.len(),
            _ => 0,
        };
        if len > 0 {
            self.selected_index = (self.selected_index + 1) % len;
        }
    }

    pub fn previous(&mut self) {
        let len = match self.current_view {
            View::RepoList => self.filtered_repos().len(),
            View::ArtifactList => self.artifacts.len(),
            _ => 0,
        };
        if len > 0 {
            if self.selected_index > 0 {
                self.selected_index -= 1;
            } else {
                self.selected_index = len - 1;
            }
        }
    }

    pub fn selected_repo(&self) -> Option<&Repository> {
        let filtered = self.filtered_repos();
        filtered.get(self.selected_index).copied()
    }

    pub fn selected_artifact(&self) -> Option<&Artifact> {
        self.artifacts.get(self.selected_index)
    }

    pub fn filtered_repos(&self) -> Vec<&Repository> {
        if self.filter_text.is_empty() {
            self.repos.iter().collect()
        } else {
            let filter = self.filter_text.to_lowercase();
            self.repos
                .iter()
                .filter(|r| {
                    r.name.to_lowercase().contains(&filter)
                        || r.description
                            .as_ref()
                            .map(|d| d.to_lowercase().contains(&filter))
                            .unwrap_or(false)
                })
                .collect()
        }
    }
}
