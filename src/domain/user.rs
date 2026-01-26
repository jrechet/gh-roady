use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub login: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub avatar_url: String,
    pub bio: Option<String>,
    pub company: Option<String>,
    pub location: Option<String>,
    pub public_repos: u32,
    pub followers: u32,
    pub following: u32,
}

impl From<octocrab::models::UserProfile> for User {
    fn from(user: octocrab::models::UserProfile) -> Self {
        Self {
            id: user.id.0,
            login: user.login,
            name: user.name,
            email: user.email,
            avatar_url: user.avatar_url.to_string(),
            bio: user.bio,
            company: user.company,
            location: user.location,
            public_repos: user.public_repos as u32,
            followers: user.followers as u32,
            following: user.following as u32,
        }
    }
}

impl From<octocrab::models::Author> for User {
    fn from(author: octocrab::models::Author) -> Self {
        Self {
            id: author.id.0,
            login: author.login,
            name: None,
            email: None,
            avatar_url: author.avatar_url.to_string(),
            bio: None,
            company: None,
            location: None,
            public_repos: 0,
            followers: 0,
            following: 0,
        }
    }
}
