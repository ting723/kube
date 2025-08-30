use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
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

    // 将内容按行分割
    let lines: Vec<&str> = app.describe_content.lines().collect();
    let visible_height = area.height.saturating_sub(2) as usize;
    let total_lines = lines.len();
    
    // 计算显示范围
    let start_index = app.describe_scroll;
    let end_index = (start_index + visible_height).min(total_lines);
    
    // 创建可见的内容项
    let visible_lines: Vec<ListItem> = lines[start_index..end_index]
        .iter()
        .map(|line| ListItem::new(line.to_string()))
        .collect();

    let mut list_state = ListState::default();
    
    let list = List::new(visible_lines)
        .block(Block::default().borders(Borders::ALL).title(title))
        .style(Style::default().fg(Color::White));

    f.render_stateful_widget(list, area, &mut list_state);
}