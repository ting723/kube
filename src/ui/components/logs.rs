use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

use crate::app::AppState;

pub fn render(f: &mut Frame, area: Rect, app: &AppState) {
    let title = if let Some(pod) = app.get_selected_pod() {
        format!("Logs - {}/{}", app.current_namespace, pod.name)
    } else {
        "Logs".to_string()
    };

    if app.logs.is_empty() {
        let no_logs = ratatui::widgets::Paragraph::new("No logs available or loading...")
            .block(Block::default().borders(Borders::ALL).title(title))
            .style(Style::default().fg(Color::Gray));
        
        f.render_widget(no_logs, area);
        return;
    }

    let items: Vec<ListItem> = app
        .logs
        .iter()
        .map(|line| ListItem::new(line.clone()))
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(title))
        .style(Style::default().fg(Color::White));

    f.render_widget(list, area);
}