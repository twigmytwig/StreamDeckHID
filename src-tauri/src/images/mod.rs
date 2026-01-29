//! Button Image Module
//!
//! Handles all button image operations:
//! - Resolving image paths from various sources (custom, built-in, generated)
//! - Syncing images to the physical Stream Deck LCD
//!
//! Image sources:
//! - Custom user images (file paths)
//! - Built-in icons (bundled with app)
//! - Generated text images (TODO)

use crate::config::{ButtonConfig, ButtonImage};
use crate::AppState;
use tauri::{AppHandle, Manager, State};

/// Resolve the image path for a button configuration.
/// Returns an absolute file path that the frontend converts to a URL.
///
/// Priority:
/// 1. Custom image override (button_config.image)
/// 2. Action default icon (based on action type)
/// 3. None (no image)
pub fn resolve_button_image(button_config: &ButtonConfig, app_handle: &AppHandle) -> Option<String> {
    // Checking for custom image first
    if let Some(ref image) = button_config.image {
        return resolve_custom_image(image, app_handle);
    }

    // System default second
    if let Some(icon_name) = button_config.action.default_icon() {
        return resolve_builtin_icon(icon_name, app_handle);
    }

    None // no image womp womp
}

/// Resolve a custom image to an absolute file path
fn resolve_custom_image(image: &ButtonImage, app_handle: &AppHandle) -> Option<String> {
    match image {
        ButtonImage::File { path } => {
            // Return the user's custom path as-is
            Some(path.clone())
        }
        ButtonImage::Icon { name } => {
            // Built-in icon referenced by name
            resolve_builtin_icon(name, app_handle)
        }
        ButtonImage::Text { text, .. } => {
            // TODO: Generate actual text image
            None
        }
    }
}

/// Resolve a built-in icon name to an absolute file path
/// Handles both dev mode (local files) and production (bundled resources)
fn resolve_builtin_icon(icon_name: &str, app_handle: &AppHandle) -> Option<String> {
    let icon_filename = format!("icons/{}.png", icon_name);

    // First, try Tauri's resource resolver (works in production)
    if let Ok(path) = app_handle.path().resolve(&icon_filename, tauri::path::BaseDirectory::Resource) {
        if path.exists() {
            return Some(path.to_string_lossy().to_string());
        }
    }

    // In dev mode, try to find icons relative to the src-tauri directory
    // This is where cargo runs from during development
    let dev_path = std::path::PathBuf::from(&icon_filename);
    if dev_path.exists() {
        if let Ok(absolute) = dev_path.canonicalize() {
            return Some(absolute.to_string_lossy().to_string());
        }
    }

    eprintln!("Warning: Could not find icon '{}'", icon_name);
    None
}

/// Sync button images to the physical Stream Deck LCD.
///
/// This resolves images for all configured buttons on the current page
/// and sends them to the device.
pub fn sync_images_to_device(state: &State<'_, AppState>, app_handle: &AppHandle) {
    let config = state.config.lock().unwrap();

    // Get current page
    let page = match config.pages.get(config.current_page) {
        Some(p) => p,
        None => return,
    };

    // Collect image paths first, then release config lock
    let mut image_paths: Vec<(usize, String)> = Vec::new();

    for i in 0..15 {
        let button_key = i.to_string();

        if let Some(button_config) = page.buttons.get(&button_key) {
            if let Some(path) = resolve_button_image(button_config, app_handle) {
                image_paths.push((i, path));
            }
        }
    }

    drop(config); // Release config lock before acquiring streamdeck lock

    // Now send images to the device
    let mut streamdeck_guard = state.streamdeck.lock().unwrap();

    if let Some(ref mut streamdeck) = *streamdeck_guard {
        for (button_index, path) in image_paths {
            if let Err(e) = streamdeck.set_button_image(button_index, &path) {
                eprintln!("Failed to set image for button {}: {}", button_index, e);
            } else {
                println!("Set image for button {}: {}", button_index, path);
            }
        }
    }
}