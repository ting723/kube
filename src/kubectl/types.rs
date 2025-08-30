use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pod {
    pub name: String,
    pub namespace: String,
    pub status: PodStatus,
    pub ready: String,
    pub restarts: u32,
    pub age: String,
    pub node: Option<String>,
    pub ip: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PodStatus {
    pub phase: String,
    pub conditions: Option<Vec<PodCondition>>,
    pub container_statuses: Option<Vec<ContainerStatus>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PodCondition {
    pub type_: String,
    pub status: String,
    pub last_transition_time: Option<DateTime<Utc>>,
    pub reason: Option<String>,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerStatus {
    pub name: String,
    pub ready: bool,
    pub restart_count: u32,
    pub state: ContainerState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerState {
    pub running: Option<ContainerStateRunning>,
    pub waiting: Option<ContainerStateWaiting>,
    pub terminated: Option<ContainerStateTerminated>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerStateRunning {
    pub started_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerStateWaiting {
    pub reason: Option<String>,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerStateTerminated {
    pub exit_code: i32,
    pub reason: Option<String>,
    pub message: Option<String>,
    pub started_at: Option<DateTime<Utc>>,
    pub finished_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
    pub name: String,
    pub namespace: String,
    pub type_: String,
    pub cluster_ip: String,
    pub external_ip: Option<String>,
    pub ports: Vec<ServicePort>,
    pub age: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServicePort {
    pub name: Option<String>,
    pub port: u16,
    pub target_port: Option<String>,
    pub protocol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Namespace {
    pub name: String,
    pub status: String,
    pub age: String,
}