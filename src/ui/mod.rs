pub mod components;
pub mod layout;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Clear, Paragraph, Tabs},
};

use crate::app::state::{AppMode, AppState};

pub fn render_ui(f: &mut Frame, app: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Main content
            Constraint::Length(3), // Footer
            Constraint::Length(1), // Command line
        ])
        .split(f.area());

    render_header(f, chunks[0], app);
    render_main_content(f, chunks[1], app);
    render_footer(f, chunks[2], app);
    render_command_line(f, chunks[3], app);
}

fn render_header(f: &mut Frame, area: Rect, app: &AppState) {
    let titles = vec![
        "Namespaces",
        "Pods",
        "Services",
        "Deployments",
        "Jobs",
        "PVCs",
        "PVs",
        "Nodes",
        "ConfigMaps",
        "DaemonSets",
        "Secrets",
        "Help",
    ];
    let index = match app.mode {
        AppMode::NamespaceList => 0,
        AppMode::PodList => 1,
        AppMode::ServiceList => 2,
        AppMode::DeploymentList => 3,
        AppMode::JobList => 4,
        AppMode::PVCList => 5,
        AppMode::PVList => 6,
        AppMode::NodeList => 7,
        AppMode::ConfigMapList => 8,
        AppMode::DaemonSetList => 9,
        AppMode::SecretList => 10,
        AppMode::Help => 11,
        AppMode::Logs | AppMode::Describe => {
            // 根据之前的模式显示正确的Tab高亮
            match app.previous_mode {
                AppMode::PodList => 1,
                AppMode::ServiceList => 2,
                AppMode::DeploymentList => 3,
                AppMode::JobList => 4,
                AppMode::PVCList => 5,
                AppMode::PVList => 6,
                AppMode::NodeList => 7,
                AppMode::ConfigMapList => 8,
                AppMode::DaemonSetList => 9,
                AppMode::SecretList => 10,
                _ => 1,
            }
        }
        AppMode::Search | AppMode::Confirm => match app.get_previous_mode() {
            AppMode::PodList => 1,
            AppMode::ServiceList => 2,
            AppMode::DeploymentList => 3,
            AppMode::JobList => 4,
            AppMode::PVCList => 5,
            AppMode::PVList => 6,
            AppMode::NodeList => 7,
            AppMode::ConfigMapList => 8,
            AppMode::DaemonSetList => 9,
            AppMode::SecretList => 10,
            _ => 0,
        },
        AppMode::YamlView | AppMode::TopView => match app.previous_mode {
            AppMode::PodList => 1,
            AppMode::ServiceList => 2,
            AppMode::DeploymentList => 3,
            AppMode::JobList => 4,
            AppMode::PVCList => 5,
            AppMode::PVList => 6,
            AppMode::NodeList => 7,
            AppMode::ConfigMapList => 8,
            AppMode::DaemonSetList => 9,
            AppMode::SecretList => 10,
            _ => 1,
        },
        AppMode::CommandHistory => 11,
    };

    let header_title = if app.global_refresh_enabled {
        if app.language_chinese {
            "Kube TUI [刷新:开]"
        } else {
            "Kube TUI [Refresh:ON]"
        }
    } else {
        if app.language_chinese {
            "Kube TUI [刷新:关]"
        } else {
            "Kube TUI [Refresh:OFF]"
        }
    };

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title(header_title))
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .select(index);

    f.render_widget(tabs, area);
}

