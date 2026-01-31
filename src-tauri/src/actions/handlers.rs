//! Action Handler Functions
//!
//! Each function handles a specific action type. They all have the same signature:
//! fn(&Action, &AppHandle) -> Result<(), String>

use tauri::AppHandle;
use crate::config::Action;
use crate::AppState;
use tauri::Manager;

// =============================================================================
// Audio Actions
// =============================================================================

pub fn audio_volume_up(_action: &Action, _app: &AppHandle) -> Result<(), String> {
    crate::audio::volume_up()
}

pub fn audio_volume_down(_action: &Action, _app: &AppHandle) -> Result<(), String> {
    crate::audio::volume_down()
}

pub fn audio_volume_mute(_action: &Action, _app: &AppHandle) -> Result<(), String> {
    crate::audio::volume_mute()
}

pub fn audio_play_sound(action: &Action, _app: &AppHandle) -> Result<(), String> {
    let path = action.get_string_param("path")
        .ok_or("Missing 'path' parameter for play_sound")?;

    // TODO: Implement sound playback
    eprintln!("Play sound not yet implemented: {}", path);
    Ok(())
}

// =============================================================================
// Navigation Actions
// =============================================================================

pub fn navigation_next_page(_action: &Action, app: &AppHandle) -> Result<(), String> {
    let state = app.state::<AppState>();
    let mut config = state.config.lock().unwrap();

    let num_pages = config.pages.len();
    if num_pages > 0 {
        config.current_page = (config.current_page + 1) % num_pages;
        println!("Switched to page {}: {}", config.current_page, config.pages[config.current_page].name);
    }

    Ok(())
}

pub fn navigation_previous_page(_action: &Action, app: &AppHandle) -> Result<(), String> {
    let state = app.state::<AppState>();
    let mut config = state.config.lock().unwrap();

    let num_pages = config.pages.len();
    if num_pages > 0 {
        config.current_page = if config.current_page == 0 {
            num_pages - 1
        } else {
            config.current_page - 1
        };
        println!("Switched to page {}: {}", config.current_page, config.pages[config.current_page].name);
    }

    Ok(())
}

pub fn navigation_go_to_page(action: &Action, app: &AppHandle) -> Result<(), String> {
    let page_index = action.get_int_param("page")
        .ok_or("Missing 'page' parameter for go_to_page")? as usize;

    let state = app.state::<AppState>();
    let mut config = state.config.lock().unwrap();

    if page_index < config.pages.len() {
        config.current_page = page_index;
        println!("Jumped to page {}: {}", config.current_page, config.pages[config.current_page].name);
    } else {
        return Err(format!("Page index {} out of range", page_index));
    }

    Ok(())
}

// =============================================================================
// System Actions
// =============================================================================

pub fn system_launch_app(action: &Action, _app: &AppHandle) -> Result<(), String> {
    let path = action.get_string_param("path")
        .ok_or("Missing 'path' parameter for launch_app")?;

    // Get optional arguments
    let args: Vec<String> = action.params.get("args")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
        .unwrap_or_default();

    println!("Launching: {} {:?}", path, args);

    std::process::Command::new(path)
        .args(&args)
        .spawn()
        .map_err(|e| format!("Failed to launch app: {}", e))?;

    Ok(())
}

pub fn system_open_url(action: &Action, _app: &AppHandle) -> Result<(), String> {
    let url = action.get_string_param("url")
        .ok_or("Missing 'url' parameter for open_url")?;

    println!("Opening URL: {}", url);

    // Use Windows 'start' command to open URL in default browser
    std::process::Command::new("cmd")
        .args(["/C", "start", "", url])
        .spawn()
        .map_err(|e| format!("Failed to open URL: {}", e))?;

    Ok(())
}

pub fn system_hotkey(action: &Action, _app: &AppHandle) -> Result<(), String> {
    let keys = action.get_string_param("keys")
        .ok_or("Missing 'keys' parameter for hotkey")?;

    // TODO: Implement hotkey sending
    eprintln!("Hotkey not yet implemented: {}", keys);
    Ok(())
}

// =============================================================================
// Special Actions
// =============================================================================

pub fn action_none(_action: &Action, _app: &AppHandle) -> Result<(), String> {
    // Do nothing - this is intentional
    Ok(())
}
