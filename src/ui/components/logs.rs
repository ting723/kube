use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Scrollbar, ScrollbarOrientation, ScrollbarState, Paragraph, Wrap},
    Frame,
};

use crate::app::AppState;

pub fn render(f: &mut Frame, area: Rect, app: &AppState) {
    let title = if let Some(pod) = app.get_selected_pod() {
        format!(
            "Logs - {}/{} (j/k:scroll, PgUp/PgDn:page, A:auto-scroll:{}, R:auto-refresh:{})", 
            app.current_namespace, 
            pod.name,
            if app.logs_auto_scroll { "ON" } else { "OFF" },
            if app.logs_auto_refresh { "ON" } else { "OFF" }
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

    // 将日志内容连接成一个字符串，支持换行显示
    let log_content = app.logs
        .iter()
        .enumerate()
        .map(|(i, line)| format!("[{}] {}", i + 1, line))
        .collect::<Vec<String>>()
        .join("\n");

    // 使用Paragraph组件显示日志，支持自动换行
    let paragraph = Paragraph::new(log_content)
        .block(Block::default().borders(Borders::ALL).title(title))
        .style(Style::default().fg(Color::White))
        .wrap(Wrap { trim: true })
        .scroll((app.logs_scroll as u16, 0));  // 使用scroll属性处理滚动

    f.render_widget(paragraph, area);
    
    // 添加垂直滚动条
    let total_lines = app.logs.len();
    let visible_height = area.height.saturating_sub(2) as usize;
    
    if total_lines > visible_height {
        let mut scrollbar_state = ScrollbarState::default()
            .content_length(total_lines)
            .viewport_content_length(visible_height)
            .position(app.logs_scroll);
        
        let scrollbar = Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"));
        
        f.render_stateful_widget(
            scrollbar,
            area.inner(ratatui::layout::Margin { vertical: 1, horizontal: 0 }),
            &mut scrollbar_state,
        );
    }
}