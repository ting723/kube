use std::process::Command;
use anyhow::{Result, anyhow};

pub fn get_namespaces() -> Result<Vec<String>> {
    let output = Command::new("kubectl")
        .args(&["get", "namespaces", "-o", "name"])
        .output()?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("kubectl failed: {}", error));
    }

    let namespaces = String::from_utf8(output.stdout)?
        .lines()
        .map(|line| line.replace("namespace/", ""))
        .collect();

    Ok(namespaces)
}

pub fn get_pods(namespace: &str) -> Result<String> {
    let output = Command::new("kubectl")
        .args(&["get", "pods", "-n", namespace, "-o", "json"])
        .output()?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("kubectl failed: {}", error));
    }

    Ok(String::from_utf8(output.stdout)?)
}

pub fn get_services(namespace: &str) -> Result<String> {
    let output = Command::new("kubectl")
        .args(&["get", "services", "-n", namespace, "-o", "json"])
        .output()?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("kubectl failed: {}", error));
    }

    Ok(String::from_utf8(output.stdout)?)
}

#[allow(dead_code)]
pub fn get_nodes() -> Result<String> {
    let output = Command::new("kubectl")
        .args(&["get", "nodes", "-o", "json"])
        .output()?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("kubectl failed: {}", error));
    }

    Ok(String::from_utf8(output.stdout)?)
}

#[allow(dead_code)]
pub fn get_configmaps(namespace: &str) -> Result<String> {
    let output = Command::new("kubectl")
        .args(&["get", "configmaps", "-n", namespace, "-o", "json"])
        .output()?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("kubectl failed: {}", error));
    }

    Ok(String::from_utf8(output.stdout)?)
}

#[allow(dead_code)]
pub fn get_secrets(namespace: &str) -> Result<String> {
    let output = Command::new("kubectl")
        .args(&["get", "secrets", "-n", namespace, "-o", "json"])
        .output()?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("kubectl failed: {}", error));
    }

    Ok(String::from_utf8(output.stdout)?)
}

pub fn get_pod_logs(namespace: &str, pod_name: &str, lines: u32) -> Result<String> {
    let output = Command::new("kubectl")
        .args(&["logs", "-n", namespace, pod_name, "--tail", &lines.to_string()])
        .output()?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("kubectl failed: {}", error));
    }

    Ok(String::from_utf8(output.stdout)?)
}

#[allow(dead_code)]
pub fn describe_pod(namespace: &str, pod_name: &str) -> Result<String> {
    let output = Command::new("kubectl")
        .args(&["describe", "pod", "-n", namespace, pod_name])
        .output()?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("kubectl failed: {}", error));
    }

    Ok(String::from_utf8(output.stdout)?)
}

#[allow(dead_code)]
pub fn describe_service(namespace: &str, service_name: &str) -> Result<String> {
    let output = Command::new("kubectl")
        .args(&["describe", "service", "-n", namespace, service_name])
        .output()?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("kubectl failed: {}", error));
    }

    Ok(String::from_utf8(output.stdout)?)
}

#[allow(dead_code)]
pub fn describe_deployment(namespace: &str, deployment_name: &str) -> Result<String> {
    let output = Command::new("kubectl")
        .args(&["describe", "deployment", "-n", namespace, deployment_name])
        .output()?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("kubectl failed: {}", error));
    }

    Ok(String::from_utf8(output.stdout)?)
}

#[allow(dead_code)]
pub fn describe_job(namespace: &str, job_name: &str) -> Result<String> {
    let output = Command::new("kubectl")
        .args(&["describe", "job", "-n", namespace, job_name])
        .output()?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("kubectl failed: {}", error));
    }

    Ok(String::from_utf8(output.stdout)?)
}

#[allow(dead_code)]
pub fn describe_daemonset(namespace: &str, daemonset_name: &str) -> Result<String> {
    let output = Command::new("kubectl")
        .args(&["describe", "daemonset", "-n", namespace, daemonset_name])
        .output()?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("kubectl failed: {}", error));
    }

    Ok(String::from_utf8(output.stdout)?)
}

#[allow(dead_code)]
pub fn describe_node(node_name: &str) -> Result<String> {
    let output = Command::new("kubectl")
        .args(&["describe", "node", node_name])
        .output()?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("kubectl failed: {}", error));
    }

    Ok(String::from_utf8(output.stdout)?)
}

