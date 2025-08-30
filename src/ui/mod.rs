pub mod components;
pub mod layout;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph, Tabs},
    Frame,
};

use crate::app::{AppMode, AppState};

pub fn render_ui(f: &mut Frame, app: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Main content
            Constraint::Length(3), // Footer
        ])
        .split(f.size());

    render_header(f, chunks[0], app);
    render_main_content(f, chunks[1], app);
    render_footer(f, chunks[2], app);
}

fn render_header(f: &mut Frame, area: Rect, app: &AppState) {
    let titles = vec!["Namespaces", "Pods", "Services", "Help"];
    let index = match app.mode {
        AppMode::NamespaceList => 0,
        AppMode::PodList => 1,
        AppMode::ServiceList => 2,
        AppMode::Help => 3,
        AppMode::Logs | AppMode::Describe => 1, // Stay on Pods tab when viewing logs or description
    };

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("Kube TUI"))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .select(index);

    f.render_widget(tabs, area);
}

fn render_main_content(f: &mut Frame, area: Rect, app: &AppState) {
    match app.mode {
        AppMode::NamespaceList => components::namespace_list::render(f, area, app),
        AppMode::PodList => components::pod_list::render(f, area, app),
        AppMode::ServiceList => components::service_list::render(f, area, app),
        AppMode::Logs => components::logs::render(f, area, app),
        AppMode::Describe => components::describe::render(f, area, app),
        AppMode::Help => components::help::render(f, area, app),
    }
}

fn render_footer(f: &mut Frame, area: Rect, app: &AppState) {
    let help_text = match app.mode {
        AppMode::NamespaceList => "↑/↓ Navigate • Enter Select • Tab Switch • q Quit • ? Help",
        AppMode::PodList => "↑/↓ Navigate • l Logs • d Describe • e Exec • Del Delete • Tab Switch • q Quit",
        AppMode::ServiceList => "↑/↓ Navigate • d Describe • Tab Switch • q Quit",
        AppMode::Logs => "Esc Back • q Quit",
        AppMode::Describe => "Esc Back • q Quit",
        AppMode::Help => "Esc Back • q Quit",
    };

    let footer = Paragraph::new(help_text)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::Gray));

    f.render_widget(footer, area);
}