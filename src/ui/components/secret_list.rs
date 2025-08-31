use ratatui::{
    layout::{Rect, Constraint},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Row, Table, Cell},
    Frame,
};

use crate::app::AppState;

pub fn render(f: &mut Frame, area: Rect, app: &AppState) {
    if app.secrets.is_empty() {
        let no_secrets = ratatui::widgets::Paragraph::new("No secrets found in this namespace")
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!("Secrets in namespace: {}", app.current_namespace))
            )
            .style(Style::default().fg(Color::Gray));
        
        f.render_widget(no_secrets, area);
        return;
    }

    let rows: Vec<Row> = app
        .secrets
        .iter()
        .enumerate()
        .map(|(i, secret)| {
            let style = if i == app.selected_secret_index {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            Row::new(vec![
                Cell::from(secret.name.clone()),
                Cell::from(secret.type_.clone()),
                Cell::from(secret.data_count.to_string()),
                Cell::from(secret.age.clone()),
            ]).style(style)
        })
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Percentage(40),
            Constraint::Percentage(30),
            Constraint::Percentage(15),
            Constraint::Percentage(15),
        ]
    )
        .header(
            Row::new(vec!["Name", "Type", "Data", "Age"])
                .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("Secrets in namespace: {} ({})", app.current_namespace, app.secrets.len()))
        )
        .row_highlight_style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        );

    f.render_stateful_widget(table, area, &mut create_table_state(app.selected_secret_index));
}

fn create_table_state(selected: usize) -> ratatui::widgets::TableState {
    let mut state = ratatui::widgets::TableState::default();
    state.select(Some(selected));
    state
}