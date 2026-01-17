use crate::presenter::tui::app::{App, View, InputMode, MenuItem};
use crate::domain::storage::StorageItemType;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph, Wrap},
    Frame,
};

const ASCII_LOGO: &str = r#"
   ██████╗ ██╗  ██╗      ██████╗  ██████╗  █████╗ ██████╗ ██╗   ██╗
  ██╔════╝ ██║  ██║      ██╔══██╗██╔═══██╗██╔══██╗██╔══██╗╚██╗ ██╔╝
  ██║  ███╗███████║█████╗██████╔╝██║   ██║███████║██║  ██║ ╚████╔╝ 
  ██║   ██║██╔══██║╚════╝██╔══██╗██║   ██║██╔══██║██║  ██║  ╚██╔╝  
  ╚██████╔╝██║  ██║      ██║  ██║╚██████╔╝██║  ██║██████╔╝   ██║   
   ╚═════╝ ╚═╝  ╚═╝      ╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═╝╚═════╝    ╚═╝   
"#;

pub fn render(f: &mut Frame, app: &App) {
    match app.current_view {
        View::MainMenu => render_main_menu(f, app),
        View::AuthPrompt => render_auth_prompt(f),
        _ => render_feature_view(f, app),
    }
}

fn render_main_menu(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(9),   // Logo
            Constraint::Min(0),      // Menu
            Constraint::Length(3),   // Status bar
        ])
        .split(f.area());

    render_logo(f, chunks[0]);
    render_menu(f, chunks[1], app);
    render_menu_status(f, chunks[2]);
}

fn render_logo(f: &mut Frame, area: Rect) {
    let logo = Paragraph::new(ASCII_LOGO)
        .style(Style::default().fg(Color::Cyan))
        .alignment(Alignment::Center);
    f.render_widget(logo, area);
}

fn render_menu(f: &mut Frame, area: Rect, app: &App) {
    let menu_items = MenuItem::all();
    
    let items: Vec<ListItem> = menu_items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let style = if i == app.menu_index {
                Style::default()
                    .bg(Color::Rgb(60, 60, 100))
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Gray)
            };

            let content = vec![
                Line::from(Span::styled(item.label(), style)),
                Line::from(Span::styled(
                    format!("    {}", item.description()),
                    Style::default().fg(Color::DarkGray),
                )),
                Line::from(""),
            ];

            ListItem::new(content)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray))
                .title(" Main Menu ")
                .title_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        );

    f.render_widget(list, area);
}

fn render_menu_status(f: &mut Frame, area: Rect) {
    let help = "↑/↓: Navigate | Enter: Select | q: Quit";
    let status = Paragraph::new(help)
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::DarkGray)));
    f.render_widget(status, area);
}

fn render_auth_prompt(f: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(9),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.area());

    render_logo(f, chunks[0]);

    let msg = vec![
        Line::from(""),
        Line::from(Span::styled("⚠  Authentication Required", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))),
        Line::from(""),
        Line::from("Please login using the CLI first:"),
        Line::from(""),
        Line::from(Span::styled("  ghr auth login --token <YOUR_TOKEN>", Style::default().fg(Color::Cyan))),
        Line::from(""),
        Line::from("Or set GITHUB_TOKEN environment variable."),
    ];
    let paragraph = Paragraph::new(msg)
        .block(Block::default().borders(Borders::ALL).title(" Authentication "))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, chunks[1]);

    let status = Paragraph::new("q: Quit")
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(status, chunks[2]);
}

fn render_feature_view(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),   // Header
            Constraint::Min(0),      // Content
            Constraint::Length(3),   // Status bar
        ])
        .split(f.area());

    render_header(f, chunks[0], app);
    
    match app.current_view {
        View::RepoList => render_repo_list(f, chunks[1], app),
        View::RepoDetail => render_repo_detail(f, chunks[1], app),
        View::ArtifactList => render_artifact_list(f, chunks[1], app),
        View::StorageManager => render_storage_manager(f, chunks[1], app),
        _ => {}
    }
    
    render_status_bar(f, chunks[2], app);
}

