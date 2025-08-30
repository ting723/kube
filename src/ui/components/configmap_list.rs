use ratatui::{
    layout::{Rect, Constraint},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Row, Table, Cell},
    Frame,
};

use crate::app::AppState;

pub fn render(f: &mut Frame, area: Rect, app: &AppState) {
    if app.configmaps.is_empty() {
        let no_configmaps = ratatui::widgets::Paragraph::new("No configmaps found in this namespace")
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!("ConfigMaps in namespace: {}", app.current_namespace))
            )
            .style(Style::default().fg(Color::Gray));
        
        f.render_widget(no_configmaps, area);
        return;
    }

    let rows: Vec<Row> = app
        .configmaps
        .iter()
        .enumerate()
        .map(|(i, configmap)| {
            let style = if i == app.selected_configmap_index {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            Row::new(vec![
                Cell::from(configmap.name.clone()),
                Cell::from(configmap.data_count.to_string()),
                Cell::from(configmap.age.clone()),
            ]).style(style)
        })
        .collect();

    let table = Table::new(rows)
        .header(
            Row::new(vec!["Name", "Data", "Age"])
                .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("ConfigMaps in namespace: {} ({})", app.current_namespace, app.configmaps.len()))
        )
        .widths(&[
            Constraint::Percentage(60),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
        ])
        .highlight_style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        );

    f.render_stateful_widget(table, area, &mut create_table_state(app.selected_configmap_index));
}

fn create_table_state(selected: usize) -> ratatui::widgets::TableState {
    let mut state = ratatui::widgets::TableState::default();
    state.select(Some(selected));
    state
}