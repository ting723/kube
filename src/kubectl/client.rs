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
    pub async fn get_jobs(&self, namespace: &str) -> Result<Vec<Job>> {
        let json_output = commands::get_jobs(namespace)?;
        let parsed: Value = serde_json::from_str(&json_output)?;

        let items = parsed["items"]
            .as_array()
            .ok_or_else(|| anyhow!("Invalid JSON response: missing items array"))?;

        let mut jobs = Vec::new();
        
        for item in items {
            if let Ok(job) = self.parse_job(item) {
                jobs.push(job);
            }
        }

        Ok(jobs)
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

    // 新增资源类型获取方法
    #[allow(dead_code)]
    pub async fn get_statefulsets(&self, namespace: &str) -> Result<Vec<StatefulSet>> {
        let json_output = commands::get_statefulsets(namespace)?;
        let parsed: Value = serde_json::from_str(&json_output)?;

        let items = parsed["items"]
            .as_array()
            .ok_or_else(|| anyhow!("Invalid JSON response: missing items array"))?;

        let mut statefulsets = Vec::new();
        
        for item in items {
            if let Ok(statefulset) = self.parse_statefulset(item) {
                statefulsets.push(statefulset);
            }
        }

        Ok(statefulsets)
    }

    #[allow(dead_code)]
    pub async fn get_ingresses(&self, namespace: &str) -> Result<Vec<Ingress>> {
        let json_output = commands::get_ingresses(namespace)?;
        let parsed: Value = serde_json::from_str(&json_output)?;

        let items = parsed["items"]
            .as_array()
            .ok_or_else(|| anyhow!("Invalid JSON response: missing items array"))?;

        let mut ingresses = Vec::new();
        
        for item in items {
            if let Ok(ingress) = self.parse_ingress(item) {
                ingresses.push(ingress);
            }
        }

        Ok(ingresses)
    }

    #[allow(dead_code)]
    pub async fn get_network_policies(&self, namespace: &str) -> Result<Vec<NetworkPolicy>> {
        let json_output = commands::get_network_policies(namespace)?;
        let parsed: Value = serde_json::from_str(&json_output)?;

        let items = parsed["items"]
            .as_array()
            .ok_or_else(|| anyhow!("Invalid JSON response: missing items array"))?;

        let mut network_policies = Vec::new();
        
        for item in items {
            if let Ok(network_policy) = self.parse_network_policy(item) {
                network_policies.push(network_policy);
            }
        }

        Ok(network_policies)
    }

    #[allow(dead_code)]
    pub async fn get_roles(&self, namespace: &str) -> Result<Vec<Role>> {
        let json_output = commands::get_roles(namespace)?;
        let parsed: Value = serde_json::from_str(&json_output)?;

        let items = parsed["items"]
            .as_array()
            .ok_or_else(|| anyhow!("Invalid JSON response: missing items array"))?;

        let mut roles = Vec::new();
        
        for item in items {
            if let Ok(role) = self.parse_role(item) {
                roles.push(role);
            }
        }

        Ok(roles)
    }

    #[allow(dead_code)]
    pub async fn get_role_bindings(&self, namespace: &str) -> Result<Vec<RoleBinding>> {
        let json_output = commands::get_role_bindings(namespace)?;
        let parsed: Value = serde_json::from_str(&json_output)?;

        let items = parsed["items"]
            .as_array()
            .ok_or_else(|| anyhow!("Invalid JSON response: missing items array"))?;

        let mut role_bindings = Vec::new();
        
        for item in items {
            if let Ok(role_binding) = self.parse_role_binding(item) {
                role_bindings.push(role_binding);
            }
        }

        Ok(role_bindings)
    }

    #[allow(dead_code)]
    pub async fn get_cluster_roles(&self) -> Result<Vec<ClusterRole>> {
        let json_output = commands::get_cluster_roles()?;
        let parsed: Value = serde_json::from_str(&json_output)?;

        let items = parsed["items"]
            .as_array()
            .ok_or_else(|| anyhow!("Invalid JSON response: missing items array"))?;

        let mut cluster_roles = Vec::new();
        
        for item in items {
            if let Ok(cluster_role) = self.parse_cluster_role(item) {
                cluster_roles.push(cluster_role);
            }
        }

        Ok(cluster_roles)
    }

    #[allow(dead_code)]
    pub async fn get_cluster_role_bindings(&self) -> Result<Vec<ClusterRoleBinding>> {
        let json_output = commands::get_cluster_role_bindings()?;
        let parsed: Value = serde_json::from_str(&json_output)?;

        let items = parsed["items"]
            .as_array()
            .ok_or_else(|| anyhow!("Invalid JSON response: missing items array"))?;

        let mut cluster_role_bindings = Vec::new();
        
        for item in items {
            if let Ok(cluster_role_binding) = self.parse_cluster_role_binding(item) {
                cluster_role_bindings.push(cluster_role_binding);
            }
        }

        Ok(cluster_role_bindings)
    }

    #[allow(dead_code)]
    pub async fn get_service_accounts(&self, namespace: &str) -> Result<Vec<ServiceAccount>> {
        let json_output = commands::get_service_accounts(namespace)?;
        let parsed: Value = serde_json::from_str(&json_output)?;

        let items = parsed["items"]
            .as_array()
            .ok_or_else(|| anyhow!("Invalid JSON response: missing items array"))?;

        let mut service_accounts = Vec::new();
        
        for item in items {
            if let Ok(service_account) = self.parse_service_account(item) {
                service_accounts.push(service_account);
            }
        }

        Ok(service_accounts)
    }

    pub async fn get_pod_logs(&self, namespace: &str, pod_name: &str, lines: u32) -> Result<Vec<String>> {
        let logs = commands::get_pod_logs(namespace, pod_name, lines)?;
        Ok(logs.lines().map(|line| line.to_string()).collect())
    }

    pub async fn describe_pod(&self, namespace: &str, pod_name: &str) -> Result<String> {
        commands::describe_pod(namespace, pod_name)
    }

    pub async fn describe_service(&self, namespace: &str, service_name: &str) -> Result<String> {
        commands::describe_service(namespace, service_name)
    }

    pub async fn describe_deployment(&self, namespace: &str, deployment_name: &str) -> Result<String> {
        commands::describe_deployment(namespace, deployment_name)
    }

    pub async fn describe_job(&self, namespace: &str, job_name: &str) -> Result<String> {
        commands::describe_job(namespace, job_name)
    }

    pub async fn describe_daemonset(&self, namespace: &str, daemonset_name: &str) -> Result<String> {
        commands::describe_daemonset(namespace, daemonset_name)
    }

    pub async fn describe_node(&self, node_name: &str) -> Result<String> {
        commands::describe_node(node_name)
    }

    pub async fn describe_configmap(&self, namespace: &str, configmap_name: &str) -> Result<String> {
        commands::describe_configmap(namespace, configmap_name)
    }

    pub async fn describe_secret(&self, namespace: &str, secret_name: &str) -> Result<String> {
        commands::describe_secret(namespace, secret_name)
    }

    pub async fn describe_pvc(&self, namespace: &str, pvc_name: &str) -> Result<String> {
        commands::describe_pvc(namespace, pvc_name)
    }

    pub async fn describe_pv(&self, pv_name: &str) -> Result<String> {
        commands::describe_pv(pv_name)
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
    fn parse_job(&self, item: &Value) -> Result<Job> {
        let metadata = &item["metadata"];
        let spec = &item["spec"];
        let status = &item["status"];

        let name = metadata["name"]
            .as_str()
            .ok_or_else(|| anyhow!("Missing job name"))?
            .to_string();

        let namespace = metadata["namespace"]
            .as_str()
            .ok_or_else(|| anyhow!("Missing job namespace"))?
            .to_string();

        let completions = spec["completions"].as_u64().map(|c| c as u32);
        let successful = status["succeeded"].as_u64().unwrap_or(0) as u32;
        
        // 计算作业状态
        let job_status = if status["succeeded"].as_u64().unwrap_or(0) > 0 {
            "Complete".to_string()
        } else if status["failed"].as_u64().unwrap_or(0) > 0 {
            "Failed".to_string()
        } else if status["active"].as_u64().unwrap_or(0) > 0 {
            "Running".to_string()
        } else {
            "Pending".to_string()
        };

        // 计算持续时间
        let duration = if let (Some(_start_time), Some(_completion_time)) = (
            status["startTime"].as_str(),
            status["completionTime"].as_str()
        ) {
            // 这里可以计算真实的持续时间，简化处理
            Some("<calculated>".to_string())
        } else {
            None
        };

        let age = self.calculate_age(metadata["creationTimestamp"].as_str());

        Ok(Job {
            name,
            namespace,
            completions,
            successful,
            age,
            duration,
            status: job_status,
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

    // 解析StatefulSet
    fn parse_statefulset(&self, item: &Value) -> Result<StatefulSet> {
        let metadata = &item["metadata"];
        let name = metadata["name"].as_str().unwrap_or("Unknown").to_string();
        let namespace = metadata["namespace"].as_str().unwrap_or("default").to_string();
        let creation_timestamp = metadata["creationTimestamp"].as_str();
        let age = self.calculate_age(creation_timestamp);
        
        let status = &item["status"];
        let replicas = status["replicas"].as_u64().unwrap_or(0);
        let ready_replicas = status["readyReplicas"].as_u64().unwrap_or(0);
        let ready = format!("{}/{}", ready_replicas, replicas);
        
        Ok(StatefulSet {
            name,
            namespace,
            ready,
            age,
        })
    }

    // 解析Ingress
    fn parse_ingress(&self, item: &Value) -> Result<Ingress> {
        let metadata = &item["metadata"];
        let name = metadata["name"].as_str().unwrap_or("Unknown").to_string();
        let namespace = metadata["namespace"].as_str().unwrap_or("default").to_string();
        let creation_timestamp = metadata["creationTimestamp"].as_str();
        let age = self.calculate_age(creation_timestamp);
        
        let mut hosts = Vec::new();
        if let Some(rules) = item["spec"]["rules"].as_array() {
            for rule in rules {
                if let Some(host) = rule["host"].as_str() {
                    hosts.push(host.to_string());
                }
            }
        }
        
        Ok(Ingress {
            name,
            namespace,
            hosts,
            age,
        })
    }

    // 解析NetworkPolicy
    fn parse_network_policy(&self, item: &Value) -> Result<NetworkPolicy> {
        let metadata = &item["metadata"];
        let name = metadata["name"].as_str().unwrap_or("Unknown").to_string();
        let namespace = metadata["namespace"].as_str().unwrap_or("default").to_string();
        let creation_timestamp = metadata["creationTimestamp"].as_str();
        let age = self.calculate_age(creation_timestamp);
        
        Ok(NetworkPolicy {
            name,
            namespace,
            age,
        })
    }

    // 解析Role
    fn parse_role(&self, item: &Value) -> Result<Role> {
        let metadata = &item["metadata"];
        let name = metadata["name"].as_str().unwrap_or("Unknown").to_string();
        let namespace = metadata["namespace"].as_str().unwrap_or("default").to_string();
        let creation_timestamp = metadata["creationTimestamp"].as_str();
        let age = self.calculate_age(creation_timestamp);
        
        Ok(Role {
            name,
            namespace,
            age,
        })
    }

    // 解析RoleBinding
    fn parse_role_binding(&self, item: &Value) -> Result<RoleBinding> {
        let metadata = &item["metadata"];
        let name = metadata["name"].as_str().unwrap_or("Unknown").to_string();
        let namespace = metadata["namespace"].as_str().unwrap_or("default").to_string();
        let creation_timestamp = metadata["creationTimestamp"].as_str();
        let age = self.calculate_age(creation_timestamp);
        
        Ok(RoleBinding {
            name,
            namespace,
            age,
        })
    }

    // 解析ClusterRole
    fn parse_cluster_role(&self, item: &Value) -> Result<ClusterRole> {
        let metadata = &item["metadata"];
        let name = metadata["name"].as_str().unwrap_or("Unknown").to_string();
        let creation_timestamp = metadata["creationTimestamp"].as_str();
        let age = self.calculate_age(creation_timestamp);
        
        Ok(ClusterRole {
            name,
            age,
        })
    }

    // 解析ClusterRoleBinding
    fn parse_cluster_role_binding(&self, item: &Value) -> Result<ClusterRoleBinding> {
        let metadata = &item["metadata"];
        let name = metadata["name"].as_str().unwrap_or("Unknown").to_string();
        let creation_timestamp = metadata["creationTimestamp"].as_str();
        let age = self.calculate_age(creation_timestamp);
        
        Ok(ClusterRoleBinding {
            name,
            age,
        })
    }

    // 解析ServiceAccount
    fn parse_service_account(&self, item: &Value) -> Result<ServiceAccount> {
        let metadata = &item["metadata"];
        let name = metadata["name"].as_str().unwrap_or("Unknown").to_string();
        let namespace = metadata["namespace"].as_str().unwrap_or("default").to_string();
        let creation_timestamp = metadata["creationTimestamp"].as_str();
        let age = self.calculate_age(creation_timestamp);
        
        Ok(ServiceAccount {
            name,
            namespace,
            age,
        })
    }

    // YAML配置相关方法
    pub async fn get_yaml(&self, resource_type: &str, namespace: Option<&str>, name: &str) -> Result<String> {
        match resource_type {
            "pod" => {
                if let Some(ns) = namespace {
                    commands::get_pod_yaml(ns, name)
                } else {
                    Err(anyhow!("Pod requires namespace"))
                }
            },
            "service" => {
                if let Some(ns) = namespace {
                    commands::get_service_yaml(ns, name)
                } else {
                    Err(anyhow!("Service requires namespace"))
                }
            },
            "deployment" => {
                if let Some(ns) = namespace {
                    commands::get_deployment_yaml(ns, name)
                } else {
                    Err(anyhow!("Deployment requires namespace"))
                }
            },
            "job" => {
                if let Some(ns) = namespace {
                    commands::get_job_yaml(ns, name)
                } else {
                    Err(anyhow!("Job requires namespace"))
                }
            },
            "daemonset" => {
                if let Some(ns) = namespace {
                    commands::get_daemonset_yaml(ns, name)
                } else {
                    Err(anyhow!("DaemonSet requires namespace"))
                }
            },
            "node" => commands::get_node_yaml(name),
            "configmap" => {
                if let Some(ns) = namespace {
                    commands::get_configmap_yaml(ns, name)
                } else {
                    Err(anyhow!("ConfigMap requires namespace"))
                }
            },
            "secret" => {
                if let Some(ns) = namespace {
                    commands::get_secret_yaml(ns, name)
                } else {
                    Err(anyhow!("Secret requires namespace"))
                }
            },
            "pvc" => {
                if let Some(ns) = namespace {
                    commands::get_pvc_yaml(ns, name)
                } else {
                    Err(anyhow!("PVC requires namespace"))
                }
            },
            "pv" => commands::get_pv_yaml(name),
            _ => Err(anyhow!("Unsupported resource type: {}", resource_type))
        }
    }

    // 资源监控相关方法
    pub async fn get_pod_metrics(&self, namespace: &str) -> Result<Vec<ResourceMetrics>> {
        let output = commands::get_top_pods(namespace)?;
        let mut metrics = Vec::new();
        
        for line in output.lines() {
            if line.trim().is_empty() {
                continue;
            }
            
            if let Ok(metric) = self.parse_pod_metrics_line(line, namespace).await {
                metrics.push(metric);
            }
        }
        
        Ok(metrics)
    }

    async fn parse_pod_metrics_line(&self, line: &str, namespace: &str) -> Result<ResourceMetrics> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 3 {
            return Err(anyhow!("Invalid metrics line format"));
        }
        
        let name = parts[0].to_string();
        let cpu = parts[1].to_string();
        let memory = parts[2].to_string();
        
        // 尝试解析CPU和内存百分比（如果有的话）
        let cpu_percentage = self.parse_cpu_percentage(&cpu);
        let memory_percentage = self.parse_memory_percentage(&memory);
        
        // 获取容器级别的详细信息
        let containers = self.get_container_metrics(namespace, &name).await.unwrap_or_default();
        
        Ok(ResourceMetrics {
            name,
            namespace: namespace.to_string(),
            cpu,
            memory,
            cpu_percentage,
            memory_percentage,
            containers,
        })
    }

    async fn get_container_metrics(&self, namespace: &str, pod_name: &str) -> Result<Vec<crate::kubectl::types::ContainerMetrics>> {
        let output = commands::get_top_pod(namespace, pod_name)?;
        let mut containers = Vec::new();
        
        for line in output.lines() {
            if line.trim().is_empty() {
                continue;
            }
            
            if let Ok(container) = self.parse_container_metrics_line(line) {
                containers.push(container);
            }
        }
        
        Ok(containers)
    }

    fn parse_container_metrics_line(&self, line: &str) -> Result<crate::kubectl::types::ContainerMetrics> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 4 {
            return Err(anyhow!("Invalid container metrics line format"));
        }
        
        let name = parts[1].to_string(); // 第一列是pod名，第二列是容器名
        let cpu = parts[2].to_string();
        let memory = parts[3].to_string();
        
        let cpu_percentage = self.parse_cpu_percentage(&cpu);
        let memory_percentage = self.parse_memory_percentage(&memory);
        
        Ok(crate::kubectl::types::ContainerMetrics {
            name,
            cpu,
            memory,
            cpu_percentage,
            memory_percentage,
        })
    }

    fn parse_cpu_percentage(&self, cpu_str: &str) -> Option<f64> {
        if cpu_str.ends_with('m') {
            cpu_str.trim_end_matches('m').parse::<f64>().ok().map(|v| v / 10.0) // 毫核转换为百分比
        } else {
            cpu_str.parse::<f64>().ok().map(|v| v * 100.0) // 核转换为百分比
        }
    }

    fn parse_memory_percentage(&self, _memory_str: &str) -> Option<f64> {
        // 这里需要根据实际的Pod容限来计算百分比，这里暂时返回None
        None
    }
}