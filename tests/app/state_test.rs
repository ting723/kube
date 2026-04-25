use kube_tui::app::state::{AppState, ConfirmAction, AppMode};
use std::collections::{HashMap, HashSet};
use std::time::Instant;

#[test]
fn test_default_app_state() {
    let state = AppState::default();
    assert_eq!(state.mode, AppMode::NamespaceList);
    assert!(!state.should_quit);
    assert_eq!(state.current_namespace, "default");
    assert!(state.global_refresh_enabled);
    assert!(!state.batch_mode);
    assert!(state.favorite_namespaces.is_empty());
}

#[test]
fn test_should_refresh() {
    let mut state = AppState::default();
    state.refresh_interval = std::time::Duration::from_millis(100);
    std::thread::sleep(std::time::Duration::from_millis(150));
    assert!(state.should_refresh());
}

#[test]
fn test_toggle_global_refresh() {
    let mut state = AppState::default();
    let original = state.global_refresh_enabled;
    state.toggle_global_refresh();
    assert_eq!(state.global_refresh_enabled, !original);
}

#[test]
fn test_toggle_mouse_mode() {
    let mut state = AppState::default();
    let original = state.text_selection_mode;
    state.toggle_mouse_mode();
    assert_eq!(state.text_selection_mode, !original);
}

#[test]
fn test_toggle_language() {
    let mut state = AppState::default();
    let original = state.language_chinese;
    state.toggle_language();
    assert_eq!(state.language_chinese, !original);
}

#[test]
fn test_save_restore_position() {
    let mut state = AppState::default();
    state.mode = AppMode::PodList;
    state.selected_pod_index = 5;
    state.save_selected_position();
    state.selected_pod_index = 0;
    state.restore_selected_position();
    assert_eq!(state.selected_pod_index, 5);
}

#[test]
fn test_toggle_favorite_namespace() {
    let mut state = AppState::default();
    state.namespaces = vec!["default".to_string(), "kube-system".to_string()];
    state.selected_namespace_index = 0;
    assert!(!state.is_current_namespace_favorite());
    state.toggle_favorite_namespace();
    assert!(state.is_current_namespace_favorite());
    state.toggle_favorite_namespace();
    assert!(!state.is_current_namespace_favorite());
}

#[test]
fn test_command_history() {
    let mut state = AppState::default();
    assert!(state.command_history.is_empty());

    state.command_history.push("kubectl get pods".to_string());
    state.command_history.push("kubectl get services".to_string());

    assert_eq!(state.command_history.len(), 2);
    assert_eq!(state.command_history[0], "kubectl get pods");
    assert_eq!(state.command_history[1], "kubectl get services");
}

#[test]
fn test_batch_mode() {
    let mut state = AppState::default();
    assert!(!state.batch_mode);

    state.batch_mode = true;
    assert!(state.batch_mode);

    state.selected_batch_items.insert(0);
    state.selected_batch_items.insert(1);

    assert_eq!(state.selected_batch_items.len(), 2);
    assert!(state.selected_batch_items.contains(&0));
    assert!(state.selected_batch_items.contains(&1));
}

#[test]
fn test_log_search_functionality() {
    let mut state = AppState::default();

    state.log_search_query = "error".to_string();
    assert_eq!(state.log_search_query, "error");

    state.log_search_results = vec![0, 2, 4];
    state.current_log_search_index = 1;

    assert_eq!(state.log_search_results.len(), 3);
    assert_eq!(state.current_log_search_index, 1);
    assert_eq!(state.log_search_results[1], 4);
}

#[test]
fn test_should_enable_mouse_capture() {
    let mut state = AppState::default();

    // Test Logs mode
    state.mode = AppMode::Logs;
    state.text_selection_mode = false;
    assert!(state.should_enable_mouse_capture());

    state.text_selection_mode = true;
    assert!(!state.should_enable_mouse_capture());

    // Test Describe mode
    state.mode = AppMode::Describe;
    state.text_selection_mode = false;
    assert!(state.should_enable_mouse_capture());

    // Test YamlView mode
    state.mode = AppMode::YamlView;
    state.text_selection_mode = false;
    assert!(state.should_enable_mouse_capture());

    // Test TopView mode
    state.mode = AppMode::TopView;
    state.text_selection_mode = false;
    assert!(state.should_enable_mouse_capture());

    // Test other modes should return false
    state.mode = AppMode::PodList;
    assert!(!state.should_enable_mouse_capture());
}

#[test]
fn test_confirm_action_delete_batch() {
    let action = ConfirmAction::DeleteBatch {
        items: vec![
            ("default".to_string(), "Pod".to_string(), "test-pod".to_string()),
            ("kube-system".to_string(), "Deployment".to_string(), "kube-dns".to_string()),
        ]
    };

    match action {
        ConfirmAction::DeleteBatch { items } => {
            assert_eq!(items.len(), 2);
            assert_eq!(items[0].0, "default");
            assert_eq!(items[0].1, "Pod");
            assert_eq!(items[0].2, "test-pod");
            assert_eq!(items[1].0, "kube-system");
            assert_eq!(items[1].1, "Deployment");
            assert_eq!(items[1].2, "kube-dns");
        }
        _ => panic!("Expected DeleteBatch variant"),
    }
}

#[test]
fn test_app_mode_command_history() {
    // Verify CommandHistory variant exists
    let _mode = AppMode::CommandHistory;

    // Test that all variants can be matched
    let mode = AppMode::CommandHistory;
    let is_command_history = matches!(mode, AppMode::CommandHistory);
    assert!(is_command_history);
}
