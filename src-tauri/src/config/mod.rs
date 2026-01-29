//! Configuration Module
//!
//! Defines the structure for storing button mappings, actions, and images.
//! Configuration is persisted as JSON in the app data directory.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Root configuration structure
#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    /// Config file version for future migrations
    pub version: u32,
    /// Index of the currently active page
    pub current_page: usize,
    /// All pages of button configurations
    pub pages: Vec<Page>,
}

/// A single page of button configurations
#[derive(Serialize, Deserialize, Clone)]
pub struct Page {
    /// Display name for this page
    pub name: String,
    /// Button configurations, keyed by button index ("0" through "14")
    pub buttons: HashMap<String, ButtonConfig>,
}

/// Configuration for a single button
#[derive(Serialize, Deserialize, Clone)]
pub struct ButtonConfig {
    /// The action to perform when this button is pressed
    pub action: Action,
    /// Optional display label (shown in UI, can be rendered on button)
    pub label: Option<String>,
    /// Optional custom image (overrides action's default image)
    pub image: Option<ButtonImage>,
}

/// Image to display on a button
#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum ButtonImage {
    /// Image loaded from a file path
    #[serde(rename = "file")]
    File { path: String },

    /// Built-in icon referenced by name
    #[serde(rename = "icon")]
    Icon { name: String },

    /// Dynamically generated text image
    #[serde(rename = "text")]
    Text {
        text: String,
        #[serde(default)]
        background_color: Option<String>,
        #[serde(default)]
        text_color: Option<String>,
    },
}

/// Action data - just a type string and optional parameters.
/// The actual execution logic lives in the ActionRegistry.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Action {
    /// Action type identifier (e.g., "audio.volume_up", "navigation.next_page")
    #[serde(rename = "type")]
    pub action_type: String,

    /// Optional parameters for the action
    #[serde(default)]
    pub params: HashMap<String, serde_json::Value>,
}

impl Action {
    /// Create a new action with no parameters
    pub fn new(action_type: &str) -> Self {
        Self {
            action_type: action_type.to_string(),
            params: HashMap::new(),
        }
    }

    /// Create a new action with parameters
    pub fn with_params(action_type: &str, params: HashMap<String, serde_json::Value>) -> Self {
        Self {
            action_type: action_type.to_string(),
            params,
        }
    }

    /// Get a string parameter
    pub fn get_string_param(&self, key: &str) -> Option<&str> {
        self.params.get(key).and_then(|v| v.as_str())
    }

    /// Get an integer parameter
    pub fn get_int_param(&self, key: &str) -> Option<i64> {
        self.params.get(key).and_then(|v| v.as_i64())
    }

    /// Returns the default icon name based on action type
    pub fn default_icon(&self) -> Option<&'static str> {
        match self.action_type.as_str() {
            "audio.volume_up" => Some("volume-up"),
            "audio.volume_down" => Some("volume-down"),
            "audio.volume_mute" => Some("volume-mute"),
            "audio.play_sound" => Some("music"),
            "navigation.next_page" => Some("arrow-right"),
            "navigation.previous_page" => Some("arrow-left"),
            "navigation.go_to_page" => Some("layers"),
            "system.launch_app" => Some("terminal"),
            "system.open_url" => Some("globe"),
            "system.hotkey" => Some("keyboard"),
            _ => None,
        }
    }
}

impl Default for Config {
    /// Creates a default configuration with one page and volume controls
    fn default() -> Self {
        let mut buttons = HashMap::new();

        buttons.insert(
            "0".to_string(),
            ButtonConfig {
                action: Action::new("audio.volume_up"),
                label: Some("Vol +".to_string()),
                image: None,
            },
        );

        buttons.insert(
            "1".to_string(),
            ButtonConfig {
                action: Action::new("audio.volume_down"),
                label: Some("Vol -".to_string()),
                image: None,
            },
        );

        Config {
            version: 1,
            current_page: 0,
            pages: vec![Page {
                name: "Main".to_string(),
                buttons,
            }],
        }
    }
}
