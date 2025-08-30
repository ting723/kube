use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};

use crate::app::AppState;

pub fn render(f: &mut Frame, area: Rect, app: &AppState) {
    let title = if let Some(pod) = app.get_selected_pod() {
        format!(
            "Logs - {}/{} (↑/↓:navigate, J/K:scroll, PgUp/PgDn:page, Auto-scroll:{})", 
            app.current_namespace, 
            pod.name,
            if app.logs_auto_scroll { "ON" } else { "OFF" }
        )
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

    // 计算可见区域的高度（减去边框）
    let visible_height = area.height.saturating_sub(2) as usize;
    let total_lines = app.logs.len();
    
    // 自动滚动到底部或使用手动滚动位置
    let start_index = if app.logs_auto_scroll && total_lines > visible_height {
        total_lines - visible_height
    } else {
        app.logs_scroll.min(total_lines.saturating_sub(visible_height))
    };
    
    let end_index = (start_index + visible_height).min(total_lines);
    
    // 创建可见的日志项
    let visible_logs: Vec<ListItem> = app.logs[start_index..end_index]
        .iter()
        .enumerate()
        .map(|(i, line)| {
            let line_number = start_index + i + 1;
            ListItem::new(format!("[{}] {}", line_number, line))
        })
        .collect();

    let mut list_state = ListState::default();
    // 不需要选中任何项，因为这是日志显示
    
    let list = List::new(visible_logs)
        .block(Block::default().borders(Borders::ALL).title(title))
        .style(Style::default().fg(Color::White));

    f.render_stateful_widget(list, area, &mut list_state);
}