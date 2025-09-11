use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::{AppState, ConfirmAction};

pub fn render(f: &mut Frame, area: Rect, app: &AppState) {
    if let Some(ref action) = app.confirm_action {
        // 根据操作类型生成标题和消息
        let (title, message) = match action {
            ConfirmAction::DeletePod { namespace, name } => {
                if app.language_chinese {
                    ("确认删除Pod", format!("确定要删除Pod {}/{} 吗？此操作不可撤销。", namespace, name))
                } else {
                    ("Confirm Delete Pod", format!("Are you sure you want to delete Pod {}/{}? This action cannot be undone.", namespace, name))
                }
            }
            ConfirmAction::DeleteService { namespace, name } => {
                if app.language_chinese {
                    ("确认删除Service", format!("确定要删除Service {}/{} 吗？此操作不可撤销。", namespace, name))
                } else {
                    ("Confirm Delete Service", format!("Are you sure you want to delete Service {}/{}? This action cannot be undone.", namespace, name))
                }
            }
            ConfirmAction::DeleteDeployment { namespace, name } => {
                if app.language_chinese {
                    ("确认删除Deployment", format!("确定要删除Deployment {}/{} 吗？此操作不可撤销。", namespace, name))
                } else {
                    ("Confirm Delete Deployment", format!("Are you sure you want to delete Deployment {}/{}? This action cannot be undone.", namespace, name))
                }
            }
            ConfirmAction::DeleteJob { namespace, name } => {
                if app.language_chinese {
                    ("确认删除Job", format!("确定要删除Job {}/{} 吗？此操作不可撤销。", namespace, name))
                } else {
                    ("Confirm Delete Job", format!("Are you sure you want to delete Job {}/{}? This action cannot be undone.", namespace, name))
                }
            }
            ConfirmAction::DeleteDaemonSet { namespace, name } => {
                if app.language_chinese {
                    ("确认删除DaemonSet", format!("确定要删除DaemonSet {}/{} 吗？此操作不可撤销。", namespace, name))
                } else {
                    ("Confirm Delete DaemonSet", format!("Are you sure you want to delete DaemonSet {}/{}? This action cannot be undone.", namespace, name))
                }
            }
            ConfirmAction::DeleteConfigMap { namespace, name } => {
                if app.language_chinese {
                    ("确认删除ConfigMap", format!("确定要删除ConfigMap {}/{} 吗？此操作不可撤销。", namespace, name))
                } else {
                    ("Confirm Delete ConfigMap", format!("Are you sure you want to delete ConfigMap {}/{}? This action cannot be undone.", namespace, name))
                }
            }
            ConfirmAction::DeleteSecret { namespace, name } => {
                if app.language_chinese {
                    ("确认删除Secret", format!("确定要删除Secret {}/{} 吗？此操作不可撤销。", namespace, name))
                } else {
                    ("Confirm Delete Secret", format!("Are you sure you want to delete Secret {}/{}? This action cannot be undone.", namespace, name))
                }
            }
            ConfirmAction::DeletePVC { namespace, name } => {
                if app.language_chinese {
                    ("确认删除PVC", format!("确定要删除PVC {}/{} 吗？此操作不可撤销。", namespace, name))
                } else {
                    ("Confirm Delete PVC", format!("Are you sure you want to delete PVC {}/{}? This action cannot be undone.", namespace, name))
                }
            }
            ConfirmAction::DeletePV { name } => {
                if app.language_chinese {
                    ("确认删除PV", format!("确定要删除PV {} 吗？此操作不可撤销。", name))
                } else {
                    ("Confirm Delete PV", format!("Are you sure you want to delete PV {}? This action cannot be undone.", name))
                }
            }
            ConfirmAction::DeleteStatefulSet { namespace, name } => {
                if app.language_chinese {
                    ("确认删除StatefulSet", format!("确定要删除StatefulSet {}/{} 吗？此操作不可撤销。", namespace, name))
                } else {
                    ("Confirm Delete StatefulSet", format!("Are you sure you want to delete StatefulSet {}/{}? This action cannot be undone.", namespace, name))
                }
            }
            ConfirmAction::DeleteIngress { namespace, name } => {
                if app.language_chinese {
                    ("确认删除Ingress", format!("确定要删除Ingress {}/{} 吗？此操作不可撤销。", namespace, name))
                } else {
                    ("Confirm Delete Ingress", format!("Are you sure you want to delete Ingress {}/{}? This action cannot be undone.", namespace, name))
                }
            }
            ConfirmAction::DeleteNetworkPolicy { namespace, name } => {
                if app.language_chinese {
                    ("确认删除NetworkPolicy", format!("确定要删除NetworkPolicy {}/{} 吗？此操作不可撤销。", namespace, name))
                } else {
                    ("Confirm Delete NetworkPolicy", format!("Are you sure you want to delete NetworkPolicy {}/{}? This action cannot be undone.", namespace, name))
                }
            }
            ConfirmAction::DeleteRole { namespace, name } => {
                if app.language_chinese {
                    ("确认删除Role", format!("确定要删除Role {}/{} 吗？此操作不可撤销。", namespace, name))
                } else {
                    ("Confirm Delete Role", format!("Are you sure you want to delete Role {}/{}? This action cannot be undone.", namespace, name))
                }
            }
            ConfirmAction::DeleteRoleBinding { namespace, name } => {
                if app.language_chinese {
                    ("确认删除RoleBinding", format!("确定要删除RoleBinding {}/{} 吗？此操作不可撤销。", namespace, name))
                } else {
                    ("Confirm Delete RoleBinding", format!("Are you sure you want to delete RoleBinding {}/{}? This action cannot be undone.", namespace, name))
                }
            }
            ConfirmAction::DeleteClusterRole { name } => {
                if app.language_chinese {
                    ("确认删除ClusterRole", format!("确定要删除ClusterRole {} 吗？此操作不可撤销。", name))
                } else {
                    ("Confirm Delete ClusterRole", format!("Are you sure you want to delete ClusterRole {}? This action cannot be undone.", name))
                }
            }
            ConfirmAction::DeleteClusterRoleBinding { name } => {
                if app.language_chinese {
                    ("确认删除ClusterRoleBinding", format!("确定要删除ClusterRoleBinding {} 吗？此操作不可撤销。", name))
                } else {
                    ("Confirm Delete ClusterRoleBinding", format!("Are you sure you want to delete ClusterRoleBinding {}? This action cannot be undone.", name))
                }
            }
            ConfirmAction::DeleteServiceAccount { namespace, name } => {
                if app.language_chinese {
                    ("确认删除ServiceAccount", format!("确定要删除ServiceAccount {}/{} 吗？此操作不可撤销。", namespace, name))
                } else {
                    ("Confirm Delete ServiceAccount", format!("Are you sure you want to delete ServiceAccount {}/{}? This action cannot be undone.", namespace, name))
                }
            }
        };

        // 创建确认对话框
        let confirm_text = if app.language_chinese {
            format!("{}\n\n按 'y' 确认, 'n' 或 'Esc' 取消", message)
        } else {
            format!("{}\n\nPress 'y' to confirm, 'n' or 'Esc' to cancel", message)
        };

        let confirm_block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Red));

        let confirm_paragraph = Paragraph::new(confirm_text)
            .block(confirm_block)
            .style(Style::default().fg(Color::White));

        f.render_widget(confirm_paragraph, area);
    }
}