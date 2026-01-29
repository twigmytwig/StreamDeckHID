//! Stream Deck Tauri Commands
//!
//! This module provides the Tauri command handlers for Stream Deck operations.
//! These commands are invoked from the TypeScript frontend via `invoke()`.
//!
//! # Architecture Notes
//!
//! In a full implementation, you would:
//! 1. Store the connected StreamDeck in Tauri's managed state
//! 2. Use a background thread/task for continuous button polling
//! 3. Emit events to the frontend when button states change

use crate::hid::device::{DeviceInfo, StreamDeck};
use tauri::State;
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
///
/// # TODO
///
/// - Store the connection in Tauri's managed state
/// - Start a background task to poll for button changes
/// - Emit "streamdeck://button-state" events when buttons change
#[tauri::command]
pub fn connect_device(device_path: String, state: State<'_, AppState>) -> Result<(), String> {
    let streamdeck = StreamDeck::connect(&device_path)?;

    //Lock the mutex, get mutable acces to the Option inside
    let mut guard = state.streamdeck.lock().unwrap(); 
    *guard = Some(streamdeck); //the *guard here is accessing the option within which is the streamdeck state
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
