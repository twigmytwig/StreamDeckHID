import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";

/**
 * Device information returned from the Rust backend.
 */
export interface DeviceInfo {
  /** USB device path for connecting */
  path: string;
  /** Product name (e.g., "Stream Deck MK.2") */
  product_name: string;
  /** Device serial number, if available */
  serial_number: string | null;
  /** USB Vendor ID (should be 0x0fd9 for Elgato) */
  vendor_id: number;
  /** USB Product ID (0x0060 for Original, 0x0080 for MK.2) */
  product_id: number;
}

/**
 * Button event emitted from Rust when button states change.
 */
export interface ButtonEvent {
  /** Array of 15 boolean values for each button's press state */
  buttons: boolean[];
}

/**
 * Composable for interacting with Stream Deck devices via Tauri commands.
 *
 * Provides reactive state and methods for:
 * - Discovering connected Stream Deck devices
 * - Connecting/disconnecting to a specific device
 * - Receiving real-time button press events
 */
export function useStreamDeck() {
  /** List of discovered Stream Deck devices */
  const devices = ref<DeviceInfo[]>([]);

  /** Currently connected device, or null if not connected */
  const connectedDevice = ref<DeviceInfo | null>(null);

  /** Current button states (15 booleans for 5x3 grid) */
  const buttonStates = ref<boolean[]>(new Array(15).fill(false));

  /** Unlisten function for cleaning up event listener */
  let unlistenFn: UnlistenFn | null = null;

  /**
   * Discover and list all connected Stream Deck devices.
   */
  async function listDevices(): Promise<void> {
    const result = await invoke<DeviceInfo[]>("list_devices");
    devices.value = result;
  }

  /**
   * Connect to a Stream Deck device by its USB path.
   */
  async function connect(devicePath: string): Promise<void> {
    await invoke("connect_device", { devicePath });
    const device = devices.value.find((d) => d.path === devicePath);
    if (device) {
      connectedDevice.value = device;
    }
  }

  /**
   * Disconnect from the currently connected Stream Deck.
   */
  async function disconnect(): Promise<void> {
    await invoke("disconnect_device");
    connectedDevice.value = null;
    buttonStates.value = new Array(15).fill(false);
  }

  /**
   * Set up listener for button state change events from Rust.
   * Call this in onMounted() and cleanupButtonListener() in onUnmounted().
   */
  async function setupButtonListener(): Promise<void> {
    // TODO: The Rust backend will emit "streamdeck://button-state" events
    // when button states change
    unlistenFn = await listen<ButtonEvent>("streamdeck://button-state", (event) => {
      buttonStates.value = event.payload.buttons;
    });
  }

  /**
   * Clean up the button state event listener.
   */
  function cleanupButtonListener(): void {
    if (unlistenFn) {
      unlistenFn();
      unlistenFn = null;
    }
  }

  return {
    devices,
    connectedDevice,
    buttonStates,
    listDevices,
    connect,
    disconnect,
    setupButtonListener,
    cleanupButtonListener,
  };
}
