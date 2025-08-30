use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub enum ConfirmAction {
    DeletePod { namespace: String, name: String },
    #[allow(dead_code)]
    DeleteService { namespace: String, name: String },
    #[allow(dead_code)]
    DeleteConfigMap { namespace: String, name: String },
    #[allow(dead_code)]
    DeleteSecret { namespace: String, name: String },
}

#[derive(Debug, Clone, PartialEq)]
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
    #[allow(dead_code)]
    Search,
    Confirm,
    Help,
}

#[derive(Debug, Clone)]
pub struct AppState {
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
    pub last_update: Instant,
    pub auto_refresh: bool,
    pub refresh_interval: Duration,
    // 滚动相关
    pub logs_scroll: usize,
    pub describe_scroll: usize,
    // 搜索相关
    pub search_query: String,
    pub search_mode: bool,
    pub search_results: Vec<usize>,
    pub current_search_index: usize,
    pub previous_mode: AppMode,
    // 确认对话框
    pub confirm_action: Option<ConfirmAction>,
    // 当前执行的命令
    pub current_command: String,
    // 日志自动滚动
    pub logs_auto_scroll: bool,
    // 日志自动刷新
    pub logs_auto_refresh: bool,
    pub logs_refresh_interval: Duration,
    pub last_logs_refresh: Instant,
    // 执行操作标志
    pub pending_exec: Option<String>,
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
            pending_exec: None,
        }
    }
}

