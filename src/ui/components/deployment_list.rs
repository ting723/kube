use ratatui::{
    layout::{Rect, Constraint},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Row, Table, Cell},
    Frame,
};

use crate::app::AppState;

pub fn render(f: &mut Frame, area: Rect, app: &AppState) {
    if app.deployments.is_empty() {
        let no_deployments = ratatui::widgets::Paragraph::new("No deployments found or loading...")
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Deployments")
            )
            .style(Style::default().fg(Color::Gray));
        
        f.render_widget(no_deployments, area);
        return;
    }

    let rows: Vec<Row> = app
        .deployments
        .iter()
        .enumerate()
        .map(|(i, deployment)| {
            let style = if i == app.selected_deployment_index {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            Row::new(vec![
                Cell::from(deployment.name.clone()),
                Cell::from(deployment.ready.clone()),
                Cell::from(deployment.up_to_date.to_string()),
                Cell::from(deployment.available.to_string()),
                Cell::from(deployment.age.clone()),
            ]).style(style)
        })
        .collect();

    let table = Table::new(rows)
        .header(
            Row::new(vec!["Name", "Ready", "Up-to-date", "Available", "Age"])
                .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("Deployments ({})", app.deployments.len()))
        )
        .widths(&[
            Constraint::Percentage(35),
            Constraint::Percentage(15),
            Constraint::Percentage(15),
            Constraint::Percentage(15),
            Constraint::Percentage(20),
        ])
        .highlight_style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        );

    f.render_stateful_widget(table, area, &mut create_table_state(app.selected_deployment_index));
}

fn create_table_state(selected: usize) -> ratatui::widgets::TableState {
    let mut state = ratatui::widgets::TableState::default();
    state.select(Some(selected));
    state
}