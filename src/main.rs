mod domain;
mod infra;
mod app;
mod presenter;

use clap::Parser;
pub use presenter::cli::{Cli, Commands, AuthCommands, ArtifactCommands};
use domain::error::Result;
use tracing_subscriber::{fmt, EnvFilter};

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let cli = Cli::parse();

    match cli.command {
        Commands::Tui => {
            fmt()
                .with_env_filter(EnvFilter::new("gh_tui=info"))
                .with_writer(std::io::stderr)
                .init();
            presenter::tui::run().await?;
        }
        _ => {
            fmt()
                .with_env_filter(EnvFilter::new("gh_tui=info"))
                .init();
            
            match cli.command {
                Commands::Auth { action } => {
                    match action {
                        AuthCommands::Login { token } => {
                            presenter::cli::auth::login(token).await?;
                        }
                        AuthCommands::Logout => {
                            presenter::cli::auth::logout()?;
                        }
                        AuthCommands::Status => {
                            presenter::cli::auth::status().await?;
                        }
                    }
                }
                Commands::Ls { owner, private, public, all } => {
                    presenter::cli::repos::list_repos(owner, private, public, all).await?;
                }
                Commands::Artifacts { action } => {
                    presenter::cli::artifacts::handle_artifacts(action).await?;
                }
                Commands::Df => {
                    presenter::cli::df::handle_df().await?;
                }
                Commands::Tui => unreachable!(),
            }
        }
    }

    Ok(())
}
