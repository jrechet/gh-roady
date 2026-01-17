pub mod storage;

use crate::domain::error::{GhTuiError, Result};
use dirs::config_dir;
use keyring::Entry;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

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

    /// Retrieve GitHub token from system keyring
    pub fn get_token(&self) -> Result<String> {
        let entry = Entry::new(SERVICE_NAME, "default")?;
        let token = entry.get_password()?;
        Ok(token)
    }

    /// Remove stored token
    pub fn delete_token(&self) -> Result<()> {
        let entry = Entry::new(SERVICE_NAME, "default")?;
        entry.delete_password()?;
        Ok(())
    }

    /// Check if token exists
    pub fn has_token(&self) -> bool {
        self.get_token().is_ok()
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
