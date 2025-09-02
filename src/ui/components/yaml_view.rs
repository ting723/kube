use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Span, Line},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
    Frame,
};

use crate::app::{AppState, AppMode};

pub fn render(f: &mut Frame, area: Rect, app: &AppState) {
    let mode_suffix = if !app.get_mouse_mode_text().is_empty() {
        format!(" - {}", app.get_mouse_mode_text())
    } else {
        String::new()
    };
    
    let title = match app.previous_mode {
        AppMode::PodList => {
            if let Some(pod) = app.get_selected_pod() {
                let mode_hint = if app.language_chinese {
                    "(j/k:滚动, M:切换模式, PgUp/PgDn:翻页)"
                } else {
                    "(j/k:scroll, M:toggle mode, PgUp/PgDn:page)"
                };
                format!("YAML配置 - Pod: {}/{}{}  {}", app.current_namespace, pod.name, mode_suffix, mode_hint)
            } else {
                let resource_type = if app.language_chinese { "Pod" } else { "Pod" };
                format!("YAML配置 - {}: {}", resource_type, mode_suffix)
            }
        }
        AppMode::ServiceList => {
            if let Some(service) = app.get_selected_service() {
                let mode_hint = if app.language_chinese {
                    "(j/k:滚动, M:切换模式, PgUp/PgDn:翻页)"
                } else {
                    "(j/k:scroll, M:toggle mode, PgUp/PgDn:page)"
                };
                format!("YAML配置 - Service: {}/{}{}  {}", app.current_namespace, service.name, mode_suffix, mode_hint)
            } else {
                let resource_type = if app.language_chinese { "Service" } else { "Service" };
                format!("YAML配置 - {}: {}", resource_type, mode_suffix)
            }
        }
        AppMode::DeploymentList => {
            if let Some(deployment) = app.get_selected_deployment() {
                format!("YAML配置 - Deployment {}/{} (j/k:滚动, PgUp/PgDn:翻页)", app.current_namespace, deployment.name)
            } else {
                "YAML配置 - Deployment".to_string()
            }
        }
        AppMode::JobList => {
            if let Some(job) = app.get_selected_job() {
                format!("YAML配置 - Job {}/{} (j/k:滚动, PgUp/PgDn:翻页)", app.current_namespace, job.name)
            } else {
                "YAML配置 - Job".to_string()
            }
        }
        AppMode::DaemonSetList => {
            if let Some(daemonset) = app.get_selected_daemonset() {
                format!("YAML配置 - DaemonSet {}/{} (j/k:滚动, PgUp/PgDn:翻页)", app.current_namespace, daemonset.name)
            } else {
                "YAML配置 - DaemonSet".to_string()
            }
        }
        AppMode::NodeList => {
            if let Some(node) = app.get_selected_node() {
                format!("YAML配置 - Node {} (j/k:滚动, PgUp/PgDn:翻页)", node.name)
            } else {
                "YAML配置 - Node".to_string()
            }
        }
        AppMode::ConfigMapList => {
            if let Some(configmap) = app.get_selected_configmap() {
                format!("YAML配置 - ConfigMap {}/{} (j/k:滚动, PgUp/PgDn:翻页)", app.current_namespace, configmap.name)
            } else {
                "YAML配置 - ConfigMap".to_string()
            }
        }
        AppMode::SecretList => {
            if let Some(secret) = app.get_selected_secret() {
                format!("YAML配置 - Secret {}/{} (j/k:滚动, PgUp/PgDn:翻页)", app.current_namespace, secret.name)
            } else {
                "YAML配置 - Secret".to_string()
            }
        }
        AppMode::PVCList => {
            if let Some(pvc) = app.get_selected_pvc() {
                format!("YAML配置 - PVC {}/{} (j/k:滚动, PgUp/PgDn:翻页)", app.current_namespace, pvc.name)
            } else {
                "YAML配置 - PVC".to_string()
            }
        }
        AppMode::PVList => {
            if let Some(pv) = app.get_selected_pv() {
                format!("YAML配置 - PV {} (j/k:滚动, PgUp/PgDn:翻页)", pv.name)
            } else {
                "YAML配置 - PV".to_string()
            }
        }
        _ => "YAML配置".to_string(),
    };

    if app.yaml_content.is_empty() {
        let no_content = Paragraph::new("正在加载YAML配置...")
            .block(Block::default().borders(Borders::ALL).title(title))
            .style(Style::default().fg(Color::Gray));
        
        f.render_widget(no_content, area);
        return;
    }

    // 将内容按行分割并应用语法高亮
    let lines: Vec<&str> = app.yaml_content.lines().collect();
    let visible_height = area.height.saturating_sub(2) as usize;
    let total_lines = lines.len();
    
    // 计算显示范围
    let start_index = app.yaml_scroll;
    let end_index = (start_index + visible_height).min(total_lines);
    
    // 创建带语法高亮的可见内容项
    let visible_lines: Vec<ListItem> = lines[start_index..end_index]
        .iter()
        .map(|line| ListItem::new(highlight_yaml_line(line)))
        .collect();

    let mut list_state = ListState::default();
    
    // 计算滚动条状态
    let scrollbar_state = if total_lines > visible_height {
        let state = ScrollbarState::default()
            .content_length(total_lines)
            .viewport_content_length(visible_height)
            .position(start_index);
        Some(state)
    } else {
        None
    };
    
    let list = List::new(visible_lines)
        .block(Block::default().borders(Borders::ALL).title(title))
        .style(Style::default().fg(Color::White));

    f.render_stateful_widget(list, area, &mut list_state);
    
    // 渲染滚动条
    if let Some(mut state) = scrollbar_state {
        let scrollbar = Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"));
        f.render_stateful_widget(
            scrollbar,
            area.inner(ratatui::layout::Margin { vertical: 1, horizontal: 0 }),
            &mut state,
        );
    }
}

