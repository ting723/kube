use ratatui::{
    layout::{Rect, Constraint},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Row, Table, Cell},
    Frame,
};

use crate::app::AppState;

pub fn render(f: &mut Frame, area: Rect, app: &AppState) {
    if app.roles.is_empty() {
        let no_roles = ratatui::widgets::Paragraph::new("No roles found or loading...")
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Roles")
            )
            .style(Style::default().fg(Color::Gray));
        
        f.render_widget(no_roles, area);
        return;
    }

    let rows: Vec<Row> = app
        .roles
        .iter()
        .enumerate()
        .map(|(i, role)| {
            let style = if i == app.selected_role_index {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            Row::new(vec![
                Cell::from(role.name.clone()),
                Cell::from(role.namespace.clone()),
                Cell::from(role.age.clone()),
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
                .title(format!("Roles ({})", app.roles.len()))
        )
        .row_highlight_style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        );

    f.render_stateful_widget(table, area, &mut create_table_state(app.selected_role_index));
}

fn create_table_state(selected: usize) -> ratatui::widgets::TableState {
    let mut state = ratatui::widgets::TableState::default();
    state.select(Some(selected));
    state
}