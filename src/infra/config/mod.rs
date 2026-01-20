pub mod storage;

use crate::domain::error::{GhTuiError, Result};
use dirs::config_dir;
use keyring::Entry;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

const APP_NAME: &str = "ghr";
const SERVICE_NAME: &str = "ghr-token";

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub user: Option<UserConfig>,
    pub preferences: Preferences,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserConfig {
    pub username: String,
    pub email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Preferences {
    pub theme: String,
    pub default_view: String,
}

impl Default for Preferences {
    fn default() -> Self {
        Self {
            theme: "dark".to_string(),
            default_view: "repos".to_string(),
        }
    }
}

pub struct ConfigManager {
    config_path: PathBuf,
}

impl ConfigManager {
    pub fn new() -> Result<Self> {
        let config_path = Self::get_config_path()?;
        Ok(Self { config_path })
    }

    fn get_config_path() -> Result<PathBuf> {
        let mut path = config_dir()
            .ok_or_else(|| GhTuiError::Config("Cannot find config directory".into()))?;
        path.push(APP_NAME);
        
        // Create dir if not exists
        if !path.exists() {
            fs::create_dir_all(&path)?;
        }
        
        path.push("config.toml");
        Ok(path)
    }

    pub fn load(&self) -> Result<Config> {
        if !self.config_path.exists() {
            return Ok(Config::default());
        }

        let content = fs::read_to_string(&self.config_path)?;
        let config: Config = toml::from_str(&content)
            .map_err(|e| GhTuiError::Config(format!("Invalid config: {}", e)))?;
        
        Ok(config)
    }

    pub fn save(&self, config: &Config) -> Result<()> {
        let content = toml::to_string_pretty(config)
            .map_err(|e| GhTuiError::Config(format!("Cannot serialize config: {}", e)))?;
        
        fs::write(&self.config_path, content)?;
        Ok(())
    }

    /// Store GitHub token securely in system keyring
    pub fn store_token(&self, token: &str) -> Result<()> {
        let entry = Entry::new(SERVICE_NAME, "default")?;
        entry.set_password(token)?;
        Ok(())
    }

    /// Retrieve GitHub token - tries multiple sources in order:
    /// 1. Environment variable GITHUB_TOKEN
    /// 2. gh CLI token (via `gh auth token`)
    /// 3. ghr's own keyring
    pub fn get_token(&self) -> Result<String> {
        // 1. Try environment variable first
        if let Ok(token) = std::env::var("GITHUB_TOKEN") {
            if !token.is_empty() {
                return Ok(token);
            }
        }

        // 2. Try gh CLI token
        if let Some(token) = self.get_gh_cli_token() {
            return Ok(token);
        }

        // 3. Try our own keyring
        match Entry::new(SERVICE_NAME, "default") {
            Ok(entry) => {
                match entry.get_password() {
                    Ok(token) => Ok(token),
                    Err(_) => Err(GhTuiError::Auth(
                        "Not authenticated. Please run one of:\n\
                         \n\
                         • gh auth login          (recommended - uses GitHub CLI)\n\
                         • ghr auth login --token YOUR_TOKEN\n\
                         • export GITHUB_TOKEN=YOUR_TOKEN".into()
                    ))
                }
            }
            Err(_) => Err(GhTuiError::Auth(
                "Not authenticated. Please run one of:\n\
                 \n\
                 • gh auth login          (recommended - uses GitHub CLI)\n\
                 • ghr auth login --token YOUR_TOKEN\n\
                 • export GITHUB_TOKEN=YOUR_TOKEN".into()
            ))
        }
    }

    /// Try to get token from gh CLI
    fn get_gh_cli_token(&self) -> Option<String> {
        let output = Command::new("gh")
            .args(["auth", "token"])
            .output()
            .ok()?;

        if output.status.success() {
            let token = String::from_utf8(output.stdout).ok()?;
            let token = token.trim();
            if !token.is_empty() {
                return Some(token.to_string());
            }
        }
        None
    }

    /// Remove stored token
    pub fn delete_token(&self) -> Result<()> {
        let entry = Entry::new(SERVICE_NAME, "default")?;
        entry.delete_password()?;
        Ok(())
    }

    /// Check if token exists (any source)
    pub fn has_token(&self) -> bool {
        self.get_token().is_ok()
    }

    /// Check specifically if gh CLI is authenticated
    pub fn has_gh_cli_token(&self) -> bool {
        self.get_gh_cli_token().is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.preferences.theme, "dark");
    }
}