fn render_header(f: &mut Frame, area: Rect, app: &App) {
    let title = match app.current_view {
        View::RepoList => " 📚 Repositories ",
        View::RepoDetail => " 📖 Repository Details ",
        View::ArtifactList => " 📦 Artifacts ",
        View::StorageManager => " 💾 Storage Manager ",
        _ => " gh-roady ",
    };

    let header = Paragraph::new(title)
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::DarkGray)));
    f.render_widget(header, area);
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
        let loading = Paragraph::new(format!("⏳ {}", app.loading_message))
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(loading, list_chunks[0]);
        return;
    }

    if let Some(ref error) = app.error_message {
        let error_widget = Paragraph::new(format!("❌ {}", error))
            .style(Style::default().fg(Color::Red))
            .block(Block::default().borders(Borders::ALL).title(" Error "));
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
                    .bg(Color::Rgb(60, 60, 100))
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            let visibility = if repo.private { "🔒" } else { "📂" };
            let stars = format!("⭐ {}", repo.stargazers_count);
            let lang = repo.language.as_deref().unwrap_or("—");

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
                .border_style(Style::default().fg(Color::DarkGray))
                .title(format!(" {}/{} ", repos.len(), app.repos.len())),
        );

    f.render_widget(list, list_chunks[0]);

    if !app.filter_text.is_empty() || app.input_mode == InputMode::Editing {
        let filter_style = if app.input_mode == InputMode::Editing {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };
        
        let filter_bar = Paragraph::new(format!(" 🔍 {}", app.filter_text))
            .style(filter_style)
            .block(Block::default().borders(Borders::ALL).title(" Search "));
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
            .block(Block::default().borders(Borders::ALL))
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, area);
    }
}

