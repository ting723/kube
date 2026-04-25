//! Handler for batch mode key events

use crossterm::event::{KeyCode, KeyEvent};
use super::super::state::{AppState};
use anyhow::Result;

/// Handle key events in batch mode
pub fn handle_batch_mode_key(app: &mut AppState, key_event: KeyEvent) -> Result<()> {
    match key_event.code {
        // Navigation
        KeyCode::Down | KeyCode::Char('j') => {
            if app.selected_batch_index + 1 < app.batch_items.len() {
                app.selected_batch_index += 1;
            }
        }
        KeyCode::Up | KeyCode::Char('k') => {
            if app.selected_batch_index > 0 {
                app.selected_batch_index -= 1;
            }
        }

        // Toggle selection
        KeyCode::Char(' ') => {
            if app.selected_batch_index < app.batch_items.len() {
                if app.selected_batch_items.contains(&app.selected_batch_index) {
                    app.selected_batch_items.remove(&app.selected_batch_index);
                } else {
                    app.selected_batch_items.insert(app.selected_batch_index);
                }
            }
        }

        // Select all
        KeyCode::Char('a') => {
            for i in 0..app.batch_items.len() {
                app.selected_batch_items.insert(i);
            }
        }

        // Clear selection
        KeyCode::Char('c') => {
            app.selected_batch_items.clear();
        }

        // Execute batch action
        KeyCode::Enter => {
            // TODO: Implement batch execution
        }

        // Exit batch mode
        KeyCode::Esc => {
            app.batch_mode = false;
            app.selected_batch_items.clear();
        }

        _ => {}
    }
    Ok(())
}