//! Stream Deck Device Abstraction
//!
//! This module provides the `StreamDeck` struct which abstracts the HID
//! communication with Stream Deck devices. It handles device discovery,
//! connection management, and button state reading.

use std::io::Cursor;
use std::path::Path;

use hidapi::{DeviceInfo as HidDeviceInfo, HidApi, HidDevice};
use image::{DynamicImage, ImageFormat};
use serde::{Deserialize, Serialize};

use super::constants::{
    is_supported_device, BUTTON_COUNT, ELGATO_VENDOR_ID, IMAGE_SIZE, JPEG_QUALITY,
    MK2_HEADER_SIZE, MK2_IMAGE_DATA_PER_PACKET, MK2_PACKET_SIZE, STREAM_DECK_MK2_PID,
};

/// Byte offset where button data starts in HID input report
pub const BUTTON_DATA_OFFSET: usize = 4;

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
    device: HidDevice,
    /// Cached device information
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
    /// The device handle is automatically closed when dropped, so this method is more so redundant.
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
                // Parse the input report
                // The button states are at specific offsets depending on the model
                // For Stream Deck Original/MK.2, buttons start at byte 4 or 5
                for i in 0..BUTTON_COUNT {
                    self.button_states[i] = buf[BUTTON_DATA_OFFSET + i] != 0;
                }
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

    // =========================================================================
    // Button Image Setting
    // =========================================================================

    /// Set the image for a button from a file path.
    ///
    /// # Arguments
    ///
    /// * `button_index` - The button index (0-14)
    /// * `image_path` - Path to the image file (PNG, JPEG, etc.)
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The button index is out of range
    /// - The image file cannot be read
    /// - Image processing fails
    /// - Writing to the device fails
    pub fn set_button_image<P: AsRef<Path>>(&mut self, button_index: usize, image_path: P) -> Result<(), String> {
        if button_index >= BUTTON_COUNT {
            return Err(format!("Button index {} out of range (0-{})", button_index, BUTTON_COUNT - 1));
        }

        // Load the image from file
        let img = image::open(image_path.as_ref())
            .map_err(|e| format!("Failed to load image: {}", e))?;

        self.set_button_image_from_data(button_index, img)
    }

    /// Set the image for a button from image data.
    ///
    /// # Arguments
    ///
    /// * `button_index` - The button index (0-14)
    /// * `img` - The image data
    pub fn set_button_image_from_data(&mut self, button_index: usize, img: DynamicImage) -> Result<(), String> {
        if button_index >= BUTTON_COUNT {
            return Err(format!("Button index {} out of range (0-{})", button_index, BUTTON_COUNT - 1));
        }

        // Process the image: resize, rotate, and encode
        let image_data = self.prepare_image(img)?;

        // Send the image to the device
        self.write_image_to_device(button_index, &image_data)
    }

    /// Clear a button's image (set to black).
    ///
    /// # Arguments
    ///
    /// * `button_index` - The button index (0-14)
    pub fn clear_button_image(&mut self, button_index: usize) -> Result<(), String> {
        if button_index >= BUTTON_COUNT {
            return Err(format!("Button index {} out of range (0-{})", button_index, BUTTON_COUNT - 1));
        }

        // Create a black image
        let black_img = DynamicImage::new_rgb8(IMAGE_SIZE, IMAGE_SIZE);
        self.set_button_image_from_data(button_index, black_img)
    }

    /// Clear all button images (set all to black).
    pub fn clear_all_buttons(&mut self) -> Result<(), String> {
        for i in 0..BUTTON_COUNT {
            self.clear_button_image(i)?;
        }
        Ok(())
    }

    /// Prepare an image for the Stream Deck.
    ///
    /// This function:
    /// 1. Resizes to 72x72 pixels
    /// 2. Rotates 180° (Stream Deck displays images upside down)
    /// 3. Encodes as JPEG (for MK.2) or BMP (for Original)
    fn prepare_image(&self, img: DynamicImage) -> Result<Vec<u8>, String> {
        // Resize to 72x72, maintaining aspect ratio and filling
        let resized = img.resize_to_fill(
            IMAGE_SIZE,
            IMAGE_SIZE,
            image::imageops::FilterType::Lanczos3,
        );

        // Rotate 180° (the Stream Deck displays images upside down)
        // This is equivalent to flipping both horizontally and vertically
        let rotated = resized.rotate180();

        // Encode based on device type
        if self.info.product_id == STREAM_DECK_MK2_PID {
            self.encode_jpeg(&rotated)
        } else {
            self.encode_bmp(&rotated)
        }
    }

    /// Encode an image as JPEG for MK.2.
    fn encode_jpeg(&self, img: &DynamicImage) -> Result<Vec<u8>, String> {
        let mut buffer = Cursor::new(Vec::new());

        // Convert to RGB8 for JPEG encoding
        let rgb_img = img.to_rgb8();

        // Create a JPEG encoder with the specified quality
        let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut buffer, JPEG_QUALITY);

        encoder
            .encode(
                rgb_img.as_raw(),
                rgb_img.width(),
                rgb_img.height(),
                image::ExtendedColorType::Rgb8,
            )
            .map_err(|e| format!("Failed to encode JPEG: {}", e))?;

        Ok(buffer.into_inner())
    }

    /// Encode an image as BMP for Original.
    fn encode_bmp(&self, img: &DynamicImage) -> Result<Vec<u8>, String> {
        let mut buffer = Cursor::new(Vec::new());

        img.write_to(&mut buffer, ImageFormat::Bmp)
            .map_err(|e| format!("Failed to encode BMP: {}", e))?;

        Ok(buffer.into_inner())
    }

    /// Write image data to the Stream Deck device.
    ///
    /// The image is sent in chunks via HID output reports.
    fn write_image_to_device(&mut self, button_index: usize, image_data: &[u8]) -> Result<(), String> {
        if self.info.product_id == STREAM_DECK_MK2_PID {
            self.write_image_mk2(button_index, image_data)
        } else {
            self.write_image_original(button_index, image_data)
        }
    }

    /// Write image to MK.2 Stream Deck.
    ///
    /// MK.2 uses 1024-byte packets with an 8-byte header.
    fn write_image_mk2(&mut self, button_index: usize, image_data: &[u8]) -> Result<(), String> {
        let total_length = image_data.len();
        let mut bytes_sent = 0;
        let mut page_number = 0;

        while bytes_sent < total_length {
            // Calculate how much data to send in this packet
            let remaining = total_length - bytes_sent;
            let payload_length = remaining.min(MK2_IMAGE_DATA_PER_PACKET);
            let is_last_packet = bytes_sent + payload_length >= total_length;

            // Build the packet
            let mut packet = vec![0u8; MK2_PACKET_SIZE];

            // MK.2 Header format (8 bytes):
            // [0]: Report ID (0x02)
            // [1]: Command (0x07 for set image)
            // [2]: Button index
            // [3]: Is last packet (0x01 if true, 0x00 if false)
            // [4-5]: Payload length (little-endian u16)
            // [6-7]: Page number (little-endian u16)
            packet[0] = 0x02;
            packet[1] = 0x07;
            packet[2] = button_index as u8;
            packet[3] = if is_last_packet { 0x01 } else { 0x00 };
            packet[4] = (payload_length & 0xFF) as u8;
            packet[5] = ((payload_length >> 8) & 0xFF) as u8;
            packet[6] = (page_number & 0xFF) as u8;
            packet[7] = ((page_number >> 8) & 0xFF) as u8;

            // Copy image data into packet
            let data_slice = &image_data[bytes_sent..bytes_sent + payload_length];
            packet[MK2_HEADER_SIZE..MK2_HEADER_SIZE + payload_length].copy_from_slice(data_slice);

            // Write to device
            self.device
                .write(&packet)
                .map_err(|e| format!("Failed to write image packet: {}", e))?;

            bytes_sent += payload_length;
            page_number += 1;
        }

        Ok(())
    }

    /// Write image to Original Stream Deck.
    ///
    /// Original uses different packet structure with BMP format.
    fn write_image_original(&mut self, button_index: usize, image_data: &[u8]) -> Result<(), String> {
        // Original Stream Deck uses 8191-byte packets with a 16-byte header
        const ORIGINAL_PACKET_SIZE: usize = 8191;
        const ORIGINAL_HEADER_SIZE: usize = 16;
        const ORIGINAL_DATA_PER_PACKET: usize = ORIGINAL_PACKET_SIZE - ORIGINAL_HEADER_SIZE;

        let total_length = image_data.len();
        let mut bytes_sent = 0;
        let mut page_number = 1; // Original uses 1-based page numbers

        while bytes_sent < total_length {
            let remaining = total_length - bytes_sent;
            let payload_length = remaining.min(ORIGINAL_DATA_PER_PACKET);
            let is_last_packet = bytes_sent + payload_length >= total_length;

            let mut packet = vec![0u8; ORIGINAL_PACKET_SIZE];

            // Original Header format (16 bytes):
            // [0]: Report ID (0x02)
            // [1]: Command (0x01 for set image)
            // [2-3]: Page number (little-endian u16)
            // [4]: Padding (0x00)
            // [5]: Is last packet (0x01 if true, 0x00 if false)
            // [6]: Button index (+ 1, 1-based)
            // [7-15]: Padding
            packet[0] = 0x02;
            packet[1] = 0x01;
            packet[2] = (page_number & 0xFF) as u8;
            packet[3] = ((page_number >> 8) & 0xFF) as u8;
            packet[4] = 0x00;
            packet[5] = if is_last_packet { 0x01 } else { 0x00 };
            packet[6] = (button_index + 1) as u8;
            // Bytes 7-15 are padding (already 0)

            let data_slice = &image_data[bytes_sent..bytes_sent + payload_length];
            packet[ORIGINAL_HEADER_SIZE..ORIGINAL_HEADER_SIZE + payload_length].copy_from_slice(data_slice);

            self.device
                .write(&packet)
                .map_err(|e| format!("Failed to write image packet: {}", e))?;

            bytes_sent += payload_length;
            page_number += 1;
        }

        Ok(())
    }
}