fn render_artifact_list(f: &mut Frame, area: Rect, app: &App) {
    if app.loading {
        let loading = Paragraph::new(format!("⏳ {}", app.loading_message))
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(loading, area);
        return;
    }

    if let Some(ref error) = app.error_message {
        let error_widget = Paragraph::new(format!("❌ {}", error))
            .style(Style::default().fg(Color::Red))
            .block(Block::default().borders(Borders::ALL).title(" Error "));
        f.render_widget(error_widget, area);
        return;
    }

    if app.artifacts.is_empty() {
        let empty = Paragraph::new("No artifacts found.")
            .style(Style::default().fg(Color::DarkGray))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(empty, area);
        return;
    }

    let items: Vec<ListItem> = app.artifacts
        .iter()
        .enumerate()
        .map(|(i, a)| {
            let style = if i == app.selected_index {
                Style::default()
                    .bg(Color::Rgb(60, 60, 100))
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            let size = format!("{:.2} MB", a.size_in_bytes as f64 / 1_048_576.0);
            let content = Line::from(vec![
                Span::styled(format!("{:<30}", a.name), style),
                Span::raw(" "),
                Span::styled(format!("[{:>10}]", size), Style::default().fg(Color::Blue)),
                Span::raw(" "),
                Span::styled(format!("{}/{}", a.repository_owner, a.repository_name), Style::default().fg(Color::DarkGray)),
            ]);

            ListItem::new(content).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(format!(" {} artifacts ", app.artifacts.len())));

    f.render_widget(list, area);
}

fn render_storage_manager(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),  // Usage gauge
            Constraint::Min(0),     // Item list
        ])
        .split(area);

    // Storage gauge
    if let Some(ref report) = app.storage_report {
        let percentage = if report.max_allowed > 0 {
            (report.total_used as f64 / report.max_allowed as f64 * 100.0) as u16
        } else {
            0
        };
        let percentage = std::cmp::min(percentage, 100);

        let gauge_color = if percentage > 90 {
            Color::Red
        } else if percentage > 70 {
            Color::Yellow
        } else {
            Color::Green
        };

        let used_mb = report.total_used as f64 / 1_048_576.0;
        let max_mb = report.max_allowed as f64 / 1_048_576.0;
        let label = format!("{:.2} MB / {:.2} MB ({:.1}%)", used_mb, max_mb, percentage);

        let gauge = Gauge::default()
            .block(Block::default().borders(Borders::ALL).title(" Storage Usage "))
            .gauge_style(Style::default().fg(gauge_color))
            .percent(percentage)
            .label(label);

        f.render_widget(gauge, chunks[0]);
    } else {
        let loading = Paragraph::new("Loading storage info...")
            .block(Block::default().borders(Borders::ALL).title(" Storage Usage "));
        f.render_widget(loading, chunks[0]);
    }

    // Item list
    if app.loading {
        let loading = Paragraph::new(format!("⏳ {}", app.loading_message))
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(loading, chunks[1]);
        return;
    }

    if app.storage_items.is_empty() {
        let empty = Paragraph::new("No items found (>1KB). Your storage is clean!")
            .style(Style::default().fg(Color::Green))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(empty, chunks[1]);
        return;
    }

    let items: Vec<ListItem> = app.storage_items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let is_selected = app.selected_storage_items.contains(&i);
            let is_current = i == app.selected_index;
            
            let style = if is_current {
                Style::default()
                    .bg(Color::Rgb(60, 60, 100))
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD)
            } else if is_selected {
                Style::default().fg(Color::Red)
            } else {
                Style::default()
            };

            let checkbox = if is_selected { "[×]" } else { "[ ]" };
            let type_icon = match item.item_type {
                StorageItemType::Artifact => "📦",
                StorageItemType::Cache => "🗄️ ",
            };
            let size = format!("{:.2} MB", item.size_in_bytes as f64 / 1_048_576.0);

            let content = Line::from(vec![
                Span::styled(checkbox, if is_selected { Style::default().fg(Color::Red) } else { Style::default() }),
                Span::raw(" "),
                Span::raw(type_icon),
                Span::raw(" "),
                Span::styled(format!("{:<40}", item.name), style),
                Span::styled(format!("{:>10}", size), Style::default().fg(Color::Blue)),
                Span::raw(" "),
                Span::styled(format!("[{}/{}]", item.owner, item.repo), Style::default().fg(Color::DarkGray)),
            ]);

            ListItem::new(content)
        })
        .collect();

    let selected_count = app.selected_storage_items.len();
    let title = if selected_count > 0 {
        format!(" {} items | {} selected for deletion ", app.storage_items.len(), selected_count)
    } else {
        format!(" {} items ", app.storage_items.len())
    };

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(title));

    f.render_widget(list, chunks[1]);
}

fn render_status_bar(f: &mut Frame, area: Rect, app: &App) {
    let help = match app.current_view {
        View::MainMenu => "↑/↓: Navigate | Enter: Select | q: Quit",
        View::RepoList => if app.input_mode == InputMode::Normal {
            "↑/↓: Navigate | Enter: Details | a: Artifacts | /: Filter | r: Refresh | Esc: Menu | q: Quit"
        } else {
            "Type to filter | Enter/Esc: Stop"
        },
        View::RepoDetail => "a: Artifacts | Esc: Back | q: Quit",
        View::ArtifactList => "↑/↓: Navigate | d: Delete | r: Refresh | Esc: Back | q: Quit",
        View::StorageManager => "↑/↓: Navigate | Space: Toggle | d: Delete selected | r: Refresh | Esc: Menu | q: Quit",
        View::AuthPrompt => "q: Quit",
    };

    // Show success message if present
    let text = if let Some(ref msg) = app.success_message {
        msg.as_str()
    } else {
        help
    };

    let style = if app.success_message.is_some() {
        Style::default().fg(Color::Green)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let status = Paragraph::new(text)
        .style(style)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::DarkGray)));

    f.render_widget(status, area);
}
