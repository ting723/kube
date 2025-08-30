use ratatui::layout::{Constraint, Direction, Layout, Rect};

#[allow(dead_code)]
pub fn main_layout(area: Rect) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Main content
            Constraint::Length(3), // Footer
        ])
        .split(area)
        .to_vec()
}

#[allow(dead_code)]
pub fn horizontal_split(area: Rect, left_percentage: u16) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(left_percentage),
            Constraint::Percentage(100 - left_percentage),
        ])
        .split(area)
        .to_vec()
}

#[allow(dead_code)]
pub fn vertical_split(area: Rect, top_percentage: u16) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(top_percentage),
            Constraint::Percentage(100 - top_percentage),
        ])
        .split(area)
        .to_vec()
}