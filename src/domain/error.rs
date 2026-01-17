use thiserror::Error;

pub type Result<T> = std::result::Result<T, GhTuiError>;

#[derive(Error, Debug)]
pub enum GhTuiError {
    #[error("GitHub API error: {0}")]
    GitHubApi(#[from] octocrab::Error),

    #[error("Authentication error: {0}")]
    Auth(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Dialoguer error: {0}")]
    Dialoguer(String),

    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("Keyring error: {0}")]
    Keyring(#[from] keyring::Error),

    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("TUI error: {0}")]
    Tui(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl From<dialoguer::Error> for GhTuiError {
    fn from(err: dialoguer::Error) -> Self {
        GhTuiError::Dialoguer(err.to_string())
    }
}
