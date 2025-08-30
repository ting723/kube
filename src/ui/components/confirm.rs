use ratatui::{
    layout::{Constraint, Direction, Layout, Rect, Alignment},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Clear},
    Frame,
};

use crate::app::{AppState, ConfirmAction};

pub fn render(f: &mut Frame, area: Rect, app: &AppState) {
    if let Some(ref action) = app.confirm_action {
        // 创建一个居中的对话框
        let popup_area = centered_rect(60, 20, area);
        
        // 清除背景
        f.render_widget(Clear, popup_area);
        
        let (title, message) = match action {
            ConfirmAction::DeletePod { namespace, name } => {
                ("Confirm Delete Pod".to_string(), format!("Delete pod '{}/{}' ?", namespace, name))
            }
            ConfirmAction::DeleteService { namespace, name } => {
                ("Confirm Delete Service".to_string(), format!("Delete service '{}/{}' ?", namespace, name))
            }
            ConfirmAction::DeleteConfigMap { namespace, name } => {
                ("Confirm Delete ConfigMap".to_string(), format!("Delete configmap '{}/{}' ?", namespace, name))
            }
            ConfirmAction::DeleteSecret { namespace, name } => {
                ("Confirm Delete Secret".to_string(), format!("Delete secret '{}/{}' ?", namespace, name))
            }
        };

        let text = format!("{}\n\nThis action cannot be undone!\n\nPress 'y' to confirm or 'n' to cancel.", message);
        
        let paragraph = Paragraph::new(text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(title)
                    .style(Style::default().fg(Color::Red))
            )
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Center);

        f.render_widget(paragraph, popup_area);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}