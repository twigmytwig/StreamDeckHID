# Stream Deck Controller - Implementation Guide

## MVP Goal
One button press on the Stream Deck increases system volume on Windows 11.

---

## Phase 1: Verify Device Discovery

**Goal:** Confirm the app can see your Stream Deck.

### Steps

1. **Run the app** with your Stream Deck plugged in:
   ```bash
   npm run tauri dev
   ```

2. **Click Refresh** - you should see your device listed with its name and serial number.

3. **If no device appears**, check:
   - Device is plugged in and recognized by Windows (Device Manager > Human Interface Devices)
   - Close any other Stream Deck software (Elgato's app claims exclusive access)

### Files to check/modify
- `src-tauri/src/hid/device.rs` - `StreamDeck::discover()` is already implemented
- `src-tauri/src/hid/constants.rs` - Verify your device's PID is listed

### Debugging
Add temporary logging to `list_devices` command in `src-tauri/src/commands/streamdeck.rs`:
```rust
#[tauri::command]
pub fn list_devices() -> Result<Vec<DeviceInfo>, String> {
    let devices = StreamDeck::discover()?;
    println!("Found {} devices: {:?}", devices.len(), devices);
    Ok(devices)
}
```

---

## Phase 2: Establish Device Connection

**Goal:** Successfully open a HID connection to the Stream Deck.

### Steps

1. **Add managed state** to store the connection. Update `src-tauri/src/lib.rs`:
   ```rust
   use std::sync::Mutex;
   use hid::StreamDeck;

   pub struct AppState {
       pub streamdeck: Mutex<Option<StreamDeck>>,
   }
   ```

2. **Register state** in the Tauri builder:
   ```rust
   .manage(AppState {
       streamdeck: Mutex::new(None),
   })
   ```

3. **Update `connect_device`** to store the connection:
   ```rust
   #[tauri::command]
   pub fn connect_device(
       device_path: String,
       state: State<'_, AppState>,
   ) -> Result<(), String> {
       let streamdeck = StreamDeck::connect(&device_path)?;
       *state.streamdeck.lock().unwrap() = Some(streamdeck);
       Ok(())
   }
   ```

4. **Update `disconnect_device`** similarly.

### Files to modify
- `src-tauri/src/lib.rs` - Add AppState struct and .manage()
- `src-tauri/src/commands/streamdeck.rs` - Add State parameter to commands

### Testing
- Click Connect in the UI
- If it succeeds without error, the HID connection is working
- If you get a permission error, ensure Elgato's Stream Deck software is closed

---

## Phase 3: Read Button Presses

**Goal:** Detect when a button is pressed and log it.

### Steps

1. **Implement button parsing** in `src-tauri/src/hid/device.rs`. Update `read_buttons()`:
   ```rust
   pub fn read_buttons(&mut self) -> Result<&[bool; BUTTON_COUNT], String> {
       let mut buf = [0u8; 512];  // MK.2 needs larger buffer

       match self.device.read(&mut buf) {
           Ok(bytes_read) if bytes_read > 0 => {
               // Stream Deck MK.2 button data starts at byte 4
               // Original starts at byte 1
               // Each button is 1 byte: 0x00 = released, 0x01 = pressed
               let offset = 4;  // Adjust based on your model
               for i in 0..BUTTON_COUNT {
                   self.button_states[i] = buf[offset + i] != 0;
               }
           }
           Ok(_) => {}  // No data available
           Err(e) => return Err(format!("Read error: {}", e)),
       }

       Ok(&self.button_states)
   }
   ```

2. **Create a polling loop** that emits events. Add to `connect_device`:
   ```rust
   use tauri::Emitter;
   use std::thread;
   use std::time::Duration;

   // After storing streamdeck in state, spawn polling thread
   let app_handle = app.clone();
   thread::spawn(move || {
       loop {
           // Get lock, read buttons, emit event
           // Release lock between reads
           thread::sleep(Duration::from_millis(50));
       }
   });
   ```

3. **Emit button events** to the frontend:
   ```rust
   app_handle.emit("streamdeck://button-state", serde_json::json!({
       "buttons": button_states.to_vec()
   })).ok();
   ```

### Files to modify
- `src-tauri/src/hid/device.rs` - Button parsing logic
- `src-tauri/src/commands/streamdeck.rs` - Polling thread

### Testing
- Connect to device
- Press buttons and watch the console for logged states
- The frontend grid should highlight pressed buttons

### Debugging button offsets
If buttons don't register correctly, print the raw buffer:
```rust
println!("Raw HID report ({} bytes): {:02X?}", bytes_read, &buf[..bytes_read]);
```
Press a button and note which byte changes from 00 to 01.

---

## Phase 4: Add Volume Control (Windows)

**Goal:** Increase system volume when a specific button is pressed.

### Steps

1. **Add the `windows` crate** to Cargo.toml:
   ```bash
   cd src-tauri
   cargo add windows --features "Win32_Media_Audio,Win32_System_Com"
   ```

2. **Create a new module** `src-tauri/src/audio/mod.rs`:
   ```rust
   use windows::Win32::Media::Audio::{
       ISimpleAudioVolume,
       eMultimedia, eRender,
   };
   use windows::Win32::Media::Audio::Endpoints::IAudioEndpointVolume;
   use windows::Win32::System::Com::{
       CoCreateInstance, CoInitializeEx, CLSCTX_ALL, COINIT_MULTITHREADED,
   };

   pub fn increase_volume(amount: f32) -> Result<(), String> {
       // TODO: Implement using Windows Audio API
       // 1. CoInitializeEx
       // 2. Get IMMDeviceEnumerator
       // 3. GetDefaultAudioEndpoint
       // 4. Activate IAudioEndpointVolume
       // 5. GetMasterVolumeLevelScalar + SetMasterVolumeLevelScalar
       Ok(())
   }
   ```

3. **Alternative: Use PowerShell** (simpler, works immediately):
   ```rust
   use std::process::Command;

   pub fn increase_volume_simple() -> Result<(), String> {
       Command::new("powershell")
           .args([
               "-Command",
               "(New-Object -ComObject WScript.Shell).SendKeys([char]175)"
           ])
           .output()
           .map_err(|e| e.to_string())?;
       Ok(())
   }
   ```
   Note: `[char]175` is the Volume Up key scancode.

4. **Alternative: Use nircmd** (most reliable):
   - Download nircmd from https://www.nirsoft.net/utils/nircmd.html
   - Place `nircmd.exe` in your project or PATH
   ```rust
   Command::new("nircmd")
       .args(["changesysvolume", "5000"])  // Increase by ~7%
       .output()
       .map_err(|e| e.to_string())?;
   ```

### Files to create/modify
- `src-tauri/src/audio/mod.rs` - New module for audio control
- `src-tauri/src/lib.rs` - Add `mod audio;`
- `src-tauri/Cargo.toml` - Add windows crate (if using native API)

---

## Phase 5: Wire Button to Volume

**Goal:** Connect button press detection to volume increase.

### Steps

1. **Choose a button** (e.g., button 0 = top-left).

2. **Track previous state** to detect press (not hold):
   ```rust
   let mut prev_states = [false; 15];

   loop {
       let current = streamdeck.read_buttons()?;

       // Button 0: Volume Up
       if current[0] && !prev_states[0] {
           // Button just pressed (rising edge)
           audio::increase_volume_simple()?;
       }

       prev_states = *current;
       thread::sleep(Duration::from_millis(50));
   }
   ```

3. **Update the UI** to show which button has the volume action assigned (optional for MVP).

### Files to modify
- `src-tauri/src/commands/streamdeck.rs` - Add action trigger in polling loop

---

## Phase 6: Polish & Test

### Checklist
- [ ] App starts without errors
- [ ] Device is discovered and listed
- [ ] Connect button works
- [ ] Button presses are detected (grid highlights)
- [ ] Pressing button 0 increases system volume
- [ ] Disconnect button works
- [ ] No crashes on unplug/replug

### Known Issues to Handle
1. **Device disconnect** - Check for read errors and clean up state
2. **Multiple devices** - The scaffold supports listing multiple, but only connects to one
3. **Thread cleanup** - Signal the polling thread to stop on disconnect

---

## Recommended Implementation Order

```
1. Device Discovery (already working)
        ↓
2. Device Connection (managed state)
        ↓
3. Button Reading (HID parsing)
        ↓
4. Volume Control (standalone test)
        ↓
5. Wire Together (button → action)
        ↓
6. Polish (error handling, UI feedback)
```

---

## Quick Reference: HID Report Format

### Stream Deck MK.2 (0x0080)
```
Byte 0:     Report ID (0x01)
Byte 1-3:   Header
Byte 4-18:  Button states (15 bytes, 1 per button)
```

### Stream Deck Original (0x0060)
```
Byte 0:     Report ID (0x01)
Byte 1-15:  Button states (15 bytes, 1 per button)
```

### Button Index Layout
```
┌────┬────┬────┬────┬────┐
│  0 │  1 │  2 │  3 │  4 │  ← Volume Up on button 0
├────┼────┼────┼────┼────┤
│  5 │  6 │  7 │  8 │  9 │
├────┼────┼────┼────┼────┤
│ 10 │ 11 │ 12 │ 13 │ 14 │
└────┴────┴────┴────┴────┘
```

---

## Useful Commands

```bash
# Build and run
npm run tauri dev

# Build release
npm run tauri build

# Check Rust code
cd src-tauri && cargo check

# See connected HID devices (PowerShell)
Get-PnpDevice -Class HIDClass | Where-Object {$_.Status -eq 'OK'}
```
