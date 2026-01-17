use crate::presenter::tui::app::{App, View, InputMode};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

pub fn render(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Min(0),     // Main content
            Constraint::Length(3),  // Status bar
        ])
        .split(f.area());

    render_header(f, chunks[0]);
    
    match app.current_view {
        View::RepoList => render_repo_list(f, chunks[1], app),
        View::RepoDetail => render_repo_detail(f, chunks[1], app),
        View::ArtifactList => render_artifact_list(f, chunks[1], app),
        View::AuthPrompt => render_auth_prompt(f, chunks[1]),
    }
    
    render_status_bar(f, chunks[2], app);
}

fn render_header(f: &mut Frame, area: Rect) {
    let title = Paragraph::new("ghr")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, area);
}

fn render_auth_prompt(f: &mut Frame, area: Rect) {
    let msg = vec![
        Line::from("Authentication Required").style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
        Line::from(""),
        Line::from("Please login using the CLI first:"),
        Line::from("  ghr auth login").style(Style::default().fg(Color::Cyan)),
        Line::from(""),
        Line::from("Press 'q' to quit."),
    ];
    let paragraph = Paragraph::new(msg)
        .block(Block::default().borders(Borders::ALL).title("Authentication"))
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}

fn render_repo_list(f: &mut Frame, area: Rect, app: &App) {
    let constraints = if !app.filter_text.is_empty() || app.input_mode == InputMode::Editing {
        vec![Constraint::Min(0), Constraint::Length(3)]
    } else {
        vec![Constraint::Min(0)]
    };

    let list_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(area);

    if app.loading {
        let loading = Paragraph::new("Loading repositories...")
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL).title("Repositories"));
        f.render_widget(loading, list_chunks[0]);
        return;
    }

    if let Some(ref error) = app.error_message {
        let error_widget = Paragraph::new(error.as_str())
            .style(Style::default().fg(Color::Red))
            .block(Block::default().borders(Borders::ALL).title("Error"));
        f.render_widget(error_widget, list_chunks[0]);
        return;
    }

    let repos = app.filtered_repos();
    let items: Vec<ListItem> = repos
        .iter()
        .enumerate()
        .map(|(i, repo)| {
            let style = if i == app.selected_index {
                Style::default()
                    .bg(Color::DarkGray)
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            let visibility = if repo.private { "🔒" } else { "📂" };
            let stars = format!("⭐ {}", repo.stargazers_count);
            let lang = repo.language.as_deref().unwrap_or("Unknown");

            let content = Line::from(vec![
                Span::raw(visibility),
                Span::raw(" "),
                Span::styled(format!("{:<30}", repo.name), style),
                Span::raw(" "),
                Span::styled(format!("[{:<10}]", lang), Style::default().fg(Color::Blue)),
                Span::raw(" "),
                Span::styled(stars, Style::default().fg(Color::Yellow)),
            ]);

            ListItem::new(content).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("Repositories ({}/{})", repos.len(), app.repos.len())),
        );

    f.render_widget(list, list_chunks[0]);

    if !app.filter_text.is_empty() || app.input_mode == InputMode::Editing {
        let filter_style = if app.input_mode == InputMode::Editing {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };
        
        let filter_bar = Paragraph::new(format!(" Filter: {}", app.filter_text))
            .style(filter_style)
            .block(Block::default().borders(Borders::ALL).title("Search"));
        f.render_widget(filter_bar, list_chunks[1]);
    }
}

fn render_repo_detail(f: &mut Frame, area: Rect, app: &App) {
    if let Some(repo) = app.selected_repo() {
        let details = format!(
            "Full Name: {}\n\
             Owner: {}\n\
             Visibility: {}\n\
             Language: {}\n\
             Stars: ⭐ {}\n\
             Forks: 🍴 {}\n\
             Issues: 📋 {}\n\
             Default Branch: {}\n\
             Created: {}\n\
             Updated: {}\n\
             \n\
             Description:\n\
             {}\n\
             \n\
             URL: {}",
            repo.full_name,
            repo.owner,
            if repo.private { "Private 🔒" } else { "Public 📂" },
            repo.language.as_deref().unwrap_or("Unknown"),
            repo.stargazers_count,
            repo.forks_count,
            repo.open_issues_count,
            repo.default_branch,
            repo.created_at,
            repo.updated_at,
            repo.description.as_deref().unwrap_or("No description"),
            repo.html_url
        );

        let paragraph = Paragraph::new(details)
            .block(Block::default().borders(Borders::ALL).title("Repository Details"))
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, area);
    }
}

fn render_artifact_list(f: &mut Frame, area: Rect, app: &App) {
    if app.loading {
        let loading = Paragraph::new("Loading artifacts...")
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL).title("Artifacts"));
        f.render_widget(loading, area);
        return;
    }

    if let Some(ref error) = app.error_message {
        let error_widget = Paragraph::new(error.as_str())
            .style(Style::default().fg(Color::Red))
            .block(Block::default().borders(Borders::ALL).title("Error"));
        f.render_widget(error_widget, area);
        return;
    }

    let items: Vec<ListItem> = app.artifacts
        .iter()
        .enumerate()
        .map(|(i, a)| {
            let style = if i == app.selected_index {
                Style::default()
                    .bg(Color::DarkGray)
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            let size = format!("{:.2} MB", a.size_in_bytes as f64 / 1_048_576.0);
            let content = Line::from(vec![
                Span::styled(format!("{:<30}", a.name), style),
                Span::raw(" "),
                Span::styled(format!("[{:<10}]", size), Style::default().fg(Color::Blue)),
                Span::raw(" "),
                Span::styled(a.created_at.to_string(), Style::default().fg(Color::Gray)),
            ]);

            ListItem::new(content).style(style)
        })
        .collect();

    let title = if let Some(repo) = app.selected_repo() {
        format!("Artifacts for {} ({})", repo.name, app.artifacts.len())
    } else {
        format!("Artifacts ({})", app.artifacts.len())
    };

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(title));

    f.render_widget(list, area);
}

fn render_status_bar(f: &mut Frame, area: Rect, app: &App) {
    let help = match app.current_view {
        View::AuthPrompt => "q: Quit",
        View::RepoList => if app.input_mode == InputMode::Normal {
            "↑/↓: Navigate | Enter: Details | a: Artifacts | /: Filter | r: Refresh | q: Quit"
        } else {
            "Type to filter | Enter/Esc: Stop filtering"
        },
        View::RepoDetail => "Esc: Back | a: Artifacts | q: Quit",
        View::ArtifactList => "↑/↓: Navigate | Esc: Back | r: Refresh | q: Quit",
    };

    let status = Paragraph::new(help)
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(status, area);
}
