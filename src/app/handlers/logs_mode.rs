//! Handler for logs mode key events

use crossterm::event::{KeyCode, KeyEvent};
use super::super::state::{AppState, AppMode};
use anyhow::Result;

/// Handle key events in logs mode
pub fn handle_logs_mode_key(app: &mut AppState, key_event: KeyEvent) -> Result<()> {
    match key_event.code {
        // Scroll operations
        KeyCode::Down | KeyCode::Char('j') => app.scroll_down(),
        KeyCode::Up | KeyCode::Char('k') => app.scroll_up(),
        KeyCode::PageDown => app.scroll_page_down(),
        KeyCode::PageUp => app.scroll_page_up(),

        // Exit logs view
        KeyCode::Esc => {
            app.reset_scroll();
            app.mode = app.get_previous_mode();
        }

        // Auto-scroll toggle
        KeyCode::Char('A') => {
            app.logs_auto_scroll = !app.logs_auto_scroll;
        }

        // Refresh toggle
        KeyCode::Char('R') => {
            app.logs_auto_refresh = !app.logs_auto_refresh;
            app.update_refresh_status();
        }

        // Mouse mode toggle
        KeyCode::Char('M') | KeyCode::Char('m') => app.toggle_mouse_mode(),

        // Language toggle
        KeyCode::Char('I') | KeyCode::Char('i') => app.toggle_language(),

        _ => {}
    }
    Ok(())
}