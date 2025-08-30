use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

use crate::app::AppState;

pub fn render(f: &mut Frame, area: Rect, app: &AppState) {
    let title = if let Some(pod) = app.get_selected_pod() {
        format!(
            "Describe - {}/{} (↑/↓:navigate, J/K:scroll, PgUp/PgDn:page)", 
            app.current_namespace, 
            pod.name
        )
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

    // 将内容按行分割
    let lines: Vec<&str> = app.describe_content.lines().collect();
    let visible_height = area.height.saturating_sub(2) as usize;
    let total_lines = lines.len();
    
    // 计算显示范围
    let start_index = app.describe_scroll;
    let end_index = (start_index + visible_height).min(total_lines);
    
    // 创建可见的内容项
    let visible_lines: Vec<ListItem> = lines[start_index..end_index]
        .iter()
        .map(|line| ListItem::new(line.to_string()))
        .collect();

    let mut list_state = ListState::default();
    
    let list = List::new(visible_lines)
        .block(Block::default().borders(Borders::ALL).title(title))
        .style(Style::default().fg(Color::White));

    f.render_stateful_widget(list, area, &mut list_state);
}