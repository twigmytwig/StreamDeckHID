//! Stream Deck Controller Library
//!
//! This module exports the Tauri application builder and all command handlers.
//! The HID communication logic is organized into submodules.
mod commands;
mod audio;
mod hid;

use std::sync::Mutex;
use crate::hid::device::StreamDeck;
use commands::streamdeck::{connect_device, disconnect_device, get_button_state, list_devices};

pub struct AppState{
    pub streamdeck: Mutex<Option<StreamDeck>>,
}

/// Runs the Tauri application.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        // Register all Tauri commands that can be invoked from the frontend
        .invoke_handler(tauri::generate_handler![
            list_devices,
            connect_device,
            disconnect_device,
            get_button_state,
        ])
        // TODO: Add setup hook to initialize HID context and managed state
        // .setup(|app| {
        //     // Initialize the HID API context here
        //     // Store it in app state for use by commands
        //     Ok(())
        // })
        .manage(AppState {
            streamdeck: Mutex::new(None),
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
