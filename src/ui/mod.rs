pub mod components;
pub mod layout;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph, Tabs},
    Frame,
};

use crate::app::{AppMode, AppState};

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
    // 根据使用频次重新设计标签页，只显示核心资源类型
    let titles = vec!["Namespaces", "Pods", "Services", "Deployments", "More Resources", "Help"];
    let index = match app.mode {
        AppMode::NamespaceList => 0,
        AppMode::PodList => 1,
        AppMode::ServiceList => 2,
        AppMode::DeploymentList => 3,
        // 更多资源面板
        AppMode::MoreResources => 4,
        AppMode::Help => 5,
        // 其他资源类型面板在标签页中不直接显示，但需要处理
        AppMode::JobList | AppMode::PVCList | AppMode::PVList | AppMode::NodeList | 
        AppMode::ConfigMapList | AppMode::DaemonSetList | AppMode::SecretList => 4, // 显示为More Resources
        // 视图模式保持原有的高亮逻辑
        AppMode::Logs | AppMode::Describe | AppMode::YamlView | AppMode::TopView => {
            // 根据之前的模式显示正确的Tab高亮
            match app.previous_mode {
                AppMode::NamespaceList => 0,
                AppMode::PodList => 1,
                AppMode::ServiceList => 2,
                AppMode::DeploymentList => 3,
                AppMode::MoreResources | AppMode::JobList | AppMode::PVCList | AppMode::PVList | 
                AppMode::NodeList | AppMode::ConfigMapList | AppMode::DaemonSetList | AppMode::SecretList => 4,
                AppMode::Help => 5,
                _ => 1,
            }
        }
        AppMode::Search | AppMode::Confirm => match app.get_previous_mode() {
            AppMode::NamespaceList => 0,
            AppMode::PodList => 1,
            AppMode::ServiceList => 2,
            AppMode::DeploymentList => 3,
            AppMode::MoreResources | AppMode::JobList | AppMode::PVCList | AppMode::PVList | 
            AppMode::NodeList | AppMode::ConfigMapList | AppMode::DaemonSetList | AppMode::SecretList => 4,
            AppMode::Help => 5,
            _ => 0,
        },
    };

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("Kube TUI"))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
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
        // 新增更多资源面板
        AppMode::MoreResources => components::more_resources::render(f, area, app),
    }
}

