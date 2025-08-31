use ratatui::{
    layout::{Rect, Constraint},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Row, Table, Cell},
    Frame,
};

use crate::app::AppState;

pub fn render(f: &mut Frame, area: Rect, app: &AppState) {
    if app.pvs.is_empty() {
        let no_pvs = ratatui::widgets::Paragraph::new("No PVs found or loading...")
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("PersistentVolumes")
            )
            .style(Style::default().fg(Color::Gray));
        
        f.render_widget(no_pvs, area);
        return;
    }

    let rows: Vec<Row> = app
        .pvs
        .iter()
        .enumerate()
        .map(|(i, pv)| {
            let style = if i == app.selected_pv_index {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            let status_color = match pv.status.as_str() {
                "Available" => Color::Green,
                "Bound" => Color::Blue,
                "Released" => Color::Yellow,
                "Failed" => Color::Red,
                _ => Color::Gray,
            };

            let access_modes_str = pv.access_modes.join(",");
            let claim = pv.claim.as_ref().unwrap_or(&"<none>".to_string()).clone();
            let storage_class = pv.storage_class.as_ref().unwrap_or(&"<none>".to_string()).clone();

            Row::new(vec![
                Cell::from(pv.name.clone()),
                Cell::from(pv.capacity.clone()),
                Cell::from(access_modes_str),
                Cell::from(pv.reclaim_policy.clone()),
                Cell::from(pv.status.clone()).style(Style::default().fg(status_color)),
                Cell::from(claim),
                Cell::from(storage_class),
                Cell::from(pv.age.clone()),
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
            Constraint::Percentage(15),
            Constraint::Percentage(15),
        ]
    )
        .header(
            Row::new(vec!["Name", "Capacity", "Access Modes", "Reclaim Policy", "Status", "Claim", "Storage Class", "Age"])
                .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("PersistentVolumes ({})", app.pvs.len()))
        )
        .row_highlight_style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        );

    f.render_stateful_widget(table, area, &mut create_table_state(app.selected_pv_index));
}

fn create_table_state(selected: usize) -> ratatui::widgets::TableState {
    let mut state = ratatui::widgets::TableState::default();
    state.select(Some(selected));
    state
}