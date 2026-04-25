//! Data loading functions for different app modes

use anyhow::Result;
use super::state::{AppState, AppMode};
use crate::kubectl::KubectlClient;

/// Load initial data when app starts
pub async fn load_initial_data(app: &mut AppState, client: &KubectlClient) -> Result<()> {
    app.set_current_command("kubectl cluster-info");

    // Load namespaces
    app.set_current_command("kubectl get namespaces");
    if let Ok(namespaces) = client.get_namespaces().await {
        app.namespaces = namespaces.into_iter().map(|ns| ns.name).collect();
        if !app.namespaces.is_empty() {
            app.current_namespace = app.namespaces[0].clone();
        }
    }
    app.clear_current_command();

    // Load pods for default namespace
    if let Ok(pods) = client.get_pods(&app.current_namespace).await {
        app.pods = pods;
    }

    // Load services for default namespace
    if let Ok(services) = client.get_services(&app.current_namespace).await {
        app.services = services;
    }

    app.refresh_data();
    Ok(())
}

/// Refresh data based on current mode
pub async fn refresh_current_mode(app: &mut AppState, client: &KubectlClient) -> Result<()> {
    match app.mode {
        AppMode::NamespaceList => {
            load_namespaces(app, client).await;
        }
        AppMode::PodList => {
            load_pods(app, client).await;
        }
        AppMode::ServiceList => {
            load_services(app, client).await;
        }
        AppMode::DeploymentList => {
            load_deployments(app, client).await;
        }
        AppMode::JobList => {
            load_jobs(app, client).await;
        }
        AppMode::DaemonSetList => {
            load_daemonsets(app, client).await;
        }
        AppMode::ConfigMapList => {
            load_configmaps(app, client).await;
        }
        AppMode::SecretList => {
            load_secrets(app, client).await;
        }
        AppMode::PVCList => {
            load_pvcs(app, client).await;
        }
        AppMode::PVList => {
            load_pvs(app, client).await;
        }
        AppMode::NodeList => {
            load_nodes(app, client).await;
        }
        AppMode::TopView => {
            load_pod_metrics(app, client).await;
        }
        _ => {}
    }
    Ok(())
}

/// Load namespaces
pub async fn load_namespaces(app: &mut AppState, client: &KubectlClient) {
    app.set_current_command("kubectl get namespaces");
    if let Ok(namespaces) = client.get_namespaces().await {
        app.namespaces = namespaces.into_iter().map(|ns| ns.name).collect();
        if !app.namespaces.is_empty() && app.current_namespace.is_empty() {
            app.current_namespace = app.namespaces[0].clone();
        }
    }
    app.clear_current_command();
}

/// Load pods for current namespace
pub async fn load_pods(app: &mut AppState, client: &KubectlClient) {
    app.set_current_command(&format!("kubectl get pods -n {}", app.current_namespace));
    if let Ok(pods) = client.get_pods(&app.current_namespace).await {
        app.pods = pods;
    }
    app.clear_current_command();
}

/// Load services for current namespace
pub async fn load_services(app: &mut AppState, client: &KubectlClient) {
    app.set_current_command(&format!("kubectl get services -n {}", app.current_namespace));
    if let Ok(services) = client.get_services(&app.current_namespace).await {
        app.services = services;
    }
    app.clear_current_command();
}

/// Load deployments for current namespace
pub async fn load_deployments(app: &mut AppState, client: &KubectlClient) {
    app.set_current_command(&format!("kubectl get deployments -n {}", app.current_namespace));
    if let Ok(deployments) = client.get_deployments(&app.current_namespace).await {
        app.deployments = deployments;
    }
    app.clear_current_command();
}

/// Load jobs for current namespace
pub async fn load_jobs(app: &mut AppState, client: &KubectlClient) {
    app.set_current_command(&format!("kubectl get jobs -n {}", app.current_namespace));
    if let Ok(jobs) = client.get_jobs(&app.current_namespace).await {
        app.jobs = jobs;
    }
    app.clear_current_command();
}

/// Load daemonsets for current namespace
pub async fn load_daemonsets(app: &mut AppState, client: &KubectlClient) {
    app.set_current_command(&format!("kubectl get daemonsets -n {}", app.current_namespace));
    if let Ok(daemonsets) = client.get_daemonsets(&app.current_namespace).await {
        app.daemonsets = daemonsets;
    }
    app.clear_current_command();
}

