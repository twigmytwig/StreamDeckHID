<script setup lang="ts">
/**
 * StreamDeckGrid.vue
 *
 * Visual representation of the Stream Deck's 5x3 button grid.
 * Each button shows its index (0-14) and highlights when pressed.
 *
 * Button layout (indices):
 * ┌────┬────┬────┬────┬────┐
 * │  0 │  1 │  2 │  3 │  4 │
 * ├────┼────┼────┼────┼────┤
 * │  5 │  6 │  7 │  8 │  9 │
 * ├────┼────┼────┼────┼────┤
 * │ 10 │ 11 │ 12 │ 13 │ 14 │
 * └────┴────┴────┴────┴────┘
 */

defineProps<{
  /** Array of 15 boolean values representing button press states */
  buttonStates: boolean[];
}>();

// Grid dimensions for Stream Deck Original/MK.2
const BUTTON_COUNT = 15;

// Generate button indices 0-14
const buttons = Array.from({ length: BUTTON_COUNT }, (_, i) => i);
</script>

<template>
  <div class="streamdeck-grid">
    <div
      v-for="index in buttons"
      :key="index"
      class="button"
      :class="{ pressed: buttonStates[index] }"
    >
      <span class="button-index">{{ index }}</span>
    </div>
  </div>
</template>

<style scoped>
.streamdeck-grid {
  display: grid;
  grid-template-columns: repeat(5, 1fr);
  grid-template-rows: repeat(3, 1fr);
  gap: 8px;
  max-width: 500px;
  margin: 0 auto;
  padding: 16px;
  background-color: #1a1a1a;
  border-radius: 12px;
}

.button {
  aspect-ratio: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: #333;
  border-radius: 8px;
  border: 2px solid #444;
  transition: all 0.1s ease;
  min-height: 60px;
}

.button.pressed {
  background-color: #007bff;
  border-color: #0056b3;
  transform: scale(0.95);
  box-shadow: 0 0 10px rgba(0, 123, 255, 0.5);
}

.button-index {
  color: #888;
  font-size: 1.25rem;
  font-weight: 600;
  user-select: none;
}

.button.pressed .button-index {
  color: white;
}
</style>
