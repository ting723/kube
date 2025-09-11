use ratatui::{
    layout::{Rect, Constraint},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Row, Table, Cell},
    Frame,
};

use crate::app::AppState;

pub fn render(f: &mut Frame, area: Rect, app: &AppState) {
    if app.nodes.is_empty() {
        let no_nodes = ratatui::widgets::Paragraph::new("No nodes found or loading...")
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Nodes")
            )
            .style(Style::default().fg(Color::Gray));
        
        f.render_widget(no_nodes, area);
        return;
    }

    let rows: Vec<Row> = app
        .nodes
        .iter()
        .enumerate()
        .map(|(i, node)| {
            let style = if i == app.selected_node_index {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            let status_color = match node.status.as_str() {
                "Ready" => Color::Green,
                "NotReady" => Color::Red,
                _ => Color::Gray,
            };

            let roles_str = node.roles.join(",");

            Row::new(vec![
                Cell::from(node.name.clone()),
                Cell::from(node.status.clone()).style(Style::default().fg(status_color)),
                Cell::from(roles_str),
                Cell::from(node.age.clone()),
                Cell::from(node.version.clone()),
                Cell::from(node.internal_ip.clone().unwrap_or_else(|| "<none>".to_string())),
                Cell::from(node.external_ip.clone().unwrap_or_else(|| "<none>".to_string())),
                Cell::from(node.os_image.clone().unwrap_or_else(|| "<none>".to_string())),
                Cell::from(node.kernel_version.clone().unwrap_or_else(|| "<none>".to_string())),
                Cell::from(node.container_runtime.clone().unwrap_or_else(|| "<none>".to_string())),
            ]).style(style)
        })
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Percentage(15), // Name
            Constraint::Percentage(10), // Status
            Constraint::Percentage(10), // Roles
            Constraint::Percentage(5),  // Age
            Constraint::Percentage(10), // Version
            Constraint::Percentage(10), // Internal-IP
            Constraint::Percentage(10), // External-IP
            Constraint::Percentage(10), // OS Image
            Constraint::Percentage(10), // Kernel Version
            Constraint::Percentage(10), // Container Runtime
        ]
    )
        .header(
            Row::new(vec![
                "Name", 
                "Status", 
                "Roles", 
                "Age", 
                "Version", 
                "Internal-IP", 
                "External-IP", 
                "OS Image", 
                "Kernel", 
                "Runtime"
            ])
                .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("Nodes ({})", app.nodes.len()))
        )
        .row_highlight_style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        );

    f.render_stateful_widget(table, area, &mut create_table_state(app.selected_node_index));
}

fn create_table_state(selected: usize) -> ratatui::widgets::TableState {
    let mut state = ratatui::widgets::TableState::default();
    state.select(Some(selected));
    state
}