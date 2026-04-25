use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};
use super::state::{AppState, ConfirmAction, AppMode};

impl AppState {
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
            KeyCode::Esc => match self.mode {
                AppMode::Help
                | AppMode::Logs
                | AppMode::Describe
                | AppMode::YamlView
                | AppMode::TopView => {
                    self.reset_scroll();
                    self.mode = self.get_previous_mode();
                }
                AppMode::PodList
                | AppMode::ServiceList
                | AppMode::NodeList
                | AppMode::DeploymentList
                | AppMode::JobList
                | AppMode::DaemonSetList
                | AppMode::PVCList
                | AppMode::PVList
                | AppMode::ConfigMapList
                | AppMode::SecretList => {
                    self.mode = AppMode::NamespaceList;
                }
                _ => {}
            },
            // 滚动操作（仅在 Logs、Describe、YamlView 和 TopView 模式下）
            KeyCode::Char('j') => {
                match self.mode {
                    AppMode::Logs | AppMode::Describe | AppMode::YamlView | AppMode::TopView => {
                        self.scroll_down();
                    }
                    _ => self.move_selection_down(), // 在列表模式下正常导航
                }
            }
            KeyCode::Char('k') => {
                match self.mode {
                    AppMode::Logs | AppMode::Describe | AppMode::YamlView | AppMode::TopView => {
                        self.scroll_up();
                    }
                    _ => self.move_selection_up(), // 在列表模式下正常导航
                }
            }
            KeyCode::Down => self.move_selection_down(),
            KeyCode::Up => self.move_selection_up(),
            KeyCode::Char('h') | KeyCode::Left => self.handle_left_navigation(),
            KeyCode::Char('l') | KeyCode::Right => self.handle_right_navigation(),
            KeyCode::PageDown => self.scroll_page_down(),
            KeyCode::PageUp => self.scroll_page_up(),
            // 资源操作
            KeyCode::Enter => self.handle_enter(),
            KeyCode::Char(' ') => self.handle_describe(), // Space 键查看详情
            KeyCode::Char('L') => self.handle_logs(),     // L 查看日志
            KeyCode::Char('D') => self.handle_delete(),   // D 删除（需确认）
            KeyCode::Char('E') => self.handle_exec(),     // E 进入容器
            KeyCode::Char('Y') => self.handle_yaml_view(), // Y 查看YAML配置
            KeyCode::Char('T') => self.handle_top_view(), // T 查看资源使用
            // 搜索
            KeyCode::Char('/') => self.start_search(),
            KeyCode::Char('n') => self.search_next(),
            KeyCode::Char('N') => self.search_previous(),
            // 自动刷新相关快捷键
            KeyCode::Char('A') => {
                match self.mode {
                    AppMode::Logs => {
                        self.logs_auto_scroll = !self.logs_auto_scroll;
                    }
                    _ => {
                        // 在其他模式下，A键切换全局刷新
                        self.toggle_global_refresh();
                    }
                }
            }
            // 日志自动刷新切换（仅在日志模式下）
            KeyCode::Char('R') => {
                match self.mode {
                    AppMode::Logs => {
                        self.logs_auto_refresh = !self.logs_auto_refresh;
                        self.update_refresh_status();
                    }
                    AppMode::Describe => {
                        // 在Describe模式下，R键切换describe自动刷新
                        self.toggle_describe_refresh();
                    }
                    AppMode::YamlView => {
                        // 在YAML模式下，R键切换YAML自动刷新
                        self.toggle_yaml_refresh();
                    }
                    _ => {
                        // 在其他模式下，R键手动刷新当前数据
                        self.force_refresh_current_mode();
                    }
                }
            }
            // Tab 切换面板
            KeyCode::Tab => self.switch_panel(),
            KeyCode::BackTab => self.switch_panel_left(), // Shift+Tab 向后切换
            // M键在YAML/Describe模式下切换鼠标模式
            KeyCode::Char('M') | KeyCode::Char('m') => match self.mode {
                AppMode::Describe | AppMode::YamlView | AppMode::Logs => {
                    self.toggle_mouse_mode();
                }
                _ => {}
            },
            // I键切换语言（International）
            KeyCode::Char('I') | KeyCode::Char('i') => {
                self.toggle_language();
            }
            _ => {}
        }
        Ok(())
    }

    // 搜索相关方法
    pub fn start_search(&mut self) {
        // 只在列表模式下才能搜索
        match self.mode {
            AppMode::NamespaceList
            | AppMode::PodList
            | AppMode::ServiceList
            | AppMode::NodeList
            | AppMode::DeploymentList
            | AppMode::DaemonSetList
            | AppMode::PVCList
            | AppMode::PVList
            | AppMode::ConfigMapList
            | AppMode::SecretList => {
                self.previous_mode = self.mode.clone();
                self.search_mode = true;
                self.search_query.clear();
                self.mode = AppMode::Search;
            }
            _ => {}
        }
    }

    pub fn search_next(&mut self) {
        if !self.search_results.is_empty() {
            self.current_search_index = (self.current_search_index + 1) % self.search_results.len();
            self.jump_to_search_result();
        }
    }

    pub fn search_previous(&mut self) {
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
                AppMode::NamespaceList => self.selected_namespace_index = index,
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
            // 移除j/k的特殊处理，让它们可以正常输入到搜索框
            // 用户可以使用方向键或Tab来导航搜索结果
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
            AppMode::NamespaceList => {
                for (index, namespace) in self.namespaces.iter().enumerate() {
                    if namespace.to_lowercase().contains(&query) {
                        self.search_results.push(index);
                    }
                }
            }
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

    // 其他需要的辅助方法
    pub fn move_selection_down(&mut self) {
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

    pub fn move_selection_up(&mut self) {
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

    pub fn switch_panel(&mut self) {
        self.switch_panel_right();
    }

    pub fn switch_panel_right(&mut self) {
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

    pub fn switch_panel_left(&mut self) {
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

    pub fn handle_left_navigation(&mut self) {
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

    pub fn handle_right_navigation(&mut self) {
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

    pub fn handle_enter(&mut self) {
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
            // 在资源列表模式下，Enter键也可以进入Describe模式
            AppMode::PodList
            | AppMode::ServiceList
            | AppMode::NodeList
            | AppMode::DeploymentList
            | AppMode::JobList
            | AppMode::DaemonSetList
            | AppMode::PVCList
            | AppMode::PVList
            | AppMode::ConfigMapList
            | AppMode::SecretList => {
                self.handle_describe();
            }
            _ => {}
        }
    }

    pub fn handle_describe(&mut self) {
        match self.mode {
            AppMode::PodList
            | AppMode::ServiceList
            | AppMode::NodeList
            | AppMode::DeploymentList
            | AppMode::JobList
            | AppMode::DaemonSetList
            | AppMode::PVCList
            | AppMode::PVList
            | AppMode::ConfigMapList
            | AppMode::SecretList => {
                self.previous_mode = self.mode.clone();
                self.reset_scroll();
                // 清理之前的describe内容
                self.describe_content.clear();
                self.mode = AppMode::Describe;
                // 默认为鼠标滚动模式，方便快速浏览内容
                self.text_selection_mode = false;
            }
            _ => {}
        }
    }

    pub fn handle_logs(&mut self) {
        match self.mode {
            AppMode::PodList => {
                self.previous_mode = self.mode.clone();
                self.reset_scroll();
                self.mode = AppMode::Logs;
            }
            _ => {}
        }
    }

    pub fn handle_delete(&mut self) {
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

    pub fn handle_exec(&mut self) {
        match self.mode {
            AppMode::PodList => {
                if let Some(pod) = self.get_selected_pod() {
                    let cmd = format!(
                        "kubectl exec -it -n {} {} -- /bin/sh",
                        self.current_namespace, pod.name
                    );
                    self.set_current_command(&cmd);
                    self.pending_exec = Some(cmd);
                }
            }
            _ => {}
        }
    }

    pub fn handle_yaml_view(&mut self) {
        match self.mode {
            AppMode::PodList
            | AppMode::ServiceList
            | AppMode::DeploymentList
            | AppMode::JobList
            | AppMode::DaemonSetList
            | AppMode::NodeList
            | AppMode::ConfigMapList
            | AppMode::SecretList
            | AppMode::PVCList
            | AppMode::PVList => {
                self.previous_mode = self.mode.clone();
                self.mode = AppMode::YamlView;
                self.yaml_scroll = 0;
                // 默认为鼠标滚动模式，方便快速浏览YAML内容
                self.text_selection_mode = false;
                // 在主循环中会加载相应的YAML内容
            }
            _ => {}
        }
    }

    pub fn handle_top_view(&mut self) {
        match self.mode {
            AppMode::PodList => {
                self.previous_mode = self.mode.clone();
                self.mode = AppMode::TopView;
                self.metrics_scroll = 0;
                // 在主循环中会加载Pod的资源使用情况
            }
            _ => {}
        }
    }

    }

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    fn create_test_state() -> AppState {
        AppState::default()
    }

    #[test]
    fn test_quit_key() {
        let mut state = create_test_state();
        let key = KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE);
        state.handle_key_event(key).unwrap();
        assert!(state.should_quit);
    }

    #[test]
    fn test_help_key() {
        let mut state = create_test_state();
        let key = KeyEvent::new(KeyCode::Char('?'), KeyModifiers::NONE);
        state.handle_key_event(key).unwrap();
        assert_eq!(state.mode, AppMode::Help);
    }

    #[test]
    fn test_navigation_down_in_namespace_list() {
        let mut state = create_test_state();
        state.namespaces = vec!["default".to_string(), "kube-system".to_string()];
        state.mode = AppMode::NamespaceList;
        let key = KeyEvent::new(KeyCode::Down, KeyModifiers::NONE);
        state.handle_key_event(key).unwrap();
        assert_eq!(state.selected_namespace_index, 1);
    }

    #[test]
    fn test_navigation_up_in_pod_list() {
        let mut state = create_test_state();
        use crate::kubectl::types::{Pod, PodStatus};
        state.pods = vec![
            Pod {
                name: "pod1".to_string(),
                namespace: "default".to_string(),
                status: PodStatus { phase: "Running".to_string(), conditions: None, container_statuses: None },
                ready: "1/1".to_string(),
                restarts: 0,
                age: "1d".to_string(),
                node: None,
                ip: None,
            },
            Pod {
                name: "pod2".to_string(),
                namespace: "default".to_string(),
                status: PodStatus { phase: "Running".to_string(), conditions: None, container_statuses: None },
                ready: "1/1".to_string(),
                restarts: 0,
                age: "1d".to_string(),
                node: None,
                ip: None,
            },
        ];
        state.mode = AppMode::PodList;
        state.selected_pod_index = 1;
        let key = KeyEvent::new(KeyCode::Up, KeyModifiers::NONE);
        state.handle_key_event(key).unwrap();
        assert_eq!(state.selected_pod_index, 0);
    }

    #[test]
    fn test_panel_switch_right() {
        let mut state = create_test_state();
        state.mode = AppMode::NamespaceList;
        state.switch_panel_right();
        assert_eq!(state.mode, AppMode::PodList);
    }

    #[test]
    fn test_panel_switch_left() {
        let mut state = create_test_state();
        state.mode = AppMode::PodList;
        state.switch_panel_left();
        assert_eq!(state.mode, AppMode::NamespaceList);
    }

    #[test]
    fn test_start_search() {
        let mut state = create_test_state();
        state.mode = AppMode::PodList;
        state.start_search();
        assert!(state.search_mode);
        assert_eq!(state.mode, AppMode::Search);
        assert_eq!(state.previous_mode, AppMode::PodList);
    }

    #[test]
    fn test_language_toggle() {
        let mut state = create_test_state();
        let original = state.language_chinese;
        state.toggle_language();
        assert_eq!(state.language_chinese, !original);
    }

    #[test]
    fn test_mouse_mode_toggle() {
        let mut state = create_test_state();
        state.mode = AppMode::Describe;
        let original = state.text_selection_mode;
        state.toggle_mouse_mode();
        assert_eq!(state.text_selection_mode, !original);
    }
}
