use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::app::AppState;

pub fn render(f: &mut Frame, _area: Rect, _app: &AppState) {
    let help_text = r#"
Kube TUI - Kubernetes Terminal Interface

NAVIGATION:
  ↑/↓ or j/k    Navigate lists
  Enter         Select item / Enter mode
  Tab           Switch between panels
  Esc           Go back / Exit mode
  q             Quit application
  ?             Show this help

NAMESPACE VIEW:
  Enter         Switch to selected namespace

POD VIEW:
  l             View pod logs
  d             Describe pod
  e             Exec into pod (opens new terminal)
  Delete        Delete pod (with confirmation)

SERVICE VIEW:
  d             Describe service

LOGS VIEW:
  Esc           Return to pod list

GENERAL:
  Auto-refresh is enabled (every 5 seconds)
  Status colors: Green=Running, Yellow=Pending, Red=Failed
"#;

    let paragraph = Paragraph::new(help_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Help")
        )
        .style(Style::default().fg(Color::White))
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, _area);
}