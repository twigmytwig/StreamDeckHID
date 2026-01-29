# Stream Deck Controller

A desktop application for controlling Windows system functions using an Elgato Stream Deck. Built with Tauri, Vue 3, and TypeScript.

## MVP Goal

One button press on the Stream Deck increases system volume on Windows 11.

## Tech Stack

- **Frontend**: Vue 3 + TypeScript + Vite
- **Backend**: Tauri 2 (Rust)
- **Target Platform**: Windows 11
- **Communication**: Tauri IPC between Vue webview and native Rust backend

## Prerequisites

- [Node.js](https://nodejs.org/) (LTS recommended)
- [Rust](https://rustup.rs/) (stable toolchain)
- Elgato Stream Deck device
- Windows 11

## Development

```bash
# Install dependencies
npm install

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build

# Type-check the frontend
npm run build
```

## Project Structure

```
├── src/                    # Vue frontend
│   ├── components/         # Vue components
│   ├── composables/        # Shared logic (useStreamDeck.ts)
│   └── App.vue             # Main application component
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── lib.rs          # Tauri app setup, managed state
│   │   ├── commands/       # Tauri IPC commands
│   │   ├── config/         # Configuration structs
│   │   ├── actions/        # Action registry and handlers
│   │   ├── hid/            # Stream Deck HID communication
│   │   └── audio/          # Windows audio control
│   └── Cargo.toml          # Rust dependencies
├── docs/                   # Documentation
└── package.json            # Node dependencies
```

## Action System

The app uses a **registry pattern** for button actions. Actions are identified by strings (e.g., `"audio.volume_up"`) and mapped to handler functions at runtime. This makes it easy to add new actions without modifying enums or large match statements.

### Adding a New Action

#### Step 1: Create the Handler Function

Add your handler in `src-tauri/src/actions/handlers.rs`:

```rust
/// Description of what this action does
pub fn my_new_action(action: &Action, app: &AppHandle) -> Result<(), String> {
    // Your logic here
    println!("My action executed!");
    Ok(())
}
```

**Handler signature:** All handlers have the same signature:
```rust
fn(&Action, &AppHandle) -> Result<(), String>
```

- `&Action` - Access parameters via `action.get_string_param("key")` or `action.get_int_param("key")`
- `&AppHandle` - Access app state via `app.state::<AppState>()`
- Returns `Ok(())` on success, `Err("message".to_string())` on failure

#### Step 2: Register the Handler

Add one line in `src-tauri/src/actions/mod.rs` inside `register_defaults()`:

```rust
fn register_defaults(&mut self) {
    // ... existing registrations ...

    self.register("category.my_new_action", handlers::my_new_action);
}
```

**Naming convention:** Use `category.action_name` format:
- `audio.*` - Sound and volume actions
- `navigation.*` - Page switching
- `system.*` - OS-level actions (launch apps, hotkeys)
- `media.*` - Media playback control

#### Step 3: Assign to a Button

In `src-tauri/src/config/mod.rs`, add to `Config::default()` or your config file:

```rust
buttons.insert(
    "5".to_string(),  // Button index (0-14)
    ButtonConfig {
        action: Action::new("category.my_new_action"),
        label: Some("My Action".to_string()),
        image: None,
    },
);
```

Or in JSON config:
```json
{
  "5": {
    "action": { "type": "category.my_new_action" },
    "label": "My Action"
  }
}
```

### Actions with Parameters

Some actions need parameters. Access them in your handler:

```rust
pub fn audio_play_sound(action: &Action, _app: &AppHandle) -> Result<(), String> {
    let path = action.get_string_param("path")
        .ok_or("Missing 'path' parameter")?;

    // Use path...
    Ok(())
}
```

Config with parameters:
```json
{
  "action": {
    "type": "audio.play_sound",
    "params": { "path": "C:/sounds/alert.mp3" }
  }
}
```

### Built-in Actions

| Action Type | Description | Parameters |
|-------------|-------------|------------|
| `audio.volume_up` | Increase system volume | None |
| `audio.volume_down` | Decrease system volume | None |
| `audio.volume_mute` | Toggle mute | None |
| `audio.play_sound` | Play a sound file | `path`: file path |
| `navigation.next_page` | Go to next page | None |
| `navigation.previous_page` | Go to previous page | None |
| `navigation.go_to_page` | Jump to specific page | `page`: page index |
| `system.launch_app` | Launch an application | `path`: exe path, `args`: optional array |
| `system.open_url` | Open URL in browser | `url`: the URL |
| `system.hotkey` | Send keyboard shortcut | `keys`: key combination |
| `none` | Do nothing (spacer) | None |

## Important Notes

- **Close Elgato Software**: The Elgato Stream Deck software claims exclusive HID access. You must close it before running this application.
- **Device Manager**: If no device appears, verify it's recognized in Windows Device Manager under "Human Interface Devices".

## Documentation

See the `docs/` folder for detailed implementation guides:
- `implementation-guide.md` - Step-by-step development guide
- `tauri-vue-architecture.md` - Architecture overview
