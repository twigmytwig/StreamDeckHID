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
│   └── App.vue             # Main application component
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── lib.rs          # Tauri app setup
│   │   ├── commands/       # Tauri IPC commands
│   │   ├── hid/            # Stream Deck HID communication
│   │   └── audio/          # Windows audio control
│   └── Cargo.toml          # Rust dependencies
├── docs/                   # Documentation
└── package.json            # Node dependencies
```

## Important Notes

- **Close Elgato Software**: The Elgato Stream Deck software claims exclusive HID access. You must close it before running this application.
- **Device Manager**: If no device appears, verify it's recognized in Windows Device Manager under "Human Interface Devices".

## Documentation

See the `docs/` folder for detailed implementation guides:
- `implementation-guide.md` - Step-by-step development guide
- `tauri-vue-architecture.md` - Architecture overview
