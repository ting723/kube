//! Handler for list mode key events (Namespace, Pod, Service, etc.)

use crossterm::event::{KeyCode, KeyEvent};
use super::super::state::{AppState};
use anyhow::Result;

/// Handle key events in list modes
pub fn handle_list_mode_key(app: &mut AppState, key_event: KeyEvent) -> Result<()> {
    match key_event.code {
        // Navigation
        KeyCode::Down | KeyCode::Char('j') => app.move_selection_down(),
        KeyCode::Up | KeyCode::Char('k') => app.move_selection_up(),
        KeyCode::PageDown => app.scroll_page_down(),
        KeyCode::PageUp => app.scroll_page_up(),

        // Panel switching
        KeyCode::Tab => app.switch_panel_right(),
        KeyCode::BackTab => app.switch_panel_left(),
        KeyCode::Char('h') | KeyCode::Left => app.handle_left_navigation(),
        KeyCode::Char('l') | KeyCode::Right => app.handle_right_navigation(),

        // Actions
        KeyCode::Enter => app.handle_enter(),
        KeyCode::Char(' ') => app.handle_describe(),
        KeyCode::Char('L') => app.handle_logs(),
        KeyCode::Char('D') => app.handle_delete(),
        KeyCode::Char('E') => app.handle_exec(),
        KeyCode::Char('Y') => app.handle_yaml_view(),
        KeyCode::Char('T') => app.handle_top_view(),

        // Search
        KeyCode::Char('/') => app.start_search(),
        KeyCode::Char('n') => app.search_next(),
        KeyCode::Char('N') => app.search_previous(),

        // Refresh
        KeyCode::Char('R') => app.force_refresh_current_mode(),
        KeyCode::Char('A') => app.toggle_global_refresh(),

        // Language toggle
        KeyCode::Char('I') | KeyCode::Char('i') => app.toggle_language(),

        _ => {}
    }
    Ok(())
}