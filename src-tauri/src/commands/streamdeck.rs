//! Stream Deck Tauri Commands
//!
//! This module provides the Tauri command handlers for Stream Deck operations.
//! These commands are invoked from the TypeScript frontend via `invoke()`.

use std::thread;
use std::time::Duration;

use serde_json::json;
use tauri::{AppHandle, Emitter, Manager, State};

use crate::actions::ActionRegistry;
use crate::hid::device::{DeviceInfo, StreamDeck};
use crate::AppState;

/// List all connected Stream Deck devices.
///
/// # Frontend Usage
///
/// ```typescript
/// const devices = await invoke<DeviceInfo[]>('list_devices');
/// ```
#[tauri::command]
pub fn list_devices() -> Result<Vec<DeviceInfo>, String> {
    StreamDeck::discover()
}

/// Connect to a Stream Deck device.
///
/// # Arguments
///
/// * `device_path` - The USB device path from `DeviceInfo.path`
///
/// # Frontend Usage
///
/// ```typescript
/// await invoke('connect_device', { devicePath: '/dev/hidraw0' });
/// ```
#[tauri::command]
pub fn connect_device(device_path: String, state: State<'_, AppState>, app_handle: AppHandle) -> Result<(), String> {
    let streamdeck = StreamDeck::connect(&device_path)?;

    //Lock the mutex, get mutable acces to the Option inside
    let mut guard = state.streamdeck.lock().unwrap(); 
    *guard = Some(streamdeck); //the *guard here is accessing the option within which is the streamdeck state
    
    drop(guard); // Release the lock before spawning thread

    // Clone the handle for the thread
    let handle = app_handle.clone();

    thread::spawn(move || {
        polling_loop(handle);
    });

    Ok(())
}

/// Disconnect from the currently connected Stream Deck.
///
/// # Frontend Usage
///
/// ```typescript
/// await invoke('disconnect_device');
/// ```
#[tauri::command]
pub fn disconnect_device(state: State<'_, AppState>) -> Result<(), String> {
    let mut guard = state.streamdeck.lock().unwrap();
    *guard = None; //setting the option within (aka stream deck state) to none
    Ok(())
}

/// Get the current button states.
///
/// # Returns
///
/// An array of 15 boolean values (true = pressed).
/// Button indices are left-to-right, top-to-bottom (0-14).
///
/// # Frontend Usage
///
/// ```typescript
/// const states = await invoke<boolean[]>('get_button_state');
/// ```
#[tauri::command]
pub fn get_button_state(state: State<'_, AppState>) -> Result<Vec<bool>, String> {
    let mut guard = state.streamdeck.lock().unwrap();

    match &mut *guard {
        Some(streamdeck) => {
            //streamdeck is now a variable holding &mut of the option (StreamDeck)
            let buttons = streamdeck.read_buttons()?;
            Ok(buttons.to_vec())
        }
        None => {
            //no device connected
            Err("No device connected".to_string())
        }
    }
}

fn polling_loop(app_handle: AppHandle) {
    let mut prev_states = [false; 15];

    loop {
        // Get access to state and registry through the app handle
        let state = app_handle.state::<AppState>();
        let registry = app_handle.state::<ActionRegistry>();

        let mut streamdeck_guard = state.streamdeck.lock().unwrap();

        match &mut *streamdeck_guard {
            Some(streamdeck) => {
                if let Ok(buttons) = streamdeck.read_buttons() {
                    // Check each button for rising edge (just pressed)
                    for i in 0..15 {
                        if buttons[i] && !prev_states[i] {
                            // Button i was just pressed - look up its action
                            let config_guard = state.config.lock().unwrap();

                            if let Some(page) = config_guard.pages.get(config_guard.current_page) {
                                if let Some(button_config) = page.buttons.get(&i.to_string()) {
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
                    if buttons != &prev_states {
                        prev_states = *buttons;

                        // Emit event to frontend
                        let _ = app_handle.emit(
                            "streamdeck://button-state",
                            json!({ "buttons": buttons.to_vec() }),
                        );
                    }
                }
            }
            None => {
                // Device disconnected, exit the loop
                break;
            }
        }

        // Release lock before sleeping!
        drop(streamdeck_guard);

        thread::sleep(Duration::from_millis(50));
    }
}