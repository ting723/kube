use std::process::Command;
use anyhow::{Result, anyhow};
use std::sync::OnceLock;

// 全局的 kubectl 命令类型检测器
static KUBECTL_CMD: OnceLock<KubectlCommand> = OnceLock::new();

#[derive(Debug, Clone)]
enum KubectlCommand {
    Direct,           // 直接使用 kubectl
    Minikube,         // 使用 minikube kubectl --
}

// 获取当前系统可用的 kubectl 命令类型
fn get_kubectl_command() -> &'static KubectlCommand {
    KUBECTL_CMD.get_or_init(|| {
        if check_kubectl_command("kubectl") {
            KubectlCommand::Direct
        } else if check_minikube_kubectl() {
            KubectlCommand::Minikube
        } else {
            // 默认返回 Direct，即使不可用也让错误显示
            KubectlCommand::Direct
        }
    })
}

// 执行 kubectl 命令，自动选择适合的命令方式
fn execute_kubectl(args: &[&str]) -> Result<String> {
    let kubectl_cmd = get_kubectl_command();
    
    let output = match kubectl_cmd {
        KubectlCommand::Direct => {
            Command::new("kubectl")
                .args(args)
                .output()
                .map_err(|e| anyhow!("Failed to execute kubectl: {}", e))?
        }
        KubectlCommand::Minikube => {
            let mut minikube_args = vec!["kubectl", "--"];
            minikube_args.extend(args);
            Command::new("minikube")
                .args(&minikube_args)
                .output()
                .map_err(|e| anyhow!("Failed to execute minikube kubectl: {}", e))?
        }
    };
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("kubectl command failed: {}", stderr));
    }
    
    String::from_utf8(output.stdout)
        .map_err(|e| anyhow!("Invalid UTF-8 output: {}", e))
}

pub fn get_namespaces() -> Result<Vec<String>> {
    let output = execute_kubectl(&["get", "namespaces", "-o", "name"])?;

    let namespaces = output
        .lines()
        .map(|line| line.replace("namespace/", ""))
        .collect();

    Ok(namespaces)
}

pub fn get_pods(namespace: &str) -> Result<String> {
    execute_kubectl(&["get", "pods", "-n", namespace, "-o", "json"])
}

pub fn get_services(namespace: &str) -> Result<String> {
    execute_kubectl(&["get", "services", "-n", namespace, "-o", "json"])
}

#[allow(dead_code)]
pub fn get_nodes() -> Result<String> {
    execute_kubectl(&["get", "nodes", "-o", "json"])
}

#[allow(dead_code)]
pub fn get_configmaps(namespace: &str) -> Result<String> {
    execute_kubectl(&["get", "configmaps", "-n", namespace, "-o", "json"])
}

#[allow(dead_code)]
pub fn get_secrets(namespace: &str) -> Result<String> {
    execute_kubectl(&["get", "secrets", "-n", namespace, "-o", "json"])
}

pub fn get_pod_logs(namespace: &str, pod_name: &str, lines: u32) -> Result<String> {
    execute_kubectl(&["logs", "-n", namespace, pod_name, "--tail", &lines.to_string()])
}

#[allow(dead_code)]
pub fn describe_pod(namespace: &str, pod_name: &str) -> Result<String> {
    execute_kubectl(&["describe", "pod", "-n", namespace, pod_name])
}

#[allow(dead_code)]
pub fn describe_service(namespace: &str, service_name: &str) -> Result<String> {
    execute_kubectl(&["describe", "service", "-n", namespace, service_name])
}

// 已删除重复的describe_deployment函数，使用下方新的execute_kubectl()系统版本

// 已删除重复的describe_job函数

// 已删除重复的describe_daemonset函数

// 已删除重复的describe_node函数

// 已删除重复的describe_configmap函数

// 已删除重复的describe_secret函数

// 已删除重复的describe_pvc函数

// 已删除重复的describe_pv函数

// 已删除重复的delete_pod函数

pub fn check_kubectl_available() -> bool {
    // 尝试多种方法检查 kubectl 是否可用
    
    // 方法1: 尝试直接使用 kubectl
    if check_kubectl_command("kubectl") {
        return true;
    }
    
    // 方法2: 尝试 minikube kubectl -- （支持 minikube 环境）
    if check_minikube_kubectl() {
        return true;
    }
    
    false
}

