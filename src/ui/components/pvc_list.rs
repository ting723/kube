use ratatui::{
    layout::{Rect, Constraint},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Row, Table, Cell},
    Frame,
};

use crate::app::AppState;

pub fn render(f: &mut Frame, area: Rect, app: &AppState) {
    if app.pvcs.is_empty() {
        let no_pvcs = ratatui::widgets::Paragraph::new("No PVCs found or loading...")
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("PersistentVolumeClaims")
            )
            .style(Style::default().fg(Color::Gray));
        
        f.render_widget(no_pvcs, area);
        return;
    }

    let rows: Vec<Row> = app
        .pvcs
        .iter()
        .enumerate()
        .map(|(i, pvc)| {
            let style = if i == app.selected_pvc_index {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            let status_color = match pvc.status.as_str() {
                "Bound" => Color::Green,
                "Pending" => Color::Yellow,
                "Lost" => Color::Red,
                _ => Color::Gray,
            };

            let access_modes_str = pvc.access_modes.join(",");
            let capacity = pvc.capacity.as_ref().unwrap_or(&"<none>".to_string()).clone();
            let volume = pvc.volume.as_ref().unwrap_or(&"<none>".to_string()).clone();
            let storage_class = pvc.storage_class.as_ref().unwrap_or(&"<none>".to_string()).clone();

            Row::new(vec![
                Cell::from(pvc.name.clone()),
                Cell::from(pvc.status.clone()).style(Style::default().fg(status_color)),
                Cell::from(volume),
                Cell::from(capacity),
                Cell::from(access_modes_str),
                Cell::from(storage_class),
                Cell::from(pvc.age.clone()),
            ]).style(style)
        })
        .collect();

    let table = Table::new(rows)
        .header(
            Row::new(vec!["Name", "Status", "Volume", "Capacity", "Access Modes", "Storage Class", "Age"])
                .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("PersistentVolumeClaims ({})", app.pvcs.len()))
        )
        .widths(&[
            Constraint::Percentage(20),
            Constraint::Percentage(10),
            Constraint::Percentage(15),
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

    f.render_stateful_widget(table, area, &mut create_table_state(app.selected_pvc_index));
}

fn create_table_state(selected: usize) -> ratatui::widgets::TableState {
    let mut state = ratatui::widgets::TableState::default();
    state.select(Some(selected));
    state
}