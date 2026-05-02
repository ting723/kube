use crate::app::state::{ActivePane, AppState};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Clear, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, Wrap},
};

fn log_pane_title(app: &AppState, pod_name: &str, pane: &ActivePane) -> String {
    let marker = match pane {
        ActivePane::Left if app.active_pane == ActivePane::Left => "◉",
        ActivePane::Right if app.active_pane == ActivePane::Right => "◉",
        _ => " ",
    };
    if app.language_chinese {
        format!("{} 日志 - {}/{}", marker, app.current_namespace, pod_name)
    } else {
        format!("{} Logs - {}/{}", marker, app.current_namespace, pod_name)
    }
}

fn render_log_pane(f: &mut Frame, area: Rect, logs: &[String], scroll: usize, title: &str) {
    if logs.is_empty() {
        let widget = Paragraph::new("Loading logs...")
            .block(Block::default().borders(Borders::ALL).title(title))
            .style(Style::default().fg(Color::Gray));
        f.render_widget(widget, area);
        return;
    }
    let content = logs
        .iter()
        .enumerate()
        .map(|(i, line)| format!("[{}] {}", i + 1, line))
        .collect::<Vec<_>>()
        .join("\n");
    let paragraph = Paragraph::new(content)
        .block(Block::default().borders(Borders::ALL).title(title))
        .style(Style::default().fg(Color::White))
        .wrap(Wrap { trim: true })
        .scroll((scroll as u16, 0));
    f.render_widget(paragraph, area);
    let total = logs.len();
    let visible = area.height.saturating_sub(2) as usize;
    if total > visible {
        let mut state = ScrollbarState::default()
            .content_length(total)
            .viewport_content_length(visible)
            .position(scroll);
        let sb = Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"));
        f.render_stateful_widget(
            sb,
            area.inner(ratatui::layout::Margin {
                vertical: 1,
                horizontal: 0,
            }),
            &mut state,
        );
    }
}

fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let popup_width = area.width * percent_x / 100;
    let popup_height = area.height * percent_y / 100;
    let x = area.x + (area.width.saturating_sub(popup_width)) / 2;
    let y = area.y + (area.height.saturating_sub(popup_height)) / 2;
    Rect::new(x, y, popup_width, popup_height)
}

fn render_pod_picker(f: &mut Frame, area: Rect, app: &AppState) {
    let picker_area = popup_area(area, 60, 70);
    f.render_widget(Clear, picker_area);

    let pod_names: Vec<String> = app
        .pods
        .iter()
        .enumerate()
        .map(|(i, p)| {
            let marker = if i == app.split_pod_selection_index {
                "▶ "
            } else {
                "  "
            };
            format!("{}{}", marker, p.name)
        })
        .collect();

    let title = if app.language_chinese {
        "选择要对比的 Pod (j/k:导航 Enter:确认 Esc:取消)"
    } else {
        "Select Pod to Compare (j/k:nav Enter:select Esc:cancel)"
    };

    let paragraph = Paragraph::new(pod_names.join("\n"))
        .block(Block::default().borders(Borders::ALL).title(title).style(
            Style::default().fg(Color::Yellow),
        ))
        .style(Style::default().fg(Color::White))
        .scroll((
            app.split_pod_selection_index.saturating_sub(picker_area.height.saturating_sub(3) as usize / 2) as u16,
            0,
        ));

    f.render_widget(paragraph, picker_area);
}

pub fn render(f: &mut Frame, area: Rect, app: &AppState) {
    if app.split_pod_selection_mode {
        // 先渲染当前日志作为背景
        let name = app
            .pods
            .get(app.selected_pod_index)
            .map(|p| p.name.as_str())
            .unwrap_or("?");
        let title = if app.language_chinese {
            format!("日志 - {}/{}", app.current_namespace, name)
        } else {
            format!("Logs - {}/{}", app.current_namespace, name)
        };
        render_log_pane(f, area, &app.logs, app.logs_scroll, &title);
        // 叠加 Pod 选择弹窗
        render_pod_picker(f, area, app);
    } else if app.split_log_mode {
        let panes = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);
        let left_name = app
            .pods
            .get(app.selected_pod_index)
            .map(|p| p.name.as_str())
            .unwrap_or("?");
        render_log_pane(
            f,
            panes[0],
            &app.logs,
            app.logs_scroll,
            &log_pane_title(app, left_name, &ActivePane::Left),
        );
        render_log_pane(
            f,
            panes[1],
            &app.split_log_content,
            app.split_log_scroll,
            &log_pane_title(app, &app.split_log_pod_name, &ActivePane::Right),
        );
    } else {
        let name = app
            .pods
            .get(app.selected_pod_index)
            .map(|p| p.name.as_str())
            .unwrap_or("?");
        let title = if app.language_chinese {
            format!("日志 - {}/{}", app.current_namespace, name)
        } else {
            format!("Logs - {}/{}", app.current_namespace, name)
        };
        render_log_pane(f, area, &app.logs, app.logs_scroll, &title);
    }
}