// 检查指定的 kubectl 命令是否可用
fn check_kubectl_command(kubectl_cmd: &str) -> bool {
    // 方法1: 尝试 kubectl version --client
    if let Ok(output) = Command::new(kubectl_cmd)
        .args(&["version", "--client"])
        .output()
    {
        if output.status.success() {
            return true;
        }
    }
    
    // 方法2: 尝试简单的 kubectl version
    if let Ok(output) = Command::new(kubectl_cmd)
        .args(&["version"])
        .output()
    {
        if output.status.success() {
            return true;
        }
    }
    
    // 方法3: 尝试 kubectl --help
    if let Ok(output) = Command::new(kubectl_cmd)
        .args(&["--help"])
        .output()
    {
        return output.status.success();
    }
    
    false
}

// 检查 minikube kubectl 是否可用
fn check_minikube_kubectl() -> bool {
    // 检查 minikube 是否安装
    if let Ok(output) = Command::new("minikube")
        .args(&["status"])
        .output()
    {
        if !output.status.success() {
            return false;
        }
    } else {
        return false;
    }
    
    // 尝试 minikube kubectl 命令
    // 方法1: minikube kubectl -- version --client
    if let Ok(output) = Command::new("minikube")
        .args(&["kubectl", "--", "version", "--client"])
        .output()
    {
        if output.status.success() {
            return true;
        }
    }
    
    // 方法2: minikube kubectl -- version
    if let Ok(output) = Command::new("minikube")
        .args(&["kubectl", "--", "version"])
        .output()
    {
        if output.status.success() {
            return true;
        }
    }
    
    // 方法3: minikube kubectl -- --help
    if let Ok(output) = Command::new("minikube")
        .args(&["kubectl", "--", "--help"])
        .output()
    {
        return output.status.success();
    }
    
    false
}

// 更多资源获取的命令
#[allow(dead_code)]
pub fn get_deployments(namespace: &str) -> Result<String> {
    execute_kubectl(&["get", "deployments", "-n", namespace, "-o", "json"])
}

#[allow(dead_code)]
pub fn get_jobs(namespace: &str) -> Result<String> {
    execute_kubectl(&["get", "jobs", "-n", namespace, "-o", "json"])
}

#[allow(dead_code)]
pub fn get_daemonsets(namespace: &str) -> Result<String> {
    execute_kubectl(&["get", "daemonsets", "-n", namespace, "-o", "json"])
}

#[allow(dead_code)]
pub fn get_pvcs(namespace: &str) -> Result<String> {
    execute_kubectl(&["get", "pvc", "-n", namespace, "-o", "json"])
}

#[allow(dead_code)]
pub fn get_pvs() -> Result<String> {
    execute_kubectl(&["get", "pv", "-o", "json"])
}

// 更多DESCRIBE命令
#[allow(dead_code)]
pub fn describe_deployment(namespace: &str, deployment_name: &str) -> Result<String> {
    execute_kubectl(&["describe", "deployment", "-n", namespace, deployment_name])
}

#[allow(dead_code)]
pub fn describe_job(namespace: &str, job_name: &str) -> Result<String> {
    execute_kubectl(&["describe", "job", "-n", namespace, job_name])
}

#[allow(dead_code)]
pub fn describe_daemonset(namespace: &str, daemonset_name: &str) -> Result<String> {
    execute_kubectl(&["describe", "daemonset", "-n", namespace, daemonset_name])
}

#[allow(dead_code)]
pub fn describe_node(node_name: &str) -> Result<String> {
    execute_kubectl(&["describe", "node", node_name])
}

#[allow(dead_code)]
pub fn describe_configmap(namespace: &str, configmap_name: &str) -> Result<String> {
    execute_kubectl(&["describe", "configmap", "-n", namespace, configmap_name])
}

#[allow(dead_code)]
pub fn describe_secret(namespace: &str, secret_name: &str) -> Result<String> {
    execute_kubectl(&["describe", "secret", "-n", namespace, secret_name])
}

#[allow(dead_code)]
pub fn describe_pvc(namespace: &str, pvc_name: &str) -> Result<String> {
    execute_kubectl(&["describe", "pvc", "-n", namespace, pvc_name])
}

#[allow(dead_code)]
pub fn describe_pv(pv_name: &str) -> Result<String> {
    execute_kubectl(&["describe", "pv", pv_name])
}

