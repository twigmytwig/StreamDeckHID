//! HID Communication Module
//!
//! This module contains all the logic for communicating with Stream Deck
//! devices over USB HID. It provides device discovery, connection management,
//! and button state reading functionality.

pub mod constants;
pub mod device;

// Re-export commonly used items for convenience
pub use constants::{ELGATO_VENDOR_ID, STREAM_DECK_MK2_PID, STREAM_DECK_ORIGINAL_PID};
pub use device::StreamDeck;
