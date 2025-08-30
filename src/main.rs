mod app;
mod events;
mod kubectl;
mod ui;

use anyhow::Result;
use app::{AppState, AppMode};
use crossterm::{
    event::Event,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use kubectl::KubectlClient;
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::{
    io,
    time::Duration,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize error handling
    color_eyre::install().expect("Failed to install color-eyre");

    // Check if kubectl is available
    let client = KubectlClient::new();
    if !client.check_available() {
        eprintln!("Error: kubectl is not available in PATH");
        eprintln!("Please install kubectl and ensure it's configured to access your cluster");
        eprintln!("");
        eprintln!("Troubleshooting tips:");
        eprintln!("1. Check if kubectl is installed: which kubectl");
        eprintln!("2. Test kubectl manually: kubectl version");
        eprintln!("3. Verify cluster access: kubectl cluster-info");
        std::process::exit(1);
    }

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    // 不启用完整的鼠标捕获，允许文本选中操作
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = AppState::new();
    
    // Load initial data
    if let Err(e) = load_initial_data(&mut app, &client).await {
        eprintln!("Failed to load initial data: {}", e);
    }

    // Main loop
    let result = run_app(&mut terminal, &mut app, &client).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;

    if let Err(err) = result {
        eprintln!("Application error: {}", err);
    }

    Ok(())
}

async fn load_initial_data(app: &mut AppState, client: &KubectlClient) -> Result<()> {
    // 显示cluster-info命令
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

async fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut AppState,
    client: &KubectlClient,
) -> Result<()> {
    loop {
        // Render UI
        terminal.draw(|f| ui::render_ui(f, app))?;

        // Handle events
        if let Some(event) = events::poll_events(Duration::from_millis(100))? {
            match event {
                Event::Key(key_event) => {
                    app.handle_key_event(key_event)?;
                    
                    // 处理待执行的exec命令
                    if let Some(exec_cmd) = app.pending_exec.take() {
                        execute_external_command(&exec_cmd, terminal).await?;
                        app.clear_current_command();
                        
                        // 在exec后强制刷新界面和数据
                        app.logs.clear();
                        app.describe_content.clear();
                        
                        // 强制返回到PodList模式并重新加载所有数据
                        app.mode = AppMode::PodList;
                        app.previous_mode = AppMode::PodList;
                        
                        // 重新加载所有必要的数据
                        if let Ok(pods) = client.get_pods(&app.current_namespace).await {
                            app.pods = pods;
                        }
                        if let Ok(services) = client.get_services(&app.current_namespace).await {
                            app.services = services;
                        }
                        if let Ok(deployments) = client.get_deployments(&app.current_namespace).await {
                            app.deployments = deployments;
                        }
                        if let Ok(jobs) = client.get_jobs(&app.current_namespace).await {
                            app.jobs = jobs;
                        }
                        
                        app.refresh_data();
                        
                        // 强制重绘界面确保显示正常
                        terminal.draw(|f| ui::render_ui(f, app))?;
                    }
                    
                    // Handle mode changes that require data loading
                    match app.mode {
                        AppMode::PodList => {
                            if app.pods.is_empty() || app.should_refresh() {
                                if let Ok(pods) = client.get_pods(&app.current_namespace).await {
                                    app.pods = pods;
                                    app.refresh_data();
                                }
                            }
                        }
                        AppMode::ServiceList => {
                            if app.services.is_empty() || app.should_refresh() {
                                if let Ok(services) = client.get_services(&app.current_namespace).await {
                                    app.services = services;
                                    app.refresh_data();
                                }
                            }
                        }
                        AppMode::DeploymentList => {
                            if app.deployments.is_empty() || app.should_refresh() {
                                if let Ok(deployments) = client.get_deployments(&app.current_namespace).await {
                                    app.deployments = deployments;
                                    app.refresh_data();
                                }
                            }
                        }
                        AppMode::JobList => {
                            if app.jobs.is_empty() || app.should_refresh() {
                                if let Ok(jobs) = client.get_jobs(&app.current_namespace).await {
                                    app.jobs = jobs;
                                    app.refresh_data();
                                }
                            }
                        }
                        AppMode::DaemonSetList => {
                            if app.daemonsets.is_empty() || app.should_refresh() {
                                if let Ok(daemonsets) = client.get_daemonsets(&app.current_namespace).await {
                                    app.daemonsets = daemonsets;
                                    app.refresh_data();
                                }
                            }
                        }
                        AppMode::PVCList => {
                            if app.pvcs.is_empty() || app.should_refresh() {
                                if let Ok(pvcs) = client.get_pvcs(&app.current_namespace).await {
                                    app.pvcs = pvcs;
                                    app.refresh_data();
                                }
                            }
                        }
                        AppMode::PVList => {
                            if app.pvs.is_empty() || app.should_refresh() {
                                if let Ok(pvs) = client.get_pvs().await {
                                    app.pvs = pvs;
                                    app.refresh_data();
                                }
                            }
                        }
                        AppMode::ConfigMapList => {
                            if app.configmaps.is_empty() || app.should_refresh() {
                                if let Ok(configmaps) = client.get_configmaps(&app.current_namespace).await {
                                    app.configmaps = configmaps;
                                    app.refresh_data();
                                }
                            }
                        }
                        AppMode::SecretList => {
                            if app.secrets.is_empty() || app.should_refresh() {
                                if let Ok(secrets) = client.get_secrets(&app.current_namespace).await {
                                    app.secrets = secrets;
                                    app.refresh_data();
                                }
                            }
                        }
                        AppMode::Logs => {
                            if let Some(pod) = app.get_selected_pod() {
                                if app.logs.is_empty() || app.should_refresh() {
                                    let pod_name = pod.name.clone();
                                    let namespace = app.current_namespace.clone();
                                    app.set_current_command(&format!("kubectl logs -f -n {} {} --tail=100", namespace, pod_name));
                                    if let Ok(logs) = client.get_pod_logs(&namespace, &pod_name, 100).await {
                                        app.logs = logs;
                                        // 如果开启了自动滚动，滚动到最新位置
                                        if app.logs_auto_scroll {
                                            app.logs_scroll = app.logs.len().saturating_sub(1);
                                        }
                                    }
                                    app.clear_current_command();
                                }
                            }
                        }
                        AppMode::Describe => {
                            if app.describe_content.is_empty() {
                                match app.previous_mode {
                                    AppMode::PodList => {
                                        if let Some(pod) = app.get_selected_pod() {
                                            let pod_name = pod.name.clone();
                                            let namespace = app.current_namespace.clone();
                                            app.set_current_command(&format!("kubectl describe pod -n {} {}", namespace, pod_name));
                                            if let Ok(description) = client.describe_pod(&namespace, &pod_name).await {
                                                app.describe_content = description;
                                            }
                                            app.clear_current_command();
                                        }
                                    }
                                    AppMode::ServiceList => {
                                        if let Some(service) = app.get_selected_service() {
                                            let service_name = service.name.clone();
                                            let namespace = app.current_namespace.clone();
                                            app.set_current_command(&format!("kubectl describe service -n {} {}", namespace, service_name));
                                            if let Ok(description) = client.describe_service(&namespace, &service_name).await {
                                                app.describe_content = description;
                                            }
                                            app.clear_current_command();
                                        }
                                    }
                                    AppMode::DeploymentList => {
                                        if let Some(deployment) = app.get_selected_deployment() {
                                            let deployment_name = deployment.name.clone();
                                            let namespace = app.current_namespace.clone();
                                            app.set_current_command(&format!("kubectl describe deployment -n {} {}", namespace, deployment_name));
                                            if let Ok(description) = client.describe_deployment(&namespace, &deployment_name).await {
                                                app.describe_content = description;
                                            }
                                            app.clear_current_command();
                                        }
                                    }
                                    AppMode::JobList => {
                                        if let Some(job) = app.get_selected_job() {
                                            let job_name = job.name.clone();
                                            let namespace = app.current_namespace.clone();
                                            app.set_current_command(&format!("kubectl describe job -n {} {}", namespace, job_name));
                                            if let Ok(description) = client.describe_job(&namespace, &job_name).await {
                                                app.describe_content = description;
                                            }
                                            app.clear_current_command();
                                        }
                                    }
                                    AppMode::DaemonSetList => {
                                        if let Some(daemonset) = app.get_selected_daemonset() {
                                            let daemonset_name = daemonset.name.clone();
                                            let namespace = app.current_namespace.clone();
                                            app.set_current_command(&format!("kubectl describe daemonset -n {} {}", namespace, daemonset_name));
                                            if let Ok(description) = client.describe_daemonset(&namespace, &daemonset_name).await {
                                                app.describe_content = description;
                                            }
                                            app.clear_current_command();
                                        }
                                    }
                                    AppMode::NodeList => {
                                        if let Some(node) = app.get_selected_node() {
                                            let node_name = node.name.clone();
                                            app.set_current_command(&format!("kubectl describe node {}", node_name));
                                            if let Ok(description) = client.describe_node(&node_name).await {
                                                app.describe_content = description;
                                            }
                                            app.clear_current_command();
                                        }
                                    }
                                    AppMode::ConfigMapList => {
                                        if let Some(configmap) = app.get_selected_configmap() {
                                            let configmap_name = configmap.name.clone();
                                            let namespace = app.current_namespace.clone();
                                            app.set_current_command(&format!("kubectl describe configmap -n {} {}", namespace, configmap_name));
                                            if let Ok(description) = client.describe_configmap(&namespace, &configmap_name).await {
                                                app.describe_content = description;
                                            }
                                            app.clear_current_command();
                                        }
                                    }
                                    AppMode::SecretList => {
                                        if let Some(secret) = app.get_selected_secret() {
                                            let secret_name = secret.name.clone();
                                            let namespace = app.current_namespace.clone();
                                            app.set_current_command(&format!("kubectl describe secret -n {} {}", namespace, secret_name));
                                            if let Ok(description) = client.describe_secret(&namespace, &secret_name).await {
                                                app.describe_content = description;
                                            }
                                            app.clear_current_command();
                                        }
                                    }
                                    AppMode::PVCList => {
                                        if let Some(pvc) = app.get_selected_pvc() {
                                            let pvc_name = pvc.name.clone();
                                            let namespace = app.current_namespace.clone();
                                            app.set_current_command(&format!("kubectl describe pvc -n {} {}", namespace, pvc_name));
                                            if let Ok(description) = client.describe_pvc(&namespace, &pvc_name).await {
                                                app.describe_content = description;
                                            }
                                            app.clear_current_command();
                                        }
                                    }
                                    AppMode::PVList => {
                                        if let Some(pv) = app.get_selected_pv() {
                                            let pv_name = pv.name.clone();
                                            app.set_current_command(&format!("kubectl describe pv {}", pv_name));
                                            if let Ok(description) = client.describe_pv(&pv_name).await {
                                                app.describe_content = description;
                                            }
                                            app.clear_current_command();
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
                    }
                }
                Event::Resize(_, _) => {
                    // Terminal was resized, will be handled by next render
                }
                _ => {}
            }
        }

        // Auto-refresh data
        if app.should_refresh() {
            match app.mode {
                AppMode::PodList => {
                    app.set_current_command(&format!("kubectl get pods -n {}", app.current_namespace));
                    if let Ok(pods) = client.get_pods(&app.current_namespace).await {
                        app.pods = pods;
                    }
                    app.clear_current_command();
                }
                AppMode::ServiceList => {
                    app.set_current_command(&format!("kubectl get services -n {}", app.current_namespace));
                    if let Ok(services) = client.get_services(&app.current_namespace).await {
                        app.services = services;
                    }
                    app.clear_current_command();
                }
                AppMode::DeploymentList => {
                    app.set_current_command(&format!("kubectl get deployments -n {}", app.current_namespace));
                    if let Ok(deployments) = client.get_deployments(&app.current_namespace).await {
                        app.deployments = deployments;
                    }
                    app.clear_current_command();
                }
                AppMode::JobList => {
                    app.set_current_command(&format!("kubectl get jobs -n {}", app.current_namespace));
                    if let Ok(jobs) = client.get_jobs(&app.current_namespace).await {
                        app.jobs = jobs;
                    }
                    app.clear_current_command();
                }
                AppMode::DaemonSetList => {
                    app.set_current_command(&format!("kubectl get daemonsets -n {}", app.current_namespace));
                    if let Ok(daemonsets) = client.get_daemonsets(&app.current_namespace).await {
                        app.daemonsets = daemonsets;
                    }
                    app.clear_current_command();
                }
                AppMode::PVCList => {
                    app.set_current_command(&format!("kubectl get pvc -n {}", app.current_namespace));
                    if let Ok(pvcs) = client.get_pvcs(&app.current_namespace).await {
                        app.pvcs = pvcs;
                    }
                    app.clear_current_command();
                }
                AppMode::PVList => {
                    app.set_current_command("kubectl get pv");
                    if let Ok(pvs) = client.get_pvs().await {
                        app.pvs = pvs;
                    }
                    app.clear_current_command();
                }
                AppMode::ConfigMapList => {
                    app.set_current_command(&format!("kubectl get configmaps -n {}", app.current_namespace));
                    if let Ok(configmaps) = client.get_configmaps(&app.current_namespace).await {
                        app.configmaps = configmaps;
                    }
                    app.clear_current_command();
                }
                AppMode::SecretList => {
                    app.set_current_command(&format!("kubectl get secrets -n {}", app.current_namespace));
                    if let Ok(secrets) = client.get_secrets(&app.current_namespace).await {
                        app.secrets = secrets;
                    }
                    app.clear_current_command();
                }
                AppMode::Logs => {
                    // 日志自动刷新
                    if let Some(pod) = app.get_selected_pod() {
                        if let Ok(logs) = client.get_pod_logs(&app.current_namespace, &pod.name, 100).await {
                            app.logs = logs;
                            if app.logs_auto_scroll {
                                app.logs_scroll = app.logs.len().saturating_sub(1);
                            }
                        }
                    }
                    app.refresh_logs();
                }
                _ => {}
            }
            app.refresh_data();
        }

        // Auto-refresh logs if needed (tail -f effect)
        if app.should_refresh_logs() {
            if let Some(pod) = app.get_selected_pod() {
                let pod_name = pod.name.clone();
                let namespace = app.current_namespace.clone();
                app.set_current_command(&format!("kubectl logs -f -n {} {} --tail=100 (auto-refresh)", namespace, pod_name));
                if let Ok(logs) = client.get_pod_logs(&namespace, &pod_name, 100).await {
                    app.logs = logs;
                    if app.logs_auto_scroll {
                        app.logs_scroll = app.logs.len().saturating_sub(1);
                    }
                }
                app.clear_current_command();
            }
            app.refresh_logs();
        }

        if app.should_quit {
            break;
        }
    }

    Ok(())
}

// 执行外部命令（如kubectl exec）
async fn execute_external_command(
    command: &str,
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
) -> Result<()> {
    // 退出TUI模式
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    // 执行命令
    println!("Executing: {}", command);
    
    let status = std::process::Command::new("sh")
        .arg("-c")
        .arg(command)
        .status();

    let is_success = match &status {
        Ok(exit_status) => exit_status.success(),
        Err(_) => false,
    };

    match status {
        Ok(exit_status) => {
            if exit_status.success() {
                println!("Command executed successfully.");
            } else {
                println!("Command failed with exit code: {:?}", exit_status.code());
            }
        }
        Err(e) => {
            println!("Failed to execute command: {}", e);
        }
    }

    // 智能处理：根据命令结果提供不同的用户体验
    if !is_success {
        println!("\n=== Command completed with issues ===");
        println!("The exec session ended. This might be normal if:");
        println!("- You typed 'exit' to leave the container");
        println!("- The container doesn't have the requested command");
        println!("- The shell environment is limited");
        println!("\nPress Enter to return to Kube TUI...");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
    } else {
        // 成功执行后等待1秒，然后自动返回
        println!("\nReturning to application in 1 second...");
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }

    // 重新进入TUI模式并强制刷新
    enable_raw_mode()?;
    execute!(terminal.backend_mut(), EnterAlternateScreen)?;
    terminal.hide_cursor()?;
    terminal.clear()?; // 强制清屏
    
    // 确保终端完全恢复
    std::thread::sleep(std::time::Duration::from_millis(100));

    Ok(())
}
