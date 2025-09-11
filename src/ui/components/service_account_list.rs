use ratatui::{
    layout::{Rect, Constraint},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Row, Table, Cell},
    Frame,
};

use crate::app::AppState;

pub fn render(f: &mut Frame, area: Rect, app: &AppState) {
    if app.service_accounts.is_empty() {
        let no_service_accounts = ratatui::widgets::Paragraph::new("No service accounts found or loading...")
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Service Accounts")
            )
            .style(Style::default().fg(Color::Gray));
        
        f.render_widget(no_service_accounts, area);
        return;
    }

    let rows: Vec<Row> = app
        .service_accounts
        .iter()
        .enumerate()
        .map(|(i, service_account)| {
            let style = if i == app.selected_service_account_index {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            Row::new(vec![
                Cell::from(service_account.name.clone()),
                Cell::from(service_account.namespace.clone()),
                Cell::from(service_account.age.clone()),
            ]).style(style)
        })
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Percentage(40),
            Constraint::Percentage(40),
            Constraint::Percentage(20),
        ]
    )
        .header(
            Row::new(vec!["Name", "Namespace", "Age"])
                .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("Service Accounts ({})", app.service_accounts.len()))
        )
        .row_highlight_style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        );

    f.render_stateful_widget(table, area, &mut create_table_state(app.selected_service_account_index));
}

fn create_table_state(selected: usize) -> ratatui::widgets::TableState {
    let mut state = ratatui::widgets::TableState::default();
    state.select(Some(selected));
    state
}