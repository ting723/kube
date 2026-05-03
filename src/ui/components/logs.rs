use crate::app::state::AppState;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{
        Block, Borders, Clear, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, Wrap,
    },
};

fn log_pane_title(app: &AppState, pod_name: &str, is_active: bool) -> String {
    let marker = if is_active { "◉" } else { " " };
    let status = log_search_status(app);
    let auto = if app.logs_auto_scroll {
        if app.language_chinese {
            " [自动滚动:开]"
        } else {
            " [AutoScroll:ON]"
        }
    } else {
        if app.language_chinese {
            " [自动滚动:关]"
        } else {
            " [AutoScroll:OFF]"
        }
    };
    if app.language_chinese {
        format!(
            "{} 日志 - {}/{}{}{}",
            marker, app.current_namespace, pod_name, status, auto
        )
    } else {
        format!(
            "{} Logs - {}/{}{}{}",
            marker, app.current_namespace, pod_name, status, auto
        )
    }
}

fn log_search_status(app: &AppState) -> String {
    if app.log_search_mode && !app.log_search_query.is_empty() {
        let total = app.log_search_results.len();
        let current = if total > 0 {
            app.current_log_search_index + 1
        } else {
            0
        };
        if app.language_chinese {
            format!(" [搜索: {} ({}/{})]", app.log_search_query, current, total)
        } else {
            format!(
                " [Search: {} ({}/{})]",
                app.log_search_query, current, total
            )
        }
    } else if app.log_search_mode && app.log_search_query.is_empty() {
        if app.language_chinese {
            " [搜索中...]".to_string()
        } else {
            " [Searching...]".to_string()
        }
    } else if !app.log_search_results.is_empty() {
        // 已退出搜索模式但仍显示匹配结果计数
        let total = app.log_search_results.len();
        let current = app.current_log_search_index + 1;
        if app.language_chinese {
            format!(" [{} 个匹配 ({}/{})]", total, current, total)
        } else {
            format!(" [{} matches ({}/{})]", total, current, total)
        }
    } else {
        String::new()
    }
}

fn render_log_pane(
    f: &mut Frame,
    area: Rect,
    logs: &[String],
    scroll: usize,
    title: &str,
    app: &AppState,
) {
    if logs.is_empty() {
        let widget = Paragraph::new("Loading logs...")
            .block(Block::default().borders(Borders::ALL).title(title))
            .style(Style::default().fg(Color::Gray));
        f.render_widget(widget, area);
        return;
    }
    let content: Vec<Line> = logs
        .iter()
        .enumerate()
        .map(|(i, line)| {
            let prefix = format!("[{}] ", i + 1);
            if app.log_search_mode && !app.log_search_query.is_empty() {
                let lower = line.to_lowercase();
                let query_lower = app.log_search_query.to_lowercase();
                if let Some(pos) = lower.find(&query_lower) {
                    let mut spans = vec![Span::from(prefix)];
                    if pos > 0 {
                        spans.push(Span::from(&line[..pos]));
                    }
                    spans.push(Span::styled(
                        &line[pos..pos + query_lower.len()],
                        Style::default().fg(Color::Black).bg(Color::Yellow),
                    ));
                    if pos + query_lower.len() < line.len() {
                        spans.push(Span::from(&line[pos + query_lower.len()..]));
                    }
                    return Line::from(spans);
                }
            }
            Line::from(format!("{}{}", prefix, line))
        })
        .collect();
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
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .style(Style::default().fg(Color::Yellow)),
        )
        .style(Style::default().fg(Color::White))
        .scroll((
            app.split_pod_selection_index
                .saturating_sub(picker_area.height.saturating_sub(3) as usize / 2)
                as u16,
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
            format!(
                "日志 - {}/{}{}",
                app.current_namespace,
                name,
                log_search_status(app)
            )
        } else {
            format!(
                "Logs - {}/{}{}",
                app.current_namespace,
                name,
                log_search_status(app)
            )
        };
        render_log_pane(f, area, &app.logs, app.logs_scroll, &title, app);
        // 叠加 Pod 选择弹窗
        render_pod_picker(f, area, app);
    } else if app.split_log_mode && !app.log_panes.is_empty() {
        let count = app.log_panes.len();
        let constraints: Vec<Constraint> = (0..count)
            .map(|_| Constraint::Percentage(100 / count as u16))
            .collect();
        let panes = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .split(area);

        for (i, pane) in app.log_panes.iter().enumerate() {
            let is_active = i == app.active_pane_index;
            render_log_pane(
                f,
                panes[i],
                &pane.content,
                pane.scroll,
                &log_pane_title(app, &pane.pod_name, is_active),
                app,
            );
        }
    } else {
        let name = app
            .pods
            .get(app.selected_pod_index)
            .map(|p| p.name.as_str())
            .unwrap_or("?");
        let title = if app.language_chinese {
            format!(
                "日志 - {}/{}{}",
                app.current_namespace,
                name,
                log_search_status(app)
            )
        } else {
            format!(
                "Logs - {}/{}{}",
                app.current_namespace,
                name,
                log_search_status(app)
            )
        };
        render_log_pane(f, area, &app.logs, app.logs_scroll, &title, app);
    }
}
