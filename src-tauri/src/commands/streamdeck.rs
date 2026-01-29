//! Stream Deck Tauri Commands
//!
//! This module provides the Tauri command handlers for Stream Deck operations.
//! These commands are invoked from the TypeScript frontend via `invoke()`.

use tauri::{AppHandle, Manager, State};

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

    // Sync button images to the Stream Deck LCD
    crate::images::sync_images_to_device(&state, &app_handle);

    // Start the button polling loop in a background thread
    crate::polling::start_polling(app_handle);

    Ok(())
}

/// Disconnect from the currently connected Stream Deck.
///
/// Clears all button images before disconnecting.
///
/// # Frontend Usage
///
/// ```typescript
/// await invoke('disconnect_device');
/// ```
#[tauri::command]
pub fn disconnect_device(state: State<'_, AppState>) -> Result<(), String> {
    let mut guard = state.streamdeck.lock().unwrap();

    // Clear all button images before disconnecting
    if let Some(ref mut streamdeck) = *guard {
        if let Err(e) = streamdeck.clear_all_buttons() {
            eprintln!("Warning: Failed to clear buttons on disconnect: {}", e);
        }
    }

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

/// Get images for all buttons on the current page.
///
/// # Returns
///
/// An array of 15 Option<String> values. Each is either:
/// - Some(path) - Absolute file path (frontend converts with convertFileSrc)
/// - None - No image configured
///
/// # Frontend Usage
///
/// ```typescript
/// import { convertFileSrc } from '@tauri-apps/api/core';
/// const paths = await invoke<(string | null)[]>('get_button_images');
/// const urls = paths.map(p => p ? convertFileSrc(p) : null);
/// ```
#[tauri::command]
pub fn get_button_images(state: State<'_, AppState>, app_handle: AppHandle) -> Vec<Option<String>> {
    crate::images::get_current_page_images(&state, &app_handle)
}