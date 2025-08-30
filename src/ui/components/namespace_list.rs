use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};

use crate::app::AppState;

pub fn render(f: &mut Frame, area: Rect, app: &AppState) {
    let items: Vec<ListItem> = app
        .namespaces
        .iter()
        .enumerate()
        .map(|(i, namespace)| {
            let style = if i == app.selected_namespace_index {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            let prefix = if namespace == &app.current_namespace {
                "● "
            } else {
                "  "
            };

            ListItem::new(format!("{}{}", prefix, namespace)).style(style)
        })
        .collect();

    let mut list_state = ListState::default();
    list_state.select(Some(app.selected_namespace_index));

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("Namespaces ({})", app.namespaces.len()))
        )
        .highlight_style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        );

    f.render_stateful_widget(list, area, &mut list_state);
}