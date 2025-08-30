use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::app::AppState;

pub fn render(f: &mut Frame, area: Rect, app: &AppState) {
    let title = if let Some(pod) = app.get_selected_pod() {
        format!("Describe - {}/{}", app.current_namespace, pod.name)
    } else {
        "Describe".to_string()
    };

    if app.describe_content.is_empty() {
        let no_content = Paragraph::new("No description available or loading...")
            .block(Block::default().borders(Borders::ALL).title(title))
            .style(Style::default().fg(Color::Gray));
        
        f.render_widget(no_content, area);
        return;
    }

    let paragraph = Paragraph::new(app.describe_content.clone())
        .block(Block::default().borders(Borders::ALL).title(title))
        .style(Style::default().fg(Color::White))
        .wrap(Wrap { trim: false });

    f.render_widget(paragraph, area);
}