use crate::presenter::tui::app::{App, View, InputMode, MenuItem};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::time::Duration;

pub async fn handle_events(app: &mut App) -> crate::domain::error::Result<()> {
    // Process pending loads
    if let Some(item) = app.pending_load {
        app.pending_load = None; // clear it
        match item {
            MenuItem::Repositories => app.load_repos().await?,
            MenuItem::StorageManager => app.load_storage().await?,
            MenuItem::Quit => {}
        }
        return Ok(());
    }

    // Clear success message after any key press
    if app.success_message.is_some() {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(_) = event::read()? {
                app.success_message = None;
            }
        }
        return Ok(());
    }

    if event::poll(Duration::from_millis(10))? { // Reduced poll time for smoother animation
        if let Event::Key(key) = event::read()? {
            handle_key_event(app, key).await?;
        }
    }
    Ok(())
}

async fn handle_key_event(app: &mut App, key: KeyEvent) -> crate::domain::error::Result<()> {
    // Global quit
    if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
        app.should_quit = true;
        return Ok(());
    }

    match app.input_mode {
        InputMode::Normal => match app.current_view {
            View::MainMenu => handle_main_menu_keys(app, key).await?,
            View::RepoList => handle_repo_list_keys(app, key).await?,
            View::RepoDetail => handle_repo_detail_keys(app, key).await?,
            View::ArtifactList => handle_artifact_list_keys(app, key).await?,
            View::StorageManager => handle_storage_manager_keys(app, key).await?,
            View::AuthPrompt => handle_auth_prompt_keys(app, key),
        },
        InputMode::Editing => handle_editing_keys(app, key),
    }
    Ok(())
}

async fn handle_main_menu_keys(app: &mut App, key: KeyEvent) -> crate::domain::error::Result<()> {
    match key.code {
        KeyCode::Char('q') | KeyCode::Char('Q') => {
            app.should_quit = true;
        }
        KeyCode::Down | KeyCode::Char('j') => {
            app.next_menu();
        }
        KeyCode::Up | KeyCode::Char('k') => {
            app.previous_menu();
        }
        KeyCode::Enter => {
            match app.current_menu_item() {
                MenuItem::Repositories => {
                    app.current_view = View::RepoList;
                    app.loading = true;
                    app.loading_message = "Loading repositories...".into();
                    app.selected_index = 0;
                    app.pending_load = Some(MenuItem::Repositories);
                }
                MenuItem::StorageManager => {
                    app.current_view = View::StorageManager;
                    app.loading = true;
                    app.loading_message = "Scanning storage usage...".into();
                    app.selected_index = 0;
                    app.pending_load = Some(MenuItem::StorageManager);
                }
                MenuItem::Quit => {
                    app.should_quit = true;
                }
            }
        }
        _ => {}
    }
    Ok(())
}

async fn handle_repo_list_keys(app: &mut App, key: KeyEvent) -> crate::domain::error::Result<()> {
    match key.code {
        KeyCode::Char('q') | KeyCode::Char('Q') => {
            app.should_quit = true;
        }
        KeyCode::Esc => {
            app.current_view = View::MainMenu;
            app.filter_text.clear();
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
            app.current_view = View::MainMenu;
        }
        KeyCode::Down | KeyCode::Char('j') => {
            app.next();
        }
        KeyCode::Up | KeyCode::Char('k') => {
            app.previous();
        }
        KeyCode::Char('r') | KeyCode::Char('R') => {
            app.load_all_artifacts().await?;
        }
        KeyCode::Char('d') | KeyCode::Char('D') => {
            // Delete selected artifact
            if let Some(artifact) = app.selected_artifact() {
                if let Some(ref client) = app.client {
                    use crate::domain::github::GithubRepository;
                    let _ = client.delete_artifact(&artifact.repository_owner, &artifact.repository_name, artifact.id).await;
                    app.success_message = Some(format!("✓ Deleted artifact: {}", artifact.name));
                    app.load_all_artifacts().await?;
                }
            }
        }
        _ => {}
    }
    Ok(())
}

async fn handle_storage_manager_keys(app: &mut App, key: KeyEvent) -> crate::domain::error::Result<()> {
    match key.code {
        KeyCode::Char('q') | KeyCode::Char('Q') => {
            app.should_quit = true;
        }
        KeyCode::Esc => {
            app.current_view = View::MainMenu;
            app.selected_storage_items.clear();
        }
        KeyCode::Down | KeyCode::Char('j') => {
            app.next();
        }
        KeyCode::Up | KeyCode::Char('k') => {
            app.previous();
        }
        KeyCode::Char(' ') => {
            app.toggle_storage_selection();
        }
        KeyCode::Char('r') | KeyCode::Char('R') => {
            app.load_storage().await?;
        }
        KeyCode::Char('d') | KeyCode::Char('D') => {
            if !app.selected_storage_items.is_empty() {
                app.delete_selected_storage_items().await?;
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
            app.filter_text.clear();
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
