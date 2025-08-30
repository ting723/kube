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
    // Load namespaces
    if let Ok(namespaces) = client.get_namespaces().await {
        app.namespaces = namespaces.into_iter().map(|ns| ns.name).collect();
        if !app.namespaces.is_empty() {
            app.current_namespace = app.namespaces[0].clone();
        }
    }

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
                        AppMode::Logs => {
                            if let Some(pod) = app.get_selected_pod() {
                                if app.logs.is_empty() {
                                    if let Ok(logs) = client.get_pod_logs(&app.current_namespace, &pod.name, 100).await {
                                        app.logs = logs;
                                    }
                                }
                            }
                        }
                        AppMode::Describe => {
                            if let Some(pod) = app.get_selected_pod() {
                                if app.describe_content.is_empty() {
                                    if let Ok(description) = client.describe_pod(&app.current_namespace, &pod.name).await {
                                        app.describe_content = description;
                                    }
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
                    if let Ok(pods) = client.get_pods(&app.current_namespace).await {
                        app.pods = pods;
                    }
                }
                AppMode::ServiceList => {
                    if let Ok(services) = client.get_services(&app.current_namespace).await {
                        app.services = services;
                    }
                }
                _ => {}
            }
            app.refresh_data();
        }

        if app.should_quit {
            break;
        }
    }

    Ok(())
}
