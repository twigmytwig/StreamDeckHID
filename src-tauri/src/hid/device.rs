//! Stream Deck Device Abstraction
//!
//! This module provides the `StreamDeck` struct which abstracts the HID
//! communication with Stream Deck devices. It handles device discovery,
//! connection management, and button state reading.

use hidapi::{DeviceInfo as HidDeviceInfo, HidApi, HidDevice};
use serde::{Deserialize, Serialize};

use super::constants::{is_supported_device, BUTTON_COUNT, ELGATO_VENDOR_ID};

/// Information about a discovered Stream Deck device.
/// This is returned to the frontend when listing available devices.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    /// The USB device path used to open this device
    pub path: String,
    /// Human-readable product name
    pub product_name: String,
    /// Device serial number (may be empty for some devices)
    pub serial_number: Option<String>,
    /// USB Vendor ID (should be ELGATO_VENDOR_ID)
    pub vendor_id: u16,
    /// USB Product ID (identifies the specific Stream Deck model)
    pub product_id: u16,
}

impl DeviceInfo {
    /// Create a DeviceInfo from hidapi's DeviceInfo
    fn from_hid_device(device: &HidDeviceInfo) -> Self {
        Self {
            path: device.path().to_string_lossy().into_owned(),
            product_name: device
                .product_string()
                .map(|s| s.to_string())
                .unwrap_or_else(|| "Unknown Device".to_string()),
            serial_number: device.serial_number().map(|s| s.to_string()),
            vendor_id: device.vendor_id(),
            product_id: device.product_id(),
        }
    }
}

/// Represents a connected Stream Deck device.
///
/// This struct manages the HID connection and provides methods for
/// reading button states and getting device information.
pub struct StreamDeck {
    /// The underlying HID device handle
    #[allow(dead_code)]
    device: HidDevice,
    /// Cached device information
    #[allow(dead_code)]
    info: DeviceInfo,
    /// Current button states (true = pressed)
    button_states: [bool; BUTTON_COUNT],
}

impl StreamDeck {
    // =========================================================================
    // Device Discovery
    // =========================================================================

    /// Discover all connected Stream Deck devices.
    ///
    /// This function enumerates all USB HID devices and filters for
    /// Elgato Stream Deck devices (matching VID and supported PIDs).
    ///
    /// # Returns
    ///
    /// A vector of `DeviceInfo` for each discovered Stream Deck.
    ///
    /// # Errors
    ///
    /// Returns an error if the HID API cannot be initialized.
    pub fn discover() -> Result<Vec<DeviceInfo>, String> {
        // TODO: Initialize the HID API
        // The HidApi::new() call initializes the underlying platform-specific
        // HID library (hidraw on Linux, IOHidManager on macOS, etc.)
        let api = HidApi::new().map_err(|e| format!("Failed to initialize HID API: {}", e))?;

        // TODO: Enumerate all HID devices and filter for Stream Decks
        // We filter by:
        // 1. Vendor ID must be Elgato (0x0fd9)
        // 2. Product ID must be a supported Stream Deck model
        let devices: Vec<DeviceInfo> = api
            .device_list()
            .filter(|device| {
                device.vendor_id() == ELGATO_VENDOR_ID
                    && is_supported_device(device.product_id())
            })
            .map(DeviceInfo::from_hid_device)
            .collect();

        Ok(devices)
    }

    // =========================================================================
    // Connection Management
    // =========================================================================

    /// Connect to a Stream Deck device by its path.
    ///
    /// # Arguments
    ///
    /// * `device_path` - The USB device path (from DeviceInfo::path)
    ///
    /// # Returns
    ///
    /// A connected `StreamDeck` instance.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The HID API cannot be initialized
    /// - The device cannot be opened (permissions, already in use, etc.)
    /// - The device is not a supported Stream Deck
    pub fn connect(device_path: &str) -> Result<Self, String> {
        // TODO: Initialize the HID API
        let api = HidApi::new().map_err(|e| format!("Failed to initialize HID API: {}", e))?;

        // TODO: Open the device by path
        // This establishes the HID connection to the specific device
        let device = api
            .open_path(std::ffi::CString::new(device_path).unwrap().as_c_str())
            .map_err(|e| format!("Failed to open device: {}", e))?;

        // TODO: Get device info for caching
        // We need to re-enumerate to get the DeviceInfo for this path
        let info = api
            .device_list()
            .find(|d| d.path().to_string_lossy() == device_path)
            .map(|d| DeviceInfo::from_hid_device(d))
            .ok_or_else(|| "Device not found after opening".to_string())?;

        // TODO: Set non-blocking mode for button reading
        // Non-blocking allows us to poll for button states without
        // blocking the thread when no data is available
        device
            .set_blocking_mode(false)
            .map_err(|e| format!("Failed to set non-blocking mode: {}", e))?;

        Ok(Self {
            device,
            info,
            button_states: [false; BUTTON_COUNT],
        })
    }

    /// Disconnect from the Stream Deck.
    ///
    /// This consumes the StreamDeck instance and releases the HID connection.
    /// The device handle is automatically closed when dropped.
    pub fn disconnect(self) {
        // The HidDevice is automatically closed when dropped
        drop(self);
    }

    // =========================================================================
    // Button State Reading
    // =========================================================================

    /// Read the current button states from the device.
    ///
    /// This method polls the HID device for input reports and updates
    /// the internal button state array.
    ///
    /// # Returns
    ///
    /// A reference to the current button states array.
    /// Index 0-14 corresponds to buttons left-to-right, top-to-bottom.
    ///
    /// # Errors
    ///
    /// Returns an error if reading from the device fails.
    pub fn read_buttons(&mut self) -> Result<&[bool; BUTTON_COUNT], String> {
        // TODO: Read HID input report from the device
        // Buffer size needs to accommodate the full report
        // Stream Deck Original/MK.2 reports are typically 17-32 bytes
        let mut buf = [0u8; 32];

        // TODO: Attempt to read from the device (non-blocking)
        // Returns the number of bytes read, or 0 if no data available
        match self.device.read(&mut buf) {
            Ok(bytes_read) if bytes_read > 0 => {
                // TODO: Parse the input report
                // The button states are at specific offsets depending on the model
                // For Stream Deck Original/MK.2, buttons start at byte 4 or 5
                //
                // Example parsing (actual offsets may vary):
                // for i in 0..BUTTON_COUNT {
                //     self.button_states[i] = buf[4 + i] != 0;
                // }
                let _ = bytes_read; // Acknowledge we received data
            }
            Ok(_) => {
                // No data available (non-blocking read returned 0)
            }
            Err(e) => {
                return Err(format!("Failed to read from device: {}", e));
            }
        }

        Ok(&self.button_states)
    }

    /// Get the current button states without reading from the device.
    pub fn get_button_states(&self) -> &[bool; BUTTON_COUNT] {
        &self.button_states
    }

    // =========================================================================
    // Device Information
    // =========================================================================

    /// Get information about this device.
    #[allow(dead_code)]
    pub fn get_device_info(&self) -> &DeviceInfo {
        &self.info
    }

    /// Get the device's serial number.
    #[allow(dead_code)]
    pub fn get_serial_number(&self) -> Option<&str> {
        self.info.serial_number.as_deref()
    }

    /// Get the firmware version of the device.
    ///
    /// # TODO
    ///
    /// This requires sending a feature report request to the device.
    #[allow(dead_code)]
    pub fn get_firmware_version(&self) -> Result<String, String> {
        // TODO: Send feature report to get firmware version
        Err("Firmware version reading not yet implemented".to_string())
    }
}
