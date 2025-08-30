use ratatui::{
    layout::{Rect, Constraint},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Row, Table, Cell},
    Frame,
};

use crate::app::AppState;

pub fn render(f: &mut Frame, area: Rect, app: &AppState) {
    if app.daemonsets.is_empty() {
        let no_daemonsets = ratatui::widgets::Paragraph::new("No daemonsets found or loading...")
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("DaemonSets")
            )
            .style(Style::default().fg(Color::Gray));
        
        f.render_widget(no_daemonsets, area);
        return;
    }

    let rows: Vec<Row> = app
        .daemonsets
        .iter()
        .enumerate()
        .map(|(i, daemonset)| {
            let style = if i == app.selected_daemonset_index {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            Row::new(vec![
                Cell::from(daemonset.name.clone()),
                Cell::from(daemonset.desired.to_string()),
                Cell::from(daemonset.current.to_string()),
                Cell::from(daemonset.ready.to_string()),
                Cell::from(daemonset.up_to_date.to_string()),
                Cell::from(daemonset.available.to_string()),
                Cell::from(daemonset.age.clone()),
            ]).style(style)
        })
        .collect();

    let table = Table::new(rows)
        .header(
            Row::new(vec!["Name", "Desired", "Current", "Ready", "Up-to-date", "Available", "Age"])
                .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("DaemonSets ({})", app.daemonsets.len()))
        )
        .widths(&[
            Constraint::Percentage(25),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(15),
            Constraint::Percentage(15),
            Constraint::Percentage(15),
        ])
        .highlight_style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        );

    f.render_stateful_widget(table, area, &mut create_table_state(app.selected_daemonset_index));
}

fn create_table_state(selected: usize) -> ratatui::widgets::TableState {
    let mut state = ratatui::widgets::TableState::default();
    state.select(Some(selected));
    state
}