fn render_footer(f: &mut Frame, area: Rect, app: &AppState) {
    let help_text = if app.language_chinese {
        // 中文提示
        match app.mode {
            AppMode::NamespaceList => "j/k ↑↓ 导航 • Enter 选择 • Tab/Shift+Tab 切换面板 • F1-F7 快速访问 • / 搜索 • I 切换语言 • q 退出 • ? 帮助",
            AppMode::PodList => "j/k 导航 • Space 详情 • Y YAML • T 监控 • L 日志 • D 删除 • E 进入 • / 搜索 • I 切换语言 • q 退出 • R 刷新",
            AppMode::ServiceList => "j/k 导航 • Space 详情 • Y YAML • D 删除 • / 搜索 • I 切换语言 • q 退出 • R 刷新",
            AppMode::DeploymentList => "j/k 导航 • Space 详情 • Y YAML • / 搜索 • I 切换语言 • q 退出 • R 刷新",
            AppMode::MoreResources => "1-7 访问资源 • Tab/Shift+Tab 切换面板 • Esc 返回 • q 退出",
            AppMode::JobList => "j/k 导航 • Space 详情 • Y YAML • / 搜索 • I 切换语言 • q 退出 • R 刷新",
            AppMode::DaemonSetList => "j/k 导航 • Space 详情 • Y YAML • / 搜索 • I 切换语言 • q 退出 • R 刷新",
            AppMode::PVCList => "j/k 导航 • Space 详情 • Y YAML • / 搜索 • I 切换语言 • q 退出 • R 刷新",
            AppMode::PVList => "j/k 导航 • Space 详情 • Y YAML • / 搜索 • I 切换语言 • q 退出 • R 刷新",
            AppMode::NodeList => "j/k 导航 • Space 详情 • Y YAML • / 搜索 • I 切换语言 • q 退出 • R 刷新",
            AppMode::ConfigMapList => "j/k 导航 • Space 详情 • Y YAML • D 删除 • / 搜索 • I 切换语言 • q 退出 • R 刷新",
            AppMode::SecretList => "j/k 导航 • Space 详情 • Y YAML • D 删除 • / 搜索 • I 切换语言 • q 退出 • R 刷新",
            AppMode::Logs => {
                if app.text_selection_mode {
                    "j/k 滚动 • PgUp/PgDn 翻页 • A 切换自动滚动 • R 切换自动刷新 • M 切换到滚轮模式 • 可选中复制文本 • I 切换语言 • Esc 返回 • q 退出"
                } else {
                    "j/k 滚动 • PgUp/PgDn 翻页 • A 切换自动滚动 • R 切换自动刷新 • M 切换到选择模式 • 鼠标滚轮滚动 • I 切换语言 • Esc 返回 • q 退出"
                }
            },
            AppMode::Describe => {
                if app.text_selection_mode {
                    "j/k 滚动 • R 切换自动刷新 • M 切换到滚轮模式 • 可选中复制文本 • I 切换语言 • Esc 返回 • q 退出"
                } else {
                    "j/k 滚动 • R 切换自动刷新 • M 切换到选择模式 • 鼠标滚轮滚动 • I 切换语言 • Esc 返回 • q 退出"
                }
            },
            AppMode::YamlView => {
                if app.text_selection_mode {
                    "j/k 滚动 • R 切换自动刷新 • M 切换到滚轮模式 • 可选中复制文本 • I 切换语言 • Esc 返回 • q 退出"
                } else {
                    "j/k 滚动 • R 切换自动刷新 • M 切换到选择模式 • 鼠标滚轮滚动 • I 切换语言 • Esc 返回 • q 退出"
                }
            },
            AppMode::TopView => "j/k 滚动 • PgUp/PgDn 翻页 • I 切换语言 • Esc 返回 • q 退出",
            AppMode::Search => "输入搜索内容 • Enter 选择 • I 切换语言 • Esc 取消",
            AppMode::Confirm => "y/Y 确认 • n/N/Esc 取消",
            AppMode::Help => "I 切换语言 • Esc 返回 • q 退出",
        }
    } else {
        // English prompts
        match app.mode {
            AppMode::NamespaceList => "j/k ↑↓ Navigate • Enter Select • Tab/Shift+Tab Switch panels • F1-F7 Quick access • / Search • I Language • q Quit • ? Help",
            AppMode::PodList => "j/k Navigate • Space Describe • Y YAML • T Top • L Logs • D Delete • E Exec • / Search • I Language • q Quit • R Refresh",
            AppMode::ServiceList => "j/k Navigate • Space Describe • Y YAML • D Delete • / Search • I Language • q Quit • R Refresh",
            AppMode::DeploymentList => "j/k Navigate • Space Describe • Y YAML • / Search • I Language • q Quit • R Refresh",
            AppMode::MoreResources => "1-7 Access resources • Tab/Shift+Tab Switch panels • Esc Back • q Quit",
            AppMode::JobList => "j/k Navigate • Space Describe • Y YAML • / Search • I Language • q Quit • R Refresh",
            AppMode::DaemonSetList => "j/k Navigate • Space Describe • Y YAML • / Search • I Language • q Quit • R Refresh",
            AppMode::PVCList => "j/k Navigate • Space Describe • Y YAML • / Search • I Language • q Quit • R Refresh",
            AppMode::PVList => "j/k Navigate • Space Describe • Y YAML • / Search • I Language • q Quit • R Refresh",
            AppMode::NodeList => "j/k Navigate • Space Describe • Y YAML • / Search • I Language • q Quit • R Refresh",
            AppMode::ConfigMapList => "j/k Navigate • Space Describe • Y YAML • D Delete • / Search • I Language • q Quit • R Refresh",
            AppMode::SecretList => "j/k Navigate • Space Describe • Y YAML • D Delete • / Search • I Language • q Quit • R Refresh",
            AppMode::Logs => {
                if app.text_selection_mode {
                    "j/k Scroll • PgUp/PgDn Page • A Toggle Auto-scroll • R Toggle Auto-refresh • M Switch to scroll mode • Can select text • I Language • Esc Back • q Quit"
                } else {
                    "j/k Scroll • PgUp/PgDn Page • A Toggle Auto-scroll • R Toggle Auto-refresh • M Switch to select mode • Mouse wheel scroll • I Language • Esc Back • q Quit"
                }
            },
            AppMode::Describe => {
                if app.text_selection_mode {
                    "j/k Scroll • R Toggle Auto-refresh • M Switch to scroll mode • Can select text • I Language • Esc Back • q Quit"
                } else {
                    "j/k Scroll • R Toggle Auto-refresh • M Switch to select mode • Mouse wheel scroll • I Language • Esc Back • q Quit"
                }
            },
            AppMode::YamlView => {
                if app.text_selection_mode {
                    "j/k Scroll • R Toggle Auto-refresh • M Switch to scroll mode • Can select text • I Language • Esc Back • q Quit"
                } else {
                    "j/k Scroll • R Toggle Auto-refresh • M Switch to select mode • Mouse wheel scroll • I Language • Esc Back • q Quit"
                }
            },
            AppMode::TopView => "j/k Scroll • PgUp/PgDn Page • I Language • Esc Back • q Quit",
            AppMode::Search => "Type to search • Enter Select • I Language • Esc Cancel",
            AppMode::Confirm => "y/Y Confirm • n/N/Esc Cancel",
            AppMode::Help => "I Language • Esc Back • q Quit",
        }
    };

    let footer = Paragraph::new(help_text)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::Gray));

    f.render_widget(footer, area);
}