// 操作命令
#[allow(dead_code)]
pub fn delete_pod(namespace: &str, pod_name: &str) -> Result<String> {
    execute_kubectl(&["delete", "pod", "-n", namespace, pod_name])
}

// exec命令需要特殊处理，因为它需要交互式终端
#[allow(dead_code)]
pub fn exec_pod(namespace: &str, pod_name: &str, command: &str) -> Result<()> {
    // exec命令需要保持直接调用，因为它需要交互式终端
    let kubectl_cmd = get_kubectl_command();
    
    let status = match kubectl_cmd {
        KubectlCommand::Direct => {
            Command::new("kubectl")
                .args(&["exec", "-it", "-n", namespace, pod_name, "--", "sh", "-c", command])
                .status()?
        }
        KubectlCommand::Minikube => {
            Command::new("minikube")
                .args(&["kubectl", "--", "exec", "-it", "-n", namespace, pod_name, "--", "sh", "-c", command])
                .status()?
        }
    };
    
    if !status.success() {
        return Err(anyhow!("kubectl exec failed"));
    }
    
    Ok(())
}

// YAML配置相关命令
pub fn get_pod_yaml(namespace: &str, pod_name: &str) -> Result<String> {
    execute_kubectl(&["get", "pod", "-n", namespace, pod_name, "-o", "yaml"])
}

pub fn get_service_yaml(namespace: &str, service_name: &str) -> Result<String> {
    execute_kubectl(&["get", "service", "-n", namespace, service_name, "-o", "yaml"])
}

pub fn get_deployment_yaml(namespace: &str, deployment_name: &str) -> Result<String> {
    execute_kubectl(&["get", "deployment", "-n", namespace, deployment_name, "-o", "yaml"])
}

pub fn get_job_yaml(namespace: &str, job_name: &str) -> Result<String> {
    execute_kubectl(&["get", "job", "-n", namespace, job_name, "-o", "yaml"])
}

pub fn get_daemonset_yaml(namespace: &str, daemonset_name: &str) -> Result<String> {
    execute_kubectl(&["get", "daemonset", "-n", namespace, daemonset_name, "-o", "yaml"])
}

pub fn get_node_yaml(node_name: &str) -> Result<String> {
    execute_kubectl(&["get", "node", node_name, "-o", "yaml"])
}

pub fn get_configmap_yaml(namespace: &str, configmap_name: &str) -> Result<String> {
    execute_kubectl(&["get", "configmap", "-n", namespace, configmap_name, "-o", "yaml"])
}

pub fn get_secret_yaml(namespace: &str, secret_name: &str) -> Result<String> {
    execute_kubectl(&["get", "secret", "-n", namespace, secret_name, "-o", "yaml"])
}

pub fn get_pvc_yaml(namespace: &str, pvc_name: &str) -> Result<String> {
    execute_kubectl(&["get", "pvc", "-n", namespace, pvc_name, "-o", "yaml"])
}

pub fn get_pv_yaml(pv_name: &str) -> Result<String> {
    execute_kubectl(&["get", "pv", pv_name, "-o", "yaml"])
}

// 资源监控相关命令
pub fn get_top_pods(namespace: &str) -> Result<String> {
    execute_kubectl(&["top", "pods", "-n", namespace, "--no-headers"])
        .map_err(|e| anyhow!("kubectl top failed: {}. Note: metrics-server might not be installed.", e))
}

pub fn get_top_pod(namespace: &str, pod_name: &str) -> Result<String> {
    execute_kubectl(&["top", "pod", "-n", namespace, pod_name, "--containers", "--no-headers"])
        .map_err(|e| anyhow!("kubectl top failed: {}. Note: metrics-server might not be installed.", e))
}

// 新增资源获取命令
#[allow(dead_code)]
pub fn get_statefulsets(namespace: &str) -> Result<String> {
    execute_kubectl(&["get", "statefulsets", "-n", namespace, "-o", "json"])
}

#[allow(dead_code)]
pub fn get_ingresses(namespace: &str) -> Result<String> {
    execute_kubectl(&["get", "ingresses", "-n", namespace, "-o", "json"])
}

#[allow(dead_code)]
pub fn get_network_policies(namespace: &str) -> Result<String> {
    execute_kubectl(&["get", "networkpolicies", "-n", namespace, "-o", "json"])
}

