use crate::infra::github::auth::AuthManager;
use crate::infra::github::client::GitHubClient;
use crate::domain::repo::Repository;
use crate::domain::artifact::Artifact;
use crate::domain::storage::{StorageItem, StorageUsageReport, StorageItemType};
use crate::domain::error::Result;
use crate::domain::github::GithubRepository;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum View {
    MainMenu,
    RepoList,
    RepoDetail,
    ArtifactList,
    StorageManager,
    AuthPrompt,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputMode {
    Normal,
    Editing,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MenuItem {
    Repositories,
    StorageManager,
    Quit,
}

impl MenuItem {
    pub fn all() -> Vec<MenuItem> {
        vec![
            MenuItem::Repositories,
            MenuItem::StorageManager,
            MenuItem::Quit,
        ]
    }

    pub fn label(&self) -> &'static str {
        match self {
            MenuItem::Repositories => "Repositories",
            MenuItem::StorageManager => "Storage Manager",
            MenuItem::Quit => "Quit",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            MenuItem::Repositories => "Browse and explore your GitHub repositories",
            MenuItem::StorageManager => "Analyze and clean up storage usage",
            MenuItem::Quit => "Exit the application",
        }
    }
}

pub struct App {
    pub should_quit: bool,
    pub current_view: View,
    pub input_mode: InputMode,
    pub repos: Vec<Repository>,
    pub artifacts: Vec<Artifact>,
    pub storage_items: Vec<StorageItem>,
    pub storage_report: Option<StorageUsageReport>,
    pub selected_index: usize,
    pub menu_index: usize,
    pub selected_storage_items: Vec<usize>,
    pub loading: bool,
    pub loading_message: String,
    pub error_message: Option<String>,
    pub success_message: Option<String>,
    pub filter_text: String,
    pub client: Option<GitHubClient>,
    pub tick: u64,
    pub pending_load: Option<MenuItem>,
}

impl App {
    pub async fn new() -> Result<Self> {
        let auth = AuthManager::new()?;
        let client = auth.get_client().ok();
        let current_view = if client.is_some() { View::MainMenu } else { View::AuthPrompt };

        Ok(Self {
            should_quit: false,
            current_view,
            input_mode: InputMode::Normal,
            repos: Vec::new(),
            artifacts: Vec::new(),
            storage_items: Vec::new(),
            storage_report: None,
            selected_index: 0,
            menu_index: 0,
            selected_storage_items: Vec::new(),
            loading: false,
            loading_message: String::new(),
            error_message: None,
            success_message: None,
            filter_text: String::new(),
            client,
            tick: 0,
            pending_load: None, // New field
        })
    }

    pub fn current_menu_item(&self) -> MenuItem {
        MenuItem::all()[self.menu_index]
    }

    pub async fn load_repos(&mut self) -> Result<()> {
        if let Some(ref client) = self.client {
            self.loading = true;
            self.loading_message = "Loading repositories...".into();
            self.error_message = None;

            match client.list_user_repos().await {
                Ok(repos) => {
                    self.repos = repos;
                    self.selected_index = 0;
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

    pub async fn load_all_artifacts(&mut self) -> Result<()> {
        if let Some(ref client) = self.client {
            self.loading = true;
            self.loading_message = "Loading all artifacts...".into();
            self.error_message = None;

            match client.list_all_artifacts().await {
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

    pub async fn load_artifacts(&mut self, owner: &str, repo: &str) -> Result<()> {
        if let Some(ref client) = self.client {
            self.loading = true;
            self.loading_message = format!("Loading artifacts for {}/{}...", owner, repo);
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

    pub async fn load_storage(&mut self) -> Result<()> {
        if let Some(ref client) = self.client {
            self.loading = true;
            self.loading_message = "Scanning storage usage...".into();
            self.error_message = None;

            match client.get_storage_usage().await {
                Ok(report) => {
                    self.storage_items = report.items.clone()
                        .into_iter()
                        .filter(|i| i.size_in_bytes >= 1024)
                        .collect();
                    self.storage_items.sort_by(|a, b| b.size_in_bytes.cmp(&a.size_in_bytes));
                    self.storage_report = Some(report);
                    self.selected_index = 0;
                    self.selected_storage_items.clear();
                }
                Err(e) => {
                    self.error_message = Some(format!("Error loading storage: {}", e));
                }
            }

            self.loading = false;
        }
        Ok(())
    }

    pub async fn delete_selected_storage_items(&mut self) -> Result<()> {
        if let Some(ref client) = self.client {
            let items_to_delete: Vec<_> = self.selected_storage_items.iter()
                .filter_map(|&idx| self.storage_items.get(idx).cloned())
                .collect();

            for item in items_to_delete {
                match item.item_type {
                    StorageItemType::Artifact => {
                        client.delete_artifact(&item.owner, &item.repo, item.id).await?;
                    }
                    StorageItemType::Cache => {
                        client.delete_cache(&item.owner, &item.repo, item.id).await?;
                    }
                }
            }

            let count = self.selected_storage_items.len();
            self.success_message = Some(format!("✓ Deleted {} items", count));
            self.selected_storage_items.clear();
            
            // Reload storage
            self.load_storage().await?;
        }
        Ok(())
    }

    pub fn toggle_storage_selection(&mut self) {
        if self.storage_items.is_empty() {
            return;
        }
        
        let idx = self.selected_index;
        if self.selected_storage_items.contains(&idx) {
            self.selected_storage_items.retain(|&i| i != idx);
        } else {
            self.selected_storage_items.push(idx);
        }
    }

    pub fn next(&mut self) {
        let len = self.current_list_len();
        if len > 0 {
            self.selected_index = (self.selected_index + 1) % len;
        }
    }

    pub fn previous(&mut self) {
        let len = self.current_list_len();
        if len > 0 {
            if self.selected_index > 0 {
                self.selected_index -= 1;
            } else {
                self.selected_index = len - 1;
            }
        }
    }

    pub fn next_menu(&mut self) {
        let len = MenuItem::all().len();
        self.menu_index = (self.menu_index + 1) % len;
    }

    pub fn previous_menu(&mut self) {
        let len = MenuItem::all().len();
        if self.menu_index > 0 {
            self.menu_index -= 1;
        } else {
            self.menu_index = len - 1;
        }
    }

    fn current_list_len(&self) -> usize {
        match self.current_view {
            View::MainMenu => MenuItem::all().len(),
            View::RepoList => self.filtered_repos().len(),
            View::ArtifactList => self.artifacts.len(),
            View::StorageManager => self.storage_items.len(),
            _ => 0,
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
