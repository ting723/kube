use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant};

use anyhow::Result;
use crossterm::event::MouseEvent;

#[derive(Debug, Clone)]
pub enum ConfirmAction {
    DeletePod { namespace: String, name: String },
    DeleteService { namespace: String, name: String },
    DeleteConfigMap { namespace: String, name: String },
    DeleteSecret { namespace: String, name: String },
    DeleteBatch { items: Vec<(String, String, String)> }, // namespace, type, name
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
    pub yaml_content: String,
    pub yaml_lines_cache: Vec<String>,
    pub yaml_scroll: usize,
    pub mouse_capture_enabled: bool,
    pub text_selection_mode: bool,
    pub language_chinese: bool,
    pub pod_metrics: Vec<crate::kubectl::types::PodMetrics>,
    pub metrics_scroll: usize,

    // New fields for added features
    pub favorite_namespaces: Vec<String>,
    pub last_selected_positions: HashMap<AppMode, usize>,
    pub batch_mode: bool,
    pub selected_batch_items: HashSet<usize>,
    pub active_portforwards: Vec<()>, // Temporary workaround: use empty tuples instead of Child which isn't Clone
    pub log_search_query: String,
    pub log_search_results: Vec<usize>,
    pub current_log_search_index: usize,
    pub log_search_mode: bool,
    pub streaming_logs: bool,
    pub command_history: Vec<String>,
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
            describe_auto_refresh: false,
            last_describe_refresh: Instant::now(),
            yaml_auto_refresh: false,
            last_yaml_refresh: Instant::now(),
            pending_exec: None,
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
            selected_batch_items: HashSet::new(),
            active_portforwards: Vec::new(),
            log_search_query: String::new(),
            log_search_results: Vec::new(),
            current_log_search_index: 0,
            log_search_mode: false,
            streaming_logs: false,
            command_history: Vec::new(),
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
        self.global_refresh_enabled && self.auto_refresh && self.last_update.elapsed() >= self.refresh_interval
    }

    pub fn should_refresh_logs(&self) -> bool {
        self.global_refresh_enabled && self.logs_auto_refresh && self.mode == AppMode::Logs
            && self.last_logs_refresh.elapsed() >= self.logs_refresh_interval
    }

    pub fn should_refresh_describe(&self) -> bool {
        self.global_refresh_enabled && self.describe_auto_refresh && self.mode == AppMode::Describe
            && self.last_describe_refresh.elapsed() >= self.refresh_interval
    }

    pub fn should_refresh_yaml(&self) -> bool {
        self.global_refresh_enabled && self.yaml_auto_refresh && self.mode == AppMode::YamlView
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
                status_parts.push(if self.language_chinese { "列表" } else { "Lists" });
            }
            if self.logs_auto_refresh {
                status_parts.push(if self.language_chinese { "日志" } else { "Logs" });
            }
            if self.describe_auto_refresh {
                status_parts.push(if self.language_chinese { "描述" } else { "Describe" });
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
                let prefix = if self.language_chinese { "[自动刷新: " } else { "[Auto-refresh: " };
                self.refresh_status_text = format!("{}{} ]", prefix, status_parts.join(", "));
            }
        }
    }

    pub fn force_refresh_current_mode(&mut self) {
        match self.mode {
            AppMode::NamespaceList | AppMode::PodList | AppMode::ServiceList | AppMode::NodeList
            | AppMode::DeploymentList | AppMode::JobList | AppMode::DaemonSetList | AppMode::PVCList
            | AppMode::PVList | AppMode::ConfigMapList | AppMode::SecretList => {
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
                if self.logs_scroll > 0 {
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
                if self.logs_scroll + 1 < self.logs.len() {
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
                self.logs_scroll = self.logs_scroll.saturating_sub(10);
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
                let max_scroll = self.logs.len().saturating_sub(1);
                self.logs_scroll = (self.logs_scroll + 10).min(max_scroll);
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
        self.describe_scroll = 0;
        self.yaml_scroll = 0;
        self.metrics_scroll = 0;
    }

    pub fn get_previous_mode(&self) -> AppMode {
        match self.mode {
            AppMode::Logs | AppMode::Describe | AppMode::YamlView | AppMode::TopView => {
                self.previous_mode.clone()
            }
            AppMode::Search | AppMode::Confirm => {
                self.previous_mode.clone()
            }
            _ => AppMode::NamespaceList,
        }
    }

    pub fn should_enable_mouse_capture(&self) -> bool {
        match self.mode {
            AppMode::Logs | AppMode::TopView => {
                !self.text_selection_mode
            }
            AppMode::Describe | AppMode::YamlView => {
                !self.text_selection_mode
            }
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
    pub fn is_current_namespace_favorite(&self) -> bool {
        if let Some(ns) = self.namespaces.get(self.selected_namespace_index) {
            self.favorite_namespaces.contains(ns)
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}

