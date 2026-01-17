use std::sync::Arc;
use crate::infra::github::auth::AuthManager;
use crate::domain::error::Result;
use crate::domain::user::User;

pub struct AuthUseCase {
    manager: AuthManager,
}

impl AuthUseCase {
    pub fn new() -> Result<Self> {
        Ok(Self {
            manager: AuthManager::new()?,
        })
    }

    pub async fn login(&self, token: String) -> Result<User> {
        self.manager.login(token).await
    }

    pub fn logout(&self) -> Result<()> {
        self.manager.logout()
    }

    pub async fn status(&self) -> Result<()> {
        self.manager.status().await
    }
}
