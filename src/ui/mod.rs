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
        .split(f.size());

    render_header(f, chunks[0], app);
    render_main_content(f, chunks[1], app);
    render_footer(f, chunks[2], app);
    render_command_line(f, chunks[3], app);
}

fn render_header(f: &mut Frame, area: Rect, app: &AppState) {
    let titles = vec!["Namespaces", "Pods", "Services", "Deployments", "Jobs", "PVCs", "PVs", "Nodes", "ConfigMaps", "DaemonSets", "Secrets", "Help"];
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
    }
}

fn render_footer(f: &mut Frame, area: Rect, app: &AppState) {
    let help_text = if app.language_chinese {
        // 中文提示
        match app.mode {
            AppMode::NamespaceList => "j/k ↑↓ 导航 • Enter 选择 • h/l ←→ 切换 • Tab/Shift+Tab 标签页 • I 切换语言 • q 退出 • ? 帮助",
            AppMode::PodList => "j/k 导航 • Space 详情 • Y YAML • T 监控 • L 日志 • D 删除 • E 进入 • / 搜索 • I 切换语言 • q 退出",
            AppMode::ServiceList => "j/k 导航 • Space 详情 • Y YAML • D 删除 • / 搜索 • I 切换语言 • q 退出",
            AppMode::NodeList => "j/k 导航 • Space 详情 • Y YAML • / 搜索 • I 切换语言 • q 退出",
            AppMode::DeploymentList => "j/k 导航 • Space 详情 • Y YAML • / 搜索 • I 切换语言 • q 退出",
            AppMode::JobList => "j/k 导航 • Space 详情 • Y YAML • / 搜索 • I 切换语言 • q 退出",
            AppMode::DaemonSetList => "j/k 导航 • Space 详情 • Y YAML • / 搜索 • I 切换语言 • q 退出",
            AppMode::PVCList => "j/k 导航 • Space 详情 • Y YAML • / 搜索 • I 切换语言 • q 退出",
            AppMode::PVList => "j/k 导航 • Space 详情 • Y YAML • / 搜索 • I 切换语言 • q 退出",
            AppMode::ConfigMapList => "j/k 导航 • Space 详情 • Y YAML • D 删除 • / 搜索 • I 切换语言 • q 退出",
            AppMode::SecretList => "j/k 导航 • Space 详情 • Y YAML • D 删除 • / 搜索 • I 切换语言 • q 退出",
            AppMode::Logs => "J/K 滚动 • PgUp/PgDn 翻页 • A 切换自动滚动 • R 切换自动刷新 • I 切换语言 • Esc 返回 • q 退出",
            AppMode::Describe => {
                if app.text_selection_mode {
                    "J/K 滚动 • M 切换到滚轮模式 • 可选中复制文本 • I 切换语言 • Esc 返回 • q 退出"
                } else {
                    "J/K 滚动 • M 切换到选择模式 • 鼠标滚轮滚动 • I 切换语言 • Esc 返回 • q 退出"
                }
            },
            AppMode::YamlView => {
                if app.text_selection_mode {
                    "J/K 滚动 • M 切换到滚轮模式 • 可选中复制文本 • I 切换语言 • Esc 返回 • q 退出"
                } else {
                    "J/K 滚动 • M 切换到选择模式 • 鼠标滚轮滚动 • I 切换语言 • Esc 返回 • q 退出"
                }
            },
            AppMode::TopView => "J/K 滚动 • PgUp/PgDn 翻页 • I 切换语言 • Esc 返回 • q 退出",
            AppMode::Search => "输入搜索内容 • Enter 选择 • I 切换语言 • Esc 取消",
            AppMode::Confirm => "y/Y 确认 • n/N/Esc 取消",
            AppMode::Help => "I 切换语言 • Esc 返回 • q 退出",
        }
    } else {
        // English prompts
        match app.mode {
            AppMode::NamespaceList => "j/k ↑↓ Navigate • Enter Select • h/l ←→ Switch • Tab/Shift+Tab Tabs • I Language • q Quit • ? Help",
            AppMode::PodList => "j/k Navigate • Space Describe • Y YAML • T Top • L Logs • D Delete • E Exec • / Search • I Language • q Quit",
            AppMode::ServiceList => "j/k Navigate • Space Describe • Y YAML • D Delete • / Search • I Language • q Quit",
            AppMode::NodeList => "j/k Navigate • Space Describe • Y YAML • / Search • I Language • q Quit",
            AppMode::DeploymentList => "j/k Navigate • Space Describe • Y YAML • / Search • I Language • q Quit",
            AppMode::JobList => "j/k Navigate • Space Describe • Y YAML • / Search • I Language • q Quit",
            AppMode::DaemonSetList => "j/k Navigate • Space Describe • Y YAML • / Search • I Language • q Quit",
            AppMode::PVCList => "j/k Navigate • Space Describe • Y YAML • / Search • I Language • q Quit",
            AppMode::PVList => "j/k Navigate • Space Describe • Y YAML • / Search • I Language • q Quit",
            AppMode::ConfigMapList => "j/k Navigate • Space Describe • Y YAML • D Delete • / Search • I Language • q Quit",
            AppMode::SecretList => "j/k Navigate • Space Describe • Y YAML • D Delete • / Search • I Language • q Quit",
            AppMode::Logs => "J/K Scroll • PgUp/PgDn Page • A Toggle Auto-scroll • R Toggle Auto-refresh • I Language • Esc Back • q Quit",
            AppMode::Describe => {
                if app.text_selection_mode {
                    "J/K Scroll • M Switch to scroll mode • Can select text • I Language • Esc Back • q Quit"
                } else {
                    "J/K Scroll • M Switch to select mode • Mouse wheel scroll • I Language • Esc Back • q Quit"
                }
            },
            AppMode::YamlView => {
                if app.text_selection_mode {
                    "J/K Scroll • M Switch to scroll mode • Can select text • I Language • Esc Back • q Quit"
                } else {
                    "J/K Scroll • M Switch to select mode • Mouse wheel scroll • I Language • Esc Back • q Quit"
                }
            },
            AppMode::TopView => "J/K Scroll • PgUp/PgDn Page • I Language • Esc Back • q Quit",
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
        // 在空闲时显示当前模式的相关命令提示
        match app.mode {
            AppMode::PodList => format!("Ready - Use: Space (describe), Y (yaml), T (top), L (logs), E (exec), D (delete), / (search) - kubectl get pods -n {}", app.current_namespace),
            AppMode::ServiceList => format!("Ready - Use: Space (describe), Y (yaml), D (delete), / (search) - kubectl get services -n {}", app.current_namespace),
            AppMode::DeploymentList => format!("Ready - Use: Space (describe), Y (yaml), D (delete), / (search) - kubectl get deployments -n {}", app.current_namespace),
            AppMode::JobList => format!("Ready - Use: Space (describe), Y (yaml), D (delete), / (search) - kubectl get jobs -n {}", app.current_namespace),
            AppMode::DaemonSetList => format!("Ready - Use: Space (describe), Y (yaml), D (delete), / (search) - kubectl get daemonsets -n {}", app.current_namespace),
            AppMode::NodeList => "Ready - Use: Space (describe), Y (yaml), / (search) - kubectl get nodes".to_string(),
            AppMode::ConfigMapList => format!("Ready - Use: Space (describe), Y (yaml), D (delete), / (search) - kubectl get configmaps -n {}", app.current_namespace),
            AppMode::SecretList => format!("Ready - Use: Space (describe), Y (yaml), D (delete), / (search) - kubectl get secrets -n {}", app.current_namespace),
            AppMode::PVCList => format!("Ready - Use: Space (describe), Y (yaml), D (delete), / (search) - kubectl get pvc -n {}", app.current_namespace),
            AppMode::PVList => "Ready - Use: Space (describe), Y (yaml), D (delete), / (search) - kubectl get pv".to_string(),
            AppMode::Logs => {
                if let Some(pod) = app.get_selected_pod() {
                    format!("Logs Mode - J/K (scroll), A (auto-scroll), R (auto-refresh) - kubectl logs -f -n {} {} --tail=100", app.current_namespace, pod.name)
                } else {
                    "Logs Mode - No pod selected".to_string()
                }
            },
            AppMode::Describe => {
                match app.previous_mode {
                    AppMode::PodList => {
                        if let Some(pod) = app.get_selected_pod() {
                            format!("Describe Mode - J/K (scroll), Esc (back) - kubectl describe pod -n {} {}", app.current_namespace, pod.name)
                        } else {
                            "Describe Mode - No pod selected".to_string()
                        }
                    },
                    AppMode::ServiceList => {
                        if let Some(service) = app.get_selected_service() {
                            format!("Describe Mode - J/K (scroll), Esc (back) - kubectl describe service -n {} {}", app.current_namespace, service.name)
                        } else {
                            "Describe Mode - No service selected".to_string()
                        }
                    },
                    AppMode::DeploymentList => {
                        if let Some(deployment) = app.get_selected_deployment() {
                            format!("Describe Mode - J/K (scroll), Esc (back) - kubectl describe deployment -n {} {}", app.current_namespace, deployment.name)
                        } else {
                            "Describe Mode - No deployment selected".to_string()
                        }
                    },
                    AppMode::JobList => {
                        if let Some(job) = app.get_selected_job() {
                            format!("Describe Mode - J/K (scroll), Esc (back) - kubectl describe job -n {} {}", app.current_namespace, job.name)
                        } else {
                            "Describe Mode - No job selected".to_string()
                        }
                    },
                    AppMode::NodeList => {
                        if let Some(node) = app.get_selected_node() {
                            format!("Describe Mode - J/K (scroll), Esc (back) - kubectl describe node {}", node.name)
                        } else {
                            "Describe Mode - No node selected".to_string()
                        }
                    },
                    _ => "Describe Mode - J/K (scroll), Esc (back)".to_string(),
                }
            },
            AppMode::YamlView => {
                match app.previous_mode {
                    AppMode::PodList => {
                        if let Some(pod) = app.get_selected_pod() {
                            format!("YAML View Mode - J/K (scroll), Esc (back) - kubectl get pod -n {} {} -o yaml", app.current_namespace, pod.name)
                        } else {
                            "YAML View Mode - No pod selected".to_string()
                        }
                    },
                    AppMode::ServiceList => {
                        if let Some(service) = app.get_selected_service() {
                            format!("YAML View Mode - J/K (scroll), Esc (back) - kubectl get service -n {} {} -o yaml", app.current_namespace, service.name)
                        } else {
                            "YAML View Mode - No service selected".to_string()
                        }
                    },
                    AppMode::DeploymentList => {
                        if let Some(deployment) = app.get_selected_deployment() {
                            format!("YAML View Mode - J/K (scroll), Esc (back) - kubectl get deployment -n {} {} -o yaml", app.current_namespace, deployment.name)
                        } else {
                            "YAML View Mode - No deployment selected".to_string()
                        }
                    },
                    AppMode::NodeList => {
                        if let Some(node) = app.get_selected_node() {
                            format!("YAML View Mode - J/K (scroll), Esc (back) - kubectl get node {} -o yaml", node.name)
                        } else {
                            "YAML View Mode - No node selected".to_string()
                        }
                    },
                    _ => "YAML View Mode - J/K (scroll), Esc (back)".to_string(),
                }
            },
            AppMode::TopView => {
                format!("Resource Usage Mode - J/K (scroll), Esc (back) - kubectl top pods -n {}", app.current_namespace)
            },
            AppMode::Search => "Search Mode - Type to search, Enter (select), Esc (cancel)".to_string(),
            AppMode::NamespaceList => "Ready - Select namespace - kubectl get namespaces".to_string(),
            AppMode::Confirm => "Confirmation - y/Y (confirm), n/N/Esc (cancel)".to_string(),
            AppMode::Help => "Help Mode - Esc (back), q (quit)".to_string(),
        }
    };

    let command_line = Paragraph::new(command_text)
        .style(Style::default().fg(Color::Cyan));

    f.render_widget(command_line, area);
}