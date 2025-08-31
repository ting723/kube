use ratatui::{
    layout::{Rect, Constraint},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Row, Table, Cell},
    Frame,
};

use crate::app::AppState;

pub fn render(f: &mut Frame, area: Rect, app: &AppState) {
    if app.services.is_empty() {
        let no_services = ratatui::widgets::Paragraph::new("No services found in this namespace")
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!("Services in namespace: {}", app.current_namespace))
            )
            .style(Style::default().fg(Color::Gray));
        
        f.render_widget(no_services, area);
        return;
    }

    let rows: Vec<Row> = app
        .services
        .iter()
        .enumerate()
        .map(|(i, service)| {
            let style = if i == app.selected_service_index {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            let ports_str = service
                .ports
                .iter()
                .map(|port| {
                    if let Some(name) = &port.name {
                        format!("{}:{}/{}", name, port.port, port.protocol)
                    } else {
                        format!("{}/{}", port.port, port.protocol)
                    }
                })
                .collect::<Vec<_>>()
                .join(",");

            Row::new(vec![
                Cell::from(service.name.clone()),
                Cell::from(service.type_.clone()),
                Cell::from(service.cluster_ip.clone()),
                Cell::from(service.external_ip.clone().unwrap_or_else(|| "<none>".to_string())),
                Cell::from(ports_str),
                Cell::from(service.age.clone()),
            ]).style(style)
        })
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Percentage(25),
            Constraint::Percentage(15),
            Constraint::Percentage(15),
            Constraint::Percentage(15),
            Constraint::Percentage(20),
            Constraint::Percentage(10),
        ]
    )
        .header(
            Row::new(vec!["Name", "Type", "Cluster-IP", "External-IP", "Ports", "Age"])
                .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("Services in namespace: {} ({})", app.current_namespace, app.services.len()))
        )
        .row_highlight_style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        );

    f.render_stateful_widget(table, area, &mut create_table_state(app.selected_service_index));
}

fn create_table_state(selected: usize) -> ratatui::widgets::TableState {
    let mut state = ratatui::widgets::TableState::default();
    state.select(Some(selected));
    state
}