impl AppState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn should_refresh(&self) -> bool {
        self.auto_refresh && self.last_update.elapsed() >= self.refresh_interval
    }

    pub fn should_refresh_logs(&self) -> bool {
        self.logs_auto_refresh && self.mode == AppMode::Logs 
            && self.last_logs_refresh.elapsed() >= self.logs_refresh_interval
    }

    pub fn refresh_logs(&mut self) {
        self.last_logs_refresh = Instant::now();
    }

    pub fn refresh_data(&mut self) {
        self.last_update = Instant::now();
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        // 处理搜索模式
        if self.search_mode {
            return self.handle_search_key_event(key_event);
        }

        // 处理确认对话框
        if self.confirm_action.is_some() {
            return self.handle_confirm_key_event(key_event);
        }

        match key_event.code {
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Char('?') | KeyCode::F(1) => self.mode = AppMode::Help,
            KeyCode::Esc => {
                match self.mode {
                    AppMode::Help | AppMode::Logs | AppMode::Describe => {
                        self.reset_scroll();
                        self.mode = self.get_previous_mode();
                    }
                    AppMode::PodList | AppMode::ServiceList | AppMode::NodeList 
                    | AppMode::DeploymentList | AppMode::JobList | AppMode::DaemonSetList | AppMode::PVCList | AppMode::PVList
                    | AppMode::ConfigMapList | AppMode::SecretList => {
                        self.mode = AppMode::NamespaceList;
                    }
                    _ => {}
                }
            }
            // Vim 风格导航
            KeyCode::Char('j') | KeyCode::Down => self.move_selection_down(),
            KeyCode::Char('k') | KeyCode::Up => self.move_selection_up(),
            KeyCode::Char('h') | KeyCode::Left => self.handle_left_navigation(),
            KeyCode::Char('l') | KeyCode::Right => self.handle_right_navigation(),
            // 滚动操作（仅在 Logs 和 Describe 模式下）
            KeyCode::Char('J') => self.scroll_down(),
            KeyCode::Char('K') => self.scroll_up(),
            KeyCode::PageDown => self.scroll_page_down(),
            KeyCode::PageUp => self.scroll_page_up(),
            // 资源操作
            KeyCode::Enter => self.handle_enter(),
            KeyCode::Char(' ') => self.handle_describe(), // Space 键查看详情
            KeyCode::Char('L') => self.handle_logs(),       // L 查看日志
            KeyCode::Char('D') => self.handle_delete(),     // D 删除（需确认）
            KeyCode::Char('E') => self.handle_exec(),       // E 进入容器
            // 搜索
            KeyCode::Char('/') => self.start_search(),
            KeyCode::Char('n') => self.search_next(),
            KeyCode::Char('N') => self.search_previous(),
            // 自动滚动切换（仅在日志模式下）
            KeyCode::Char('A') => {
                if self.mode == AppMode::Logs {
                    self.logs_auto_scroll = !self.logs_auto_scroll;
                }
            }
            // 日志自动刷新切换（仅在日志模式下）
            KeyCode::Char('R') => {
                if self.mode == AppMode::Logs {
                    self.logs_auto_refresh = !self.logs_auto_refresh;
                }
            }
            // Tab 切换面板
            KeyCode::Tab => self.switch_panel(),
            _ => {}
        }
        Ok(())
    }

    fn move_selection_up(&mut self) {
        match self.mode {
            AppMode::NamespaceList => {
                if self.selected_namespace_index > 0 {
                    self.selected_namespace_index -= 1;
                }
            }
            AppMode::PodList => {
                if self.selected_pod_index > 0 {
                    self.selected_pod_index -= 1;
                }
            }
            AppMode::ServiceList => {
                if self.selected_service_index > 0 {
                    self.selected_service_index -= 1;
                }
            }
            AppMode::NodeList => {
                if self.selected_node_index > 0 {
                    self.selected_node_index -= 1;
                }
            }
            AppMode::ConfigMapList => {
                if self.selected_configmap_index > 0 {
                    self.selected_configmap_index -= 1;
                }
            }
            AppMode::SecretList => {
                if self.selected_secret_index > 0 {
                    self.selected_secret_index -= 1;
                }
            }
            AppMode::DeploymentList => {
                if self.selected_deployment_index > 0 {
                    self.selected_deployment_index -= 1;
                }
            }
            AppMode::JobList => {
                if self.selected_job_index > 0 {
                    self.selected_job_index -= 1;
                }
            }
            AppMode::DaemonSetList => {
                if self.selected_daemonset_index > 0 {
                    self.selected_daemonset_index -= 1;
                }
            }
            AppMode::PVCList => {
                if self.selected_pvc_index > 0 {
                    self.selected_pvc_index -= 1;
                }
            }
            AppMode::PVList => {
                if self.selected_pv_index > 0 {
                    self.selected_pv_index -= 1;
                }
            }
            _ => {}
        }
    }

    fn move_selection_down(&mut self) {
        match self.mode {
            AppMode::NamespaceList => {
                if self.selected_namespace_index + 1 < self.namespaces.len() {
                    self.selected_namespace_index += 1;
                }
            }
            AppMode::PodList => {
                if self.selected_pod_index + 1 < self.pods.len() {
                    self.selected_pod_index += 1;
                }
            }
            AppMode::ServiceList => {
                if self.selected_service_index + 1 < self.services.len() {
                    self.selected_service_index += 1;
                }
            }
            AppMode::NodeList => {
                if self.selected_node_index + 1 < self.nodes.len() {
                    self.selected_node_index += 1;
                }
            }
            AppMode::ConfigMapList => {
                if self.selected_configmap_index + 1 < self.configmaps.len() {
                    self.selected_configmap_index += 1;
                }
            }
            AppMode::SecretList => {
                if self.selected_secret_index + 1 < self.secrets.len() {
                    self.selected_secret_index += 1;
                }
            }
            AppMode::DeploymentList => {
                if self.selected_deployment_index + 1 < self.deployments.len() {
                    self.selected_deployment_index += 1;
                }
            }
            AppMode::JobList => {
                if self.selected_job_index + 1 < self.jobs.len() {
                    self.selected_job_index += 1;
                }
            }
            AppMode::DaemonSetList => {
                if self.selected_daemonset_index + 1 < self.daemonsets.len() {
                    self.selected_daemonset_index += 1;
                }
            }
            AppMode::PVCList => {
                if self.selected_pvc_index + 1 < self.pvcs.len() {
                    self.selected_pvc_index += 1;
                }
            }
            AppMode::PVList => {
                if self.selected_pv_index + 1 < self.pvs.len() {
                    self.selected_pv_index += 1;
                }
            }
            _ => {}
        }
    }

    fn handle_enter(&mut self) {
        match self.mode {
            AppMode::NamespaceList => {
                if let Some(namespace) = self.namespaces.get(self.selected_namespace_index) {
                    self.current_namespace = namespace.clone();
                    self.mode = AppMode::PodList;
                    self.selected_pod_index = 0;
                    // 清理所有缓存数据，强制刷新
                    self.pods.clear();
                    self.services.clear();
                    self.deployments.clear();
                    self.jobs.clear();
                    self.daemonsets.clear();
                    self.pvcs.clear();
                    self.configmaps.clear();
                    self.secrets.clear();
                    self.logs.clear();
                    self.describe_content.clear();
                    // 重置选中索引
                    self.selected_service_index = 0;
                    self.selected_deployment_index = 0;
                    self.selected_job_index = 0;
                    self.selected_daemonset_index = 0;
                    self.selected_configmap_index = 0;
                    self.selected_secret_index = 0;
                    self.selected_pvc_index = 0;
                    self.selected_pv_index = 0;
                    self.selected_node_index = 0;
                }
            }
            _ => {}
        }
    }

    fn handle_left_navigation(&mut self) {
        match self.mode {
            AppMode::Logs | AppMode::Describe => {
                // 在滚动模式下，h 键用于水平滚动（如果需要）
            }
            _ => {
                // 切换到上一个面板
                self.switch_panel_left();
            }
        }
    }

    fn handle_right_navigation(&mut self) {
        match self.mode {
            AppMode::Logs | AppMode::Describe => {
                // 在滚动模式下，l 键用于水平滚动（如果需要）
            }
            _ => {
                // 切换到下一个面板
                self.switch_panel_right();
            }
        }
    }

    fn switch_panel(&mut self) {
        self.switch_panel_right();
    }

    fn switch_panel_right(&mut self) {
        match self.mode {
            AppMode::NamespaceList => self.mode = AppMode::PodList,
            AppMode::PodList => self.mode = AppMode::ServiceList,
            AppMode::ServiceList => self.mode = AppMode::DeploymentList,
            AppMode::DeploymentList => self.mode = AppMode::JobList,
            AppMode::JobList => self.mode = AppMode::PVCList,
            AppMode::PVCList => self.mode = AppMode::PVList,
            AppMode::PVList => self.mode = AppMode::NodeList,
            AppMode::NodeList => self.mode = AppMode::ConfigMapList,
            AppMode::ConfigMapList => self.mode = AppMode::DaemonSetList,
            AppMode::DaemonSetList => self.mode = AppMode::SecretList,
            AppMode::SecretList => self.mode = AppMode::Help,
            AppMode::Help => self.mode = AppMode::NamespaceList,
            _ => {}
        }
    }

    fn switch_panel_left(&mut self) {
        match self.mode {
            AppMode::NamespaceList => self.mode = AppMode::Help,
            AppMode::Help => self.mode = AppMode::SecretList,
            AppMode::SecretList => self.mode = AppMode::DaemonSetList,
            AppMode::DaemonSetList => self.mode = AppMode::ConfigMapList,
            AppMode::ConfigMapList => self.mode = AppMode::NodeList,
            AppMode::NodeList => self.mode = AppMode::PVList,
            AppMode::PVList => self.mode = AppMode::PVCList,
            AppMode::PVCList => self.mode = AppMode::JobList,
            AppMode::JobList => self.mode = AppMode::DeploymentList,
            AppMode::DeploymentList => self.mode = AppMode::ServiceList,
            AppMode::ServiceList => self.mode = AppMode::PodList,
            AppMode::PodList => self.mode = AppMode::NamespaceList,
            _ => {}
        }
    }

    #[allow(dead_code)]
    pub fn get_selected_namespace(&self) -> Option<&String> {
        self.namespaces.get(self.selected_namespace_index)
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

    // 滚动相关方法
    fn scroll_up(&mut self) {
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
            _ => {}
        }
    }

    fn scroll_down(&mut self) {
        match self.mode {
            AppMode::Logs => {
                if self.logs_scroll + 1 < self.logs.len() {
                    self.logs_scroll += 1;
                }
            }
            AppMode::Describe => {
                let lines: Vec<&str> = self.describe_content.lines().collect();
                if self.describe_scroll + 1 < lines.len() {
                    self.describe_scroll += 1;
                }
            }
            _ => {}
        }
    }

    fn scroll_page_up(&mut self) {
        match self.mode {
            AppMode::Logs => {
                self.logs_scroll = self.logs_scroll.saturating_sub(10);
            }
            AppMode::Describe => {
                self.describe_scroll = self.describe_scroll.saturating_sub(10);
            }
            _ => {}
        }
    }

    fn scroll_page_down(&mut self) {
        match self.mode {
            AppMode::Logs => {
                let max_scroll = self.logs.len().saturating_sub(1);
                self.logs_scroll = (self.logs_scroll + 10).min(max_scroll);
            }
            AppMode::Describe => {
                let lines: Vec<&str> = self.describe_content.lines().collect();
                let max_scroll = lines.len().saturating_sub(1);
                self.describe_scroll = (self.describe_scroll + 10).min(max_scroll);
            }
            _ => {}
        }
    }

    fn reset_scroll(&mut self) {
        self.logs_scroll = 0;
        self.describe_scroll = 0;
    }

    // 操作相关方法
    fn handle_describe(&mut self) {
        match self.mode {
            AppMode::PodList | AppMode::ServiceList | AppMode::NodeList 
            | AppMode::DeploymentList | AppMode::JobList | AppMode::DaemonSetList | AppMode::PVCList | AppMode::PVList
            | AppMode::ConfigMapList | AppMode::SecretList => {
                self.previous_mode = self.mode.clone();
                self.reset_scroll();
                // 清理之前的describe内容
                self.describe_content.clear();
                self.mode = AppMode::Describe;
            }
            _ => {}
        }
    }

    fn handle_logs(&mut self) {
        match self.mode {
            AppMode::PodList => {
                self.previous_mode = self.mode.clone();
                self.reset_scroll();
                self.mode = AppMode::Logs;
            }
            _ => {}
        }
    }

    fn handle_delete(&mut self) {
        match self.mode {
            AppMode::PodList => {
                if let Some(pod) = self.get_selected_pod() {
                    self.confirm_action = Some(ConfirmAction::DeletePod {
                        namespace: self.current_namespace.clone(),
                        name: pod.name.clone(),
                    });
                    self.mode = AppMode::Confirm;
                }
            }
            _ => {}
        }
    }

    fn handle_exec(&mut self) {
        match self.mode {
            AppMode::PodList => {
                if let Some(pod) = self.get_selected_pod() {
                    let cmd = format!("kubectl exec -it -n {} {} -- /bin/sh", self.current_namespace, pod.name);
                    self.set_current_command(&cmd);
                    self.pending_exec = Some(cmd);
                }
            }
            _ => {}
        }
    }

    pub fn get_previous_mode(&self) -> AppMode {
        match self.mode {
            AppMode::Logs | AppMode::Describe => {
                // 从之前记录的模式返回
                self.previous_mode.clone()
            }
            AppMode::Search | AppMode::Confirm => {
                // 搜索和确认模式需要记住之前的模式
                self.previous_mode.clone()
            }
            _ => AppMode::NamespaceList,
        }
    }

    // 搜索相关方法
    fn start_search(&mut self) {
        // 只在列表模式下才能搜索
        match self.mode {
            AppMode::PodList | AppMode::ServiceList | AppMode::NodeList 
            | AppMode::DeploymentList | AppMode::DaemonSetList | AppMode::PVCList | AppMode::PVList
            | AppMode::ConfigMapList | AppMode::SecretList => {
                self.previous_mode = self.mode.clone();
                self.search_mode = true;
                self.search_query.clear();
                self.mode = AppMode::Search;
            }
            _ => {}
        }
    }

    fn search_next(&mut self) {
        if !self.search_results.is_empty() {
            self.current_search_index = (self.current_search_index + 1) % self.search_results.len();
            self.jump_to_search_result();
        }
    }

    fn search_previous(&mut self) {
        if !self.search_results.is_empty() {
            self.current_search_index = if self.current_search_index == 0 {
                self.search_results.len() - 1
            } else {
                self.current_search_index - 1
            };
            self.jump_to_search_result();
        }
    }

    fn jump_to_search_result(&mut self) {
        if let Some(&index) = self.search_results.get(self.current_search_index) {
            match self.previous_mode {
                AppMode::PodList => self.selected_pod_index = index,
                AppMode::ServiceList => self.selected_service_index = index,
                AppMode::NodeList => self.selected_node_index = index,
                AppMode::DeploymentList => self.selected_deployment_index = index,
                AppMode::JobList => self.selected_job_index = index,
                AppMode::DaemonSetList => self.selected_daemonset_index = index,
                AppMode::PVCList => self.selected_pvc_index = index,
                AppMode::PVList => self.selected_pv_index = index,
                AppMode::ConfigMapList => self.selected_configmap_index = index,
                AppMode::SecretList => self.selected_secret_index = index,
                _ => {}
            }
        }
    }

    // 搜索事件处理
    fn handle_search_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        match key_event.code {
            KeyCode::Esc => {
                self.search_mode = false;
                self.search_query.clear();
                self.search_results.clear();
                self.mode = self.previous_mode.clone();
            }
            KeyCode::Enter => {
                // 直接跳转到选中的搜索结果，并退出搜索模式
                if !self.search_results.is_empty() {
                    self.jump_to_search_result();
                    // 退出搜索模式，返回到列表模式
                    self.search_mode = false;
                    self.mode = self.previous_mode.clone();
                    // 保留搜索结果以便后续操作
                }
            }
            KeyCode::Backspace => {
                self.search_query.pop();
                // 实时搜索
                self.perform_search();
            }
            KeyCode::Down => {
                // 在搜索结果中向下导航
                self.search_next();
            }
            KeyCode::Up => {
                // 在搜索结果中向上导航
                self.search_previous();
            }
            KeyCode::Char('j') => {
                // 在搜索结果中向下导航
                self.search_next();
            }
            KeyCode::Char('k') => {
                // 在搜索结果中向上导航
                self.search_previous();
            }
            KeyCode::Char(c) => {
                self.search_query.push(c);
                // 实时搜索
                self.perform_search();
            }
            _ => {}
        }
        Ok(())
    }

    fn perform_search(&mut self) {
        self.search_results.clear();
        self.current_search_index = 0;

        let query = self.search_query.to_lowercase();
        if query.is_empty() {
            return;
        }

        match self.previous_mode {
            AppMode::PodList => {
                for (index, pod) in self.pods.iter().enumerate() {
                    if pod.name.to_lowercase().contains(&query) {
                        self.search_results.push(index);
                    }
                }
            }
            AppMode::ServiceList => {
                for (index, service) in self.services.iter().enumerate() {
                    if service.name.to_lowercase().contains(&query) {
                        self.search_results.push(index);
                    }
                }
            }
            AppMode::NodeList => {
                for (index, node) in self.nodes.iter().enumerate() {
                    if node.name.to_lowercase().contains(&query) {
                        self.search_results.push(index);
                    }
                }
            }
            AppMode::DeploymentList => {
                for (index, deployment) in self.deployments.iter().enumerate() {
                    if deployment.name.to_lowercase().contains(&query) {
                        self.search_results.push(index);
                    }
                }
            }
            AppMode::JobList => {
                for (index, job) in self.jobs.iter().enumerate() {
                    if job.name.to_lowercase().contains(&query) {
                        self.search_results.push(index);
                    }
                }
            }
            AppMode::DaemonSetList => {
                for (index, daemonset) in self.daemonsets.iter().enumerate() {
                    if daemonset.name.to_lowercase().contains(&query) {
                        self.search_results.push(index);
                    }
                }
            }
            AppMode::PVCList => {
                for (index, pvc) in self.pvcs.iter().enumerate() {
                    if pvc.name.to_lowercase().contains(&query) {
                        self.search_results.push(index);
                    }
                }
            }
            AppMode::PVList => {
                for (index, pv) in self.pvs.iter().enumerate() {
                    if pv.name.to_lowercase().contains(&query) {
                        self.search_results.push(index);
                    }
                }
            }
            AppMode::ConfigMapList => {
                for (index, configmap) in self.configmaps.iter().enumerate() {
                    if configmap.name.to_lowercase().contains(&query) {
                        self.search_results.push(index);
                    }
                }
            }
            AppMode::SecretList => {
                for (index, secret) in self.secrets.iter().enumerate() {
                    if secret.name.to_lowercase().contains(&query) {
                        self.search_results.push(index);
                    }
                }
            }
            _ => {}
        }

        if !self.search_results.is_empty() {
            self.jump_to_search_result();
        }
    }

    // 确认对话框事件处理
    fn handle_confirm_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        match key_event.code {
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                // 执行确认操作
                self.execute_confirm_action();
                self.confirm_action = None;
                self.mode = self.get_previous_mode();
            }
            KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                // 取消操作
                self.confirm_action = None;
                self.mode = self.get_previous_mode();
            }
            _ => {}
        }
        Ok(())
    }

    fn execute_confirm_action(&mut self) {
        if let Some(ref action) = self.confirm_action {
            match action {
                ConfirmAction::DeletePod { namespace, name } => {
                    let cmd = format!("kubectl delete pod -n {} {}", namespace, name);
                    self.set_current_command(&cmd);
                    // 这里将在主循环中实际执行删除操作
                }
                _ => {
                    // 其他删除操作的实现
                }
            }
        }
    }
}