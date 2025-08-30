use anyhow::{Result, anyhow};
use serde_json::Value;
use std::time::Duration;

use super::types::*;
use super::commands;

pub struct KubectlClient {
    // timeout字段保留以备将来使用
    #[allow(dead_code)]
    timeout: Duration,
}

impl Default for KubectlClient {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(30),
        }
    }
}

impl KubectlClient {
    pub fn new() -> Self {
        Self::default()
    }

    #[allow(dead_code)]
    pub fn with_timeout(timeout: Duration) -> Self {
        Self { timeout }
    }

    pub fn check_available(&self) -> bool {
        commands::check_kubectl_available()
    }

    pub async fn get_namespaces(&self) -> Result<Vec<Namespace>> {
        let namespaces = commands::get_namespaces()?;
        
        Ok(namespaces
            .into_iter()
            .map(|name| Namespace {
                name,
                status: "Active".to_string(), // kubectl get namespaces doesn't provide detailed status in simple format
                age: "Unknown".to_string(),
            })
            .collect())
    }

    pub async fn get_pods(&self, namespace: &str) -> Result<Vec<Pod>> {
        let json_output = commands::get_pods(namespace)?;
        let parsed: Value = serde_json::from_str(&json_output)?;

        let items = parsed["items"]
            .as_array()
            .ok_or_else(|| anyhow!("Invalid JSON response: missing items array"))?;

        let mut pods = Vec::new();
        
        for item in items {
            if let Ok(pod) = self.parse_pod(item) {
                pods.push(pod);
            }
        }

        Ok(pods)
    }

    pub async fn get_services(&self, namespace: &str) -> Result<Vec<Service>> {
        let json_output = commands::get_services(namespace)?;
        let parsed: Value = serde_json::from_str(&json_output)?;

        let items = parsed["items"]
            .as_array()
            .ok_or_else(|| anyhow!("Invalid JSON response: missing items array"))?;

        let mut services = Vec::new();
        
        for item in items {
            if let Ok(service) = self.parse_service(item) {
                services.push(service);
            }
        }

        Ok(services)
    }

    pub async fn get_pod_logs(&self, namespace: &str, pod_name: &str, lines: u32) -> Result<Vec<String>> {
        let logs = commands::get_pod_logs(namespace, pod_name, lines)?;
        Ok(logs.lines().map(|line| line.to_string()).collect())
    }

    pub async fn describe_pod(&self, namespace: &str, pod_name: &str) -> Result<String> {
        commands::describe_pod(namespace, pod_name)
    }

    #[allow(dead_code)]
    pub async fn delete_pod(&self, namespace: &str, pod_name: &str) -> Result<String> {
        commands::delete_pod(namespace, pod_name)
    }

    fn parse_pod(&self, item: &Value) -> Result<Pod> {
        let metadata = &item["metadata"];
        let spec = &item["spec"];
        let status = &item["status"];

        let name = metadata["name"]
            .as_str()
            .ok_or_else(|| anyhow!("Missing pod name"))?
            .to_string();

        let namespace = metadata["namespace"]
            .as_str()
            .ok_or_else(|| anyhow!("Missing pod namespace"))?
            .to_string();

        let phase = status["phase"]
            .as_str()
            .unwrap_or("Unknown")
            .to_string();

        // Calculate ready containers
        let container_statuses = status["containerStatuses"].as_array();
        let ready_count = if let Some(statuses) = container_statuses {
            statuses.iter()
                .filter(|status| status["ready"].as_bool().unwrap_or(false))
                .count()
        } else {
            0
        };

        let total_count = spec["containers"]
            .as_array()
            .map(|containers| containers.len())
            .unwrap_or(0);

        let ready = format!("{}/{}", ready_count, total_count);

        // Calculate restart count
        let restarts = if let Some(statuses) = container_statuses {
            statuses.iter()
                .map(|status| status["restartCount"].as_u64().unwrap_or(0) as u32)
                .sum()
        } else {
            0
        };

        // Calculate age (simplified)
        let age = self.calculate_age(metadata["creationTimestamp"].as_str());

        let node = spec["nodeName"].as_str().map(|s| s.to_string());
        let ip = status["podIP"].as_str().map(|s| s.to_string());

        Ok(Pod {
            name,
            namespace,
            status: PodStatus {
                phase,
                conditions: None, // Simplified for now
                container_statuses: None, // Simplified for now
            },
            ready,
            restarts,
            age,
            node,
            ip,
        })
    }

    fn parse_service(&self, item: &Value) -> Result<Service> {
        let metadata = &item["metadata"];
        let spec = &item["spec"];

        let name = metadata["name"]
            .as_str()
            .ok_or_else(|| anyhow!("Missing service name"))?
            .to_string();

        let namespace = metadata["namespace"]
            .as_str()
            .ok_or_else(|| anyhow!("Missing service namespace"))?
            .to_string();

        let type_ = spec["type"]
            .as_str()
            .unwrap_or("ClusterIP")
            .to_string();

        let cluster_ip = spec["clusterIP"]
            .as_str()
            .unwrap_or("None")
            .to_string();

        let external_ip = spec["externalIPs"]
            .as_array()
            .and_then(|ips| ips.first())
            .and_then(|ip| ip.as_str())
            .map(|s| s.to_string());

        // Parse ports
        let ports = if let Some(ports_array) = spec["ports"].as_array() {
            ports_array.iter()
                .filter_map(|port| {
                    Some(ServicePort {
                        name: port["name"].as_str().map(|s| s.to_string()),
                        port: port["port"].as_u64()? as u16,
                        target_port: port["targetPort"].as_str().map(|s| s.to_string()),
                        protocol: port["protocol"].as_str().unwrap_or("TCP").to_string(),
                    })
                })
                .collect()
        } else {
            Vec::new()
        };

        let age = self.calculate_age(metadata["creationTimestamp"].as_str());

        Ok(Service {
            name,
            namespace,
            type_,
            cluster_ip,
            external_ip,
            ports,
            age,
        })
    }

    fn calculate_age(&self, creation_timestamp: Option<&str>) -> String {
        if let Some(timestamp) = creation_timestamp {
            if let Ok(created) = chrono::DateTime::parse_from_rfc3339(timestamp) {
                let now = chrono::Utc::now();
                let duration = now.signed_duration_since(created.with_timezone(&chrono::Utc));
                
                let days = duration.num_days();
                let hours = duration.num_hours() % 24;
                let minutes = duration.num_minutes() % 60;

                if days > 0 {
                    format!("{}d", days)
                } else if hours > 0 {
                    format!("{}h", hours)
                } else {
                    format!("{}m", minutes)
                }
            } else {
                "Unknown".to_string()
            }
        } else {
            "Unknown".to_string()
        }
    }
}