// YAML语法高亮函数（重用describe.rs中的函数）
fn highlight_yaml_line(line: &str) -> Line<'_> {
    let trimmed = line.trim_start();
    let indent = line.len() - trimmed.len();
    
    // 创建缩进空间
    let mut spans = Vec::new();
    if indent > 0 {
        spans.push(Span::styled(" ".repeat(indent), Style::default()));
    }
    
    // 检查不同的YAML语法元素
    if trimmed.starts_with('#') {
        // 注释行
        spans.push(Span::styled(trimmed, Style::default().fg(Color::Green)));
    } else if trimmed.starts_with('-') {
        // 列表项
        spans.push(Span::styled("-", Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)));
        if trimmed.len() > 1 {
            spans.push(Span::styled(&trimmed[1..], Style::default().fg(Color::White)));
        }
    } else if let Some(colon_pos) = trimmed.find(':') {
        // 键值对
        let key = &trimmed[..colon_pos];
        let rest = &trimmed[colon_pos..];
        
        // 高亮键名
        spans.push(Span::styled(key, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)));
        
        if rest.len() > 1 {
            // 冒号
            spans.push(Span::styled(":", Style::default().fg(Color::Cyan)));
            
            let value = &rest[1..].trim_start();
            if !value.is_empty() {
                // 检查值的类型并应用不同颜色
                if value.starts_with('"') && value.ends_with('"') {
                    // 字符串值
                    spans.push(Span::styled(format!(" {}", value), Style::default().fg(Color::Green)));
                } else if value.parse::<i64>().is_ok() || value.parse::<f64>().is_ok() {
                    // 数字值
                    spans.push(Span::styled(format!(" {}", value), Style::default().fg(Color::Magenta)));
                } else if *value == "true" || *value == "false" {
                    // 布尔值
                    spans.push(Span::styled(format!(" {}", value), Style::default().fg(Color::Blue)));
                } else if *value == "null" || *value == "~" {
                    // null值
                    spans.push(Span::styled(format!(" {}", value), Style::default().fg(Color::Gray)));
                } else {
                    // 普通值
                    spans.push(Span::styled(format!(" {}", value), Style::default().fg(Color::White)));
                }
            }
        } else {
            // 只有冒号，可能是对象开始
            spans.push(Span::styled(":", Style::default().fg(Color::Cyan)));
        }
    } else if trimmed.starts_with("apiVersion:") || 
              trimmed.starts_with("kind:") ||
              trimmed.starts_with("metadata:") ||
              trimmed.starts_with("spec:") ||
              trimmed.starts_with("status:") {
        // YAML的特殊字段
        if let Some(colon_pos) = trimmed.find(':') {
            let field = &trimmed[..colon_pos];
            let rest = &trimmed[colon_pos..];
            spans.push(Span::styled(field, Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)));
            spans.push(Span::styled(rest, Style::default().fg(Color::White)));
        } else {
            spans.push(Span::styled(trimmed, Style::default().fg(Color::White)));
        }
    } else if trimmed.starts_with("---") {
        // 分隔线
        spans.push(Span::styled(trimmed, Style::default().fg(Color::Gray)));
    } else {
        // 一般文本
        spans.push(Span::styled(trimmed, Style::default().fg(Color::White)));
    }
    
    Line::from(spans)
}