fn render_command_line(f: &mut Frame, area: Rect, app: &AppState) {
    let command_text = if !app.current_command.is_empty() {
        format!("Executing: {}", app.current_command)
    } else {
        // 在空闲时显示当前模式的相关命令提示和刷新状态
        match app.mode {
            AppMode::Logs => {
                if let Some(pod) = app.get_selected_pod() {
                    format!("kubectl logs -f -n {} {} --tail=100", app.current_namespace, pod.name)
                } else {
                    "Logs Mode - No pod selected".to_string()
                }
            },
            AppMode::Describe => {
                match app.previous_mode {
                    AppMode::PodList => {
                        if let Some(pod) = app.get_selected_pod() {
                            format!("kubectl describe pod -n {} {}", app.current_namespace, pod.name)
                        } else {
                            "Describe Mode - No pod selected".to_string()
                        }
                    },
                    AppMode::ServiceList => {
                        if let Some(service) = app.get_selected_service() {
                            format!("kubectl describe service -n {} {}", app.current_namespace, service.name)
                        } else {
                            "Describe Mode - No service selected".to_string()
                        }
                    },
                    AppMode::DeploymentList => {
                        if let Some(deployment) = app.get_selected_deployment() {
                            format!("kubectl describe deployment -n {} {}", app.current_namespace, deployment.name)
                        } else {
                            "Describe Mode - No deployment selected".to_string()
                        }
                    },
                    AppMode::JobList => {
                        if let Some(job) = app.get_selected_job() {
                            format!("kubectl describe job -n {} {}", app.current_namespace, job.name)
                        } else {
                            "Describe Mode - No job selected".to_string()
                        }
                    },
                    AppMode::NodeList => {
                        if let Some(node) = app.get_selected_node() {
                            format!("kubectl describe node {}", node.name)
                        } else {
                            "Describe Mode - No node selected".to_string()
                        }
                    },
                    AppMode::DaemonSetList => {
                        if let Some(daemonset) = app.get_selected_daemonset() {
                            format!("kubectl describe daemonset -n {} {}", app.current_namespace, daemonset.name)
                        } else {
                            "Describe Mode - No daemonset selected".to_string()
                        }
                    },
                    AppMode::ConfigMapList => {
                        if let Some(configmap) = app.get_selected_configmap() {
                            format!("kubectl describe configmap -n {} {}", app.current_namespace, configmap.name)
                        } else {
                            "Describe Mode - No configmap selected".to_string()
                        }
                    },
                    AppMode::SecretList => {
                        if let Some(secret) = app.get_selected_secret() {
                            format!("kubectl describe secret -n {} {}", app.current_namespace, secret.name)
                        } else {
                            "Describe Mode - No secret selected".to_string()
                        }
                    },
                    AppMode::PVCList => {
                        if let Some(pvc) = app.get_selected_pvc() {
                            format!("kubectl describe pvc -n {} {}", app.current_namespace, pvc.name)
                        } else {
                            "Describe Mode - No pvc selected".to_string()
                        }
                    },
                    AppMode::PVList => {
                        if let Some(pv) = app.get_selected_pv() {
                            format!("kubectl describe pv {}", pv.name)
                        } else {
                            "Describe Mode - No pv selected".to_string()
                        }
                    },
                    _ => "Describe Mode".to_string(),
                }
            },
            AppMode::YamlView => {
                match app.previous_mode {
                    AppMode::PodList => {
                        if let Some(pod) = app.get_selected_pod() {
                            format!("kubectl get pod -n {} {} -o yaml", app.current_namespace, pod.name)
                        } else {
                            "YAML View Mode - No pod selected".to_string()
                        }
                    },
                    AppMode::ServiceList => {
                        if let Some(service) = app.get_selected_service() {
                            format!("kubectl get service -n {} {} -o yaml", app.current_namespace, service.name)
                        } else {
                            "YAML View Mode - No service selected".to_string()
                        }
                    },
                    AppMode::DeploymentList => {
                        if let Some(deployment) = app.get_selected_deployment() {
                            format!("kubectl get deployment -n {} {} -o yaml", app.current_namespace, deployment.name)
                        } else {
                            "YAML View Mode - No deployment selected".to_string()
                        }
                    },
                    AppMode::NodeList => {
                        if let Some(node) = app.get_selected_node() {
                            format!("kubectl get node {} -o yaml", node.name)
                        } else {
                            "YAML View Mode - No node selected".to_string()
                        }
                    },
                    _ => "YAML View Mode".to_string(),
                }
            },
            AppMode::TopView => {
                format!("kubectl top pods -n {}", app.current_namespace)
            },
            AppMode::Search => "Search Mode".to_string(),
            AppMode::NamespaceList => "kubectl get namespaces".to_string(),
            AppMode::PodList => format!("kubectl get pods -n {}", app.current_namespace),
            AppMode::ServiceList => format!("kubectl get services -n {}", app.current_namespace),
            AppMode::DeploymentList => format!("kubectl get deployments -n {}", app.current_namespace),
            AppMode::JobList => format!("kubectl get jobs -n {}", app.current_namespace),
            AppMode::DaemonSetList => format!("kubectl get daemonsets -n {}", app.current_namespace),
            AppMode::NodeList => "kubectl get nodes".to_string(),
            AppMode::ConfigMapList => format!("kubectl get configmaps -n {}", app.current_namespace),
            AppMode::SecretList => format!("kubectl get secrets -n {}", app.current_namespace),
            AppMode::PVCList => format!("kubectl get pvc -n {}", app.current_namespace),
            AppMode::PVList => "kubectl get pv".to_string(),
            AppMode::Confirm => "Confirmation Mode".to_string(),
            AppMode::Help => "Help Mode".to_string(),
            // 新增更多资源面板模式
            AppMode::MoreResources => "More Resources Panel".to_string(),
        }
    };

    // 添加刷新状态显示
    let status_text = if !app.refresh_status_text.is_empty() {
        format!("{} - Press 'R' to refresh {}", command_text, app.refresh_status_text)
    } else {
        format!("{} - Press 'R' to refresh", command_text)
    };

    let command_line = Paragraph::new(status_text)
        .style(Style::default().fg(Color::Cyan));

    f.render_widget(command_line, area);
}