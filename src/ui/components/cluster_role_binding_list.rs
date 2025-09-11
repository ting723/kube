use ratatui::{
    layout::{Rect, Constraint},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Row, Table, Cell},
    Frame,
};

use crate::app::AppState;

pub fn render(f: &mut Frame, area: Rect, app: &AppState) {
    if app.cluster_role_bindings.is_empty() {
        let no_cluster_role_bindings = ratatui::widgets::Paragraph::new("No cluster role bindings found or loading...")
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Cluster Role Bindings")
            )
            .style(Style::default().fg(Color::Gray));
        
        f.render_widget(no_cluster_role_bindings, area);
        return;
    }

    let rows: Vec<Row> = app
        .cluster_role_bindings
        .iter()
        .enumerate()
        .map(|(i, cluster_role_binding)| {
            let style = if i == app.selected_cluster_role_binding_index {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            Row::new(vec![
                Cell::from(cluster_role_binding.name.clone()),
                Cell::from(cluster_role_binding.age.clone()),
            ]).style(style)
        })
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Percentage(60),
            Constraint::Percentage(40),
        ]
    )
        .header(
            Row::new(vec!["Name", "Age"])
                .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("Cluster Role Bindings ({})", app.cluster_role_bindings.len()))
        )
        .row_highlight_style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        );

    f.render_stateful_widget(table, area, &mut create_table_state(app.selected_cluster_role_binding_index));
}

fn create_table_state(selected: usize) -> ratatui::widgets::TableState {
    let mut state = ratatui::widgets::TableState::default();
    state.select(Some(selected));
    state
}