#[allow(dead_code)]
pub fn get_roles(namespace: &str) -> Result<String> {
    execute_kubectl(&["get", "roles", "-n", namespace, "-o", "json"])
}

#[allow(dead_code)]
pub fn get_role_bindings(namespace: &str) -> Result<String> {
    execute_kubectl(&["get", "rolebindings", "-n", namespace, "-o", "json"])
}

#[allow(dead_code)]
pub fn get_cluster_roles() -> Result<String> {
    execute_kubectl(&["get", "clusterroles", "-o", "json"])
}

#[allow(dead_code)]
pub fn get_cluster_role_bindings() -> Result<String> {
    execute_kubectl(&["get", "clusterrolebindings", "-o", "json"])
}

#[allow(dead_code)]
pub fn get_service_accounts(namespace: &str) -> Result<String> {
    execute_kubectl(&["get", "serviceaccounts", "-n", namespace, "-o", "json"])
}

// 新增DESCRIBE命令
#[allow(dead_code)]
pub fn describe_statefulset(namespace: &str, name: &str) -> Result<String> {
    execute_kubectl(&["describe", "statefulset", "-n", namespace, name])
}

#[allow(dead_code)]
pub fn describe_ingress(namespace: &str, name: &str) -> Result<String> {
    execute_kubectl(&["describe", "ingress", "-n", namespace, name])
}

#[allow(dead_code)]
pub fn describe_network_policy(namespace: &str, name: &str) -> Result<String> {
    execute_kubectl(&["describe", "networkpolicy", "-n", namespace, name])
}

#[allow(dead_code)]
pub fn describe_role(namespace: &str, name: &str) -> Result<String> {
    execute_kubectl(&["describe", "role", "-n", namespace, name])
}

#[allow(dead_code)]
pub fn describe_role_binding(namespace: &str, name: &str) -> Result<String> {
    execute_kubectl(&["describe", "rolebinding", "-n", namespace, name])
}

#[allow(dead_code)]
pub fn describe_cluster_role(name: &str) -> Result<String> {
    execute_kubectl(&["describe", "clusterrole", name])
}

#[allow(dead_code)]
pub fn describe_cluster_role_binding(name: &str) -> Result<String> {
    execute_kubectl(&["describe", "clusterrolebinding", name])
}

#[allow(dead_code)]
pub fn describe_service_account(namespace: &str, name: &str) -> Result<String> {
    execute_kubectl(&["describe", "serviceaccount", "-n", namespace, name])
}

// 新增YAML命令
#[allow(dead_code)]
pub fn get_statefulset_yaml(namespace: &str, name: &str) -> Result<String> {
    execute_kubectl(&["get", "statefulset", "-n", namespace, name, "-o", "yaml"])
}

#[allow(dead_code)]
pub fn get_ingress_yaml(namespace: &str, name: &str) -> Result<String> {
    execute_kubectl(&["get", "ingress", "-n", namespace, name, "-o", "yaml"])
}

#[allow(dead_code)]
pub fn get_network_policy_yaml(namespace: &str, name: &str) -> Result<String> {
    execute_kubectl(&["get", "networkpolicy", "-n", namespace, name, "-o", "yaml"])
}

#[allow(dead_code)]
pub fn get_role_yaml(namespace: &str, name: &str) -> Result<String> {
    execute_kubectl(&["get", "role", "-n", namespace, name, "-o", "yaml"])
}

#[allow(dead_code)]
pub fn get_role_binding_yaml(namespace: &str, name: &str) -> Result<String> {
    execute_kubectl(&["get", "rolebinding", "-n", namespace, name, "-o", "yaml"])
}

#[allow(dead_code)]
pub fn get_cluster_role_yaml(name: &str) -> Result<String> {
    execute_kubectl(&["get", "clusterrole", name, "-o", "yaml"])
}

#[allow(dead_code)]
pub fn get_cluster_role_binding_yaml(name: &str) -> Result<String> {
    execute_kubectl(&["get", "clusterrolebinding", name, "-o", "yaml"])
}

#[allow(dead_code)]
pub fn get_service_account_yaml(namespace: &str, name: &str) -> Result<String> {
    execute_kubectl(&["get", "serviceaccount", "-n", namespace, name, "-o", "yaml"])
}