#[allow(dead_code)]
pub fn describe_configmap(namespace: &str, configmap_name: &str) -> Result<String> {
    let output = Command::new("kubectl")
        .args(&["describe", "configmap", "-n", namespace, configmap_name])
        .output()?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("kubectl failed: {}", error));
    }

    Ok(String::from_utf8(output.stdout)?)
}

#[allow(dead_code)]
pub fn describe_secret(namespace: &str, secret_name: &str) -> Result<String> {
    let output = Command::new("kubectl")
        .args(&["describe", "secret", "-n", namespace, secret_name])
        .output()?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("kubectl failed: {}", error));
    }

    Ok(String::from_utf8(output.stdout)?)
}

#[allow(dead_code)]
pub fn describe_pvc(namespace: &str, pvc_name: &str) -> Result<String> {
    let output = Command::new("kubectl")
        .args(&["describe", "pvc", "-n", namespace, pvc_name])
        .output()?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("kubectl failed: {}", error));
    }

    Ok(String::from_utf8(output.stdout)?)
}

#[allow(dead_code)]
pub fn describe_pv(pv_name: &str) -> Result<String> {
    let output = Command::new("kubectl")
        .args(&["describe", "pv", pv_name])
        .output()?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("kubectl failed: {}", error));
    }

    Ok(String::from_utf8(output.stdout)?)
}

#[allow(dead_code)]
pub fn delete_pod(namespace: &str, pod_name: &str) -> Result<String> {
    let output = Command::new("kubectl")
        .args(&["delete", "pod", "-n", namespace, pod_name])
        .output()?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("kubectl failed: {}", error));
    }

    Ok(String::from_utf8(output.stdout)?)
}

#[allow(dead_code)]
pub fn exec_pod(namespace: &str, pod_name: &str, command: &str) -> Result<()> {
    let status = Command::new("kubectl")
        .args(&["exec", "-it", "-n", namespace, pod_name, "--", "sh", "-c", command])
        .status()?;

    if !status.success() {
        return Err(anyhow!("kubectl exec failed"));
    }

    Ok(())
}

pub fn check_kubectl_available() -> bool {
    // 尝试多种方法检查 kubectl 是否可用
    
    // 方法1: 尝试 kubectl version --client
    if let Ok(output) = Command::new("kubectl")
        .args(&["version", "--client"])
        .output()
    {
        if output.status.success() {
            return true;
        }
    }
    
    // 方法2: 尝试简单的 kubectl version
    if let Ok(output) = Command::new("kubectl")
        .args(&["version"])
        .output()
    {
        if output.status.success() {
            return true;
        }
    }
    
    // 方法3: 尝试 kubectl --help
    if let Ok(output) = Command::new("kubectl")
        .args(&["--help"])
        .output()
    {
        return output.status.success();
    }
    
    false
}

#[allow(dead_code)]
pub fn get_deployments(namespace: &str) -> Result<String> {
    let output = Command::new("kubectl")
        .args(&["get", "deployments", "-n", namespace, "-o", "json"])
        .output()?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("kubectl failed: {}", error));
    }

    Ok(String::from_utf8(output.stdout)?)
}

#[allow(dead_code)]
pub fn get_jobs(namespace: &str) -> Result<String> {
    let output = Command::new("kubectl")
        .args(&["get", "jobs", "-n", namespace, "-o", "json"])
        .output()?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("kubectl failed: {}", error));
    }

    Ok(String::from_utf8(output.stdout)?)
}

#[allow(dead_code)]
pub fn get_daemonsets(namespace: &str) -> Result<String> {
    let output = Command::new("kubectl")
        .args(&["get", "daemonsets", "-n", namespace, "-o", "json"])
        .output()?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("kubectl failed: {}", error));
    }

    Ok(String::from_utf8(output.stdout)?)
}

#[allow(dead_code)]
pub fn get_pvcs(namespace: &str) -> Result<String> {
    let output = Command::new("kubectl")
        .args(&["get", "pvc", "-n", namespace, "-o", "json"])
        .output()?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("kubectl failed: {}", error));
    }

    Ok(String::from_utf8(output.stdout)?)
}

