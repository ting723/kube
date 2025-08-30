use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

use crate::app::{AppState, AppMode};

pub fn render(f: &mut Frame, area: Rect, app: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(area);

    // 搜索输入框，添加光标显示
    let input_text = if app.search_mode {
        format!("{}█", app.search_query) // 添加光标字符
    } else {
        app.search_query.clone()
    };
    
    let search_input = Paragraph::new(input_text)
        .block(Block::default().borders(Borders::ALL).title("Search (type to search, j/k to navigate, Enter to select, Esc to cancel)"))
        .style(Style::default().fg(Color::White));

    f.render_widget(search_input, chunks[0]);

    // 显示搜索结果列表
    if app.search_results.is_empty() {
        let message = if app.search_query.is_empty() {
            "Enter search query..."
        } else {
            "No results found"
        };
        
        let no_results = Paragraph::new(message)
            .block(Block::default().borders(Borders::ALL).title("Results"))
            .style(Style::default().fg(Color::Gray));
        
        f.render_widget(no_results, chunks[1]);
    } else {
        // 根据搜索结果显示对应的资源列表
        let items: Vec<ListItem> = match app.previous_mode {
            AppMode::PodList => {
                app.search_results.iter().map(|&index| {
                    if let Some(pod) = app.pods.get(index) {
                        let style = if index == app.selected_pod_index {
                            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                        } else {
                            Style::default().fg(Color::White)
                        };
                        ListItem::new(format!("{} - {}", pod.name, pod.status.phase)).style(style)
                    } else {
                        ListItem::new("Invalid index").style(Style::default().fg(Color::Red))
                    }
                }).collect()
            }
            AppMode::ServiceList => {
                app.search_results.iter().map(|&index| {
                    if let Some(service) = app.services.get(index) {
                        let style = if index == app.selected_service_index {
                            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                        } else {
                            Style::default().fg(Color::White)
                        };
                        ListItem::new(format!("{} - {}", service.name, service.type_)).style(style)
                    } else {
                        ListItem::new("Invalid index").style(Style::default().fg(Color::Red))
                    }
                }).collect()
            }
            AppMode::DeploymentList => {
                app.search_results.iter().map(|&index| {
                    if let Some(deployment) = app.deployments.get(index) {
                        let style = if index == app.selected_deployment_index {
                            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                        } else {
                            Style::default().fg(Color::White)
                        };
                        ListItem::new(format!("{} - {}", deployment.name, deployment.ready)).style(style)
                    } else {
                        ListItem::new("Invalid index").style(Style::default().fg(Color::Red))
                    }
                }).collect()
            }
            AppMode::NodeList => {
                app.search_results.iter().map(|&index| {
                    if let Some(node) = app.nodes.get(index) {
                        let style = if index == app.selected_node_index {
                            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                        } else {
                            Style::default().fg(Color::White)
                        };
                        ListItem::new(format!("{} - {}", node.name, node.status)).style(style)
                    } else {
                        ListItem::new("Invalid index").style(Style::default().fg(Color::Red))
                    }
                }).collect()
            }
            AppMode::DaemonSetList => {
                app.search_results.iter().map(|&index| {
                    if let Some(daemonset) = app.daemonsets.get(index) {
                        let style = if index == app.selected_daemonset_index {
                            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                        } else {
                            Style::default().fg(Color::White)
                        };
                        ListItem::new(format!("{} - {}/{}", daemonset.name, daemonset.ready, daemonset.desired)).style(style)
                    } else {
                        ListItem::new("Invalid index").style(Style::default().fg(Color::Red))
                    }
                }).collect()
            }
            AppMode::ConfigMapList => {
                app.search_results.iter().map(|&index| {
                    if let Some(configmap) = app.configmaps.get(index) {
                        let style = if index == app.selected_configmap_index {
                            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                        } else {
                            Style::default().fg(Color::White)
                        };
                        ListItem::new(format!("{} - {}", configmap.name, configmap.age)).style(style)
                    } else {
                        ListItem::new("Invalid index").style(Style::default().fg(Color::Red))
                    }
                }).collect()
            }
            AppMode::SecretList => {
                app.search_results.iter().map(|&index| {
                    if let Some(secret) = app.secrets.get(index) {
                        let style = if index == app.selected_secret_index {
                            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                        } else {
                            Style::default().fg(Color::White)
                        };
                        ListItem::new(format!("{} - {}", secret.name, secret.type_)).style(style)
                    } else {
                        ListItem::new("Invalid index").style(Style::default().fg(Color::Red))
                    }
                }).collect()
            }
            AppMode::PVCList => {
                app.search_results.iter().map(|&index| {
                    if let Some(pvc) = app.pvcs.get(index) {
                        let style = if index == app.selected_pvc_index {
                            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                        } else {
                            Style::default().fg(Color::White)
                        };
                        ListItem::new(format!("{} - {}", pvc.name, pvc.status)).style(style)
                    } else {
                        ListItem::new("Invalid index").style(Style::default().fg(Color::Red))
                    }
                }).collect()
            }
            AppMode::PVList => {
                app.search_results.iter().map(|&index| {
                    if let Some(pv) = app.pvs.get(index) {
                        let style = if index == app.selected_pv_index {
                            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                        } else {
                            Style::default().fg(Color::White)
                        };
                        ListItem::new(format!("{} - {}", pv.name, pv.status)).style(style)
                    } else {
                        ListItem::new("Invalid index").style(Style::default().fg(Color::Red))
                    }
                }).collect()
            }
            _ => {
                vec![ListItem::new("Search not supported for this resource type").style(Style::default().fg(Color::Red))]
            }
        };

        let mut list_state = ListState::default();
        if !app.search_results.is_empty() {
            if let Some(pos) = app.search_results.iter().position(|&idx| {
                match app.previous_mode {
                    AppMode::PodList => idx == app.selected_pod_index,
                    AppMode::ServiceList => idx == app.selected_service_index,
                    AppMode::DeploymentList => idx == app.selected_deployment_index,
                    AppMode::NodeList => idx == app.selected_node_index,
                    AppMode::DaemonSetList => idx == app.selected_daemonset_index,
                    AppMode::ConfigMapList => idx == app.selected_configmap_index,
                    AppMode::SecretList => idx == app.selected_secret_index,
                    AppMode::PVCList => idx == app.selected_pvc_index,
                    AppMode::PVList => idx == app.selected_pv_index,
                    _ => false,
                }
            }) {
                list_state.select(Some(pos));
            }
        }

        let results_title = format!("Search Results ({} found)", app.search_results.len());
        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title(results_title))
            .highlight_style(
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            );

        f.render_stateful_widget(list, chunks[1], &mut list_state);
    }
}