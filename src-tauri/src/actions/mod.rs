//! Action Registry Module
//!
//! Provides a registry pattern for mapping action type strings to handler functions.
//! This allows adding new actions without modifying enums or match statements.

mod handlers;

use std::collections::HashMap;
use tauri::AppHandle;
use crate::config::Action;

/// Function signature for action handlers
/// Takes the action (with params) and app handle, returns Result
pub type ActionHandler = fn(&Action, &AppHandle) -> Result<(), String>;

/// Registry that maps action type strings to their handler functions
pub struct ActionRegistry {
    handlers: HashMap<String, ActionHandler>,
}

impl ActionRegistry {
    /// Create a new registry with all built-in actions registered
    pub fn new() -> Self {
        let mut registry = Self {
            handlers: HashMap::new(),
        };
        registry.register_defaults();
        registry
    }

    /// Register a handler for an action type
    pub fn register(&mut self, action_type: &str, handler: ActionHandler) {
        self.handlers.insert(action_type.to_string(), handler);
    }

    /// Execute an action by looking up its handler
    pub fn execute(&self, action: &Action, app_handle: &AppHandle) -> Result<(), String> {
        match self.handlers.get(&action.action_type) {
            Some(handler) => handler(action, app_handle),
            None => {
                // Log unknown action but don't crash
                eprintln!("Unknown action type: {}", action.action_type);
                Ok(())
            }
        }
    }

    /// Check if an action type is registered
    pub fn has_action(&self, action_type: &str) -> bool {
        self.handlers.contains_key(action_type)
    }

    /// Get a list of all registered action types
    pub fn list_actions(&self) -> Vec<&str> {
        self.handlers.keys().map(|s| s.as_str()).collect()
    }

    /// Register all built-in action handlers
    fn register_defaults(&mut self) {
        // Audio actions
        self.register("audio.volume_up", handlers::audio_volume_up);
        self.register("audio.volume_down", handlers::audio_volume_down);
        self.register("audio.volume_mute", handlers::audio_volume_mute);
        self.register("audio.play_sound", handlers::audio_play_sound);

        // Navigation actions
        self.register("navigation.next_page", handlers::navigation_next_page);
        self.register("navigation.previous_page", handlers::navigation_previous_page);
        self.register("navigation.go_to_page", handlers::navigation_go_to_page);

        // System actions
        self.register("system.launch_app", handlers::system_launch_app);
        self.register("system.open_url", handlers::system_open_url);
        self.register("system.hotkey", handlers::system_hotkey);

        // Special actions
        self.register("none", handlers::action_none);
    }
}

impl Default for ActionRegistry {
    fn default() -> Self {
        Self::new()
    }
}