fn render_main_content(f: &mut Frame, area: Rect, app: &AppState) {
    match app.mode {
        AppMode::NamespaceList => components::namespace_list::render(f, area, app),
        AppMode::PodList => components::pod_list::render(f, area, app),
        AppMode::ServiceList => components::service_list::render(f, area, app),
        AppMode::NodeList => components::node_list::render(f, area, app),
        AppMode::DeploymentList => components::deployment_list::render(f, area, app),
        AppMode::JobList => components::job_list::render(f, area, app),
        AppMode::DaemonSetList => components::daemonset_list::render(f, area, app),
        AppMode::PVCList => components::pvc_list::render(f, area, app),
        AppMode::PVList => components::pv_list::render(f, area, app),
        AppMode::ConfigMapList => components::configmap_list::render(f, area, app),
        AppMode::SecretList => components::secret_list::render(f, area, app),
        AppMode::Logs => components::logs::render(f, area, app),
        AppMode::Describe => components::describe::render(f, area, app),
        AppMode::Search => components::search::render(f, area, app),
        AppMode::Confirm => components::confirm::render(f, area, app),
        AppMode::Help => components::help::render(f, area, app),
        AppMode::YamlView => components::yaml_view::render(f, area, app),
        AppMode::TopView => components::top_view::render(f, area, app),
        AppMode::CommandHistory => {}
    }

    // 如果有 Context 选择弹窗，在内容之上叠加渲染
    if app.context_selection_mode {
        render_context_picker(f, area, app);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let popup_width = area.width * percent_x / 100;
    let popup_height = area.height * percent_y / 100;
    let x = area.x + (area.width.saturating_sub(popup_width)) / 2;
    let y = area.y + (area.height.saturating_sub(popup_height)) / 2;
    Rect::new(x, y, popup_width, popup_height)
}

fn render_context_picker(f: &mut Frame, area: Rect, app: &AppState) {
    let picker_area = centered_rect(60, 70, area);
    f.render_widget(Clear, picker_area);

    let context_lines: Vec<String> = app
        .available_contexts
        .iter()
        .enumerate()
        .map(|(i, ctx)| {
            let marker = if i == app.selected_context_index {
                "▶ "
            } else {
                "  "
            };
            format!("{}{}", marker, ctx)
        })
        .collect();

    let title = if app.language_chinese {
        "选择 kubeconfig Context (j/k:导航 Enter:切换 Esc:取消)"
    } else {
        "Select kubeconfig Context (j/k:nav Enter:switch Esc:cancel)"
    };

    let content = context_lines.join("\n");
    let paragraph = Paragraph::new(content)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .style(Style::default().fg(Color::Yellow)),
        )
        .style(Style::default().fg(Color::White))
        .scroll((
            app.selected_context_index
                .saturating_sub(picker_area.height.saturating_sub(3) as usize / 2)
                as u16,
            0,
        ));

    f.render_widget(paragraph, picker_area);
}

