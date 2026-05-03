use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant};

use anyhow::Result;
use crossterm::event::MouseEvent;

use super::KeyConfig;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum ConfirmAction {
    DeletePod {
        namespace: String,
        name: String,
    },
    DeleteService {
        namespace: String,
        name: String,
    },
    DeleteConfigMap {
        namespace: String,
        name: String,
    },
    DeleteSecret {
        namespace: String,
        name: String,
    },
    DeleteBatch {
        items: Vec<(String, String, String)>,
    }, // namespace, type, name
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum AppMode {
    NamespaceList,
    PodList,
    ServiceList,
    DeploymentList,
    JobList,
    PVCList,
    PVList,
    NodeList,
    ConfigMapList,
    DaemonSetList,
    SecretList,
    Logs,
    Describe,
    Search,
    Confirm,
    Help,
    YamlView,
    TopView,
    CommandHistory,
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum ActivePane {
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct LogPane {
    pub pod_name: String,
    pub content: Vec<String>,
    pub scroll: usize,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct PortForward {
    pub namespace: String,
    pub pod_name: String,
    pub local_port: u16,
    pub target_port: u16,
    pub child_pid: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct AppState {
    // Existing fields - kept compatible with original app.rs
    pub mode: AppMode,
    pub should_quit: bool,
    pub current_namespace: String,
    pub selected_namespace_index: usize,
    pub selected_pod_index: usize,
    pub selected_service_index: usize,
    pub selected_node_index: usize,
    pub selected_deployment_index: usize,
    pub selected_job_index: usize,
    pub selected_daemonset_index: usize,
    pub selected_configmap_index: usize,
    pub selected_secret_index: usize,
    pub selected_pvc_index: usize,
    pub selected_pv_index: usize,
    pub namespaces: Vec<String>,
    pub pods: Vec<crate::kubectl::types::Pod>,
    pub services: Vec<crate::kubectl::types::Service>,
    pub nodes: Vec<crate::kubectl::types::Node>,
    pub deployments: Vec<crate::kubectl::types::Deployment>,
    pub jobs: Vec<crate::kubectl::types::Job>,
    pub daemonsets: Vec<crate::kubectl::types::DaemonSet>,
    pub pvcs: Vec<crate::kubectl::types::PVC>,
    pub pvs: Vec<crate::kubectl::types::PV>,
    pub configmaps: Vec<crate::kubectl::types::ConfigMap>,
    pub secrets: Vec<crate::kubectl::types::Secret>,
    pub logs: Vec<String>,
    pub describe_content: String,
    pub describe_lines_cache: Vec<String>,
    pub last_update: Instant,
    pub auto_refresh: bool,
    pub refresh_interval: Duration,
    pub logs_scroll: usize,
    pub describe_scroll: usize,
    pub search_query: String,
    pub search_mode: bool,
    pub search_results: Vec<usize>,
    pub current_search_index: usize,
    pub previous_mode: AppMode,
    pub confirm_action: Option<ConfirmAction>,
    pub current_command: String,
    pub logs_auto_scroll: bool,
    pub logs_auto_refresh: bool,
    pub logs_refresh_interval: Duration,
    pub last_logs_refresh: Instant,
    pub global_refresh_enabled: bool,
    pub refresh_status_text: String,
    pub describe_auto_refresh: bool,
    pub last_describe_refresh: Instant,
    pub yaml_auto_refresh: bool,
    pub last_yaml_refresh: Instant,
    pub pending_exec: Option<String>,
    pub pending_commands: Vec<String>,
    pub show_port_forwards: bool,
    pub active_port_forwards: Vec<PortForward>,
    pub pending_port_forward: Option<(String, String)>,
    pub yaml_content: String,
    pub yaml_lines_cache: Vec<String>,
    pub yaml_scroll: usize,
    pub mouse_capture_enabled: bool,
    pub text_selection_mode: bool,
    pub language_chinese: bool,
    pub pod_metrics: Vec<crate::kubectl::types::PodMetrics>,
    pub metrics_scroll: usize,

    // New fields for added features
    #[allow(dead_code)]
    pub favorite_namespaces: Vec<String>,
    #[allow(dead_code)]
    pub last_selected_positions: HashMap<AppMode, usize>,
    #[allow(dead_code)]
    pub batch_mode: bool,
    pub split_log_mode: bool,
    pub log_panes: Vec<LogPane>,
    pub active_pane_index: usize,
    pub split_pod_selection_mode: bool,
    pub split_pod_selection_index: usize,
    pub sort_column: usize,
    pub sort_ascending: bool,
    #[allow(dead_code)]
    pub marked_items: HashSet<usize>,
    #[allow(dead_code)]
    pub exec_returning: bool,
    pub log_search_query: String,
    pub log_search_results: Vec<usize>,
    pub current_log_search_index: usize,
    pub log_search_mode: bool,
    pub log_search_confirmed: bool,
    #[allow(dead_code)]
    pub streaming_logs: bool,
    #[allow(dead_code)]
    pub command_history: Vec<String>,

    // Context switching
    pub available_contexts: Vec<String>,
    pub selected_context_index: usize,
    pub current_context: String,
    pub context_selection_mode: bool,
    pub pending_context_switch: Option<bool>,

    // Key configuration
    #[allow(dead_code)]
    pub key_config: KeyConfig,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            mode: AppMode::NamespaceList,
            should_quit: false,
            current_namespace: "default".to_string(),
            selected_namespace_index: 0,
            selected_pod_index: 0,
            selected_service_index: 0,
            selected_node_index: 0,
            selected_deployment_index: 0,
            selected_job_index: 0,
            selected_daemonset_index: 0,
            selected_configmap_index: 0,
            selected_secret_index: 0,
            selected_pvc_index: 0,
            selected_pv_index: 0,
            namespaces: vec!["default".to_string()],
            pods: Vec::new(),
            services: Vec::new(),
            nodes: Vec::new(),
            deployments: Vec::new(),
            jobs: Vec::new(),
            daemonsets: Vec::new(),
            pvcs: Vec::new(),
            pvs: Vec::new(),
            configmaps: Vec::new(),
            secrets: Vec::new(),
            logs: Vec::new(),
            describe_content: String::new(),
            describe_lines_cache: Vec::new(),
            last_update: Instant::now(),
            auto_refresh: true,
            refresh_interval: Duration::from_secs(5),
            logs_scroll: 0,
            describe_scroll: 0,
            search_query: String::new(),
            search_mode: false,
            search_results: Vec::new(),
            current_search_index: 0,
            previous_mode: AppMode::NamespaceList,
            confirm_action: None,
            current_command: String::new(),
            logs_auto_scroll: true,
            logs_auto_refresh: true,
            logs_refresh_interval: Duration::from_secs(2),
            last_logs_refresh: Instant::now(),
            global_refresh_enabled: true,
            refresh_status_text: String::new(),
            describe_auto_refresh: true,
            last_describe_refresh: Instant::now(),
            yaml_auto_refresh: true,
            last_yaml_refresh: Instant::now(),
            pending_exec: None,
            pending_commands: Vec::new(),
            show_port_forwards: false,
            active_port_forwards: Vec::new(),
            pending_port_forward: None,
            yaml_content: String::new(),
            yaml_lines_cache: Vec::new(),
            yaml_scroll: 0,
            mouse_capture_enabled: false,
            text_selection_mode: false,
            language_chinese: true,
            pod_metrics: Vec::new(),
            metrics_scroll: 0,

            // Initialize new fields
            favorite_namespaces: Vec::new(),
            last_selected_positions: HashMap::new(),
            batch_mode: false,
            split_log_mode: false,
            log_panes: Vec::new(),
            active_pane_index: 0,
            split_pod_selection_mode: false,
            split_pod_selection_index: 0,
            sort_column: 0,
            sort_ascending: true,
            marked_items: HashSet::new(),
            exec_returning: false,
            log_search_query: String::new(),
            log_search_results: Vec::new(),
            current_log_search_index: 0,
            log_search_mode: false,
            log_search_confirmed: false,
            streaming_logs: false,
            command_history: Vec::new(),

            available_contexts: Vec::new(),
            selected_context_index: 0,
            current_context: String::new(),
            context_selection_mode: false,
            pending_context_switch: None,

            key_config: KeyConfig::load(),
        }
    }
}

impl AppState {
    pub fn new() -> Self {
        let mut app = Self::default();
        app.update_refresh_status();
        app
    }

    // Keep the same methods as original app.rs to maintain compatibility
    pub fn should_refresh(&self) -> bool {
        self.global_refresh_enabled
            && self.auto_refresh
            && self.last_update.elapsed() >= self.refresh_interval
    }

    pub fn should_refresh_logs(&self) -> bool {
        self.global_refresh_enabled
            && self.logs_auto_refresh
            && self.mode == AppMode::Logs
            && self.last_logs_refresh.elapsed() >= self.logs_refresh_interval
    }

    pub fn should_refresh_describe(&self) -> bool {
        self.global_refresh_enabled
            && self.describe_auto_refresh
            && self.mode == AppMode::Describe
            && self.last_describe_refresh.elapsed() >= self.refresh_interval
    }

    pub fn should_refresh_yaml(&self) -> bool {
        self.global_refresh_enabled
            && self.yaml_auto_refresh
            && self.mode == AppMode::YamlView
            && self.last_yaml_refresh.elapsed() >= self.refresh_interval
    }

    pub fn refresh_describe(&mut self) {
        self.last_describe_refresh = Instant::now();
    }

    pub fn refresh_yaml(&mut self) {
        self.last_yaml_refresh = Instant::now();
    }

    pub fn toggle_global_refresh(&mut self) {
        self.global_refresh_enabled = !self.global_refresh_enabled;
        self.update_refresh_status();
    }

    pub fn toggle_describe_refresh(&mut self) {
        self.describe_auto_refresh = !self.describe_auto_refresh;
        self.update_refresh_status();
    }

    pub fn toggle_yaml_refresh(&mut self) {
        self.yaml_auto_refresh = !self.yaml_auto_refresh;
        self.update_refresh_status();
    }

    pub fn update_refresh_status(&mut self) {
        if !self.global_refresh_enabled {
            self.refresh_status_text = if self.language_chinese {
                "[刷新已禁用]".to_string()
            } else {
                "[Refresh Disabled]".to_string()
            };
        } else {
            let mut status_parts = Vec::new();

            if self.auto_refresh {
                status_parts.push(if self.language_chinese {
                    "列表"
                } else {
                    "Lists"
                });
            }
            if self.logs_auto_refresh {
                status_parts.push(if self.language_chinese {
                    "日志"
                } else {
                    "Logs"
                });
            }
            if self.describe_auto_refresh {
                status_parts.push(if self.language_chinese {
                    "描述"
                } else {
                    "Describe"
                });
            }
            if self.yaml_auto_refresh {
                status_parts.push("YAML");
            }

            if status_parts.is_empty() {
                self.refresh_status_text = if self.language_chinese {
                    "[无自动刷新]".to_string()
                } else {
                    "[No Auto-refresh]".to_string()
                };
            } else {
                let prefix = if self.language_chinese {
                    "[自动刷新: "
                } else {
                    "[Auto-refresh: "
                };
                self.refresh_status_text = format!("{}{} ]", prefix, status_parts.join(", "));
            }
        }
    }

    pub fn force_refresh_current_mode(&mut self) {
        match self.mode {
            AppMode::NamespaceList
            | AppMode::PodList
            | AppMode::ServiceList
            | AppMode::NodeList
            | AppMode::DeploymentList
            | AppMode::JobList
            | AppMode::DaemonSetList
            | AppMode::PVCList
            | AppMode::PVList
            | AppMode::ConfigMapList
            | AppMode::SecretList => {
                self.refresh_data();
            }
            AppMode::Logs => {
                self.refresh_logs();
            }
            AppMode::Describe => {
                self.refresh_describe();
            }
            AppMode::YamlView => {
                self.refresh_yaml();
            }
            AppMode::TopView => {
                self.refresh_data();
            }
            _ => {}
        }
    }

    pub fn refresh_logs(&mut self) {
        self.last_logs_refresh = Instant::now();
    }

    pub fn refresh_data(&mut self) {
        self.last_update = Instant::now();
    }

    pub fn toggle_mouse_mode(&mut self) {
        self.text_selection_mode = !self.text_selection_mode;
    }

    pub fn toggle_language(&mut self) {
        self.language_chinese = !self.language_chinese;
    }

    pub fn get_mouse_mode_text(&self) -> &'static str {
        match self.mode {
            AppMode::Describe | AppMode::YamlView | AppMode::Logs => {
                if self.language_chinese {
                    if self.text_selection_mode {
                        "文本选择模式"
                    } else {
                        "鼠标滚轮模式"
                    }
                } else {
                    if self.text_selection_mode {
                        "Text Selection Mode"
                    } else {
                        "Mouse Scroll Mode"
                    }
                }
            }
            _ => "",
        }
    }

    pub fn get_selected_pod(&self) -> Option<&crate::kubectl::types::Pod> {
        self.pods.get(self.selected_pod_index)
    }

    pub fn get_selected_service(&self) -> Option<&crate::kubectl::types::Service> {
        self.services.get(self.selected_service_index)
    }

    pub fn get_selected_deployment(&self) -> Option<&crate::kubectl::types::Deployment> {
        self.deployments.get(self.selected_deployment_index)
    }

    pub fn get_selected_job(&self) -> Option<&crate::kubectl::types::Job> {
        self.jobs.get(self.selected_job_index)
    }

    pub fn get_selected_daemonset(&self) -> Option<&crate::kubectl::types::DaemonSet> {
        self.daemonsets.get(self.selected_daemonset_index)
    }

    pub fn get_selected_node(&self) -> Option<&crate::kubectl::types::Node> {
        self.nodes.get(self.selected_node_index)
    }

    pub fn get_selected_configmap(&self) -> Option<&crate::kubectl::types::ConfigMap> {
        self.configmaps.get(self.selected_configmap_index)
    }

    pub fn get_selected_secret(&self) -> Option<&crate::kubectl::types::Secret> {
        self.secrets.get(self.selected_secret_index)
    }

    pub fn get_selected_pvc(&self) -> Option<&crate::kubectl::types::PVC> {
        self.pvcs.get(self.selected_pvc_index)
    }

    pub fn get_selected_pv(&self) -> Option<&crate::kubectl::types::PV> {
        self.pvs.get(self.selected_pv_index)
    }

    pub fn set_current_command(&mut self, command: &str) {
        self.current_command = command.to_string();
    }

    pub fn clear_current_command(&mut self) {
        self.current_command.clear();
    }

    pub fn set_describe_content(&mut self, content: String) {
        self.describe_lines_cache = content.lines().map(|l| l.to_string()).collect();
        self.describe_content = content;
    }

    pub fn set_yaml_content(&mut self, content: String) {
        self.yaml_lines_cache = content.lines().map(|l| l.to_string()).collect();
        self.yaml_content = content;
    }

    pub fn scroll_up(&mut self) {
        match self.mode {
            AppMode::Logs => {
                if self.split_log_mode {
                    if let Some(pane) = self.log_panes.get_mut(self.active_pane_index) {
                        if pane.scroll > 0 {
                            pane.scroll -= 1;
                        }
                    }
                } else if self.logs_scroll > 0 {
                    self.logs_scroll -= 1;
                }
            }
            AppMode::Describe => {
                if self.describe_scroll > 0 {
                    self.describe_scroll -= 1;
                }
            }
            AppMode::YamlView => {
                if self.yaml_scroll > 0 {
                    self.yaml_scroll -= 1;
                }
            }
            AppMode::TopView => {
                if self.metrics_scroll > 0 {
                    self.metrics_scroll -= 1;
                }
            }
            _ => {}
        }
    }

    pub fn scroll_down(&mut self) {
        match self.mode {
            AppMode::Logs => {
                if self.split_log_mode {
                    if let Some(pane) = self.log_panes.get_mut(self.active_pane_index) {
                        if pane.scroll + 1 < pane.content.len() {
                            pane.scroll += 1;
                        }
                    }
                } else if self.logs_scroll + 1 < self.logs.len() {
                    self.logs_scroll += 1;
                }
            }
            AppMode::Describe => {
                if self.describe_scroll + 1 < self.describe_lines_cache.len() {
                    self.describe_scroll += 1;
                }
            }
            AppMode::YamlView => {
                if self.yaml_scroll + 1 < self.yaml_lines_cache.len() {
                    self.yaml_scroll += 1;
                }
            }
            AppMode::TopView => {
                if self.metrics_scroll + 1 < self.pod_metrics.len() {
                    self.metrics_scroll += 1;
                }
            }
            _ => {}
        }
    }

    pub fn scroll_page_up(&mut self) {
        match self.mode {
            AppMode::Logs => {
                if self.split_log_mode {
                    if let Some(pane) = self.log_panes.get_mut(self.active_pane_index) {
                        pane.scroll = pane.scroll.saturating_sub(10);
                    }
                } else {
                    self.logs_scroll = self.logs_scroll.saturating_sub(10);
                }
            }
            AppMode::Describe => {
                self.describe_scroll = self.describe_scroll.saturating_sub(10);
            }
            AppMode::YamlView => {
                self.yaml_scroll = self.yaml_scroll.saturating_sub(10);
            }
            AppMode::TopView => {
                self.metrics_scroll = self.metrics_scroll.saturating_sub(10);
            }
            _ => {}
        }
    }

    pub fn scroll_page_down(&mut self) {
        match self.mode {
            AppMode::Logs => {
                if self.split_log_mode {
                    if let Some(pane) = self.log_panes.get_mut(self.active_pane_index) {
                        let max = pane.content.len().saturating_sub(1);
                        pane.scroll = (pane.scroll + 10).min(max);
                    }
                } else {
                    let max_scroll = self.logs.len().saturating_sub(1);
                    self.logs_scroll = (self.logs_scroll + 10).min(max_scroll);
                }
            }
            AppMode::Describe => {
                let max_scroll = self.describe_lines_cache.len().saturating_sub(1);
                self.describe_scroll = (self.describe_scroll + 10).min(max_scroll);
            }
            AppMode::YamlView => {
                let max_scroll = self.yaml_lines_cache.len().saturating_sub(1);
                self.yaml_scroll = (self.yaml_scroll + 10).min(max_scroll);
            }
            AppMode::TopView => {
                let max_scroll = self.pod_metrics.len().saturating_sub(1);
                self.metrics_scroll = (self.metrics_scroll + 10).min(max_scroll);
            }
            _ => {}
        }
    }

    pub fn reset_scroll(&mut self) {
        self.logs_scroll = 0;
        for pane in &mut self.log_panes {
            pane.scroll = 0;
        }
        self.describe_scroll = 0;
        self.yaml_scroll = 0;
        self.metrics_scroll = 0;
    }

    pub fn get_previous_mode(&self) -> AppMode {
        match self.mode {
            AppMode::Logs | AppMode::Describe | AppMode::YamlView | AppMode::TopView => {
                self.previous_mode.clone()
            }
            AppMode::Search | AppMode::Confirm => self.previous_mode.clone(),
            _ => AppMode::NamespaceList,
        }
    }

    pub fn should_enable_mouse_capture(&self) -> bool {
        match self.mode {
            AppMode::Logs | AppMode::TopView => !self.text_selection_mode,
            AppMode::Describe | AppMode::YamlView => !self.text_selection_mode,
            _ => false,
        }
    }

    pub fn handle_mouse_event(&mut self, mouse_event: MouseEvent) -> Result<()> {
        match self.mode {
            AppMode::Logs | AppMode::Describe | AppMode::YamlView | AppMode::TopView => {
                match mouse_event.kind {
                    crossterm::event::MouseEventKind::ScrollUp => {
                        self.scroll_up();
                    }
                    crossterm::event::MouseEventKind::ScrollDown => {
                        self.scroll_down();
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        Ok(())
    }

    /// Save current selected position for this mode before leaving
    #[allow(dead_code)]
    pub fn save_selected_position(&mut self) {
        let pos = match self.mode {
            AppMode::NamespaceList => self.selected_namespace_index,
            AppMode::PodList => self.selected_pod_index,
            AppMode::ServiceList => self.selected_service_index,
            AppMode::DeploymentList => self.selected_deployment_index,
            AppMode::JobList => self.selected_job_index,
            AppMode::DaemonSetList => self.selected_daemonset_index,
            AppMode::PVCList => self.selected_pvc_index,
            AppMode::PVList => self.selected_pv_index,
            AppMode::NodeList => self.selected_node_index,
            AppMode::ConfigMapList => self.selected_configmap_index,
            AppMode::SecretList => self.selected_secret_index,
            _ => return,
        };
        self.last_selected_positions.insert(self.mode.clone(), pos);
    }

    /// Restore saved selected position when entering this mode
    #[allow(dead_code)]
    pub fn restore_selected_position(&mut self) {
        if let Some(&pos) = self.last_selected_positions.get(&self.mode) {
            match self.mode {
                AppMode::NamespaceList => self.selected_namespace_index = pos,
                AppMode::PodList => self.selected_pod_index = pos,
                AppMode::ServiceList => self.selected_service_index = pos,
                AppMode::DeploymentList => self.selected_deployment_index = pos,
                AppMode::JobList => self.selected_job_index = pos,
                AppMode::DaemonSetList => self.selected_daemonset_index = pos,
                AppMode::PVCList => self.selected_pvc_index = pos,
                AppMode::PVList => self.selected_pv_index = pos,
                AppMode::NodeList => self.selected_node_index = pos,
                AppMode::ConfigMapList => self.selected_configmap_index = pos,
                AppMode::SecretList => self.selected_secret_index = pos,
                _ => {}
            }
        }
    }

    /// Toggle favorite for current namespace
    #[allow(dead_code)]
    pub fn toggle_favorite_namespace(&mut self) {
        if let Some(ns) = self.namespaces.get(self.selected_namespace_index) {
            if self.favorite_namespaces.contains(ns) {
                self.favorite_namespaces.retain(|x| x != ns);
            } else {
                self.favorite_namespaces.push(ns.clone());
            }
        }
    }

    /// Check if current namespace is favorited
    #[allow(dead_code)]
    pub fn is_current_namespace_favorite(&self) -> bool {
        if let Some(ns) = self.namespaces.get(self.selected_namespace_index) {
            self.favorite_namespaces.contains(ns)
        } else {
            false
        }
    }

    #[allow(dead_code)]
    pub fn toggle_batch_mode(&mut self) {
        self.batch_mode = !self.batch_mode;
        if !self.batch_mode {
            self.marked_items.clear();
        }
    }

    #[allow(dead_code)]
    pub fn toggle_mark_current(&mut self) {
        if !self.batch_mode {
            return;
        }
        let idx = self.current_selection_index();
        if self.marked_items.contains(&idx) {
            self.marked_items.remove(&idx);
        } else {
            self.marked_items.insert(idx);
        }
    }

    #[allow(dead_code)]
    pub fn mark_all(&mut self) {
        if !self.batch_mode {
            return;
        }
        let count = self.current_list_len();
        for i in 0..count {
            self.marked_items.insert(i);
        }
    }

    #[allow(dead_code)]
    fn current_selection_index(&self) -> usize {
        match self.mode {
            AppMode::NamespaceList => self.selected_namespace_index,
            AppMode::PodList => self.selected_pod_index,
            AppMode::ServiceList => self.selected_service_index,
            AppMode::NodeList => self.selected_node_index,
            AppMode::DeploymentList => self.selected_deployment_index,
            AppMode::JobList => self.selected_job_index,
            AppMode::DaemonSetList => self.selected_daemonset_index,
            AppMode::PVCList => self.selected_pvc_index,
            AppMode::PVList => self.selected_pv_index,
            AppMode::ConfigMapList => self.selected_configmap_index,
            AppMode::SecretList => self.selected_secret_index,
            _ => 0,
        }
    }

    pub fn toggle_sort(&mut self) {
        let max_cols = match self.mode {
            AppMode::NamespaceList => 2,
            AppMode::PodList => 6,
            AppMode::ServiceList => 5,
            AppMode::NodeList => 5,
            AppMode::DeploymentList => 5,
            AppMode::JobList => 4,
            AppMode::DaemonSetList => 5,
            AppMode::PVCList => 5,
            AppMode::PVList => 5,
            AppMode::ConfigMapList => 4,
            AppMode::SecretList => 3,
            _ => return,
        };
        if self.sort_column == max_cols - 1 {
            self.sort_column = 0;
            self.sort_ascending = !self.sort_ascending;
        } else {
            self.sort_column += 1;
        }
    }

    pub fn sort_pods(&mut self) {
        match self.sort_column {
            0 => self.pods.sort_by(|a, b| {
                if self.sort_ascending {
                    a.name.cmp(&b.name)
                } else {
                    b.name.cmp(&a.name)
                }
            }),
            1 => self.pods.sort_by(|a, b| {
                if self.sort_ascending {
                    a.ready.cmp(&b.ready)
                } else {
                    b.ready.cmp(&a.ready)
                }
            }),
            2 => self.pods.sort_by(|a, b| {
                if self.sort_ascending {
                    a.status.phase.cmp(&b.status.phase)
                } else {
                    b.status.phase.cmp(&a.status.phase)
                }
            }),
            3 => self.pods.sort_by(|a, b| {
                if self.sort_ascending {
                    a.restarts.cmp(&b.restarts)
                } else {
                    b.restarts.cmp(&a.restarts)
                }
            }),
            4 => self.pods.sort_by(|a, b| {
                if self.sort_ascending {
                    a.age.cmp(&b.age)
                } else {
                    b.age.cmp(&a.age)
                }
            }),
            5 => self.pods.sort_by(|a, b| {
                let na = a.node.as_deref().unwrap_or("");
                let nb = b.node.as_deref().unwrap_or("");
                if self.sort_ascending {
                    na.cmp(nb)
                } else {
                    nb.cmp(na)
                }
            }),
            _ => {}
        }
    }

    pub fn sort_services(&mut self) {
        match self.sort_column {
            0 => self.services.sort_by(|a, b| {
                if self.sort_ascending {
                    a.name.cmp(&b.name)
                } else {
                    b.name.cmp(&a.name)
                }
            }),
            1 => self.services.sort_by(|a, b| {
                if self.sort_ascending {
                    a.cluster_ip.cmp(&b.cluster_ip)
                } else {
                    b.cluster_ip.cmp(&a.cluster_ip)
                }
            }),
            2 => self.services.sort_by(|a, b| {
                if self.sort_ascending {
                    a.type_.cmp(&b.type_)
                } else {
                    b.type_.cmp(&a.type_)
                }
            }),
            3 => self.services.sort_by(|a, b| {
                let ports_a = a
                    .ports
                    .iter()
                    .map(|p| p.port.to_string())
                    .collect::<Vec<_>>()
                    .join(",");
                let ports_b = b
                    .ports
                    .iter()
                    .map(|p| p.port.to_string())
                    .collect::<Vec<_>>()
                    .join(",");
                if self.sort_ascending {
                    ports_a.cmp(&ports_b)
                } else {
                    ports_b.cmp(&ports_a)
                }
            }),
            4 => self.services.sort_by(|a, b| {
                if self.sort_ascending {
                    a.age.cmp(&b.age)
                } else {
                    b.age.cmp(&a.age)
                }
            }),
            _ => {}
        }
    }

    pub fn sort_namespaces(&mut self) {
        match self.sort_column {
            0 => self.namespaces.sort_by(|a, b| {
                if self.sort_ascending {
                    a.cmp(b)
                } else {
                    b.cmp(a)
                }
            }),
            1 => self.namespaces.sort_by(|a, b| {
                if self.sort_ascending {
                    a.cmp(b)
                } else {
                    b.cmp(a)
                }
            }),
            _ => {}
        }
    }

    #[allow(dead_code)]
    fn current_list_len(&self) -> usize {
        match self.mode {
            AppMode::NamespaceList => self.namespaces.len(),
            AppMode::PodList => self.pods.len(),
            AppMode::ServiceList => self.services.len(),
            AppMode::NodeList => self.nodes.len(),
            AppMode::DeploymentList => self.deployments.len(),
            AppMode::JobList => self.jobs.len(),
            AppMode::DaemonSetList => self.daemonsets.len(),
            AppMode::PVCList => self.pvcs.len(),
            AppMode::PVList => self.pvs.len(),
            AppMode::ConfigMapList => self.configmaps.len(),
            AppMode::SecretList => self.secrets.len(),
            _ => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kubectl::types::{
        ContainerState, ContainerStateWaiting, ContainerStatus, PodStatus,
    };

    #[test]
    fn test_default_app_state() {
        let state = AppState::default();
        assert_eq!(state.mode, AppMode::NamespaceList);
        assert!(!state.should_quit);
        assert_eq!(state.current_namespace, "default");
        assert!(state.global_refresh_enabled);
        assert!(!state.batch_mode);
        assert!(state.favorite_namespaces.is_empty());
    }

    #[test]
    fn test_should_refresh() {
        let mut state = AppState::default();
        state.refresh_interval = std::time::Duration::from_millis(100);
        std::thread::sleep(std::time::Duration::from_millis(150));
        assert!(state.should_refresh());
    }

    #[test]
    fn test_toggle_global_refresh() {
        let mut state = AppState::default();
        let original = state.global_refresh_enabled;
        state.toggle_global_refresh();
        assert_eq!(state.global_refresh_enabled, !original);
    }

    #[test]
    fn test_toggle_mouse_mode() {
        let mut state = AppState::default();
        let original = state.text_selection_mode;
        state.toggle_mouse_mode();
        assert_eq!(state.text_selection_mode, !original);
    }

    #[test]
    fn test_toggle_language() {
        let mut state = AppState::default();
        let original = state.language_chinese;
        state.toggle_language();
        assert_eq!(state.language_chinese, !original);
    }

    #[test]
    fn test_save_restore_position() {
        let mut state = AppState::default();
        state.mode = AppMode::PodList;
        state.selected_pod_index = 5;
        state.save_selected_position();
        state.selected_pod_index = 0;
        state.restore_selected_position();
        assert_eq!(state.selected_pod_index, 5);
    }

    #[test]
    fn test_toggle_favorite_namespace() {
        let mut state = AppState::default();
        state.namespaces = vec!["default".to_string(), "kube-system".to_string()];
        state.selected_namespace_index = 0;
        assert!(!state.is_current_namespace_favorite());
        state.toggle_favorite_namespace();
        assert!(state.is_current_namespace_favorite());
        state.toggle_favorite_namespace();
        assert!(!state.is_current_namespace_favorite());
    }

    #[test]
    fn test_batch_mode_toggle() {
        let mut state = AppState::default();
        state.mode = AppMode::PodList;
        state.pods.push(crate::kubectl::types::Pod {
            name: "test-pod".into(),
            namespace: "default".into(),
            status: crate::kubectl::types::PodStatus {
                phase: "Running".into(),
                conditions: None,
                container_statuses: None,
            },
            ready: "1/1".into(),
            restarts: 0,
            age: "1d".into(),
            node: None,
            ip: None,
        });
        state.toggle_batch_mode();
        assert!(state.batch_mode);
        state.toggle_mark_current();
        assert_eq!(state.marked_items.len(), 1);
        state.toggle_batch_mode();
        assert!(!state.batch_mode);
        assert!(state.marked_items.is_empty());
    }

    #[test]
    fn test_mark_all() {
        let mut state = AppState::default();
        state.mode = AppMode::PodList;
        for i in 0..3 {
            state.pods.push(crate::kubectl::types::Pod {
                name: format!("pod-{}", i),
                namespace: "default".into(),
                status: crate::kubectl::types::PodStatus {
                    phase: "Running".into(),
                    conditions: None,
                    container_statuses: None,
                },
                ready: "1/1".into(),
                restarts: 0,
                age: "1d".into(),
                node: None,
                ip: None,
            });
        }
        state.toggle_batch_mode();
        state.mark_all();
        assert_eq!(state.marked_items.len(), 3);
    }

    #[test]
    fn test_split_log_mode_defaults() {
        let state = AppState::default();
        assert!(!state.split_log_mode);
        assert!(state.log_panes.is_empty());
        assert_eq!(state.active_pane_index, 0);
    }

    #[test]
    fn test_active_pane_scroll() {
        let mut state = AppState::default();
        state.mode = AppMode::Logs;
        state.split_log_mode = true;
        state.log_panes.push(LogPane {
            pod_name: "pod2".into(),
            content: vec!["a".into(), "b".into(), "c".into()],
            scroll: 0,
        });
        state.active_pane_index = 0;
        state.scroll_down();
        assert_eq!(state.log_panes[0].scroll, 1);
        state.scroll_up();
        assert_eq!(state.log_panes[0].scroll, 0);
    }

    #[test]
    fn test_log_search_navigation_state() {
        let mut state = AppState::default();
        state.mode = AppMode::Logs;
        state.log_search_results = vec![1, 3, 5];
        state.current_log_search_index = 0;
        state.current_log_search_index =
            (state.current_log_search_index + 1) % state.log_search_results.len();
        assert_eq!(state.current_log_search_index, 1);
        state.current_log_search_index =
            (state.current_log_search_index + 1) % state.log_search_results.len();
        assert_eq!(state.current_log_search_index, 2);
        state.current_log_search_index =
            (state.current_log_search_index + 1) % state.log_search_results.len();
        assert_eq!(state.current_log_search_index, 0);
        state.current_log_search_index = if state.current_log_search_index == 0 {
            state.log_search_results.len() - 1
        } else {
            state.current_log_search_index - 1
        };
        assert_eq!(state.current_log_search_index, 2);
    }

    #[test]
    fn test_sort_toggle_cycles_columns() {
        let mut state = AppState::default();
        state.mode = AppMode::PodList;
        assert_eq!(state.sort_column, 0);
        assert!(state.sort_ascending);
        state.toggle_sort();
        assert_eq!(state.sort_column, 1);
        state.toggle_sort();
        assert_eq!(state.sort_column, 2);
        for _ in 0..4 {
            state.toggle_sort();
        }
        assert_eq!(state.sort_column, 0);
        assert!(!state.sort_ascending);
    }

    #[test]
    fn test_sort_toggle_noop_in_non_list_mode() {
        let mut state = AppState::default();
        state.mode = AppMode::Logs;
        let old = state.sort_column;
        state.toggle_sort();
        assert_eq!(state.sort_column, old);
    }

    #[test]
    fn test_log_panes_add_and_remove() {
        let mut state = AppState::default();
        state.log_panes.push(LogPane {
            pod_name: "pod-a".into(),
            content: vec!["line1".into()],
            scroll: 0,
        });
        state.log_panes.push(LogPane {
            pod_name: "pod-b".into(),
            content: vec!["line2".into()],
            scroll: 0,
        });
        assert_eq!(state.log_panes.len(), 2);
        state.active_pane_index = 0;
        state.log_panes.remove(state.active_pane_index);
        assert_eq!(state.log_panes.len(), 1);
        assert_eq!(state.log_panes[0].pod_name, "pod-b");
    }

    #[test]
    fn test_auto_refresh_defaults_enabled() {
        let state = AppState::default();
        assert!(state.auto_refresh);
        assert!(state.global_refresh_enabled);
        assert!(state.logs_auto_refresh);
        assert!(state.describe_auto_refresh);
        assert!(state.yaml_auto_refresh);
    }

    #[test]
    fn test_auto_scroll_default() {
        let mut state = AppState::default();
        assert!(state.logs_auto_scroll);
        state.logs_auto_scroll = false;
        assert!(!state.logs_auto_scroll);
    }

    #[test]
    fn test_log_panes_scroll_independent() {
        let mut state = AppState::default();
        state.mode = AppMode::Logs;
        state.split_log_mode = true;
        state.log_panes.push(LogPane {
            pod_name: "p1".into(),
            content: vec!["a".into(), "b".into(), "c".into(), "d".into()],
            scroll: 0,
        });
        state.log_panes.push(LogPane {
            pod_name: "p2".into(),
            content: vec!["x".into(), "y".into(), "z".into()],
            scroll: 0,
        });
        state.active_pane_index = 0;
        state.scroll_down();
        assert_eq!(state.log_panes[0].scroll, 1);
        assert_eq!(state.log_panes[1].scroll, 0);
        state.active_pane_index = 1;
        state.scroll_down();
        state.scroll_down();
        assert_eq!(state.log_panes[1].scroll, 2);
        assert_eq!(state.log_panes[0].scroll, 1);
    }

    #[test]
    fn test_log_search_confirmed_flag() {
        let mut state = AppState::default();
        assert!(!state.log_search_confirmed);
        state.log_search_confirmed = true;
        assert!(state.log_search_confirmed);
        state.log_search_confirmed = false;
        assert!(!state.log_search_confirmed);
    }

    #[test]
    fn test_pod_status_detailed_phase() {
        let status = PodStatus {
            phase: "Pending".into(),
            conditions: None,
            container_statuses: Some(vec![ContainerStatus {
                name: "app".into(),
                ready: false,
                restart_count: 0,
                state: ContainerState {
                    running: None,
                    waiting: Some(ContainerStateWaiting {
                        reason: Some("ImagePullBackOff".into()),
                        message: None,
                    }),
                    terminated: None,
                },
            }]),
        };
        assert_eq!(status.detailed_phase(), "Pending (ImagePullBackOff)");
        assert!(status.is_error_state());
    }

    #[test]
    fn test_pod_status_running_no_detail() {
        let status = PodStatus {
            phase: "Running".into(),
            conditions: None,
            container_statuses: None,
        };
        assert_eq!(status.detailed_phase(), "Running");
        assert!(!status.is_error_state());
    }

    #[test]
    fn test_pod_status_crash_loop() {
        let status = PodStatus {
            phase: "Running".into(),
            conditions: None,
            container_statuses: Some(vec![ContainerStatus {
                name: "app".into(),
                ready: false,
                restart_count: 5,
                state: ContainerState {
                    running: None,
                    waiting: Some(ContainerStateWaiting {
                        reason: Some("CrashLoopBackOff".into()),
                        message: None,
                    }),
                    terminated: None,
                },
            }]),
        };
        assert!(status.is_error_state());
    }
}
