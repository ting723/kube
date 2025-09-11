use crossterm::event::KeyEvent;
use crate::kubectl::types::*;
use std::time::{Duration, Instant};

// 应用模式枚举
#[derive(Debug, Clone, PartialEq)]
pub enum AppMode {
    // 资源列表模式
    NamespaceList,
    PodList,
    ServiceList,
    DeploymentList,
    JobList,
    DaemonSetList,
    PVCList,
    PVList,
    NodeList,
    ConfigMapList,
    SecretList,
    // 新增资源类型
    StatefulSetList,
    IngressList,
    NetworkPolicyList,
    RoleList,
    RoleBindingList,
    ClusterRoleList,
    ClusterRoleBindingList,
    ServiceAccountList,
    
    // 视图模式
    Logs,
    Describe,
    YamlView,
    TopView,
    
    // 其他模式
    Search,
    Confirm,
    Help,
    
    // 新增更多资源面板模式
    MoreResources,
}

// 确认操作枚举
#[derive(Debug, Clone)]
pub enum ConfirmAction {
    DeletePod { namespace: String, name: String },
    DeleteService { namespace: String, name: String },
    DeleteDeployment { namespace: String, name: String },
    DeleteJob { namespace: String, name: String },
    DeleteDaemonSet { namespace: String, name: String },
    DeleteConfigMap { namespace: String, name: String },
    DeleteSecret { namespace: String, name: String },
    DeletePVC { namespace: String, name: String },
    DeletePV { name: String },
    DeleteStatefulSet { namespace: String, name: String },
    DeleteIngress { namespace: String, name: String },
    DeleteNetworkPolicy { namespace: String, name: String },
    DeleteRole { namespace: String, name: String },
    DeleteRoleBinding { namespace: String, name: String },
    DeleteClusterRole { name: String },
    DeleteClusterRoleBinding { name: String },
    DeleteServiceAccount { namespace: String, name: String },
}

// 应用状态结构体
pub struct AppState {
    // 命名空间相关
    pub namespaces: Vec<String>,
    pub selected_namespace_index: usize,
    pub current_namespace: String,
    
    // Pod相关
    pub pods: Vec<Pod>,
    pub selected_pod_index: usize,
    
    // Service相关
    pub services: Vec<Service>,
    pub selected_service_index: usize,
    
    // Deployment相关
    pub deployments: Vec<Deployment>,
    pub selected_deployment_index: usize,
    
    // Job相关
    pub jobs: Vec<Job>,
    pub selected_job_index: usize,
    
    // DaemonSet相关
    pub daemonsets: Vec<DaemonSet>,
    pub selected_daemonset_index: usize,
    
    // PVC相关
    pub pvcs: Vec<PVC>,
    pub selected_pvc_index: usize,
    
    // PV相关
    pub pvs: Vec<PV>,
    pub selected_pv_index: usize,
    
    // Node相关
    pub nodes: Vec<Node>,
    pub selected_node_index: usize,
    
    // ConfigMap相关
    pub configmaps: Vec<ConfigMap>,
    pub selected_configmap_index: usize,
    
    // Secret相关
    pub secrets: Vec<Secret>,
    pub selected_secret_index: usize,
    
    // 新增资源类型
    pub statefulsets: Vec<StatefulSet>,
    pub selected_statefulset_index: usize,
    
    pub ingresses: Vec<Ingress>,
    pub selected_ingress_index: usize,
    
    pub network_policies: Vec<NetworkPolicy>,
    pub selected_network_policy_index: usize,
    
    pub roles: Vec<Role>,
    pub selected_role_index: usize,
    
    pub role_bindings: Vec<RoleBinding>,
    pub selected_role_binding_index: usize,
    
    pub cluster_roles: Vec<ClusterRole>,
    pub selected_cluster_role_index: usize,
    
    pub cluster_role_bindings: Vec<ClusterRoleBinding>,
    pub selected_cluster_role_binding_index: usize,
    
    pub service_accounts: Vec<ServiceAccount>,
    pub selected_service_account_index: usize,
    
    // 日志相关
    pub logs: Vec<String>,
    pub logs_scroll: usize,
    pub logs_auto_scroll: bool,
    pub logs_auto_refresh: bool,
    
    // 描述内容相关
    pub describe_content: String,
    pub describe_scroll: usize,
    
    // YAML内容相关
    pub yaml_content: String,
    pub yaml_scroll: usize,
    
    // Top视图相关
    pub pod_metrics: Vec<PodMetrics>,
    pub metrics_scroll: usize,
    
    // 搜索相关
    pub search_mode: bool,
    pub search_query: String,
    pub search_results: Vec<usize>,

    // 确认操作相关
    pub confirm_action: Option<ConfirmAction>,
    
    // 待执行命令
    pub pending_exec: Option<String>,
    
    // 当前执行的命令
    pub current_command: String,
    
    // 应用模式
    pub mode: AppMode,
    pub previous_mode: AppMode,
    
    // UI相关
    pub text_selection_mode: bool,
    pub language_chinese: bool,
    pub mouse_capture_enabled: bool,
    
    // 刷新相关
    pub global_refresh_enabled: bool,
    pub describe_auto_refresh: bool,
    pub yaml_auto_refresh: bool,
    pub last_update: Instant,
    pub refresh_status_text: String,
    
    // 更多资源面板相关
    pub selected_more_resource_index: usize,
    pub should_quit: bool,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            namespaces: Vec::new(),
            selected_namespace_index: 0,
            current_namespace: String::new(),
            
            pods: Vec::new(),
            selected_pod_index: 0,
            
            services: Vec::new(),
            selected_service_index: 0,
            
            deployments: Vec::new(),
            selected_deployment_index: 0,
            
            jobs: Vec::new(),
            selected_job_index: 0,
            
            daemonsets: Vec::new(),
            selected_daemonset_index: 0,
            
            pvcs: Vec::new(),
            selected_pvc_index: 0,
            
            pvs: Vec::new(),
            selected_pv_index: 0,
            
            nodes: Vec::new(),
            selected_node_index: 0,
            
            configmaps: Vec::new(),
            selected_configmap_index: 0,
            
            secrets: Vec::new(),
            selected_secret_index: 0,
            
            statefulsets: Vec::new(),
            selected_statefulset_index: 0,
            
            ingresses: Vec::new(),
            selected_ingress_index: 0,
            
            network_policies: Vec::new(),
            selected_network_policy_index: 0,
            
            roles: Vec::new(),
            selected_role_index: 0,
            
            role_bindings: Vec::new(),
            selected_role_binding_index: 0,
            
            cluster_roles: Vec::new(),
            selected_cluster_role_index: 0,
            
            cluster_role_bindings: Vec::new(),
            selected_cluster_role_binding_index: 0,
            
            service_accounts: Vec::new(),
            selected_service_account_index: 0,
            
            logs: Vec::new(),
            logs_scroll: 0,
            logs_auto_scroll: true,
            logs_auto_refresh: true,
            
            describe_content: String::new(),
            describe_scroll: 0,
            
            yaml_content: String::new(),
            yaml_scroll: 0,
            
            pod_metrics: Vec::new(),
            metrics_scroll: 0,
            
            search_mode: false,
            search_query: String::new(),
            search_results: Vec::new(),

            confirm_action: None,
            
            pending_exec: None,
            
            current_command: String::new(),
            
            mode: AppMode::NamespaceList,
            previous_mode: AppMode::NamespaceList,
            
            text_selection_mode: false,
            language_chinese: true,
            mouse_capture_enabled: false,
            
            global_refresh_enabled: true,
            describe_auto_refresh: false,
            yaml_auto_refresh: false,
            last_update: Instant::now(),
            refresh_status_text: String::new(),
            
