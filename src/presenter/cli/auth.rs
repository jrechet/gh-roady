use std::sync::Arc;
use crate::infra::github::auth::AuthManager;
use crate::domain::error::Result;
use crate::app::auth_use_case::AuthUseCase;
use colored::*;
use dialoguer::Password;

pub async fn login(token: Option<String>) -> Result<()> {
    let auth = AuthUseCase::new()?;

    let token = match token {
        Some(t) => t,
        None => {
            println!("{}", "Enter your GitHub Personal Access Token".cyan());
            println!("Create one at: {}", "https://github.com/settings/tokens".blue());
            println!();
            
            Password::new()
                .with_prompt("Token")
                .interact()?
        }
    };

    auth.login(token).await?;
    Ok(())
}

pub fn logout() -> Result<()> {
    let auth = AuthUseCase::new()?;
    auth.logout()
}

pub async fn status() -> Result<()> {
    let auth = AuthUseCase::new()?;
    auth.status().await
}
