use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, PartialEq)]
pub enum AppMode {
    NamespaceList,
    PodList,
    ServiceList,
    Logs,
    Describe,
    Help,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub mode: AppMode,
    pub should_quit: bool,
    pub current_namespace: String,
    pub selected_namespace_index: usize,
    pub selected_pod_index: usize,
    pub namespaces: Vec<String>,
    pub pods: Vec<crate::kubectl::types::Pod>,
    pub services: Vec<crate::kubectl::types::Service>,
    pub logs: Vec<String>,
    pub describe_content: String,
    pub last_update: Instant,
    pub auto_refresh: bool,
    pub refresh_interval: Duration,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            mode: AppMode::NamespaceList,
            should_quit: false,
            current_namespace: "default".to_string(),
            selected_namespace_index: 0,
            selected_pod_index: 0,
            namespaces: vec!["default".to_string()],
            pods: Vec::new(),
            services: Vec::new(),
            logs: Vec::new(),
            describe_content: String::new(),
            last_update: Instant::now(),
            auto_refresh: true,
            refresh_interval: Duration::from_secs(5),
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

    pub fn refresh_data(&mut self) {
        self.last_update = Instant::now();
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        match key_event.code {
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Char('?') | KeyCode::F(1) => self.mode = AppMode::Help,
            KeyCode::Esc => {
                match self.mode {
                    AppMode::Help | AppMode::Logs | AppMode::Describe => self.mode = AppMode::PodList,
                    AppMode::PodList | AppMode::ServiceList => self.mode = AppMode::NamespaceList,
                    AppMode::NamespaceList => {}
                }
            }
            KeyCode::Tab => {
                match self.mode {
                    AppMode::NamespaceList => self.mode = AppMode::PodList,
                    AppMode::PodList => self.mode = AppMode::ServiceList,
                    AppMode::ServiceList => self.mode = AppMode::NamespaceList,
                    _ => {}
                }
            }
            KeyCode::Up => self.move_selection_up(),
            KeyCode::Down => self.move_selection_down(),
            KeyCode::Enter => self.handle_enter(),
            KeyCode::Char('l') if matches!(self.mode, AppMode::PodList) => {
                self.mode = AppMode::Logs;
            }
            KeyCode::Char('d') if matches!(self.mode, AppMode::PodList) => {
                self.mode = AppMode::Describe;
            }
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
                }
            }
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
}