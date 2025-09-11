use ratatui::{
    layout::{Rect, Constraint},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Row, Table, Cell},
    Frame,
};

use crate::app::AppState;

pub fn render(f: &mut Frame, area: Rect, app: &AppState) {
    if app.ingresses.is_empty() {
        let no_ingresses = ratatui::widgets::Paragraph::new("No ingresses found or loading...")
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Ingresses")
            )
            .style(Style::default().fg(Color::Gray));
        
        f.render_widget(no_ingresses, area);
        return;
    }

    let rows: Vec<Row> = app
        .ingresses
        .iter()
        .enumerate()
        .map(|(i, ingress)| {
            let style = if i == app.selected_ingress_index {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            let hosts_str = ingress.hosts.join(",");

            Row::new(vec![
                Cell::from(ingress.name.clone()),
                Cell::from(ingress.namespace.clone()),
                Cell::from(hosts_str),
                Cell::from(ingress.age.clone()),
            ]).style(style)
        })
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Percentage(30),
            Constraint::Percentage(30),
            Constraint::Percentage(25),
            Constraint::Percentage(15),
        ]
    )
        .header(
            Row::new(vec!["Name", "Namespace", "Hosts", "Age"])
                .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("Ingresses ({})", app.ingresses.len()))
        )
        .row_highlight_style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        );

    f.render_stateful_widget(table, area, &mut create_table_state(app.selected_ingress_index));
}

fn create_table_state(selected: usize) -> ratatui::widgets::TableState {
    let mut state = ratatui::widgets::TableState::default();
    state.select(Some(selected));
    state
}