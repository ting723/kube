//! Handler for detail mode key events (Describe, YamlView, TopView)

use crossterm::event::{KeyCode, KeyEvent};
use super::super::state::{AppState, AppMode};
use anyhow::Result;

/// Handle key events in detail modes (Describe, YamlView, TopView)
pub fn handle_detail_mode_key(app: &mut AppState, key_event: KeyEvent) -> Result<()> {
    match key_event.code {
        // Scroll operations
        KeyCode::Down | KeyCode::Char('j') => app.scroll_down(),
        KeyCode::Up | KeyCode::Char('k') => app.scroll_up(),
        KeyCode::PageDown => app.scroll_page_down(),
        KeyCode::PageUp => app.scroll_page_up(),

        // Exit detail view
        KeyCode::Esc => {
            app.reset_scroll();
            app.mode = app.get_previous_mode();
        }

        // Mouse mode toggle
        KeyCode::Char('M') | KeyCode::Char('m') => app.toggle_mouse_mode(),

        // Refresh
        KeyCode::Char('R') => {
            match app.mode {
                AppMode::Describe => app.toggle_describe_refresh(),
                AppMode::YamlView => app.toggle_yaml_refresh(),
                _ => app.force_refresh_current_mode(),
            }
        }

        // Language toggle
        KeyCode::Char('I') | KeyCode::Char('i') => app.toggle_language(),

        _ => {}
    }
    Ok(())
}