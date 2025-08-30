use ratatui::{
    layout::{Constraint, Layout, Direction, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph, Row, Table, Cell},
    Frame,
};

use crate::app::AppState;

pub fn render(f: &mut Frame, area: Rect, app: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)])
        .split(area);

    if app.pods.is_empty() {
        let no_pods = Paragraph::new("No pods found in this namespace")
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!("Pods in namespace: {}", app.current_namespace))
            )
            .style(Style::default().fg(Color::Gray));
        
        f.render_widget(no_pods, chunks[0]);
        return;
    }

    // Create table rows
    let rows: Vec<Row> = app
        .pods
        .iter()
        .enumerate()
        .map(|(i, pod)| {
            let style = if i == app.selected_pod_index {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            let status_color = match pod.status.phase.as_str() {
                "Running" => Color::Green,
                "Pending" => Color::Yellow,
                "Failed" | "Error" => Color::Red,
                "Succeeded" => Color::Blue,
                _ => Color::Gray,
            };

            Row::new(vec![
                Cell::from(pod.name.clone()),
                Cell::from(pod.ready.clone()),
                Cell::from(pod.status.phase.clone()).style(Style::default().fg(status_color)),
                Cell::from(pod.restarts.to_string()),
                Cell::from(pod.age.clone()),
                Cell::from(pod.node.clone().unwrap_or_else(|| "<none>".to_string())),
            ]).style(style)
        })
        .collect();

    let table = Table::new(rows)
        .header(
            Row::new(vec!["Name", "Ready", "Status", "Restarts", "Age", "Node"])
                .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("Pods in namespace: {} ({})", app.current_namespace, app.pods.len()))
        )
        .widths(&[
            Constraint::Percentage(30),
            Constraint::Percentage(10),
            Constraint::Percentage(15),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(25),
        ])
        .highlight_style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        );

    f.render_stateful_widget(table, chunks[0], &mut create_table_state(app.selected_pod_index));
}

fn create_table_state(selected: usize) -> ratatui::widgets::TableState {
    let mut state = ratatui::widgets::TableState::default();
    state.select(Some(selected));
    state
}