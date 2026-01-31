//! Button Polling Module
//!
//! Handles the background thread that polls the Stream Deck for button presses
//! and executes configured actions.

use std::thread;
use std::time::Duration;

use serde_json::json;
use tauri::{AppHandle, Emitter, Manager};

use crate::actions::ActionRegistry;
use crate::hid::constants::BUTTON_COUNT;
use crate::AppState;

/// Start the button polling loop in a background thread.
///
/// This spawns a thread that continuously reads button states from the Stream Deck,
/// detects button presses (rising edges), and executes the configured actions.
pub fn start_polling(app_handle: AppHandle) {
    thread::spawn(move || {
        polling_loop(app_handle);
    });
}

/// The main polling loop.
///
/// Runs until the Stream Deck is disconnected (state.streamdeck becomes None).
fn polling_loop(app_handle: AppHandle) {
    let mut prev_states = [false; BUTTON_COUNT];

    loop {
        // Get access to state and registry through the app handle
        let state = app_handle.state::<AppState>();
        let registry = app_handle.state::<ActionRegistry>();

        // Read buttons with lock held, then release before processing
        let button_result = {
            let mut streamdeck_guard = state.streamdeck.lock().unwrap();
            match &mut *streamdeck_guard {
                Some(streamdeck) => streamdeck.read_buttons().ok().copied(),
                None => None,
            }
        };

        let Some(buttons) = button_result else {
            // Device disconnected
            break;
        };

        // Process button presses (no streamdeck lock held)
        for i in 0..BUTTON_COUNT {
            if buttons[i] && !prev_states[i] {
                // Button i was just pressed - look up its action
                let config_guard = state.config.lock().unwrap();

                if let Some(page) = config_guard.pages.get(config_guard.current_page) {
                    if let Some(button_config) = page.buttons.get(&i) {
                        println!(
                            "Button {} pressed - executing: {}",
                            i, button_config.action.action_type
                        );

                        // Clone the action so we can release the config lock
                        let action = button_config.action.clone();
                        drop(config_guard);

                        // Execute the action via registry
                        if let Err(e) = registry.execute(&action, &app_handle) {
                            eprintln!("Action error: {}", e);
                        }
                    }
                }
            }
        }

        // Check if anything changed, then update prev_states and emit event
        if buttons != prev_states {
            prev_states = buttons;

            // Emit event to frontend
            let _ = app_handle.emit(
                "streamdeck://button-state",
                json!({ "buttons": buttons.to_vec() }),
            );
        }

        thread::sleep(Duration::from_millis(50));
    }
}
