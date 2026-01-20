use crate::infra::github::client::GitHubClient;
use crate::infra::config::ConfigManager;
use crate::domain::user::User;
use crate::domain::error::{GhTuiError, Result};
use crate::domain::github::GithubRepository;
use colored::*;

pub struct AuthManager {
    config: ConfigManager,
}

impl AuthManager {
    pub fn new() -> Result<Self> {
        Ok(Self {
            config: ConfigManager::new()?,
        })
    }

    /// Login with PAT token
    pub async fn login(&self, token: String) -> Result<User> {
        // Verify token
        let client = GitHubClient::new(token.clone())?;
        
        let user = match client.get_current_user().await {
            Ok(u) => u,
            Err(_) => return Err(GhTuiError::Auth("Invalid token".into())),
        };

        // Store token
        self.config.store_token(&token)?;

        // Update config
        let mut config = self.config.load()?;
        config.user = Some(crate::infra::config::UserConfig {
            username: user.login.clone(),
            email: user.email.clone(),
        });
        self.config.save(&config)?;

        println!("{}", "✓ Successfully authenticated!".green());
        println!("Logged in as: {}", user.login.cyan());

        Ok(user)
    }

    /// Logout and clear credentials
    pub fn logout(&self) -> Result<()> {
        self.config.delete_token()?;
        
        let mut config = self.config.load()?;
        config.user = None;
        self.config.save(&config)?;

        println!("{}", "✓ Successfully logged out".green());
        Ok(())
    }

    /// Get auth status
    pub async fn status(&self) -> Result<()> {
        // Check gh CLI first
        if self.config.has_gh_cli_token() {
            println!("{}", "✓ Authenticated via gh CLI".green());
        }

        if !self.config.has_token() {
            println!("{}", "✗ Not authenticated".red());
            println!("\nTo authenticate, run one of:");
            println!("  {} (recommended)", "gh auth login".cyan());
            println!("  {}", "ghr auth login --token YOUR_TOKEN".cyan());
            println!("  {}", "export GITHUB_TOKEN=YOUR_TOKEN".cyan());
            return Ok(());
        }

        let token = self.config.get_token()?;
        let client = GitHubClient::new(token)?;

        match client.get_current_user().await {
            Ok(user) => {
                if !self.config.has_gh_cli_token() {
                    println!("{}", "✓ Authenticated".green());
                }
                println!("User: {}", user.login.cyan());
                if let Some(ref email) = user.email {
                    println!("Email: {}", email);
                }
            }
            Err(_) => {
                println!("{}", "✗ Token is invalid".red());
                println!("Run 'gh auth login' or 'ghr auth login' to re-authenticate");
            }
        }

        Ok(())
    }

    /// Get client if authenticated
    pub fn get_client(&self) -> Result<GitHubClient> {
        let token = self.config.get_token()?;
        GitHubClient::new(token)
    }
}
