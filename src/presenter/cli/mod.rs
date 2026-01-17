pub mod auth;
pub mod repos;
pub mod artifacts;
pub mod df;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ghr")]
#[command(about = "Modern GitHub CLI with TUI interface", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Enable verbose logging
    #[arg(short, long, global = true)]
    pub verbose: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Authenticate with GitHub
    Auth {
        #[command(subcommand)]
        action: AuthCommands,
    },
    
    /// List repositories
    #[command(name = "ls")]
    Ls {
        /// Filter by owner (username or org)
        #[arg(short, long)]
        owner: Option<String>,
        
        /// Show only private repos
        #[arg(long)]
        private: bool,
        
        /// Show only public repos
        #[arg(long)]
        public: bool,

        /// Show all repositories (including archived)
        #[arg(short, long)]
        all: bool,
    },

    /// Manage artifacts
    Artifacts {
        #[command(subcommand)]
        action: ArtifactCommands,
    },

    /// Show Action storage usage
    Df,
    
    /// Launch TUI mode
    Tui,
}

#[derive(Subcommand)]
pub enum AuthCommands {
    /// Login with Personal Access Token
    Login {
        /// GitHub Personal Access Token
        #[arg(short, long, env = "GITHUB_TOKEN")]
        token: Option<String>,
    },
    
    /// Logout and remove stored credentials
    Logout,
    
    /// Show current authentication status
    Status,
}

#[derive(Subcommand)]
pub enum ArtifactCommands {
    /// List artifacts
    List {
        /// Filter by owner
        #[arg(short, long)]
        owner: Option<String>,
        /// Filter by repository
        #[arg(short, long)]
        repo: Option<String>,
    },
    /// Delete artifacts
    Delete {
        /// Artifact ID
        #[arg(long)]
        id: Option<u64>,
        /// Delete artifacts older than X days
        #[arg(long)]
        older_than: Option<i64>,
        /// Filter by repository
        #[arg(short, long)]
        repo: Option<String>,
        /// Filter by owner
        #[arg(short, long)]
        owner: Option<String>,
    },
}
