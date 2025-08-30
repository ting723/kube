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

    #[allow(dead_code)]
    pub async fn get_nodes(&self) -> Result<Vec<Node>> {
        let json_output = commands::get_nodes()?;
        let parsed: Value = serde_json::from_str(&json_output)?;

        let items = parsed["items"]
            .as_array()
            .ok_or_else(|| anyhow!("Invalid JSON response: missing items array"))?;

        let mut nodes = Vec::new();
        
        for item in items {
            if let Ok(node) = self.parse_node(item) {
                nodes.push(node);
            }
        }

        Ok(nodes)
    }

    #[allow(dead_code)]
    pub async fn get_configmaps(&self, namespace: &str) -> Result<Vec<ConfigMap>> {
        let json_output = commands::get_configmaps(namespace)?;
        let parsed: Value = serde_json::from_str(&json_output)?;

        let items = parsed["items"]
            .as_array()
            .ok_or_else(|| anyhow!("Invalid JSON response: missing items array"))?;

        let mut configmaps = Vec::new();
        
        for item in items {
            if let Ok(configmap) = self.parse_configmap(item) {
                configmaps.push(configmap);
            }
        }

        Ok(configmaps)
    }

    #[allow(dead_code)]
    pub async fn get_secrets(&self, namespace: &str) -> Result<Vec<Secret>> {
        let json_output = commands::get_secrets(namespace)?;
        let parsed: Value = serde_json::from_str(&json_output)?;

        let items = parsed["items"]
            .as_array()
            .ok_or_else(|| anyhow!("Invalid JSON response: missing items array"))?;

        let mut secrets = Vec::new();
        
        for item in items {
            if let Ok(secret) = self.parse_secret(item) {
                secrets.push(secret);
            }
        }

        Ok(secrets)
    }

    #[allow(dead_code)]
    pub async fn get_deployments(&self, namespace: &str) -> Result<Vec<Deployment>> {
        let json_output = commands::get_deployments(namespace)?;
        let parsed: Value = serde_json::from_str(&json_output)?;

        let items = parsed["items"]
            .as_array()
            .ok_or_else(|| anyhow!("Invalid JSON response: missing items array"))?;

        let mut deployments = Vec::new();
        
        for item in items {
            if let Ok(deployment) = self.parse_deployment(item) {
                deployments.push(deployment);
            }
        }

        Ok(deployments)
    }

    #[allow(dead_code)]
    pub async fn get_daemonsets(&self, namespace: &str) -> Result<Vec<DaemonSet>> {
        let json_output = commands::get_daemonsets(namespace)?;
        let parsed: Value = serde_json::from_str(&json_output)?;

        let items = parsed["items"]
            .as_array()
            .ok_or_else(|| anyhow!("Invalid JSON response: missing items array"))?;

        let mut daemonsets = Vec::new();
        
        for item in items {
            if let Ok(daemonset) = self.parse_daemonset(item) {
                daemonsets.push(daemonset);
            }
        }

        Ok(daemonsets)
    }

    #[allow(dead_code)]
    pub async fn get_pvcs(&self, namespace: &str) -> Result<Vec<PVC>> {
        let json_output = commands::get_pvcs(namespace)?;
        let parsed: Value = serde_json::from_str(&json_output)?;

        let items = parsed["items"]
            .as_array()
            .ok_or_else(|| anyhow!("Invalid JSON response: missing items array"))?;

        let mut pvcs = Vec::new();
        
        for item in items {
            if let Ok(pvc) = self.parse_pvc(item) {
                pvcs.push(pvc);
            }
        }

        Ok(pvcs)
    }

    #[allow(dead_code)]
    pub async fn get_pvs(&self) -> Result<Vec<PV>> {
        let json_output = commands::get_pvs()?;
        let parsed: Value = serde_json::from_str(&json_output)?;

        let items = parsed["items"]
            .as_array()
            .ok_or_else(|| anyhow!("Invalid JSON response: missing items array"))?;

        let mut pvs = Vec::new();
        
        for item in items {
            if let Ok(pv) = self.parse_pv(item) {
                pvs.push(pv);
            }
        }

        Ok(pvs)
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

    #[allow(dead_code)]
    fn parse_node(&self, item: &Value) -> Result<Node> {
        let metadata = &item["metadata"];
        let _spec = &item["spec"]; // 用下划线前缀忽略警告
        let status = &item["status"];

        let name = metadata["name"]
            .as_str()
            .ok_or_else(|| anyhow!("Missing node name"))?
            .to_string();

        // 获取节点状态
        let node_status = if let Some(conditions) = status["conditions"].as_array() {
            let ready_condition = conditions.iter().find(|condition| {
                condition["type"].as_str() == Some("Ready")
            });
            
            if let Some(condition) = ready_condition {
                if condition["status"].as_str() == Some("True") {
                    "Ready".to_string()
                } else {
                    "NotReady".to_string()
                }
            } else {
                "Unknown".to_string()
            }
        } else {
            "Unknown".to_string()
        };

        // 获取节点角色
        let roles = if let Some(labels) = metadata["labels"].as_object() {
            let mut node_roles = Vec::new();
            for (key, _) in labels {
                if key.starts_with("node-role.kubernetes.io/") {
                    let role = key.strip_prefix("node-role.kubernetes.io/")
                        .unwrap_or("unknown")
                        .to_string();
                    node_roles.push(role);
                }
            }
            if node_roles.is_empty() {
                vec!["worker".to_string()]
            } else {
                node_roles
            }
        } else {
            vec!["unknown".to_string()]
        };

        let age = self.calculate_age(metadata["creationTimestamp"].as_str());
        let version = status["nodeInfo"]["kubeletVersion"]
            .as_str()
            .unwrap_or("unknown")
            .to_string();

        // 获取IP地址
        let mut internal_ip = None;
        let mut external_ip = None;
        if let Some(addresses) = status["addresses"].as_array() {
            for addr in addresses {
                match addr["type"].as_str() {
                    Some("InternalIP") => internal_ip = addr["address"].as_str().map(|s| s.to_string()),
                    Some("ExternalIP") => external_ip = addr["address"].as_str().map(|s| s.to_string()),
                    _ => {}
                }
            }
        }

        let os_image = status["nodeInfo"]["osImage"]
            .as_str()
            .map(|s| s.to_string());
        let kernel_version = status["nodeInfo"]["kernelVersion"]
            .as_str()
            .map(|s| s.to_string());
        let container_runtime = status["nodeInfo"]["containerRuntimeVersion"]
            .as_str()
            .map(|s| s.to_string());

        Ok(Node {
            name,
            status: node_status,
            roles,
            age,
            version,
            internal_ip,
            external_ip,
            os_image,
            kernel_version,
            container_runtime,
        })
    }

    #[allow(dead_code)]
    fn parse_configmap(&self, item: &Value) -> Result<ConfigMap> {
        let metadata = &item["metadata"];
        let data = &item["data"];

        let name = metadata["name"]
            .as_str()
            .ok_or_else(|| anyhow!("Missing configmap name"))?
            .to_string();

        let namespace = metadata["namespace"]
            .as_str()
            .ok_or_else(|| anyhow!("Missing configmap namespace"))?
            .to_string();

        let data_count = if let Some(data_obj) = data.as_object() {
            data_obj.len()
        } else {
            0
        };

        let age = self.calculate_age(metadata["creationTimestamp"].as_str());

        Ok(ConfigMap {
            name,
            namespace,
            data_count,
            age,
        })
    }

    #[allow(dead_code)]
    fn parse_secret(&self, item: &Value) -> Result<Secret> {
        let metadata = &item["metadata"];
        let data = &item["data"];

        let name = metadata["name"]
            .as_str()
            .ok_or_else(|| anyhow!("Missing secret name"))?
            .to_string();

        let namespace = metadata["namespace"]
            .as_str()
            .ok_or_else(|| anyhow!("Missing secret namespace"))?
            .to_string();

        let type_ = item["type"]
            .as_str()
            .unwrap_or("Opaque")
            .to_string();

        let data_count = if let Some(data_obj) = data.as_object() {
            data_obj.len()
        } else {
            0
        };

        let age = self.calculate_age(metadata["creationTimestamp"].as_str());

        Ok(Secret {
            name,
            namespace,
            type_,
            data_count,
            age,
        })
    }

    #[allow(dead_code)]
    fn parse_deployment(&self, item: &Value) -> Result<Deployment> {
        let metadata = &item["metadata"];
        let spec = &item["spec"];
        let status = &item["status"];

        let name = metadata["name"]
            .as_str()
            .ok_or_else(|| anyhow!("Missing deployment name"))?
            .to_string();

        let namespace = metadata["namespace"]
            .as_str()
            .ok_or_else(|| anyhow!("Missing deployment namespace"))?
            .to_string();

        let replicas = spec["replicas"].as_u64().unwrap_or(0) as u32;
        let ready_replicas = status["readyReplicas"].as_u64().unwrap_or(0) as u32;
        let ready = format!("{}/{}", ready_replicas, replicas);
        
        let up_to_date = status["updatedReplicas"].as_u64().unwrap_or(0) as u32;
        let available = status["availableReplicas"].as_u64().unwrap_or(0) as u32;

        let age = self.calculate_age(metadata["creationTimestamp"].as_str());

        Ok(Deployment {
            name,
            namespace,
            ready,
            up_to_date,
            available,
            age,
        })
    }

    #[allow(dead_code)]
    fn parse_daemonset(&self, item: &Value) -> Result<DaemonSet> {
        let metadata = &item["metadata"];
        let _spec = &item["spec"];
        let status = &item["status"];

        let name = metadata["name"]
            .as_str()
            .ok_or_else(|| anyhow!("Missing daemonset name"))?
            .to_string();

        let namespace = metadata["namespace"]
            .as_str()
            .ok_or_else(|| anyhow!("Missing daemonset namespace"))?
            .to_string();

        let desired = status["desiredNumberScheduled"].as_u64().unwrap_or(0) as u32;
        let current = status["currentNumberScheduled"].as_u64().unwrap_or(0) as u32;
        let ready = status["numberReady"].as_u64().unwrap_or(0) as u32;
        let up_to_date = status["updatedNumberScheduled"].as_u64().unwrap_or(0) as u32;
        let available = status["numberAvailable"].as_u64().unwrap_or(0) as u32;

        let age = self.calculate_age(metadata["creationTimestamp"].as_str());

        Ok(DaemonSet {
            name,
            namespace,
            desired,
            current,
            ready,
            up_to_date,
            available,
            age,
        })
    }

    #[allow(dead_code)]
    fn parse_pvc(&self, item: &Value) -> Result<PVC> {
        let metadata = &item["metadata"];
        let spec = &item["spec"];
        let status = &item["status"];

        let name = metadata["name"]
            .as_str()
            .ok_or_else(|| anyhow!("Missing PVC name"))?
            .to_string();

        let namespace = metadata["namespace"]
            .as_str()
            .ok_or_else(|| anyhow!("Missing PVC namespace"))?
            .to_string();

        let pvc_status = status["phase"]
            .as_str()
            .unwrap_or("Unknown")
            .to_string();

        let volume = status["volumeName"]
            .as_str()
            .map(|s| s.to_string());

        let capacity = status["capacity"]["storage"]
            .as_str()
            .map(|s| s.to_string());

        let access_modes = spec["accessModes"]
            .as_array()
            .map(|modes| {
                modes.iter()
                    .filter_map(|mode| mode.as_str())
                    .map(|s| s.to_string())
                    .collect()
            })
            .unwrap_or_default();

        let storage_class = spec["storageClassName"]
            .as_str()
            .map(|s| s.to_string());

        let age = self.calculate_age(metadata["creationTimestamp"].as_str());

        Ok(PVC {
            name,
            namespace,
            status: pvc_status,
            volume,
            capacity,
            access_modes,
            storage_class,
            age,
        })
    }

    #[allow(dead_code)]
    fn parse_pv(&self, item: &Value) -> Result<PV> {
        let metadata = &item["metadata"];
        let spec = &item["spec"];
        let status = &item["status"];

        let name = metadata["name"]
            .as_str()
            .ok_or_else(|| anyhow!("Missing PV name"))?
            .to_string();

        let capacity = spec["capacity"]["storage"]
            .as_str()
            .unwrap_or("Unknown")
            .to_string();

        let access_modes = spec["accessModes"]
            .as_array()
            .map(|modes| {
                modes.iter()
                    .filter_map(|mode| mode.as_str())
                    .map(|s| s.to_string())
                    .collect()
            })
            .unwrap_or_default();

        let reclaim_policy = spec["persistentVolumeReclaimPolicy"]
            .as_str()
            .unwrap_or("Retain")
            .to_string();

        let pv_status = status["phase"]
            .as_str()
            .unwrap_or("Unknown")
            .to_string();

        let claim = spec["claimRef"]
            .as_object()
            .map(|claim_ref| {
                format!("{}/{}", 
                    claim_ref["namespace"].as_str().unwrap_or(""),
                    claim_ref["name"].as_str().unwrap_or(""))
            });

        let storage_class = spec["storageClassName"]
            .as_str()
            .map(|s| s.to_string());

        let age = self.calculate_age(metadata["creationTimestamp"].as_str());

        Ok(PV {
            name,
            capacity,
            access_modes,
            reclaim_policy,
            status: pv_status,
            claim,
            storage_class,
            age,
        })
    }
}