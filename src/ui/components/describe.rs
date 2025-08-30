use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Span, Line},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
    Frame,
};

use crate::app::{AppState, AppMode};

pub fn render(f: &mut Frame, area: Rect, app: &AppState) {
    let title = match app.previous_mode {
        AppMode::PodList => {
            if let Some(pod) = app.get_selected_pod() {
                format!("Describe - Pod {}/{} (J/K:scroll, PgUp/PgDn:page)", app.current_namespace, pod.name)
            } else {
                "Describe - Pod".to_string()
            }
        }
        AppMode::ServiceList => {
            if let Some(service) = app.get_selected_service() {
                format!("Describe - Service {}/{} (J/K:scroll, PgUp/PgDn:page)", app.current_namespace, service.name)
            } else {
                "Describe - Service".to_string()
            }
        }
        AppMode::DeploymentList => {
            if let Some(deployment) = app.get_selected_deployment() {
                format!("Describe - Deployment {}/{} (J/K:scroll, PgUp/PgDn:page)", app.current_namespace, deployment.name)
            } else {
                "Describe - Deployment".to_string()
            }
        }
        AppMode::JobList => {
            if let Some(job) = app.get_selected_job() {
                format!("Describe - Job {}/{} (J/K:scroll, PgUp/PgDn:page)", app.current_namespace, job.name)
            } else {
                "Describe - Job".to_string()
            }
        }
        AppMode::DaemonSetList => {
            if let Some(daemonset) = app.get_selected_daemonset() {
                format!("Describe - DaemonSet {}/{} (J/K:scroll, PgUp/PgDn:page)", app.current_namespace, daemonset.name)
            } else {
                "Describe - DaemonSet".to_string()
            }
        }
        AppMode::NodeList => {
            if let Some(node) = app.get_selected_node() {
                format!("Describe - Node {} (J/K:scroll, PgUp/PgDn:page)", node.name)
            } else {
                "Describe - Node".to_string()
            }
        }
        AppMode::ConfigMapList => {
            if let Some(configmap) = app.get_selected_configmap() {
                format!("Describe - ConfigMap {}/{} (J/K:scroll, PgUp/PgDn:page)", app.current_namespace, configmap.name)
            } else {
                "Describe - ConfigMap".to_string()
            }
        }
        AppMode::SecretList => {
            if let Some(secret) = app.get_selected_secret() {
                format!("Describe - Secret {}/{} (J/K:scroll, PgUp/PgDn:page)", app.current_namespace, secret.name)
            } else {
                "Describe - Secret".to_string()
            }
        }
        AppMode::PVCList => {
            if let Some(pvc) = app.get_selected_pvc() {
                format!("Describe - PVC {}/{} (J/K:scroll, PgUp/PgDn:page)", app.current_namespace, pvc.name)
            } else {
                "Describe - PVC".to_string()
            }
        }
        AppMode::PVList => {
            if let Some(pv) = app.get_selected_pv() {
                format!("Describe - PV {} (J/K:scroll, PgUp/PgDn:page)", pv.name)
            } else {
                "Describe - PV".to_string()
            }
        }
        _ => "Describe".to_string(),
    };

    if app.describe_content.is_empty() {
        let no_content = Paragraph::new("No description available or loading...")
            .block(Block::default().borders(Borders::ALL).title(title))
            .style(Style::default().fg(Color::Gray));
        
        f.render_widget(no_content, area);
        return;
    }

    // 将内容按行分割并应用语法高亮
    let lines: Vec<&str> = app.describe_content.lines().collect();
    let visible_height = area.height.saturating_sub(2) as usize;
    let total_lines = lines.len();
    
    // 计算显示范围
    let start_index = app.describe_scroll;
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
            area.inner(&ratatui::layout::Margin { vertical: 1, horizontal: 0 }),
            &mut state,
        );
    }
}

// YAML语法高亮函数
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
                    // 字符串값
                    spans.push(Span::styled(format!(" {}", value), Style::default().fg(Color::Green)));
                } else if value.parse::<i64>().is_ok() || value.parse::<f64>().is_ok() {
                    // 数字값
                    spans.push(Span::styled(format!(" {}", value), Style::default().fg(Color::Magenta)));
                } else if *value == "true" || *value == "false" {
                    // 布尔값
                    spans.push(Span::styled(format!(" {}", value), Style::default().fg(Color::Blue)));
                } else if *value == "null" || *value == "~" {
                    // null값
                    spans.push(Span::styled(format!(" {}", value), Style::default().fg(Color::Gray)));
                } else {
                    // 普通값
                    spans.push(Span::styled(format!(" {}", value), Style::default().fg(Color::White)));
                }
            }
        } else {
            // 只有冒号，可能是对象开始
            spans.push(Span::styled(":", Style::default().fg(Color::Cyan)));
        }
    } else if trimmed.starts_with("Name:") || 
              trimmed.starts_with("Namespace:") ||
              trimmed.starts_with("Labels:") ||
              trimmed.starts_with("Annotations:") ||
              trimmed.starts_with("Status:") ||
              trimmed.starts_with("Type:") ||
              trimmed.starts_with("Events:") {
        // kubectl describe의 특별 필드
        if let Some(colon_pos) = trimmed.find(':') {
            let field = &trimmed[..colon_pos];
            let rest = &trimmed[colon_pos..];
            spans.push(Span::styled(field, Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)));
            spans.push(Span::styled(rest, Style::default().fg(Color::White)));
        } else {
            spans.push(Span::styled(trimmed, Style::default().fg(Color::White)));
        }
    } else if trimmed.starts_with("====") || trimmed.starts_with("----") {
        // 분隔선
        spans.push(Span::styled(trimmed, Style::default().fg(Color::Gray)));
    } else {
        // 일반 텍스트
        spans.push(Span::styled(trimmed, Style::default().fg(Color::White)));
    }
    
    Line::from(spans)
}