use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::AppState;

pub fn render(f: &mut Frame, area: Rect, app: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(area);

    // 搜索输入框，添加光标显示
    let input_text = if app.search_mode {
        format!("{}█", app.search_query) // 添加光标字符
    } else {
        app.search_query.clone()
    };
    
    let search_input = Paragraph::new(input_text)
        .block(Block::default().borders(Borders::ALL).title("Search (type to search, Enter to confirm, Esc to cancel)"))
        .style(Style::default().fg(Color::White));

    f.render_widget(search_input, chunks[0]);

    // 搜索结果提示
    let results_text = if app.search_results.is_empty() {
        if app.search_query.is_empty() {
            "Enter search query...".to_string()
        } else {
            "No results found".to_string()
        }
    } else {
        format!("Found {} results (use n/N to navigate)", app.search_results.len())
    };

    let results_info = Paragraph::new(results_text)
        .block(Block::default().borders(Borders::ALL).title("Results"))
        .style(Style::default().fg(Color::Gray));

    f.render_widget(results_info, chunks[1]);
}