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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub name: String,
    pub status: String,
    pub roles: Vec<String>,
    pub age: String,
    pub version: String,
    pub internal_ip: Option<String>,
    pub external_ip: Option<String>,
    pub os_image: Option<String>,
    pub kernel_version: Option<String>,
    pub container_runtime: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigMap {
    pub name: String,
    pub namespace: String,
    pub data_count: usize,
    pub age: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Secret {
    pub name: String,
    pub namespace: String,
    pub type_: String,
    pub data_count: usize,
    pub age: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deployment {
    pub name: String,
    pub namespace: String,
    pub ready: String,
    pub up_to_date: u32,
    pub available: u32,
    pub age: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    pub name: String,
    pub namespace: String,
    pub completions: Option<u32>,
    pub successful: u32,
    pub age: String,
    pub duration: Option<String>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonSet {
    pub name: String,
    pub namespace: String,
    pub desired: u32,
    pub current: u32,
    pub ready: u32,
    pub up_to_date: u32,
    pub available: u32,
    pub age: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistentVolume {
    pub name: String,
    pub capacity: String,
    pub access_modes: Vec<String>,
    pub reclaim_policy: String,
    pub status: String,
    pub claim: Option<String>,
    pub storage_class: Option<String>,
    pub age: String,
}

pub type PV = PersistentVolume;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistentVolumeClaim {
    pub name: String,
    pub namespace: String,
    pub status: String,
    pub volume: Option<String>,
    pub capacity: Option<String>,
    pub access_modes: Vec<String>,
    pub storage_class: Option<String>,
    pub age: String,
}

pub type PVC = PersistentVolumeClaim;

// 资源使用情况数据结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceMetrics {
    pub name: String,
    pub namespace: String,
    pub cpu: String,
    pub memory: String,
    pub cpu_percentage: Option<f64>,
    pub memory_percentage: Option<f64>,
    pub containers: Vec<ContainerMetrics>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerMetrics {
    pub name: String,
    pub cpu: String,
    pub memory: String,
    pub cpu_percentage: Option<f64>,
    pub memory_percentage: Option<f64>,
}