/// Load configmaps for current namespace
pub async fn load_configmaps(app: &mut AppState, client: &KubectlClient) {
    app.set_current_command(&format!("kubectl get configmaps -n {}", app.current_namespace));
    if let Ok(configmaps) = client.get_configmaps(&app.current_namespace).await {
        app.configmaps = configmaps;
    }
    app.clear_current_command();
}

/// Load secrets for current namespace
pub async fn load_secrets(app: &mut AppState, client: &KubectlClient) {
    app.set_current_command(&format!("kubectl get secrets -n {}", app.current_namespace));
    if let Ok(secrets) = client.get_secrets(&app.current_namespace).await {
        app.secrets = secrets;
    }
    app.clear_current_command();
}

/// Load PVCs for current namespace
pub async fn load_pvcs(app: &mut AppState, client: &KubectlClient) {
    app.set_current_command(&format!("kubectl get pvc -n {}", app.current_namespace));
    if let Ok(pvcs) = client.get_pvcs(&app.current_namespace).await {
        app.pvcs = pvcs;
    }
    app.clear_current_command();
}

/// Load PVs
pub async fn load_pvs(app: &mut AppState, client: &KubectlClient) {
    app.set_current_command("kubectl get pv");
    if let Ok(pvs) = client.get_pvs().await {
        app.pvs = pvs;
    }
    app.clear_current_command();
}

/// Load nodes
pub async fn load_nodes(app: &mut AppState, client: &KubectlClient) {
    app.set_current_command("kubectl get nodes");
    if let Ok(nodes) = client.get_nodes().await {
        app.nodes = nodes;
    }
    app.clear_current_command();
}

/// Load pod metrics for top view
pub async fn load_pod_metrics(app: &mut AppState, client: &KubectlClient) {
    app.set_current_command(&format!("kubectl top pods -n {}", app.current_namespace));
    if let Ok(metrics) = client.get_pod_metrics(&app.current_namespace).await {
        app.pod_metrics = metrics;
    }
    app.clear_current_command();
}

/// Load logs for selected pod
pub async fn load_logs(app: &mut AppState, client: &KubectlClient) {
    if let Some(pod) = app.pods.get(app.selected_pod_index) {
        let pod_name = pod.name.clone();
        app.set_current_command(&format!("kubectl logs -n {} {}", app.current_namespace, pod_name));
        if let Ok(logs) = client.get_pod_logs(&app.current_namespace, &pod_name, 100).await {
            app.logs = logs;
        }
    }
    app.clear_current_command();
}

/// Load describe content for selected pod
pub async fn load_describe_pod(app: &mut AppState, client: &KubectlClient) {
    if let Some(pod) = app.pods.get(app.selected_pod_index) {
        let pod_name = pod.name.clone();
        app.set_current_command(&format!("kubectl describe pod {} -n {}", pod_name, app.current_namespace));
        if let Ok(content) = client.describe_pod(&app.current_namespace, &pod_name).await {
            app.describe_content = content;
            app.describe_lines_cache = app.describe_content.lines().map(|s| s.to_string()).collect();
        }
    }
    app.clear_current_command();
}

/// Load describe content for selected service
pub async fn load_describe_service(app: &mut AppState, client: &KubectlClient) {
    if let Some(service) = app.services.get(app.selected_service_index) {
        let service_name = service.name.clone();
        app.set_current_command(&format!("kubectl describe service {} -n {}", service_name, app.current_namespace));
        if let Ok(content) = client.describe_service(&app.current_namespace, &service_name).await {
            app.describe_content = content;
            app.describe_lines_cache = app.describe_content.lines().map(|s| s.to_string()).collect();
        }
    }
    app.clear_current_command();
}

/// Load YAML content for selected resource
pub async fn load_yaml(app: &mut AppState, client: &KubectlClient, resource_type: &str, name: &str) {
    app.set_current_command(&format!("kubectl get {} {} -n {} -o yaml", resource_type, name, app.current_namespace));
    if let Ok(content) = client.get_yaml(resource_type, Some(&app.current_namespace), name).await {
        app.yaml_content = content;
        app.yaml_lines_cache = app.yaml_content.lines().map(|s| s.to_string()).collect();
    }
    app.clear_current_command();
}