# System Ideas

Future features and enhancements for the Stream Deck Controller.

---

## Multi-Page Support

- **Multiple pages** - Each page has its own set of button assignments
- **Page navigation** - Dedicate buttons to switch between pages (e.g., button 14 = next page, button 10 = previous page)
- **Page indicators** - Show current page number on the UI
- **Unlimited pages** - Allow users to create as many pages as they need
- **Folder pages** - A button can open a "folder" (sub-page) and another button returns to parent

---

## Button Images

- **Custom images** - Display user-provided images on Stream Deck LCD buttons
- **Icon library** - Built-in icons for common actions (volume, play/pause, mute, etc.)
- **Dynamic images** - Update button images based on state (e.g., muted vs unmuted)
- **Text labels** - Generate simple text-based button images
- **Image formats** - Support PNG, JPG, SVG conversion to Stream Deck format

---

## Drag & Drop Configuration UI

- **Visual button grid** - Drag actions onto buttons in the UI
- **Action palette** - Sidebar with available actions to drag from
- **Button editor** - Click a button to configure its action, image, and label
- **Undo/redo** - Support for undoing configuration changes
- **Import/export** - Save and load configurations as JSON files
- **Profiles** - Switch between different configuration profiles (e.g., "Gaming", "Streaming", "Work")

---

## Audio Playback

- **Sound bites** - Play MP3/WAV files on button press
- **Sound board mode** - Quick access to frequently used audio clips
- **Volume control per sound** - Individual volume levels for each sound
- **Stop all sounds** - Button to stop any currently playing audio
- **Loop option** - Some sounds can loop until pressed again
- **Audio preview** - Preview sounds in the configuration UI

---

## Media Controls

- **Play/Pause** - Control media playback
- **Next/Previous track** - Skip tracks
- **Volume up/down** - Already implemented!
- **Mute toggle** - Mute/unmute system audio
- **App-specific volume** - Control volume for specific applications

---

## Application Launching

- **Launch apps** - Open applications with a button press
- **Launch with arguments** - Start apps with specific command-line arguments
- **Open URLs** - Launch websites in default browser
- **Open files** - Open specific files in their default applications
- **Open folders** - Open file explorer to specific directories

---

## Keyboard & Macros

- **Hotkey triggers** - Send keyboard shortcuts (Ctrl+C, Alt+Tab, etc.)
- **Text input** - Type predefined text strings
- **Multi-step macros** - Sequence of actions with delays
- **Hold vs press** - Different actions for tap vs long-press
- **Key combinations** - Complex modifier key combinations

---

## Streaming Integration

- **OBS integration** - Scene switching, start/stop recording, toggle sources
- **Twitch integration** - Create clips, run ads, change title
- **Discord integration** - Mute/deafen, toggle overlay
- **Spotify integration** - Like current song, save to playlist

---

## System Controls

- **Screen brightness** - Adjust monitor brightness
- **Night light toggle** - Enable/disable Windows night light
- **Lock screen** - Lock the workstation
- **Sleep/shutdown** - System power controls
- **Screenshot** - Capture screen to clipboard or file

---

## Smart Home

- **Home Assistant integration** - Control lights, switches, scenes
- **Philips Hue** - Direct Hue bulb/scene control
- **MQTT support** - Publish messages to MQTT brokers

---

## Technical Improvements

- **Persistent configuration** - Save button assignments to a config file
- **Auto-reconnect** - Automatically reconnect when device is unplugged/replugged
- **Multiple devices** - Support multiple Stream Decks simultaneously
- **Device-specific configs** - Different configurations per Stream Deck model
- **Startup launch** - Option to start with Windows
- **System tray** - Minimize to system tray

---

## Plugin System

- **User plugins** - Allow users to create custom actions
- **Plugin API** - Documented API for plugin developers
- **Plugin marketplace** - Share and download community plugins

---

## Priority / MVP+

Near-term features to implement after MVP:

1. Volume down on button 1
2. Mute toggle on button 2
3. Persistent configuration (save/load)
4. Button images
5. Multi-page support
6. Drag & drop UI

---

*Last updated: January 2026*
