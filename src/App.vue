<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import StreamDeckGrid from "./components/StreamDeckGrid.vue";
import { useStreamDeck, type DeviceInfo } from "./composables/useStreamDeck";

const {
  devices,
  connectedDevice,
  buttonStates,
  listDevices,
  connect,
  disconnect,
  setupButtonListener,
  cleanupButtonListener,
} = useStreamDeck();

const isLoading = ref(false);
const error = ref<string | null>(null);

async function handleRefreshDevices() {
  isLoading.value = true;
  error.value = null;
  try {
    await listDevices();
  } catch (e) {
    error.value = e instanceof Error ? e.message : "Failed to list devices";
  } finally {
    isLoading.value = false;
  }
}

async function handleConnect(device: DeviceInfo) {
  isLoading.value = true;
  error.value = null;
  try {
    await connect(device.path);
  } catch (e) {
    error.value = e instanceof Error ? e.message : "Failed to connect";
  } finally {
    isLoading.value = false;
  }
}

async function handleDisconnect() {
  isLoading.value = true;
  error.value = null;
  try {
    await disconnect();
  } catch (e) {
    error.value = e instanceof Error ? e.message : "Failed to disconnect";
  } finally {
    isLoading.value = false;
  }
}

onMounted(() => {
  setupButtonListener();
  handleRefreshDevices();
});

onUnmounted(() => {
  cleanupButtonListener();
});
</script>

<template>
  <main class="container">
    <h1>Stream Deck Controller</h1>

    <!-- Error display -->
    <div v-if="error" class="error">
      {{ error }}
    </div>

    <!-- Device selection section -->
    <section class="device-section">
      <div class="device-header">
        <h2>Devices</h2>
        <button @click="handleRefreshDevices" :disabled="isLoading">
          {{ isLoading ? "Scanning..." : "Refresh" }}
        </button>
      </div>

      <!-- Device list -->
      <div v-if="devices.length === 0" class="no-devices">
        No Stream Deck devices found. Connect a device and click Refresh.
      </div>

      <ul v-else class="device-list">
        <li v-for="device in devices" :key="device.path" class="device-item">
          <div class="device-info">
            <span class="device-name">{{ device.product_name }}</span>
            <span class="device-serial">{{ device.serial_number || "No serial" }}</span>
          </div>
          <button
            v-if="connectedDevice?.path !== device.path"
            @click="handleConnect(device)"
            :disabled="isLoading"
          >
            Connect
          </button>
          <button v-else @click="handleDisconnect" :disabled="isLoading" class="disconnect">
            Disconnect
          </button>
        </li>
      </ul>
    </section>

    <!-- Stream Deck grid visualization -->
    <section v-if="connectedDevice" class="grid-section">
      <h2>Button Grid</h2>
      <StreamDeckGrid :button-states="buttonStates" />
    </section>
  </main>
</template>

<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }
}

.container {
  max-width: 800px;
  margin: 0 auto;
  padding: 2rem;
}

h1 {
  text-align: center;
  margin-bottom: 2rem;
}

h2 {
  margin: 0;
  font-size: 1.25rem;
}

.error {
  background-color: #fee;
  border: 1px solid #fcc;
  color: #c00;
  padding: 1rem;
  border-radius: 4px;
  margin-bottom: 1rem;
}

.device-section {
  margin-bottom: 2rem;
}

.device-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;
}

.no-devices {
  color: #666;
  font-style: italic;
  padding: 1rem;
  text-align: center;
  border: 1px dashed #ccc;
  border-radius: 4px;
}

.device-list {
  list-style: none;
  padding: 0;
  margin: 0;
}

.device-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.75rem 1rem;
  border: 1px solid #ddd;
  border-radius: 4px;
  margin-bottom: 0.5rem;
}

.device-info {
  display: flex;
  flex-direction: column;
}

.device-name {
  font-weight: 600;
}

.device-serial {
  font-size: 0.85rem;
  color: #666;
}

button {
  padding: 0.5rem 1rem;
  border: none;
  border-radius: 4px;
  background-color: #007bff;
  color: white;
  cursor: pointer;
  font-size: 0.9rem;
}

button:hover:not(:disabled) {
  background-color: #0056b3;
}

button:disabled {
  background-color: #ccc;
  cursor: not-allowed;
}

button.disconnect {
  background-color: #dc3545;
}

button.disconnect:hover:not(:disabled) {
  background-color: #c82333;
}

.grid-section {
  margin-top: 2rem;
}

.grid-section h2 {
  margin-bottom: 1rem;
}
</style>
