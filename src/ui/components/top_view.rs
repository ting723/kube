use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Table, Row, Cell},
    Frame,
};

use crate::app::AppState;

pub fn render(f: &mut Frame, area: Rect, app: &AppState) {
    let title = format!("资源使用情况 - {} (j/k:滚动, PgUp/PgDn:翻页)", app.current_namespace);

    if app.pod_metrics.is_empty() {
        let no_content = ratatui::widgets::Paragraph::new("正在加载资源使用情况...\n\n注意: 需要安装metrics-server才能查看资源使用情况")
            .block(Block::default().borders(Borders::ALL).title(title.clone()))
            .style(Style::default().fg(Color::Gray));
        
        f.render_widget(no_content, area);
        return;
    }

    // 分割区域：上半部分显示Pod级别的指标，下半部分显示Container级别的指标
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(app.pod_metrics.len() as u16 + 3), // Pod指标表格
            Constraint::Min(0), // Container指标表格
        ])
        .split(area);

    // 渲染Pod级别的资源使用表格
    render_pod_metrics_table(f, chunks[0], app);
    
    // 渲染选中Pod的Container级别详细信息
    render_container_metrics_detail(f, chunks[1], app);
}

fn render_pod_metrics_table(f: &mut Frame, area: Rect, app: &AppState) {
    let header = Row::new(vec![
        Cell::from(Span::styled("Pod名称", Style::default().add_modifier(Modifier::BOLD))),
        Cell::from(Span::styled("CPU", Style::default().add_modifier(Modifier::BOLD))),
        Cell::from(Span::styled("内存", Style::default().add_modifier(Modifier::BOLD))),
        Cell::from(Span::styled("CPU%", Style::default().add_modifier(Modifier::BOLD))),
        Cell::from(Span::styled("内存%", Style::default().add_modifier(Modifier::BOLD))),
    ])
    .style(Style::default().fg(Color::Yellow))
    .height(1);

    let rows: Vec<Row> = app.pod_metrics
        .iter()
        .enumerate()
        .map(|(i, metrics)| {
            let style = if i == app.metrics_scroll {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            Row::new(vec![
                Cell::from(metrics.name.clone()),
                Cell::from(format_cpu(&metrics.cpu)),
                Cell::from(format_memory(&metrics.memory)),
                Cell::from(format_percentage(metrics.cpu_percentage)),
                Cell::from(format_percentage(metrics.memory_percentage)),
            ])
            .style(style)
        })
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Length(30), // Pod名称
            Constraint::Length(10), // CPU
            Constraint::Length(10), // 内存
            Constraint::Length(8),  // CPU%
            Constraint::Length(8),  // 内存%
        ]
    )
        .header(header)
        .block(Block::default().borders(Borders::ALL).title("Pod资源使用情况"))
        .row_highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));

    f.render_widget(table, area);
}

fn render_container_metrics_detail(f: &mut Frame, area: Rect, app: &AppState) {
    let selected_metrics = app.pod_metrics.get(app.metrics_scroll);
    
    if let Some(metrics) = selected_metrics {
        if metrics.containers.is_empty() {
            let no_containers = ratatui::widgets::Paragraph::new("此Pod没有容器级别的详细指标")
                .block(Block::default().borders(Borders::ALL).title(format!("Pod: {} - 容器详情", metrics.name)))
                .style(Style::default().fg(Color::Gray));
            
            f.render_widget(no_containers, area);
            return;
        }

        let header = Row::new(vec![
            Cell::from(Span::styled("容器名称", Style::default().add_modifier(Modifier::BOLD))),
            Cell::from(Span::styled("CPU", Style::default().add_modifier(Modifier::BOLD))),
            Cell::from(Span::styled("内存", Style::default().add_modifier(Modifier::BOLD))),
            Cell::from(Span::styled("CPU%", Style::default().add_modifier(Modifier::BOLD))),
            Cell::from(Span::styled("内存%", Style::default().add_modifier(Modifier::BOLD))),
        ])
        .style(Style::default().fg(Color::Yellow))
        .height(1);

        let rows: Vec<Row> = metrics.containers
            .iter()
            .map(|container| {
                Row::new(vec![
                    Cell::from(container.name.clone()),
                    Cell::from(format_cpu(&container.cpu)),
                    Cell::from(format_memory(&container.memory)),
                    Cell::from(format_percentage(container.cpu_percentage)),
                    Cell::from(format_percentage(container.memory_percentage)),
                ])
                .style(Style::default().fg(Color::White))
            })
            .collect();

        let table = Table::new(
            rows,
            [
                Constraint::Length(30), // 容器名称
                Constraint::Length(10), // CPU
                Constraint::Length(10), // 内存
                Constraint::Length(8),  // CPU%
                Constraint::Length(8),  // 内存%
            ]
        )
            .header(header)
            .block(Block::default().borders(Borders::ALL).title(format!("Pod: {} - 容器详情", metrics.name)))
            .row_highlight_style(Style::default().fg(Color::Cyan));

        f.render_widget(table, area);
    } else {
        let no_selection = ratatui::widgets::Paragraph::new("请选择一个Pod查看容器详情")
            .block(Block::default().borders(Borders::ALL).title("容器详情".to_string()))
            .style(Style::default().fg(Color::Gray));
        
        f.render_widget(no_selection, area);
    }
}

fn format_cpu(cpu_str: &str) -> String {
    if cpu_str.is_empty() {
        return "N/A".to_string();
    }
    
    // 如果CPU使用量很小，显示为毫核心(m)
    if let Ok(cpu_num) = cpu_str.replace("m", "").parse::<i32>() {
        if cpu_num < 1000 {
            return format!("{}m", cpu_num);
        } else {
            return format!("{:.2}", cpu_num as f64 / 1000.0);
        }
    }
    
    cpu_str.to_string()
}

fn format_memory(memory_str: &str) -> String {
    if memory_str.is_empty() {
        return "N/A".to_string();
    }
    
    // 转换内存单位为更易读的格式
    if memory_str.ends_with("Ki") {
        if let Ok(mem_kb) = memory_str.replace("Ki", "").parse::<i64>() {
            if mem_kb >= 1024 * 1024 {
                return format!("{:.1}Gi", mem_kb as f64 / (1024.0 * 1024.0));
            } else if mem_kb >= 1024 {
                return format!("{:.1}Mi", mem_kb as f64 / 1024.0);
            }
        }
    }
    
    memory_str.to_string()
}

fn format_percentage(percentage: Option<f64>) -> String {
    match percentage {
        Some(p) => format!("{:.1}%", p),
        None => "N/A".to_string(),
    }
}