# Tauri + Vue Architecture

## Two Separate Processes

```
┌────────────────────────────────────┐     ┌────────────────────────────────────┐
│         FRONTEND PROCESS           │     │         BACKEND PROCESS            │
│         (Webview/Chromium)         │     │         (Native Rust)              │
│                                    │     │                                    │
│  ┌──────────────────────────────┐  │     │  ┌──────────────────────────────┐  │
│  │         Vue App              │  │     │  │        Rust Code             │  │
│  │                              │  │     │  │                              │  │
│  │  • Components (.vue)         │  │     │  │  • System APIs               │  │
│  │  • TypeScript logic          │  │     │  │  • File system               │  │
│  │  • Reactive state            │  │     │  │  • HID/USB devices           │  │
│  │  • UI rendering              │  │     │  │  • Native performance        │  │
│  └──────────────────────────────┘  │     │  └──────────────────────────────┘  │
│                                    │     │                                    │
└──────────────────┬─────────────────┘     └─────────────────┬──────────────────┘
                   │                                         │
                   │         Tauri IPC Bridge                │
                   │◄───────────────────────────────────────►│
                   │       (JSON serialization)              │
                   │                                         │
```

**Key insight:** Vue runs in a browser-like webview. Rust runs as native code. They're separate processes that communicate via message passing.

---

## Communication: Frontend → Backend

### `invoke()` - Call Rust functions from TypeScript

```typescript
// Frontend: src/composables/useStreamDeck.ts
import { invoke } from "@tauri-apps/api/core";

const devices = await invoke<DeviceInfo[]>("list_devices");
//                           ▲              ▲
//                           │              └── Rust function name
//                           └── Expected return type
```

```rust
// Backend: src-tauri/src/commands/streamdeck.rs
#[tauri::command]
pub fn list_devices() -> Result<Vec<DeviceInfo>, String> {
    // This runs in Rust, has full system access
    StreamDeck::discover()
}
```

```rust
// Backend: src-tauri/src/lib.rs - Register the command
.invoke_handler(tauri::generate_handler![list_devices])
```

### With arguments

```typescript
// Frontend
await invoke("connect_device", { devicePath: "/dev/hidraw0" });
//                              ▲
//                              └── Object keys become Rust parameter names
```

```rust
// Backend - parameter name must match (snake_case)
#[tauri::command]
pub fn connect_device(device_path: String) -> Result<(), String> {
    // ...
}
```

---

## Communication: Backend → Frontend

### `emit()` - Push events from Rust to TypeScript

```rust
// Backend: Push data to frontend
use tauri::Emitter;

app_handle.emit("streamdeck://button-state", json!({
    "buttons": [false, true, false, ...]
}))?;
```

```typescript
// Frontend: Listen for events
import { listen } from "@tauri-apps/api/event";

const unlisten = await listen<ButtonEvent>("streamdeck://button-state", (event) => {
    console.log(event.payload.buttons);
});

// Later: stop listening
unlisten();
```

---

## Data Flow Summary

| Direction | Method | Use Case |
|-----------|--------|----------|
| Frontend → Backend | `invoke()` | Request data, trigger actions |
| Backend → Frontend | `emit()` | Push updates, real-time events |

---

## File Structure & Responsibilities

```
StreamDeck/
├── src/                          # FRONTEND (Vue + TypeScript)
│   ├── main.ts                   # Vue app entry
│   ├── App.vue                   # Root component
│   ├── components/               # UI components
│   └── composables/              # Shared logic (useStreamDeck.ts)
│
├── src-tauri/                    # BACKEND (Rust)
│   ├── src/
│   │   ├── main.rs               # Process entry point
│   │   ├── lib.rs                # Tauri setup, command registration
│   │   ├── commands/             # Tauri command handlers
│   │   └── hid/                  # Business logic (HID communication)
│   ├── Cargo.toml                # Rust dependencies
│   └── tauri.conf.json           # Tauri configuration
│
├── package.json                  # Node dependencies
└── vite.config.ts                # Vite bundler config
```

---

## Development Workflow

### Starting Development

```bash
npm run tauri dev
```

This runs **two things simultaneously**:
1. **Vite dev server** (localhost:1420) - serves Vue app with hot reload
2. **Rust backend** - compiles and runs, opens window pointing to Vite

### Making Changes

| Change to... | What happens |
|--------------|--------------|
| Vue/TS files (`src/`) | Instant hot reload, no restart |
| Rust files (`src-tauri/src/`) | Rust recompiles (~2-10s), app restarts |
| `Cargo.toml` | Full Rust rebuild |
| `tauri.conf.json` | Requires manual restart |

### Build for Production

```bash
npm run tauri build
```

Outputs a native executable with Vue bundled inside.

---

## Key Concepts

### Serialization
All data crossing the IPC bridge is JSON serialized:
- Rust structs need `#[derive(Serialize, Deserialize)]`
- TypeScript interfaces should mirror Rust struct shapes
- Complex types (functions, handles) cannot cross the bridge

### Managed State
For data that persists across commands (like an open device connection):

```rust
// Define state
pub struct AppState {
    pub streamdeck: Mutex<Option<StreamDeck>>,
}

// Register in lib.rs
.manage(AppState { streamdeck: Mutex::new(None) })

// Access in commands
#[tauri::command]
pub fn connect(state: State<'_, AppState>) -> Result<(), String> {
    let mut guard = state.streamdeck.lock().unwrap();
    *guard = Some(StreamDeck::connect(...)?);
    Ok(())
}
```

### Error Handling
Commands return `Result<T, String>`:
- `Ok(value)` → resolves the `invoke()` promise with `value`
- `Err(message)` → rejects the promise, frontend catches it

```typescript
try {
    await invoke("connect_device", { devicePath });
} catch (e) {
    // e is the String from Err(...)
    console.error("Connection failed:", e);
}
```

---

## Mental Model

Think of it like a client-server app:
- **Vue** = client (UI, user interaction)
- **Rust** = server (business logic, system access)
- **invoke()** = HTTP request
- **emit()** = WebSocket push

But it's all local, no network, just inter-process communication.