fn render_footer(f: &mut Frame, area: Rect, app: &AppState) {
    let help_text: String = if app.language_chinese {
        // 中文提示
        match app.mode {
            AppMode::NamespaceList => {
                if app.batch_mode {
                    format!(
                        "v 退出批量 | Space 标记 | Ctrl+A 全选 | d 删除 | Esc 取消 | 已标记: {}",
                        app.marked_items.len()
                    )
                } else {
                    "j/k ↑↓ 导航 • Enter 选择 • h/l ←→ 切换 • Tab/Shift+Tab 标签页 • / 搜索 • > 排序 • C 切换集群 • I 切换语言 • q 退出 • ? 帮助".to_string()
                }
            }
            AppMode::PodList => {
                if app.batch_mode {
                    format!(
                        "v 退出批量 | Space 标记 | Ctrl+A 全选 | d 删除 | Esc 取消 | 已标记: {}",
                        app.marked_items.len()
                    )
                } else {
                    "j/k 导航 • Space 详情 • Y YAML • T 监控 • L 日志 • P 端口转发 • D 删除 • E 进入 • v 批量 • / 搜索 • > 排序 • C 切换集群 • I 切换语言 • q 退出 • R 刷新".to_string()
                }
            }
            AppMode::ServiceList => {
                if app.batch_mode {
                    format!(
                        "v 退出批量 | Space 标记 | Ctrl+A 全选 | d 删除 | Esc 取消 | 已标记: {}",
                        app.marked_items.len()
                    )
                } else {
                    "j/k 导航 • Space 详情 • Y YAML • D 删除 • v 批量 • / 搜索 • > 排序 • C 切换集群 • I 切换语言 • q 退出 • R 刷新".to_string()
                }
            }
            AppMode::NodeList => {
                if app.batch_mode {
                    format!(
                        "v 退出批量 | Space 标记 | Ctrl+A 全选 | d 删除 | Esc 取消 | 已标记: {}",
                        app.marked_items.len()
                    )
                } else {
                    "j/k 导航 • Space 详情 • Y YAML • v 批量 • / 搜索 • > 排序 • C 切换集群 • I 切换语言 • q 退出 • R 刷新".to_string()
                }
            }
            AppMode::DeploymentList => {
                if app.batch_mode {
                    format!(
                        "v 退出批量 | Space 标记 | Ctrl+A 全选 | d 删除 | Esc 取消 | 已标记: {}",
                        app.marked_items.len()
                    )
                } else {
                    "j/k 导航 • Space 详情 • Y YAML • v 批量 • / 搜索 • > 排序 • C 切换集群 • I 切换语言 • q 退出 • R 刷新".to_string()
                }
            }
            AppMode::JobList => {
                if app.batch_mode {
                    format!(
                        "v 退出批量 | Space 标记 | Ctrl+A 全选 | d 删除 | Esc 取消 | 已标记: {}",
                        app.marked_items.len()
                    )
                } else {
                    "j/k 导航 • Space 详情 • Y YAML • v 批量 • / 搜索 • > 排序 • C 切换集群 • I 切换语言 • q 退出 • R 刷新".to_string()
                }
            }
            AppMode::DaemonSetList => {
                if app.batch_mode {
                    format!(
                        "v 退出批量 | Space 标记 | Ctrl+A 全选 | d 删除 | Esc 取消 | 已标记: {}",
                        app.marked_items.len()
                    )
                } else {
                    "j/k 导航 • Space 详情 • Y YAML • v 批量 • / 搜索 • > 排序 • C 切换集群 • I 切换语言 • q 退出 • R 刷新".to_string()
                }
            }
            AppMode::PVCList => {
                if app.batch_mode {
                    format!(
                        "v 退出批量 | Space 标记 | Ctrl+A 全选 | d 删除 | Esc 取消 | 已标记: {}",
                        app.marked_items.len()
                    )
                } else {
                    "j/k 导航 • Space 详情 • Y YAML • v 批量 • / 搜索 • > 排序 • C 切换集群 • I 切换语言 • q 退出 • R 刷新".to_string()
                }
            }
            AppMode::PVList => {
                if app.batch_mode {
                    format!(
                        "v 退出批量 | Space 标记 | Ctrl+A 全选 | d 删除 | Esc 取消 | 已标记: {}",
                        app.marked_items.len()
                    )
                } else {
                    "j/k 导航 • Space 详情 • Y YAML • v 批量 • / 搜索 • > 排序 • C 切换集群 • I 切换语言 • q 退出 • R 刷新".to_string()
                }
            }
            AppMode::ConfigMapList => {
                if app.batch_mode {
                    format!(
                        "v 退出批量 | Space 标记 | Ctrl+A 全选 | d 删除 | Esc 取消 | 已标记: {}",
                        app.marked_items.len()
                    )
                } else {
                    "j/k 导航 • Space 详情 • Y YAML • D 删除 • v 批量 • / 搜索 • > 排序 • C 切换集群 • I 切换语言 • q 退出 • R 刷新".to_string()
                }
            }
            AppMode::SecretList => {
                if app.batch_mode {
                    format!(
                        "v 退出批量 | Space 标记 | Ctrl+A 全选 | d 删除 | Esc 取消 | 已标记: {}",
                        app.marked_items.len()
                    )
                } else {
                    "j/k 导航 • Space 详情 • Y YAML • D 删除 • v 批量 • / 搜索 • > 排序 • C 切换集群 • I 切换语言 • q 退出 • R 刷新".to_string()
                }
            }
            AppMode::Logs => {
                let refresh_state = if app.logs_auto_refresh {
                    if app.language_chinese { "开" } else { "ON" }
                } else {
                    if app.language_chinese { "关" } else { "OFF" }
                };
                if app.language_chinese {
                    format!(
                        "j/k 滚动 • PgUp/PgDn 翻页 • V 分屏 • W 关窗格 • Tab 切换 • / 搜索 • A 自动滚动 • R 自动刷新[{}] • M 鼠标 • I 语言 • Esc 返回 • q 退出",
                        refresh_state
                    )
                } else {
                    format!(
                        "j/k Scroll • PgUp/PgDn Page • V Split • W Close • Tab Switch • / Search • A Auto-scroll • R Refresh[{}] • M Mouse • I Lang • Esc Back • q Quit",
                        refresh_state
                    )
                }
            }
            AppMode::Describe => {
                if app.text_selection_mode {
                    "j/k 滚动 • R 切换自动刷新 • M 切换到滚轮模式 • 可选中复制文本 • I 切换语言 • Esc 返回 • q 退出".to_string()
                } else {
                    "j/k 滚动 • R 切换自动刷新 • M 切换到选择模式 • 鼠标滚轮滚动 • I 切换语言 • Esc 返回 • q 退出".to_string()
                }
            }
            AppMode::YamlView => {
                if app.text_selection_mode {
                    "j/k 滚动 • R 切换自动刷新 • M 切换到滚轮模式 • 可选中复制文本 • I 切换语言 • Esc 返回 • q 退出".to_string()
                } else {
                    "j/k 滚动 • R 切换自动刷新 • M 切换到选择模式 • 鼠标滚轮滚动 • I 切换语言 • Esc 返回 • q 退出".to_string()
                }
            }
            AppMode::TopView => {
                "j/k 滚动 • PgUp/PgDn 翻页 • I 切换语言 • Esc 返回 • q 退出".to_string()
            }
            AppMode::Search => "输入搜索内容 • Enter 选择 • I 切换语言 • Esc 取消".to_string(),
            AppMode::Confirm => "y/Y 确认 • n/N/Esc 取消".to_string(),
            AppMode::Help => "I 切换语言 • Esc 返回 • q 退出".to_string(),
            AppMode::CommandHistory => {
                "j/k 选择 • Enter 执行 • I 切换语言 • Esc 返回 • q 退出".to_string()
            }
        }
    } else {
        // English prompts
        match app.mode {
            AppMode::NamespaceList => {
                if app.batch_mode {
                    format!(
                        "v Exit Batch | Space Mark | Ctrl+A All | d Delete | Esc Cancel | Marked: {}",
                        app.marked_items.len()
                    )
                } else {
                    "j/k ↑↓ Navigate • Enter Select • h/l ←→ Switch • Tab/Shift+Tab Tabs • / Search • > Sort • C Switch Cluster • I Language • q Quit • ? Help".to_string()
                }
            }
            AppMode::PodList => {
                if app.batch_mode {
                    format!(
                        "v Exit Batch | Space Mark | Ctrl+A All | d Delete | Esc Cancel | Marked: {}",
                        app.marked_items.len()
                    )
                } else {
                    "j/k Navigate • Space Describe • Y YAML • T Top • L Logs • P Port-Forward • D Delete • E Exec • v Batch • / Search • > Sort • C Switch Cluster • I Language • q Quit • R Refresh".to_string()
                }
            }
            AppMode::ServiceList => {
                if app.batch_mode {
                    format!(
                        "v Exit Batch | Space Mark | Ctrl+A All | d Delete | Esc Cancel | Marked: {}",
                        app.marked_items.len()
                    )
                } else {
                    "j/k Navigate • Space Describe • Y YAML • D Delete • v Batch • / Search • > Sort • C Switch Cluster • I Language • q Quit • R Refresh".to_string()
                }
            }
            AppMode::NodeList => {
                if app.batch_mode {
                    format!(
                        "v Exit Batch | Space Mark | Ctrl+A All | d Delete | Esc Cancel | Marked: {}",
                        app.marked_items.len()
                    )
                } else {
                    "j/k Navigate • Space Describe • Y YAML • v Batch • / Search • > Sort • C Switch Cluster • I Language • q Quit • R Refresh".to_string()
                }
            }
            AppMode::DeploymentList => {
                if app.batch_mode {
                    format!(
                        "v Exit Batch | Space Mark | Ctrl+A All | d Delete | Esc Cancel | Marked: {}",
                        app.marked_items.len()
                    )
                } else {
                    "j/k Navigate • Space Describe • Y YAML • v Batch • / Search • > Sort • C Switch Cluster • I Language • q Quit • R Refresh".to_string()
                }
            }
            AppMode::JobList => {
                if app.batch_mode {
                    format!(
                        "v Exit Batch | Space Mark | Ctrl+A All | d Delete | Esc Cancel | Marked: {}",
                        app.marked_items.len()
                    )
                } else {
                    "j/k Navigate • Space Describe • Y YAML • v Batch • / Search • > Sort • C Switch Cluster • I Language • q Quit • R Refresh".to_string()
                }
            }
            AppMode::DaemonSetList => {
                if app.batch_mode {
                    format!(
                        "v Exit Batch | Space Mark | Ctrl+A All | d Delete | Esc Cancel | Marked: {}",
                        app.marked_items.len()
                    )
                } else {
                    "j/k Navigate • Space Describe • Y YAML • v Batch • / Search • > Sort • C Switch Cluster • I Language • q Quit • R Refresh".to_string()
                }
            }
            AppMode::PVCList => {
                if app.batch_mode {
                    format!(
                        "v Exit Batch | Space Mark | Ctrl+A All | d Delete | Esc Cancel | Marked: {}",
                        app.marked_items.len()
                    )
                } else {
                    "j/k Navigate • Space Describe • Y YAML • v Batch • / Search • > Sort • C Switch Cluster • I Language • q Quit • R Refresh".to_string()
                }
            }
            AppMode::PVList => {
                if app.batch_mode {
                    format!(
                        "v Exit Batch | Space Mark | Ctrl+A All | d Delete | Esc Cancel | Marked: {}",
                        app.marked_items.len()
                    )
                } else {
                    "j/k Navigate • Space Describe • Y YAML • v Batch • / Search • > Sort • C Switch Cluster • I Language • q Quit • R Refresh".to_string()
                }
            }
            AppMode::ConfigMapList => {
                if app.batch_mode {
                    format!(
                        "v Exit Batch | Space Mark | Ctrl+A All | d Delete | Esc Cancel | Marked: {}",
                        app.marked_items.len()
                    )
                } else {
                    "j/k Navigate • Space Describe • Y YAML • D Delete • v Batch • / Search • > Sort • C Switch Cluster • I Language • q Quit • R Refresh".to_string()
                }
            }
            AppMode::SecretList => {
                if app.batch_mode {
                    format!(
                        "v Exit Batch | Space Mark | Ctrl+A All | d Delete | Esc Cancel | Marked: {}",
                        app.marked_items.len()
                    )
                } else {
                    "j/k Navigate • Space Describe • Y YAML • D Delete • v Batch • / Search • > Sort • C Switch Cluster • I Language • q Quit • R Refresh".to_string()
                }
            }
            AppMode::Logs => {
                let refresh_state = if app.logs_auto_refresh {
                    if app.language_chinese { "开" } else { "ON" }
                } else {
                    if app.language_chinese { "关" } else { "OFF" }
                };
                if app.language_chinese {
                    format!(
                        "j/k 滚动 • PgUp/PgDn 翻页 • V 分屏 • W 关窗格 • Tab 切换 • / 搜索 • A 自动滚动 • R 自动刷新[{}] • M 鼠标 • I 语言 • Esc 返回 • q 退出",
                        refresh_state
                    )
                } else {
                    format!(
                        "j/k Scroll • PgUp/PgDn Page • V Split • W Close • Tab Switch • / Search • A Auto-scroll • R Refresh[{}] • M Mouse • I Lang • Esc Back • q Quit",
                        refresh_state
                    )
                }
            }
            AppMode::Describe => {
                if app.text_selection_mode {
                    "j/k Scroll • R Toggle Auto-refresh • M Switch to scroll mode • Can select text • I Language • Esc Back • q Quit".to_string()
                } else {
                    "j/k Scroll • R Toggle Auto-refresh • M Switch to select mode • Mouse wheel scroll • I Language • Esc Back • q Quit".to_string()
                }
            }
            AppMode::YamlView => {
                if app.text_selection_mode {
                    "j/k Scroll • R Toggle Auto-refresh • M Switch to scroll mode • Can select text • I Language • Esc Back • q Quit".to_string()
                } else {
                    "j/k Scroll • R Toggle Auto-refresh • M Switch to select mode • Mouse wheel scroll • I Language • Esc Back • q Quit".to_string()
                }
            }
            AppMode::TopView => {
                "j/k Scroll • PgUp/PgDn Page • I Language • Esc Back • q Quit".to_string()
            }
            AppMode::Search => {
                "Type to search • Enter Select • I Language • Esc Cancel".to_string()
            }
            AppMode::Confirm => "y/Y Confirm • n/N/Esc Cancel".to_string(),
            AppMode::Help => "I Language • Esc Back • q Quit".to_string(),
            AppMode::CommandHistory => {
                "j/k Select • Enter Execute • I Language • Esc Back • q Quit".to_string()
            }
        }
    };

    let mode_tag = if app.batch_mode {
        if app.language_chinese {
            "[批量模式] "
        } else {
            "[BATCH] "
        }
    } else if app.search_mode || app.log_search_mode {
        if app.language_chinese {
            "[搜索模式] "
        } else {
            "[SEARCH] "
        }
    } else if app.context_selection_mode || app.split_pod_selection_mode {
        if app.language_chinese {
            "[选择模式] "
        } else {
            "[SELECT] "
        }
    } else {
        ""
    };
    let help_text = format!("{}{}", mode_tag, help_text);

    let footer = Paragraph::new(help_text)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::Gray));

    f.render_widget(footer, area);
}

fn render_command_line(f: &mut Frame, area: Rect, app: &AppState) {
    let status_line = if !app.current_command.is_empty() {
        format!("> {}", app.current_command)
    } else {
        let mut parts = vec![];
        if !app.active_port_forwards.is_empty() {
            parts.push(format!("PF:{}", app.active_port_forwards.len()));
        }
        if !app.current_context.is_empty() {
            parts.push(format!("ctx:{}", app.current_context));
        }
        if !app.refresh_status_text.is_empty() {
            parts.push(app.refresh_status_text.clone());
        }
        parts.join(" | ")
    };

    let command_line = Paragraph::new(status_line).style(Style::default().fg(Color::Cyan));

    f.render_widget(command_line, area);
}
