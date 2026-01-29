//! Stream Deck Controller Library
//!
//! This module exports the Tauri application builder and all command handlers.
//! The HID communication logic is organized into submodules.

mod actions;
mod audio;
mod commands;
mod config;
mod hid;

use std::sync::Mutex;
use crate::actions::ActionRegistry;
use crate::config::Config;
use crate::hid::device::StreamDeck;
use commands::streamdeck::{connect_device, disconnect_device, get_button_state, list_devices};

/// Application state shared across commands
pub struct AppState {
    /// Currently connected Stream Deck device
    pub streamdeck: Mutex<Option<StreamDeck>>,
    /// Button/action configuration
    pub config: Mutex<Config>,
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
        // Manage application state
        .manage(AppState {
            streamdeck: Mutex::new(None),
            config: Mutex::new(Config::default()),
        })
        // Manage action registry separately (it doesn't need a Mutex - it's read-only after init)
        .manage(ActionRegistry::new())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
