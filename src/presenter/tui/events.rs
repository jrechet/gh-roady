use crate::presenter::tui::app::{App, View, InputMode};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::time::Duration;

pub async fn handle_events(app: &mut App) -> crate::domain::error::Result<()> {
    if event::poll(Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            handle_key_event(app, key).await?;
        }
    }
    Ok(())
}

async fn handle_key_event(app: &mut App, key: KeyEvent) -> crate::domain::error::Result<()> {
    match app.input_mode {
        InputMode::Normal => match app.current_view {
            View::RepoList => handle_repo_list_keys(app, key).await?,
            View::RepoDetail => handle_repo_detail_keys(app, key).await?,
            View::ArtifactList => handle_artifact_list_keys(app, key).await?,
            View::AuthPrompt => handle_auth_prompt_keys(app, key),
        },
        InputMode::Editing => handle_editing_keys(app, key),
    }
    Ok(())
}

async fn handle_repo_list_keys(app: &mut App, key: KeyEvent) -> crate::domain::error::Result<()> {
    match key.code {
        KeyCode::Char('q') | KeyCode::Char('Q') => {
            app.should_quit = true;
        }
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.should_quit = true;
        }
        KeyCode::Down | KeyCode::Char('j') => {
            app.next();
        }
        KeyCode::Up | KeyCode::Char('k') => {
            app.previous();
        }
        KeyCode::Enter => {
            if app.selected_repo().is_some() {
                app.current_view = View::RepoDetail;
            }
        }
        KeyCode::Char('r') | KeyCode::Char('R') => {
            app.load_repos().await?;
        }
        KeyCode::Char('/') => {
            app.input_mode = InputMode::Editing;
        }
        KeyCode::Char('a') | KeyCode::Char('A') => {
            if let Some(repo) = app.selected_repo() {
                let owner = repo.owner.clone();
                let name = repo.name.clone();
                app.load_artifacts(&owner, &name).await?;
                app.current_view = View::ArtifactList;
            }
        }
        _ => {}
    }
    Ok(())
}

async fn handle_repo_detail_keys(app: &mut App, key: KeyEvent) -> crate::domain::error::Result<()> {
    match key.code {
        KeyCode::Char('q') | KeyCode::Char('Q') => {
            app.should_quit = true;
        }
        KeyCode::Esc => {
            app.current_view = View::RepoList;
        }
        KeyCode::Char('a') | KeyCode::Char('A') => {
            if let Some(repo) = app.selected_repo() {
                let owner = repo.owner.clone();
                let name = repo.name.clone();
                app.load_artifacts(&owner, &name).await?;
                app.current_view = View::ArtifactList;
            }
        }
        _ => {}
    }
    Ok(())
}

async fn handle_artifact_list_keys(app: &mut App, key: KeyEvent) -> crate::domain::error::Result<()> {
    match key.code {
        KeyCode::Char('q') | KeyCode::Char('Q') => {
            app.should_quit = true;
        }
        KeyCode::Esc => {
            app.current_view = View::RepoList;
        }
        KeyCode::Down | KeyCode::Char('j') => {
            app.next();
        }
        KeyCode::Up | KeyCode::Char('k') => {
            app.previous();
        }
        KeyCode::Char('r') | KeyCode::Char('R') => {
            if let Some(repo) = app.selected_repo() {
                let owner = repo.owner.clone();
                let name = repo.name.clone();
                app.load_artifacts(&owner, &name).await?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn handle_auth_prompt_keys(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('q') | KeyCode::Char('Q') => {
            app.should_quit = true;
        }
        _ => {}
    }
}

fn handle_editing_keys(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Enter => {
            app.input_mode = InputMode::Normal;
        }
        KeyCode::Esc => {
            app.input_mode = InputMode::Normal;
        }
        KeyCode::Char(c) => {
            app.filter_text.push(c);
            app.selected_index = 0;
        }
        KeyCode::Backspace => {
            app.filter_text.pop();
            app.selected_index = 0;
        }
        _ => {}
    }
}
