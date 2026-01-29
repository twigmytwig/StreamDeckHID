//! Stream Deck USB HID Constants
//!
//! This module defines all the USB identifiers and hardware constants
//! needed for communicating with Elgato Stream Deck devices.

/// Elgato USB Vendor ID
/// All Elgato devices (Stream Deck, Wave, Key Light, etc.) use this VID
pub const ELGATO_VENDOR_ID: u16 = 0x0fd9;

/// Stream Deck Original Product ID
/// This is the first generation Stream Deck with 15 buttons
pub const STREAM_DECK_ORIGINAL_PID: u16 = 0x0060;

/// Stream Deck MK.2 Product ID
/// The second generation Stream Deck, also with 15 buttons
/// Has improved LCD buttons and USB-C connection
pub const STREAM_DECK_MK2_PID: u16 = 0x0080;

/// Number of buttons on Stream Deck Original and MK.2
pub const BUTTON_COUNT: usize = 15;

/// Number of columns in the button grid
pub const GRID_COLUMNS: usize = 5;

/// Number of rows in the button grid
pub const GRID_ROWS: usize = 3;

// =============================================================================
// HID Report Structure Notes (for implementation reference)
// =============================================================================
//
// The Stream Deck communicates via USB HID reports. Here's the structure:
//
// INPUT REPORTS (Button State):
// -----------------------------
// The device sends input reports when button states change.
// Report structure for Original/MK.2:
//
// Byte 0:      Report ID (0x01)
// Byte 1-4:    Header/padding (varies by model)
// Byte 5-19:   Button states (1 byte per button, 0x00 = released, 0x01 = pressed)
//
// Button indexing (left-to-right, top-to-bottom):
// ┌────┬────┬────┬────┬────┐
// │  0 │  1 │  2 │  3 │  4 │
// ├────┼────┼────┼────┼────┤
// │  5 │  6 │  7 │  8 │  9 │
// ├────┼────┼────┼────┼────┤
// │ 10 │ 11 │ 12 │ 13 │ 14 │
// └────┴────┴────┴────┴────┘
//
// OUTPUT REPORTS (Setting Button Images):
// ---------------------------------------
// To set button images, send output reports with image data.
// The Stream Deck Original uses BMP format, MK.2 uses JPEG.
// Image size is 72x72 pixels per button.
//
// This scaffold focuses on input (button reading) only.
// Image setting will be implemented in a future iteration.
//
// FEATURE REPORTS (Device Info):
// ------------------------------
// Feature reports can be used to:
// - Get firmware version
// - Get serial number
// - Reset the device
// - Set brightness
//
// =============================================================================

/// List of all supported Stream Deck Product IDs
/// Used when scanning for devices
pub const SUPPORTED_PIDS: &[u16] = &[STREAM_DECK_ORIGINAL_PID, STREAM_DECK_MK2_PID];

/// Returns true if the given Product ID is a supported Stream Deck device
pub fn is_supported_device(product_id: u16) -> bool {
    SUPPORTED_PIDS.contains(&product_id)
}

/// Returns a human-readable name for the device based on its Product ID
pub fn get_device_name(product_id: u16) -> &'static str {
    match product_id {
        STREAM_DECK_ORIGINAL_PID => "Stream Deck Original",
        STREAM_DECK_MK2_PID => "Stream Deck MK.2",
        _ => "Unknown Stream Deck",
    }
}
