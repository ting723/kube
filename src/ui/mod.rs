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
            Constraint::Length(1), // Command line
        ])
        .split(f.size());

    render_header(f, chunks[0], app);
    render_main_content(f, chunks[1], app);
    render_footer(f, chunks[2], app);
    render_command_line(f, chunks[3], app);
}

fn render_header(f: &mut Frame, area: Rect, app: &AppState) {
    let titles = vec!["Namespaces", "Pods", "Services", "Nodes", "Deployments", "DaemonSets", "ConfigMaps", "Secrets", "PVCs", "PVs", "Help"];
    let index = match app.mode {
        AppMode::NamespaceList => 0,
        AppMode::PodList => 1,
        AppMode::ServiceList => 2,
        AppMode::NodeList => 3,
        AppMode::DeploymentList => 4,
        AppMode::DaemonSetList => 5,
        AppMode::ConfigMapList => 6,
        AppMode::SecretList => 7,
        AppMode::PVCList => 8,
        AppMode::PVList => 9,
        AppMode::Help => 10,
        AppMode::Logs | AppMode::Describe => {
            // 根据之前的模式显示正确的Tab高亮
            match app.previous_mode {
                AppMode::PodList => 1,
                AppMode::ServiceList => 2,
                AppMode::NodeList => 3,
                AppMode::DeploymentList => 4,
                AppMode::DaemonSetList => 5,
                AppMode::ConfigMapList => 6,
                AppMode::SecretList => 7,
                AppMode::PVCList => 8,
                AppMode::PVList => 9,
                _ => 1,
            }
        }
        AppMode::Search | AppMode::Confirm => match app.get_previous_mode() {
            AppMode::PodList => 1,
            AppMode::ServiceList => 2,
            AppMode::NodeList => 3,
            AppMode::DeploymentList => 4,
            AppMode::DaemonSetList => 5,
            AppMode::ConfigMapList => 6,
            AppMode::SecretList => 7,
            AppMode::PVCList => 8,
            AppMode::PVList => 9,
            _ => 0,
        },
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
        AppMode::NodeList => components::node_list::render(f, area, app),
        AppMode::DeploymentList => components::deployment_list::render(f, area, app),
        AppMode::DaemonSetList => components::daemonset_list::render(f, area, app),
        AppMode::PVCList => components::pvc_list::render(f, area, app),
        AppMode::PVList => components::pv_list::render(f, area, app),
        AppMode::ConfigMapList => components::configmap_list::render(f, area, app),
        AppMode::SecretList => components::secret_list::render(f, area, app),
        AppMode::Logs => components::logs::render(f, area, app),
        AppMode::Describe => components::describe::render(f, area, app),
        AppMode::Search => components::search::render(f, area, app),
        AppMode::Confirm => components::confirm::render(f, area, app),
        AppMode::Help => components::help::render(f, area, app),
    }
}

fn render_footer(f: &mut Frame, area: Rect, app: &AppState) {
    let help_text = match app.mode {
        AppMode::NamespaceList => "j/k ↑/↓ Navigate • Enter Select • h/l ←/→ Switch • q Quit • ? Help",
        AppMode::PodList => "j/k Navigate • Space Describe • L Logs • D Delete • E Exec • / Search • Tab Switch • q Quit",
        AppMode::ServiceList => "j/k Navigate • Space Describe • D Delete • / Search • Tab Switch • q Quit",
        AppMode::NodeList => "j/k Navigate • Space Describe • / Search • Tab Switch • q Quit",
        AppMode::DeploymentList => "j/k Navigate • Space Describe • / Search • Tab Switch • q Quit",
        AppMode::DaemonSetList => "j/k Navigate • Space Describe • / Search • Tab Switch • q Quit",
        AppMode::PVCList => "j/k Navigate • Space Describe • / Search • Tab Switch • q Quit",
        AppMode::PVList => "j/k Navigate • Space Describe • / Search • Tab Switch • q Quit",
        AppMode::ConfigMapList => "j/k Navigate • Space Describe • D Delete • / Search • Tab Switch • q Quit",
        AppMode::SecretList => "j/k Navigate • Space Describe • D Delete • / Search • Tab Switch • q Quit",
        AppMode::Logs => "J/K Scroll • PgUp/PgDn Page • A Toggle Auto-scroll • R Toggle Auto-refresh • Esc Back • q Quit",
        AppMode::Describe => "J/K Scroll • PgUp/PgDn Page • Esc Back • q Quit",
        AppMode::Search => "Type to search • Enter Confirm • Esc Cancel",
        AppMode::Confirm => "y/Y Yes • n/N/Esc No",
        AppMode::Help => "Esc Back • q Quit",
    };

    let footer = Paragraph::new(help_text)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::Gray));

    f.render_widget(footer, area);
}

fn render_command_line(f: &mut Frame, area: Rect, app: &AppState) {
    let command_text = if app.current_command.is_empty() {
        "Ready".to_string()
    } else {
        format!("Executing: {}", app.current_command)
    };

    let command_line = Paragraph::new(command_text)
        .style(Style::default().fg(Color::Cyan));

    f.render_widget(command_line, area);
}