#[allow(dead_code)]
pub fn get_pvs() -> Result<String> {
    let output = Command::new("kubectl")
        .args(&["get", "pv", "-o", "json"])
        .output()?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("kubectl failed: {}", error));
    }

    Ok(String::from_utf8(output.stdout)?)
}

// YAML配置相关命令
pub fn get_pod_yaml(namespace: &str, pod_name: &str) -> Result<String> {
    let output = Command::new("kubectl")
        .args(&["get", "pod", "-n", namespace, pod_name, "-o", "yaml"])
        .output()?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("kubectl failed: {}", error));
    }

    Ok(String::from_utf8(output.stdout)?)
}

pub fn get_service_yaml(namespace: &str, service_name: &str) -> Result<String> {
    let output = Command::new("kubectl")
        .args(&["get", "service", "-n", namespace, service_name, "-o", "yaml"])
        .output()?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("kubectl failed: {}", error));
    }

    Ok(String::from_utf8(output.stdout)?)
}

pub fn get_deployment_yaml(namespace: &str, deployment_name: &str) -> Result<String> {
    let output = Command::new("kubectl")
        .args(&["get", "deployment", "-n", namespace, deployment_name, "-o", "yaml"])
        .output()?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("kubectl failed: {}", error));
    }

    Ok(String::from_utf8(output.stdout)?)
}

pub fn get_job_yaml(namespace: &str, job_name: &str) -> Result<String> {
    let output = Command::new("kubectl")
        .args(&["get", "job", "-n", namespace, job_name, "-o", "yaml"])
        .output()?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("kubectl failed: {}", error));
    }

    Ok(String::from_utf8(output.stdout)?)
}

pub fn get_daemonset_yaml(namespace: &str, daemonset_name: &str) -> Result<String> {
    let output = Command::new("kubectl")
        .args(&["get", "daemonset", "-n", namespace, daemonset_name, "-o", "yaml"])
        .output()?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("kubectl failed: {}", error));
    }

    Ok(String::from_utf8(output.stdout)?)
}

pub fn get_node_yaml(node_name: &str) -> Result<String> {
    let output = Command::new("kubectl")
        .args(&["get", "node", node_name, "-o", "yaml"])
        .output()?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("kubectl failed: {}", error));
    }

    Ok(String::from_utf8(output.stdout)?)
}

pub fn get_configmap_yaml(namespace: &str, configmap_name: &str) -> Result<String> {
    let output = Command::new("kubectl")
        .args(&["get", "configmap", "-n", namespace, configmap_name, "-o", "yaml"])
        .output()?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("kubectl failed: {}", error));
    }

    Ok(String::from_utf8(output.stdout)?)
}

pub fn get_secret_yaml(namespace: &str, secret_name: &str) -> Result<String> {
    let output = Command::new("kubectl")
        .args(&["get", "secret", "-n", namespace, secret_name, "-o", "yaml"])
        .output()?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("kubectl failed: {}", error));
    }

    Ok(String::from_utf8(output.stdout)?)
}

pub fn get_pvc_yaml(namespace: &str, pvc_name: &str) -> Result<String> {
    let output = Command::new("kubectl")
        .args(&["get", "pvc", "-n", namespace, pvc_name, "-o", "yaml"])
        .output()?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("kubectl failed: {}", error));
    }

    Ok(String::from_utf8(output.stdout)?)
}

pub fn get_pv_yaml(pv_name: &str) -> Result<String> {
    let output = Command::new("kubectl")
        .args(&["get", "pv", pv_name, "-o", "yaml"])
        .output()?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("kubectl failed: {}", error));
    }

    Ok(String::from_utf8(output.stdout)?)
}

// 资源监控相关命令
pub fn get_top_pods(namespace: &str) -> Result<String> {
    let output = Command::new("kubectl")
        .args(&["top", "pods", "-n", namespace, "--no-headers"])
        .output()?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("kubectl top failed: {}. Note: metrics-server might not be installed.", error));
    }

    Ok(String::from_utf8(output.stdout)?)
}

pub fn get_top_pod(namespace: &str, pod_name: &str) -> Result<String> {
    let output = Command::new("kubectl")
        .args(&["top", "pod", "-n", namespace, pod_name, "--containers", "--no-headers"])
        .output()?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("kubectl top failed: {}. Note: metrics-server might not be installed.", error));
    }

    Ok(String::from_utf8(output.stdout)?)
}