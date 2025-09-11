use ratatui::{
    layout::{Rect, Constraint},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Row, Table, Cell},
    Frame,
};

use crate::app::AppState;

pub fn render(f: &mut Frame, area: Rect, app: &AppState) {
    if app.statefulsets.is_empty() {
        let no_statefulsets = ratatui::widgets::Paragraph::new("No statefulsets found or loading...")
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("StatefulSets")
            )
            .style(Style::default().fg(Color::Gray));
        
        f.render_widget(no_statefulsets, area);
        return;
    }

    let rows: Vec<Row> = app
        .statefulsets
        .iter()
        .enumerate()
        .map(|(i, statefulset)| {
            let style = if i == app.selected_statefulset_index {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            Row::new(vec![
                Cell::from(statefulset.name.clone()),
                Cell::from(statefulset.namespace.clone()),
                Cell::from(statefulset.ready.clone()),
                Cell::from(statefulset.age.clone()),
            ]).style(style)
        })
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Percentage(30),
            Constraint::Percentage(30),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
        ]
    )
        .header(
            Row::new(vec!["Name", "Namespace", "Ready", "Age"])
                .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("StatefulSets ({})", app.statefulsets.len()))
        )
        .row_highlight_style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        );

    f.render_stateful_widget(table, area, &mut create_table_state(app.selected_statefulset_index));
}

fn create_table_state(selected: usize) -> ratatui::widgets::TableState {
    let mut state = ratatui::widgets::TableState::default();
    state.select(Some(selected));
    state
}