            selected_more_resource_index: 0,
            should_quit: false,
        }
    }
    
    // 重置滚动位置
    pub fn reset_scroll(&mut self) {
        self.logs_scroll = 0;
        self.describe_scroll = 0;
        self.yaml_scroll = 0;
        self.metrics_scroll = 0;
    }
    
    // 设置当前执行的命令
    pub fn set_current_command(&mut self, command: &str) {
        self.current_command = command.to_string();
    }
    
    // 清除当前执行的命令
    pub fn clear_current_command(&mut self) {
        self.current_command.clear();
    }
    
    // 刷新数据
    pub fn refresh_data(&mut self) {
        self.last_update = Instant::now();
    }
    
    // 刷新日志
    pub fn refresh_logs(&mut self) {
        self.last_update = Instant::now();
    }
    
    // 刷新描述内容
    pub fn refresh_describe(&mut self) {
        self.last_update = Instant::now();
    }
    
    // 刷新YAML内容
    pub fn refresh_yaml(&mut self) {
        self.last_update = Instant::now();
    }
    
    // 检查是否应该刷新
    pub fn should_refresh(&self) -> bool {
        self.global_refresh_enabled && self.last_update.elapsed() > Duration::from_secs(5)
    }
    
    // 检查是否应该刷新日志
    pub fn should_refresh_logs(&self) -> bool {
        self.logs_auto_refresh && self.last_update.elapsed() > Duration::from_secs(2)
    }
    
    // 检查是否应该刷新描述内容
    pub fn should_refresh_describe(&self) -> bool {
        self.describe_auto_refresh && self.last_update.elapsed() > Duration::from_secs(5)
    }
    
    // 检查是否应该刷新YAML内容
    pub fn should_refresh_yaml(&self) -> bool {
        self.yaml_auto_refresh && self.last_update.elapsed() > Duration::from_secs(5)
    }
    
    // 更新刷新状态文本
    pub fn update_refresh_status(&mut self) {
        let mut status_parts = Vec::new();
        
        if self.global_refresh_enabled {
            status_parts.push("列表");
        }
        
        if self.logs_auto_refresh {
            status_parts.push("日志");
        }
        
        if self.describe_auto_refresh {
            status_parts.push("描述");
        }
        
        if self.yaml_auto_refresh {
            status_parts.push("YAML");
        }
        
        if status_parts.is_empty() {
            self.refresh_status_text = String::new();
        } else {
            self.refresh_status_text = format!("[自动刷新: {}]", status_parts.join(", "));
        }
    }
    
    // 获取鼠标模式文本
    pub fn get_mouse_mode_text(&self) -> String {
        if self.text_selection_mode {
            if self.language_chinese {
                "文本选择模式".to_string()
            } else {
                "Text Selection Mode".to_string()
            }
        } else {
            String::new()
        }
    }
    
    // 切换鼠标模式
    pub fn toggle_mouse_mode(&mut self) {
        self.text_selection_mode = !self.text_selection_mode;
    }
    
    // 检查是否应该启用鼠标捕获
    pub fn should_enable_mouse_capture(&self) -> bool {
        // 在文本选择模式下禁用鼠标捕获以允许文本选择
        // 在Logs模式下，根据text_selection_mode决定是否启用鼠标捕获
        match self.mode {
            AppMode::Logs => !self.text_selection_mode,
            AppMode::Describe => !self.text_selection_mode,
            AppMode::YamlView => !self.text_selection_mode,
            _ => true,
        }
    }
    
    // 获取选中的Pod
    pub fn get_selected_pod(&self) -> Option<&Pod> {
        self.pods.get(self.selected_pod_index)
    }
    
    // 获取选中的Service
    pub fn get_selected_service(&self) -> Option<&Service> {
        self.services.get(self.selected_service_index)
    }
    
    // 获取选中的Deployment
    pub fn get_selected_deployment(&self) -> Option<&Deployment> {
        self.deployments.get(self.selected_deployment_index)
    }
    
    // 获取选中的Job
    pub fn get_selected_job(&self) -> Option<&Job> {
        self.jobs.get(self.selected_job_index)
    }
    
    // 获取选中的DaemonSet
    pub fn get_selected_daemonset(&self) -> Option<&DaemonSet> {
        self.daemonsets.get(self.selected_daemonset_index)
    }
    
    // 获取选中的PVC
    pub fn get_selected_pvc(&self) -> Option<&PVC> {
        self.pvcs.get(self.selected_pvc_index)
    }
    
    // 获取选中的PV
    pub fn get_selected_pv(&self) -> Option<&PV> {
        self.pvs.get(self.selected_pv_index)
    }
    
    // 获取选中的Node
    pub fn get_selected_node(&self) -> Option<&Node> {
        self.nodes.get(self.selected_node_index)
    }
    
    // 获取选中的ConfigMap
    pub fn get_selected_configmap(&self) -> Option<&ConfigMap> {
        self.configmaps.get(self.selected_configmap_index)
    }
    
    // 获取选中的Secret
    pub fn get_selected_secret(&self) -> Option<&Secret> {
        self.secrets.get(self.selected_secret_index)
    }
    
    // 获取选中的StatefulSet
    pub fn get_selected_statefulset(&self) -> Option<&StatefulSet> {
        self.statefulsets.get(self.selected_statefulset_index)
    }
    
    // 获取选中的Ingress
    pub fn get_selected_ingress(&self) -> Option<&Ingress> {
        self.ingresses.get(self.selected_ingress_index)
    }
    
    // 获取选中的NetworkPolicy
    pub fn get_selected_network_policy(&self) -> Option<&NetworkPolicy> {
        self.network_policies.get(self.selected_network_policy_index)
    }
    
    // 获取选中的Role
    pub fn get_selected_role(&self) -> Option<&Role> {
        self.roles.get(self.selected_role_index)
    }
    
    // 获取选中的RoleBinding
    pub fn get_selected_role_binding(&self) -> Option<&RoleBinding> {
        self.role_bindings.get(self.selected_role_binding_index)
    }
    
    // 获取选中的ClusterRole
    pub fn get_selected_cluster_role(&self) -> Option<&ClusterRole> {
        self.cluster_roles.get(self.selected_cluster_role_index)
    }
    
    // 获取选中的ClusterRoleBinding
    pub fn get_selected_cluster_role_binding(&self) -> Option<&ClusterRoleBinding> {
        self.cluster_role_bindings.get(self.selected_cluster_role_binding_index)
    }
    
    // 获取选中的ServiceAccount
    pub fn get_selected_service_account(&self) -> Option<&ServiceAccount> {
        self.service_accounts.get(self.selected_service_account_index)
    }
    
    // 获取前一个模式
    pub fn get_previous_mode(&self) -> AppMode {
        self.previous_mode.clone()
    }
    
    // 强制刷新当前模式
    pub fn force_refresh_current_mode(&mut self) {
        // 更新时间戳以触发刷新
        self.last_update = Instant::now().checked_sub(Duration::from_secs(10)).unwrap_or_else(|| Instant::now());
    }
    
    // 处理键盘事件
    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> anyhow::Result<()> {
        match self.mode {
            AppMode::NamespaceList => self.handle_namespace_list_navigation(key_event),
            AppMode::PodList => self.handle_pod_list_navigation(key_event),
            AppMode::ServiceList => self.handle_service_list_navigation(key_event),
            AppMode::DeploymentList => self.handle_deployment_list_navigation(key_event),
            AppMode::JobList => self.handle_job_list_navigation(key_event),
            AppMode::DaemonSetList => self.handle_daemonset_list_navigation(key_event),
            AppMode::PVCList => self.handle_pvc_list_navigation(key_event),
            AppMode::PVList => self.handle_pv_list_navigation(key_event),
            AppMode::NodeList => self.handle_node_list_navigation(key_event),
            AppMode::ConfigMapList => self.handle_configmap_list_navigation(key_event),
            AppMode::SecretList => self.handle_secret_list_navigation(key_event),
            // 新增资源类型处理
            AppMode::StatefulSetList => self.handle_statefulset_list_navigation(key_event),
            AppMode::IngressList => self.handle_ingress_list_navigation(key_event),
            AppMode::NetworkPolicyList => self.handle_network_policy_list_navigation(key_event),
            AppMode::RoleList => self.handle_role_list_navigation(key_event),
            AppMode::RoleBindingList => self.handle_role_binding_list_navigation(key_event),
            AppMode::ClusterRoleList => self.handle_cluster_role_list_navigation(key_event),
            AppMode::ClusterRoleBindingList => self.handle_cluster_role_binding_list_navigation(key_event),
            AppMode::ServiceAccountList => self.handle_service_account_list_navigation(key_event),
            // 视图模式处理
            AppMode::Logs => self.handle_logs_navigation(key_event),
            AppMode::Describe => self.handle_describe_navigation(key_event),
            AppMode::YamlView => self.handle_yaml_navigation(key_event),
            AppMode::TopView => self.handle_top_navigation(key_event),
            // 其他模式处理
            AppMode::Search => self.handle_search_navigation(key_event),
            AppMode::Confirm => self.handle_confirm_navigation(key_event),
            AppMode::Help => self.handle_help_navigation(key_event),
            // 新增更多资源面板处理
            AppMode::MoreResources => self.handle_more_resources_navigation(key_event),
        }
        Ok(())
    }
    
    // 处理命名空间列表导航
    fn handle_namespace_list_navigation(&mut self, key_event: crossterm::event::KeyEvent) {
        use crossterm::event::KeyCode;

        match key_event.code {
            KeyCode::Char('j') | KeyCode::Down => {
                if self.selected_namespace_index < self.namespaces.len().saturating_sub(1) {
                    self.selected_namespace_index += 1;
                }
            }
            KeyCode::Char('k') | KeyCode::Up => {
                if self.selected_namespace_index > 0 {
                    self.selected_namespace_index -= 1;
                }
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                // 强制刷新当前模式
                self.force_refresh_current_mode();
            }
            KeyCode::Enter => {
                if !self.namespaces.is_empty() {
                    self.current_namespace = self.namespaces[self.selected_namespace_index].clone();
                    // 切换到Pod列表模式
                    self.mode = AppMode::PodList;
                    // 重置所有资源列表的选中索引
                    self.selected_pod_index = 0;
                    self.selected_service_index = 0;
                    self.selected_deployment_index = 0;
                    self.selected_job_index = 0;
                    self.selected_daemonset_index = 0;
                    self.selected_configmap_index = 0;
                    self.selected_secret_index = 0;
                    self.selected_pvc_index = 0;
                    self.selected_pv_index = 0;
                    self.selected_node_index = 0;
                    // 重置滚动位置
                    self.reset_scroll();
                }
            }
            KeyCode::Char('1') => self.mode = AppMode::PodList,
            KeyCode::Char('2') => self.mode = AppMode::ServiceList,
            KeyCode::Char('3') => self.mode = AppMode::DeploymentList,
            KeyCode::Char('4') => self.mode = AppMode::JobList,
            KeyCode::Char('5') => self.mode = AppMode::DaemonSetList,
            KeyCode::Char('6') => self.mode = AppMode::ConfigMapList,
            KeyCode::Char('7') => self.mode = AppMode::SecretList,
            KeyCode::Char('8') => self.mode = AppMode::MoreResources, // 添加进入更多资源面板的快捷键
            KeyCode::Char('/') => {
                self.search_mode = true;
                self.mode = AppMode::Search;
            }
            KeyCode::Char('?') => self.mode = AppMode::Help,
            KeyCode::Tab => self.mode = AppMode::PodList,
            KeyCode::BackTab => self.mode = AppMode::SecretList,
            KeyCode::F(1) => self.mode = AppMode::PodList,
            KeyCode::F(2) => self.mode = AppMode::ServiceList,
            KeyCode::F(3) => self.mode = AppMode::DeploymentList,
            KeyCode::F(4) => self.mode = AppMode::JobList,
            KeyCode::F(5) => self.mode = AppMode::DaemonSetList,
            KeyCode::F(6) => self.mode = AppMode::ConfigMapList,
            KeyCode::F(7) => self.mode = AppMode::SecretList,
            _ => {}
        }
    }

    // 处理Pod列表导航
    fn handle_pod_list_navigation(&mut self, key_event: crossterm::event::KeyEvent) {
        use crossterm::event::KeyCode;

        match key_event.code {
            KeyCode::Char('j') | KeyCode::Down => {
                if self.selected_pod_index < self.pods.len().saturating_sub(1) {
                    self.selected_pod_index += 1;
                }
            }
            KeyCode::Char('k') | KeyCode::Up => {
                if self.selected_pod_index > 0 {
                    self.selected_pod_index -= 1;
                }
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                // 强制刷新当前模式
                self.force_refresh_current_mode();
            }
            KeyCode::Char(' ') => {
                if !self.pods.is_empty() {
                    self.previous_mode = self.mode.clone();
                    self.mode = AppMode::Describe;
                    self.describe_content.clear();
                    self.describe_scroll = 0;
                }
            }
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                if !self.pods.is_empty() {
                    self.previous_mode = self.mode.clone();
                    self.mode = AppMode::YamlView;
                    self.yaml_content.clear();
                    self.yaml_scroll = 0;
                }
            }
            KeyCode::Char('l') | KeyCode::Char('L') => {
                if !self.pods.is_empty() {
                    self.mode = AppMode::Logs;
                    self.logs.clear();
                    self.logs_scroll = 0;
                }
            }
            KeyCode::Char('t') | KeyCode::Char('T') => {
                self.mode = AppMode::TopView;
                self.pod_metrics.clear();
                self.metrics_scroll = 0;
            }
            KeyCode::Char('d') | KeyCode::Char('D') => {
                if !self.pods.is_empty() {
                    self.confirm_action = Some(ConfirmAction::DeletePod {
                        namespace: self.current_namespace.clone(),
                        name: self.pods[self.selected_pod_index].name.clone(),
                    });
                    self.mode = AppMode::Confirm;
                }
            }
            KeyCode::Char('e') | KeyCode::Char('E') => {
                if !self.pods.is_empty() {
                    let pod_name = self.pods[self.selected_pod_index].name.clone();
                    self.pending_exec = Some(format!("kubectl exec -it -n {} {} -- sh", self.current_namespace, pod_name));
                }
            }
            KeyCode::Char('/') => {
                self.search_mode = true;
                self.mode = AppMode::Search;
            }
            KeyCode::Tab => self.mode = AppMode::ServiceList,
            KeyCode::BackTab => self.mode = AppMode::SecretList,
            _ => {}
        }
    }

    // 处理Service列表导航
    fn handle_service_list_navigation(&mut self, key_event: crossterm::event::KeyEvent) {
        use crossterm::event::KeyCode;

        match key_event.code {
            KeyCode::Char('j') | KeyCode::Down => {
                if self.selected_service_index < self.services.len().saturating_sub(1) {
                    self.selected_service_index += 1;
                }
            }
            KeyCode::Char('k') | KeyCode::Up => {
                if self.selected_service_index > 0 {
                    self.selected_service_index -= 1;
                }
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                // 强制刷新当前模式
                self.force_refresh_current_mode();
            }
            KeyCode::Char(' ') => {
                if !self.services.is_empty() {
                    self.previous_mode = self.mode.clone();
                    self.mode = AppMode::Describe;
                    self.describe_content.clear();
                    self.describe_scroll = 0;
                }
            }
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                if !self.services.is_empty() {
                    self.previous_mode = self.mode.clone();
                    self.mode = AppMode::YamlView;
                    self.yaml_content.clear();
                    self.yaml_scroll = 0;
                }
            }
            KeyCode::Char('d') | KeyCode::Char('D') => {
                if !self.services.is_empty() {
                    self.confirm_action = Some(ConfirmAction::DeleteService {
                        namespace: self.current_namespace.clone(),
                        name: self.services[self.selected_service_index].name.clone(),
                    });
                    self.mode = AppMode::Confirm;
                }
            }
            KeyCode::Char('/') => {
                self.search_mode = true;
                self.mode = AppMode::Search;
            }
            KeyCode::Tab => self.mode = AppMode::DeploymentList,
            KeyCode::BackTab => self.mode = AppMode::PodList,
            _ => {}
        }
    }

    // 处理Deployment列表导航
    fn handle_deployment_list_navigation(&mut self, key_event: crossterm::event::KeyEvent) {
        use crossterm::event::KeyCode;

        match key_event.code {
            KeyCode::Char('j') | KeyCode::Down => {
                if self.selected_deployment_index < self.deployments.len().saturating_sub(1) {
                    self.selected_deployment_index += 1;
                }
            }
            KeyCode::Char('k') | KeyCode::Up => {
                if self.selected_deployment_index > 0 {
                    self.selected_deployment_index -= 1;
                }
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                // 强制刷新当前模式
                self.force_refresh_current_mode();
            }
            KeyCode::Char(' ') => {
                if !self.deployments.is_empty() {
                    self.previous_mode = self.mode.clone();
                    self.mode = AppMode::Describe;
                    self.describe_content.clear();
                    self.describe_scroll = 0;
                }
            }
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                if !self.deployments.is_empty() {
                    self.previous_mode = self.mode.clone();
                    self.mode = AppMode::YamlView;
                    self.yaml_content.clear();
                    self.yaml_scroll = 0;
                }
            }
            KeyCode::Char('d') | KeyCode::Char('D') => {
                if !self.deployments.is_empty() {
                    self.confirm_action = Some(ConfirmAction::DeleteDeployment {
                        namespace: self.current_namespace.clone(),
                        name: self.deployments[self.selected_deployment_index].name.clone(),
                    });
                    self.mode = AppMode::Confirm;
                }
            }
            KeyCode::Char('/') => {
                self.search_mode = true;
                self.mode = AppMode::Search;
            }
            KeyCode::Tab => self.mode = AppMode::MoreResources,
            KeyCode::BackTab => self.mode = AppMode::ServiceList,
            _ => {}
        }
    }

    // 处理Job列表导航
    fn handle_job_list_navigation(&mut self, key_event: crossterm::event::KeyEvent) {
        use crossterm::event::KeyCode;

        match key_event.code {
            KeyCode::Char('j') | KeyCode::Down => {
                if self.selected_job_index < self.jobs.len().saturating_sub(1) {
                    self.selected_job_index += 1;
                }
            }
            KeyCode::Char('k') | KeyCode::Up => {
                if self.selected_job_index > 0 {
                    self.selected_job_index -= 1;
                }
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                // 强制刷新当前模式
                self.force_refresh_current_mode();
            }
            KeyCode::Char(' ') => {
                if !self.jobs.is_empty() {
                    self.previous_mode = self.mode.clone();
                    self.mode = AppMode::Describe;
                    self.describe_content.clear();
                    self.describe_scroll = 0;
                }
            }
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                if !self.jobs.is_empty() {
                    self.previous_mode = self.mode.clone();
                    self.mode = AppMode::YamlView;
                    self.yaml_content.clear();
                    self.yaml_scroll = 0;
                }
            }
            KeyCode::Char('d') | KeyCode::Char('D') => {
                if !self.jobs.is_empty() {
                    self.confirm_action = Some(ConfirmAction::DeleteJob {
                        namespace: self.current_namespace.clone(),
                        name: self.jobs[self.selected_job_index].name.clone(),
                    });
                    self.mode = AppMode::Confirm;
                }
            }
            KeyCode::Char('/') => {
                self.search_mode = true;
                self.mode = AppMode::Search;
            }
            KeyCode::Tab => self.mode = AppMode::DaemonSetList,
            KeyCode::BackTab => self.mode = AppMode::DeploymentList,
            _ => {}
        }
    }

    // 处理DaemonSet列表导航
    fn handle_daemonset_list_navigation(&mut self, key_event: crossterm::event::KeyEvent) {
        use crossterm::event::KeyCode;

        match key_event.code {
            KeyCode::Char('j') | KeyCode::Down => {
                if self.selected_daemonset_index < self.daemonsets.len().saturating_sub(1) {
                    self.selected_daemonset_index += 1;
                }
            }
            KeyCode::Char('k') | KeyCode::Up => {
                if self.selected_daemonset_index > 0 {
                    self.selected_daemonset_index -= 1;
                }
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                // 强制刷新当前模式
                self.force_refresh_current_mode();
            }
            KeyCode::Char(' ') => {
                if !self.daemonsets.is_empty() {
                    self.previous_mode = self.mode.clone();
                    self.mode = AppMode::Describe;
                    self.describe_content.clear();
                    self.describe_scroll = 0;
                }
            }
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                if !self.daemonsets.is_empty() {
                    self.previous_mode = self.mode.clone();
                    self.mode = AppMode::YamlView;
                    self.yaml_content.clear();
                    self.yaml_scroll = 0;
                }
            }
            KeyCode::Char('d') | KeyCode::Char('D') => {
                if !self.daemonsets.is_empty() {
                    self.confirm_action = Some(ConfirmAction::DeleteDaemonSet {
                        namespace: self.current_namespace.clone(),
                        name: self.daemonsets[self.selected_daemonset_index].name.clone(),
                    });
                    self.mode = AppMode::Confirm;
                }
            }
            KeyCode::Char('/') => {
                self.search_mode = true;
                self.mode = AppMode::Search;
            }
            KeyCode::Tab => self.mode = AppMode::PVCList,
            KeyCode::BackTab => self.mode = AppMode::JobList,
            _ => {}
        }
    }

    // 处理PVC列表导航
    fn handle_pvc_list_navigation(&mut self, key_event: crossterm::event::KeyEvent) {
        use crossterm::event::KeyCode;

        match key_event.code {
            KeyCode::Char('j') | KeyCode::Down => {
                if self.selected_pvc_index < self.pvcs.len().saturating_sub(1) {
                    self.selected_pvc_index += 1;
                }
            }
            KeyCode::Char('k') | KeyCode::Up => {
                if self.selected_pvc_index > 0 {
                    self.selected_pvc_index -= 1;
                }
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                // 强制刷新当前模式
                self.force_refresh_current_mode();
            }
            KeyCode::Char(' ') => {
                if !self.pvcs.is_empty() {
                    self.previous_mode = self.mode.clone();
                    self.mode = AppMode::Describe;
                    self.describe_content.clear();
                    self.describe_scroll = 0;
                }
            }
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                if !self.pvcs.is_empty() {
                    self.previous_mode = self.mode.clone();
                    self.mode = AppMode::YamlView;
                    self.yaml_content.clear();
                    self.yaml_scroll = 0;
                }
            }
            KeyCode::Char('d') | KeyCode::Char('D') => {
                if !self.pvcs.is_empty() {
                    self.confirm_action = Some(ConfirmAction::DeletePVC {
                        namespace: self.current_namespace.clone(),
                        name: self.pvcs[self.selected_pvc_index].name.clone(),
                    });
                    self.mode = AppMode::Confirm;
                }
            }
            KeyCode::Char('/') => {
                self.search_mode = true;
                self.mode = AppMode::Search;
            }
            KeyCode::Tab => self.mode = AppMode::PVList,
            KeyCode::BackTab => self.mode = AppMode::DaemonSetList,
            _ => {}
        }
    }

    // 处理PV列表导航
    fn handle_pv_list_navigation(&mut self, key_event: crossterm::event::KeyEvent) {
        use crossterm::event::KeyCode;

        match key_event.code {
            KeyCode::Char('j') | KeyCode::Down => {
                if self.selected_pv_index < self.pvs.len().saturating_sub(1) {
                    self.selected_pv_index += 1;
                }
            }
            KeyCode::Char('k') | KeyCode::Up => {
                if self.selected_pv_index > 0 {
                    self.selected_pv_index -= 1;
                }
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                // 强制刷新当前模式
                self.force_refresh_current_mode();
            }
            KeyCode::Char(' ') => {
                if !self.pvs.is_empty() {
                    self.previous_mode = self.mode.clone();
                    self.mode = AppMode::Describe;
                    self.describe_content.clear();
                    self.describe_scroll = 0;
                }
            }
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                if !self.pvs.is_empty() {
                    self.previous_mode = self.mode.clone();
                    self.mode = AppMode::YamlView;
                    self.yaml_content.clear();
                    self.yaml_scroll = 0;
                }
            }
            KeyCode::Char('d') | KeyCode::Char('D') => {
                if !self.pvs.is_empty() {
                    self.confirm_action = Some(ConfirmAction::DeletePV {
                        name: self.pvs[self.selected_pv_index].name.clone(),
                    });
                    self.mode = AppMode::Confirm;
                }
            }
            KeyCode::Char('/') => {
                self.search_mode = true;
                self.mode = AppMode::Search;
            }
            KeyCode::Tab => self.mode = AppMode::NodeList,
            KeyCode::BackTab => self.mode = AppMode::PVCList,
            _ => {}
        }
    }

    // 处理Node列表导航
    fn handle_node_list_navigation(&mut self, key_event: crossterm::event::KeyEvent) {
        use crossterm::event::KeyCode;

        match key_event.code {
            KeyCode::Char('j') | KeyCode::Down => {
                if self.selected_node_index < self.nodes.len().saturating_sub(1) {
                    self.selected_node_index += 1;
                }
            }
            KeyCode::Char('k') | KeyCode::Up => {
                if self.selected_node_index > 0 {
                    self.selected_node_index -= 1;
                }
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                // 强制刷新当前模式
                self.force_refresh_current_mode();
            }
            KeyCode::Char(' ') => {
                if !self.nodes.is_empty() {
                    self.previous_mode = self.mode.clone();
                    self.mode = AppMode::Describe;
                    self.describe_content.clear();
                    self.describe_scroll = 0;
                }
            }
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                if !self.nodes.is_empty() {
                    self.previous_mode = self.mode.clone();
                    self.mode = AppMode::YamlView;
                    self.yaml_content.clear();
                    self.yaml_scroll = 0;
                }
            }
            KeyCode::Char('/') => {
                self.search_mode = true;
                self.mode = AppMode::Search;
            }
            KeyCode::Tab => self.mode = AppMode::ConfigMapList,
            KeyCode::BackTab => self.mode = AppMode::PVList,
            _ => {}
        }
    }

    // 处理ConfigMap列表导航
    fn handle_configmap_list_navigation(&mut self, key_event: crossterm::event::KeyEvent) {
        use crossterm::event::KeyCode;

        match key_event.code {
            KeyCode::Char('j') | KeyCode::Down => {
                if self.selected_configmap_index < self.configmaps.len().saturating_sub(1) {
                    self.selected_configmap_index += 1;
                }
            }
            KeyCode::Char('k') | KeyCode::Up => {
                if self.selected_configmap_index > 0 {
                    self.selected_configmap_index -= 1;
                }
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                // 强制刷新当前模式
                self.force_refresh_current_mode();
            }
            KeyCode::Char(' ') => {
                if !self.configmaps.is_empty() {
                    self.previous_mode = self.mode.clone();
                    self.mode = AppMode::Describe;
                    self.describe_content.clear();
                    self.describe_scroll = 0;
                }
            }
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                if !self.configmaps.is_empty() {
                    self.previous_mode = self.mode.clone();
                    self.mode = AppMode::YamlView;
                    self.yaml_content.clear();
                    self.yaml_scroll = 0;
                }
            }
            KeyCode::Char('d') | KeyCode::Char('D') => {
                if !self.configmaps.is_empty() {
                    self.confirm_action = Some(ConfirmAction::DeleteConfigMap {
                        namespace: self.current_namespace.clone(),
                        name: self.configmaps[self.selected_configmap_index].name.clone(),
                    });
                    self.mode = AppMode::Confirm;
                }
            }
            KeyCode::Char('/') => {
                self.search_mode = true;
                self.mode = AppMode::Search;
            }
            KeyCode::Tab => self.mode = AppMode::SecretList,
            KeyCode::BackTab => self.mode = AppMode::NodeList,
            _ => {}
        }
    }

    // 处理Secret列表导航
    fn handle_secret_list_navigation(&mut self, key_event: crossterm::event::KeyEvent) {
        use crossterm::event::KeyCode;

        match key_event.code {
            KeyCode::Char('j') | KeyCode::Down => {
                if self.selected_secret_index < self.secrets.len().saturating_sub(1) {
                    self.selected_secret_index += 1;
                }
            }
            KeyCode::Char('k') | KeyCode::Up => {
                if self.selected_secret_index > 0 {
                    self.selected_secret_index -= 1;
                }
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                // 强制刷新当前模式
                self.force_refresh_current_mode();
            }
            KeyCode::Char(' ') => {
                if !self.secrets.is_empty() {
                    self.previous_mode = self.mode.clone();
                    self.mode = AppMode::Describe;
                    self.describe_content.clear();
                    self.describe_scroll = 0;
                }
            }
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                if !self.secrets.is_empty() {
                    self.previous_mode = self.mode.clone();
                    self.mode = AppMode::YamlView;
                    self.yaml_content.clear();
                    self.yaml_scroll = 0;
                }
            }
            KeyCode::Char('d') | KeyCode::Char('D') => {
                if !self.secrets.is_empty() {
                    self.confirm_action = Some(ConfirmAction::DeleteSecret {
                        namespace: self.current_namespace.clone(),
                        name: self.secrets[self.selected_secret_index].name.clone(),
                    });
                    self.mode = AppMode::Confirm;
                }
            }
            KeyCode::Char('/') => {
                self.search_mode = true;
                self.mode = AppMode::Search;
            }
            KeyCode::Tab => self.mode = AppMode::PodList,
            KeyCode::BackTab => self.mode = AppMode::ConfigMapList,
            _ => {}
        }
    }

    // 处理StatefulSet列表导航
    fn handle_statefulset_list_navigation(&mut self, key_event: crossterm::event::KeyEvent) {
        use crossterm::event::KeyCode;

        match key_event.code {
            KeyCode::Char('j') | KeyCode::Down => {
                if self.selected_statefulset_index < self.statefulsets.len().saturating_sub(1) {
                    self.selected_statefulset_index += 1;
                }
            }
            KeyCode::Char('k') | KeyCode::Up => {
                if self.selected_statefulset_index > 0 {
                    self.selected_statefulset_index -= 1;
                }
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                // 强制刷新当前模式
                self.force_refresh_current_mode();
            }
            KeyCode::Char(' ') => {
                if !self.statefulsets.is_empty() {
                    self.previous_mode = self.mode.clone();
                    self.mode = AppMode::Describe;
                    self.describe_content.clear();
                    self.describe_scroll = 0;
                }
            }
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                if !self.statefulsets.is_empty() {
                    self.previous_mode = self.mode.clone();
                    self.mode = AppMode::YamlView;
                    self.yaml_content.clear();
                    self.yaml_scroll = 0;
                }
            }
            KeyCode::Char('d') | KeyCode::Char('D') => {
                if !self.statefulsets.is_empty() {
                    self.confirm_action = Some(ConfirmAction::DeleteStatefulSet {
                        namespace: self.current_namespace.clone(),
                        name: self.statefulsets[self.selected_statefulset_index].name.clone(),
                    });
                    self.mode = AppMode::Confirm;
                }
            }
            KeyCode::Char('/') => {
                self.search_mode = true;
                self.mode = AppMode::Search;
            }
            KeyCode::Tab => self.mode = AppMode::IngressList,
            KeyCode::BackTab => self.mode = AppMode::ServiceAccountList,
            _ => {}
        }
    }

    // 处理Ingress列表导航
    fn handle_ingress_list_navigation(&mut self, key_event: crossterm::event::KeyEvent) {
        use crossterm::event::KeyCode;

        match key_event.code {
            KeyCode::Char('j') | KeyCode::Down => {
                if self.selected_ingress_index < self.ingresses.len().saturating_sub(1) {
                    self.selected_ingress_index += 1;
                }
            }
            KeyCode::Char('k') | KeyCode::Up => {
                if self.selected_ingress_index > 0 {
                    self.selected_ingress_index -= 1;
                }
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                // 强制刷新当前模式
                self.force_refresh_current_mode();
            }
            KeyCode::Char(' ') => {
                if !self.ingresses.is_empty() {
                    self.previous_mode = self.mode.clone();
                    self.mode = AppMode::Describe;
                    self.describe_content.clear();
                    self.describe_scroll = 0;
                }
            }
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                if !self.ingresses.is_empty() {
                    self.previous_mode = self.mode.clone();
                    self.mode = AppMode::YamlView;
                    self.yaml_content.clear();
                    self.yaml_scroll = 0;
                }
            }
            KeyCode::Char('d') | KeyCode::Char('D') => {
                if !self.ingresses.is_empty() {
                    self.confirm_action = Some(ConfirmAction::DeleteIngress {
                        namespace: self.current_namespace.clone(),
                        name: self.ingresses[self.selected_ingress_index].name.clone(),
                    });
                    self.mode = AppMode::Confirm;
                }
            }
            KeyCode::Char('/') => {
                self.search_mode = true;
                self.mode = AppMode::Search;
            }
            KeyCode::Tab => self.mode = AppMode::NetworkPolicyList,
            KeyCode::BackTab => self.mode = AppMode::StatefulSetList,
            _ => {}
        }
    }

    // 处理NetworkPolicy列表导航
    fn handle_network_policy_list_navigation(&mut self, key_event: crossterm::event::KeyEvent) {
        use crossterm::event::KeyCode;

        match key_event.code {
            KeyCode::Char('j') | KeyCode::Down => {
                if self.selected_network_policy_index < self.network_policies.len().saturating_sub(1) {
                    self.selected_network_policy_index += 1;
                }
            }
            KeyCode::Char('k') | KeyCode::Up => {
                if self.selected_network_policy_index > 0 {
                    self.selected_network_policy_index -= 1;
                }
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                // 强制刷新当前模式
                self.force_refresh_current_mode();
            }
            KeyCode::Char(' ') => {
                if !self.network_policies.is_empty() {
                    self.previous_mode = self.mode.clone();
                    self.mode = AppMode::Describe;
                    self.describe_content.clear();
                    self.describe_scroll = 0;
                }
            }
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                if !self.network_policies.is_empty() {
                    self.previous_mode = self.mode.clone();
                    self.mode = AppMode::YamlView;
                    self.yaml_content.clear();
                    self.yaml_scroll = 0;
                }
            }
            KeyCode::Char('d') | KeyCode::Char('D') => {
                if !self.network_policies.is_empty() {
                    self.confirm_action = Some(ConfirmAction::DeleteNetworkPolicy {
                        namespace: self.current_namespace.clone(),
                        name: self.network_policies[self.selected_network_policy_index].name.clone(),
                    });
                    self.mode = AppMode::Confirm;
                }
            }
            KeyCode::Char('/') => {
                self.search_mode = true;
                self.mode = AppMode::Search;
            }
            KeyCode::Tab => self.mode = AppMode::RoleList,
            KeyCode::BackTab => self.mode = AppMode::IngressList,
            _ => {}
        }
    }

    // 处理Role列表导航
    fn handle_role_list_navigation(&mut self, key_event: crossterm::event::KeyEvent) {
        use crossterm::event::KeyCode;

        match key_event.code {
            KeyCode::Char('j') | KeyCode::Down => {
                if self.selected_role_index < self.roles.len().saturating_sub(1) {
                    self.selected_role_index += 1;
                }
            }
            KeyCode::Char('k') | KeyCode::Up => {
                if self.selected_role_index > 0 {
                    self.selected_role_index -= 1;
                }
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                // 强制刷新当前模式
                self.force_refresh_current_mode();
            }
            KeyCode::Char(' ') => {
                if !self.roles.is_empty() {
                    self.previous_mode = self.mode.clone();
                    self.mode = AppMode::Describe;
                    self.describe_content.clear();
                    self.describe_scroll = 0;
                }
            }
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                if !self.roles.is_empty() {
                    self.previous_mode = self.mode.clone();
                    self.mode = AppMode::YamlView;
                    self.yaml_content.clear();
                    self.yaml_scroll = 0;
                }
            }
            KeyCode::Char('d') | KeyCode::Char('D') => {
                if !self.roles.is_empty() {
                    self.confirm_action = Some(ConfirmAction::DeleteRole {
                        namespace: self.current_namespace.clone(),
                        name: self.roles[self.selected_role_index].name.clone(),
                    });
                    self.mode = AppMode::Confirm;
                }
            }
            KeyCode::Char('/') => {
                self.search_mode = true;
                self.mode = AppMode::Search;
            }
            KeyCode::Tab => self.mode = AppMode::RoleBindingList,
            KeyCode::BackTab => self.mode = AppMode::NetworkPolicyList,
            _ => {}
        }
    }

    // 处理RoleBinding列表导航
    fn handle_role_binding_list_navigation(&mut self, key_event: crossterm::event::KeyEvent) {
        use crossterm::event::KeyCode;

        match key_event.code {
            KeyCode::Char('j') | KeyCode::Down => {
                if self.selected_role_binding_index < self.role_bindings.len().saturating_sub(1) {
                    self.selected_role_binding_index += 1;
                }
            }
            KeyCode::Char('k') | KeyCode::Up => {
                if self.selected_role_binding_index > 0 {
                    self.selected_role_binding_index -= 1;
                }
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                // 强制刷新当前模式
                self.force_refresh_current_mode();
            }
            KeyCode::Char(' ') => {
                if !self.role_bindings.is_empty() {
                    self.previous_mode = self.mode.clone();
                    self.mode = AppMode::Describe;
                    self.describe_content.clear();
                    self.describe_scroll = 0;
                }
            }
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                if !self.role_bindings.is_empty() {
                    self.previous_mode = self.mode.clone();
                    self.mode = AppMode::YamlView;
                    self.yaml_content.clear();
                    self.yaml_scroll = 0;
                }
            }
            KeyCode::Char('d') | KeyCode::Char('D') => {
                if !self.role_bindings.is_empty() {
                    self.confirm_action = Some(ConfirmAction::DeleteRoleBinding {
                        namespace: self.current_namespace.clone(),
                        name: self.role_bindings[self.selected_role_binding_index].name.clone(),
                    });
                    self.mode = AppMode::Confirm;
                }
            }
            KeyCode::Char('/') => {
                self.search_mode = true;
                self.mode = AppMode::Search;
            }
            KeyCode::Tab => self.mode = AppMode::ClusterRoleList,
            KeyCode::BackTab => self.mode = AppMode::RoleList,
            _ => {}
        }
    }

    // 处理ClusterRole列表导航
    fn handle_cluster_role_list_navigation(&mut self, key_event: crossterm::event::KeyEvent) {
        use crossterm::event::KeyCode;

        match key_event.code {
            KeyCode::Char('j') | KeyCode::Down => {
                if self.selected_cluster_role_index < self.cluster_roles.len().saturating_sub(1) {
                    self.selected_cluster_role_index += 1;
                }
            }
            KeyCode::Char('k') | KeyCode::Up => {
                if self.selected_cluster_role_index > 0 {
                    self.selected_cluster_role_index -= 1;
                }
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                // 强制刷新当前模式
                self.force_refresh_current_mode();
            }
            KeyCode::Char(' ') => {
                if !self.cluster_roles.is_empty() {
                    self.previous_mode = self.mode.clone();
                    self.mode = AppMode::Describe;
                    self.describe_content.clear();
                    self.describe_scroll = 0;
                }
            }
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                if !self.cluster_roles.is_empty() {
                    self.previous_mode = self.mode.clone();
                    self.mode = AppMode::YamlView;
                    self.yaml_content.clear();
                    self.yaml_scroll = 0;
                }
            }
            KeyCode::Char('d') | KeyCode::Char('D') => {
                if !self.cluster_roles.is_empty() {
                    self.confirm_action = Some(ConfirmAction::DeleteClusterRole {
                        name: self.cluster_roles[self.selected_cluster_role_index].name.clone(),
                    });
                    self.mode = AppMode::Confirm;
                }
            }
            KeyCode::Char('/') => {
                self.search_mode = true;
                self.mode = AppMode::Search;
            }
            KeyCode::Tab => self.mode = AppMode::ClusterRoleBindingList,
            KeyCode::BackTab => self.mode = AppMode::RoleBindingList,
            _ => {}
        }
    }

    // 处理ClusterRoleBinding列表导航
    fn handle_cluster_role_binding_list_navigation(&mut self, key_event: crossterm::event::KeyEvent) {
        use crossterm::event::KeyCode;

        match key_event.code {
            KeyCode::Char('j') | KeyCode::Down => {
                if self.selected_cluster_role_binding_index < self.cluster_role_bindings.len().saturating_sub(1) {
                    self.selected_cluster_role_binding_index += 1;
                }
            }
            KeyCode::Char('k') | KeyCode::Up => {
                if self.selected_cluster_role_binding_index > 0 {
                    self.selected_cluster_role_binding_index -= 1;
                }
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                // 强制刷新当前模式
                self.force_refresh_current_mode();
            }
            KeyCode::Char(' ') => {
                if !self.cluster_role_bindings.is_empty() {
                    self.previous_mode = self.mode.clone();
                    self.mode = AppMode::Describe;
                    self.describe_content.clear();
                    self.describe_scroll = 0;
                }
            }
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                if !self.cluster_role_bindings.is_empty() {
                    self.previous_mode = self.mode.clone();
                    self.mode = AppMode::YamlView;
                    self.yaml_content.clear();
                    self.yaml_scroll = 0;
                }
            }
            KeyCode::Char('d') | KeyCode::Char('D') => {
                if !self.cluster_role_bindings.is_empty() {
                    self.confirm_action = Some(ConfirmAction::DeleteClusterRoleBinding {
                        name: self.cluster_role_bindings[self.selected_cluster_role_binding_index].name.clone(),
                    });
                    self.mode = AppMode::Confirm;
                }
            }
            KeyCode::Char('/') => {
                self.search_mode = true;
                self.mode = AppMode::Search;
            }
            KeyCode::Tab => self.mode = AppMode::ServiceAccountList,
            KeyCode::BackTab => self.mode = AppMode::ClusterRoleList,
            _ => {}
        }
    }

    // 处理ServiceAccount列表导航
    fn handle_service_account_list_navigation(&mut self, key_event: crossterm::event::KeyEvent) {
        use crossterm::event::KeyCode;

        match key_event.code {
            KeyCode::Char('j') | KeyCode::Down => {
                if self.selected_service_account_index < self.service_accounts.len().saturating_sub(1) {
                    self.selected_service_account_index += 1;
                }
            }
            KeyCode::Char('k') | KeyCode::Up => {
                if self.selected_service_account_index > 0 {
                    self.selected_service_account_index -= 1;
                }
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                // 强制刷新当前模式
                self.force_refresh_current_mode();
            }
            KeyCode::Char(' ') => {
                if !self.service_accounts.is_empty() {
                    self.previous_mode = self.mode.clone();
                    self.mode = AppMode::Describe;
                    self.describe_content.clear();
                    self.describe_scroll = 0;
                }
            }
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                if !self.service_accounts.is_empty() {
                    self.previous_mode = self.mode.clone();
                    self.mode = AppMode::YamlView;
                    self.yaml_content.clear();
                    self.yaml_scroll = 0;
                }
            }
            KeyCode::Char('d') | KeyCode::Char('D') => {
                if !self.service_accounts.is_empty() {
                    self.confirm_action = Some(ConfirmAction::DeleteServiceAccount {
                        namespace: self.current_namespace.clone(),
                        name: self.service_accounts[self.selected_service_account_index].name.clone(),
                    });
                    self.mode = AppMode::Confirm;
                }
            }
            KeyCode::Char('/') => {
                self.search_mode = true;
                self.mode = AppMode::Search;
            }
            KeyCode::Tab => self.mode = AppMode::StatefulSetList,
            KeyCode::BackTab => self.mode = AppMode::ClusterRoleBindingList,
            _ => {}
        }
    }

    // 处理日志导航
    fn handle_logs_navigation(&mut self, key_event: crossterm::event::KeyEvent) {
        use crossterm::event::KeyCode;

        match key_event.code {
            KeyCode::Char('j') | KeyCode::Down => {
                if !self.logs.is_empty() && self.logs_scroll < self.logs.len().saturating_sub(1) {
                    self.logs_scroll += 1;
                }
            }
            KeyCode::Char('k') | KeyCode::Up => {
                if self.logs_scroll > 0 {
                    self.logs_scroll -= 1;
                }
            }
            KeyCode::PageDown => {
                let page_size = 10; // 默认页面大小
                if !self.logs.is_empty() {
                    self.logs_scroll = (self.logs_scroll + page_size).min(self.logs.len().saturating_sub(1));
                }
            }
            KeyCode::PageUp => {
                let page_size = 10; // 默认页面大小
                self.logs_scroll = self.logs_scroll.saturating_sub(page_size);
            }
            KeyCode::Char('a') | KeyCode::Char('A') => {
                // 切换自动滚动
                self.logs_auto_scroll = !self.logs_auto_scroll;
                // 如果开启自动滚动，滚动到最新位置
                if self.logs_auto_scroll && !self.logs.is_empty() {
                    self.logs_scroll = self.logs.len().saturating_sub(1);
                }
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                // 切换自动刷新
                self.logs_auto_refresh = !self.logs_auto_refresh;
                self.update_refresh_status();
            }
            KeyCode::Char('m') | KeyCode::Char('M') => {
                // 切换鼠标模式（文本选择模式和滚动模式）
                self.toggle_mouse_mode();
            }
            KeyCode::Esc => {
                self.reset_scroll();
                self.mode = self.get_previous_mode();
            }
            _ => {}
        }
    }

    // 处理Describe视图导航
    fn handle_describe_navigation(&mut self, key_event: crossterm::event::KeyEvent) {
        use crossterm::event::KeyCode;

        match key_event.code {
            KeyCode::Char('j') | KeyCode::Down => {
                // 向下滚动
                self.describe_scroll += 1;
            }
            KeyCode::Char('k') | KeyCode::Up => {
                // 向上滚动
                self.describe_scroll = self.describe_scroll.saturating_sub(1);
            }
            KeyCode::PageDown => {
                let page_size = 10; // 默认页面大小
                self.describe_scroll += page_size;
            }
            KeyCode::PageUp => {
                let page_size = 10; // 默认页面大小
                self.describe_scroll = self.describe_scroll.saturating_sub(page_size);
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                // 切换自动刷新
                self.describe_auto_refresh = !self.describe_auto_refresh;
                self.update_refresh_status();
            }
            KeyCode::Char('m') | KeyCode::Char('M') => {
                // 切换鼠标模式（文本选择模式和滚动模式）
                self.toggle_mouse_mode();
            }
            KeyCode::Esc => {
                self.reset_scroll();
                self.mode = self.get_previous_mode();
            }
            _ => {}
        }
    }

    // 处理YAML视图导航
    fn handle_yaml_navigation(&mut self, key_event: crossterm::event::KeyEvent) {
        use crossterm::event::KeyCode;

        match key_event.code {
            KeyCode::Char('j') | KeyCode::Down => {
                // 向下滚动
                self.yaml_scroll += 1;
            }
            KeyCode::Char('k') | KeyCode::Up => {
                // 向上滚动
                self.yaml_scroll = self.yaml_scroll.saturating_sub(1);
            }
            KeyCode::PageDown => {
                let page_size = 10; // 默认页面大小
                self.yaml_scroll += page_size;
            }
            KeyCode::PageUp => {
                let page_size = 10; // 默认页面大小
                self.yaml_scroll = self.yaml_scroll.saturating_sub(page_size);
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                // 切换自动刷新
                self.yaml_auto_refresh = !self.yaml_auto_refresh;
                self.update_refresh_status();
            }
            KeyCode::Char('m') | KeyCode::Char('M') => {
                // 切换鼠标模式（文本选择模式和滚动模式）
                self.toggle_mouse_mode();
            }
            KeyCode::Esc => {
                self.reset_scroll();
                self.mode = self.get_previous_mode();
            }
            _ => {}
        }
    }

    // 处理Top视图导航
    fn handle_top_navigation(&mut self, key_event: crossterm::event::KeyEvent) {
        use crossterm::event::KeyCode;

        match key_event.code {
            KeyCode::Char('j') | KeyCode::Down => {
                if !self.pod_metrics.is_empty() && self.metrics_scroll < self.pod_metrics.len().saturating_sub(1) {
                    self.metrics_scroll += 1;
                }
            }
            KeyCode::Char('k') | KeyCode::Up => {
                if self.metrics_scroll > 0 {
                    self.metrics_scroll -= 1;
                }
            }
            KeyCode::PageDown => {
                let page_size = 10; // 默认页面大小
                if !self.pod_metrics.is_empty() {
                    self.metrics_scroll = (self.metrics_scroll + page_size).min(self.pod_metrics.len().saturating_sub(1));
                }
            }
            KeyCode::PageUp => {
                let page_size = 10; // 默认页面大小
                self.metrics_scroll = self.metrics_scroll.saturating_sub(page_size);
            }
            KeyCode::Esc => {
                self.reset_scroll();
                self.mode = self.get_previous_mode();
            }
            _ => {}
        }
    }

    // 处理搜索导航
    fn handle_search_navigation(&mut self, key_event: crossterm::event::KeyEvent) {
        use crossterm::event::KeyCode;

        match key_event.code {
            KeyCode::Enter => {
                // 执行搜索
                self.search_mode = false;
                self.mode = self.get_previous_mode();
            }
            KeyCode::Esc => {
                // 取消搜索
                self.search_query.clear();
                self.search_mode = false;
                self.mode = self.get_previous_mode();
            }
            KeyCode::Backspace => {
                // 删除最后一个字符
                self.search_query.pop();
            }
            KeyCode::Char(c) => {
                // 添加字符到搜索查询
                self.search_query.push(c);
            }
            _ => {}
        }
    }

    // 处理确认导航
    fn handle_confirm_navigation(&mut self, key_event: crossterm::event::KeyEvent) {
        use crossterm::event::KeyCode;

        match key_event.code {
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                // 确认操作
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
    }

    // 处理帮助导航
    fn handle_help_navigation(&mut self, key_event: crossterm::event::KeyEvent) {
        use crossterm::event::KeyCode;

        match key_event.code {
            KeyCode::Esc => {
                self.mode = self.get_previous_mode();
            }
            _ => {}
        }
    }

    // 处理更多资源导航
    fn handle_more_resources_navigation(&mut self, key_event: crossterm::event::KeyEvent) {
        use crossterm::event::KeyCode;

        match key_event.code {
            KeyCode::Char('j') | KeyCode::Down => {
                if self.selected_more_resource_index < 8 {
                    self.selected_more_resource_index += 1;
                }
            }
            KeyCode::Char('k') | KeyCode::Up => {
                if self.selected_more_resource_index > 0 {
                    self.selected_more_resource_index -= 1;
                }
            }
            KeyCode::Char('1') => self.mode = AppMode::PVCList,
            KeyCode::Char('2') => self.mode = AppMode::PVList,
            KeyCode::Char('3') => self.mode = AppMode::NodeList,
            KeyCode::Char('4') => self.mode = AppMode::ConfigMapList,
            KeyCode::Char('5') => self.mode = AppMode::SecretList,
            KeyCode::Char('6') => self.mode = AppMode::JobList,
            KeyCode::Char('7') => self.mode = AppMode::DaemonSetList,
            KeyCode::Char('8') => self.mode = AppMode::StatefulSetList,
            KeyCode::Char('9') => self.mode = AppMode::IngressList,
            KeyCode::Enter => {
                match self.selected_more_resource_index {
                    0 => self.mode = AppMode::PVCList,
                    1 => self.mode = AppMode::PVList,
                    2 => self.mode = AppMode::NodeList,
                    3 => self.mode = AppMode::ConfigMapList,
                    4 => self.mode = AppMode::SecretList,
                    5 => self.mode = AppMode::JobList,
                    6 => self.mode = AppMode::DaemonSetList,
                    7 => self.mode = AppMode::StatefulSetList,
                    8 => self.mode = AppMode::IngressList,
                    _ => {}
                }
            }
            KeyCode::Esc => {
                self.reset_scroll();
                self.mode = self.get_previous_mode();
            }
            _ => {}
        }
    }

    // 处理鼠标事件
    pub fn handle_mouse_event(&mut self, mouse_event: crossterm::event::MouseEvent) -> anyhow::Result<()> {
        use crossterm::event::MouseEventKind;

        // 只在特定模式下处理鼠标滚轮事件
        match self.mode {
            AppMode::Logs | AppMode::Describe | AppMode::YamlView | AppMode::TopView => {
                match mouse_event.kind {
                    MouseEventKind::ScrollUp => {
                        // 向上滚动
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
                    MouseEventKind::ScrollDown => {
                        // 向下滚动
                        match self.mode {
                            AppMode::Logs => {
                                if !self.logs.is_empty() && self.logs_scroll < self.logs.len().saturating_sub(1) {
                                    self.logs_scroll += 1;
                                }
                            }
                            AppMode::Describe => {
                                self.describe_scroll += 1;
                            }
                            AppMode::YamlView => {
                                self.yaml_scroll += 1;
                            }
                            AppMode::TopView => {
                                if !self.pod_metrics.is_empty() && self.metrics_scroll < self.pod_metrics.len().saturating_sub(1) {
                                    self.metrics_scroll += 1;
                                }
                            }
                            _ => {}
                        }
                    }
                    _ => {
                        // 忽略其他鼠标事件（点击、移动等）
                    }
                }
            }
            _ => {
                // 在其他模式下忽略鼠标事件
            }
        }

        Ok(())
    }
}