<script setup lang="ts">
import { ref, onUnmounted, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useRoute } from 'vue-router';

const route = useRoute();
const cameraPermission = ref<boolean>(false);
const frameData = ref<string | null>(null);
const isRecording = ref(false);
let intervalId: number | null = null;

async function checkPermission() {
  try {
    cameraPermission.value = await invoke('check_camera_permission');
  } catch (e) {
    console.error("Permission check failed:", e);
  }
}

async function capture() {
  if (!isRecording.value) return;
  try {
    const frame: string = await invoke('capture_frame');
    frameData.value = frame;
  } catch (e) {
    // Expected error when browser is using the camera for test recording
    // Only log in development mode to avoid console spam
    if (import.meta.env.DEV && typeof e === 'string' && !e.includes('Camera busy')) {
      console.warn("Frame capture failed:", e);
    }
    // Silently fail - this is normal during active test recording
  }
}

function startSurveillance() {
  if (isRecording.value) return;
  isRecording.value = true;
  // Capture every 5 seconds for demo purposes
  intervalId = window.setInterval(capture, 5000);
}

function stopSurveillance() {
  isRecording.value = false;
  frameData.value = null; // Clear frame
  if (intervalId) {
    clearInterval(intervalId);
    intervalId = null;
  }
}

// Watch route changes to toggle surveillance
watch(() => route.meta?.requiresSurveillance, (shouldRecord) => {
  if (shouldRecord) {
    checkPermission().then(() => startSurveillance());
  } else {
    stopSurveillance();
  }
}, { immediate: true });

onUnmounted(() => {
  stopSurveillance();
});
</script>




<template>
  <div v-if="isRecording" class="fixed top-4 right-4 z-[9999] flex flex-col items-end pointer-events-none group">
    <!-- Minimized Dot (Default) -->
    <div
      class="glass-panel p-1.5 flex items-center space-x-2 transition-all duration-300 opacity-50 group-hover:opacity-100 hover:scale-105 pointer-events-auto cursor-pointer">
      <div class="relative flex items-center justify-center w-3 h-3">
        <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-eling-emerald opacity-75"></span>
        <span class="relative inline-flex rounded-full h-2 w-2 bg-eling-emerald"></span>
      </div>
      <span class="text-[10px] font-mono text-eling-dark-text font-bold hidden group-hover:block">SECURE</span>
    </div>

    <!-- Hover Preview (Only on hover) -->
    <div
      class="absolute top-8 right-0 w-32 h-24 rounded-lg overflow-hidden glass-panel border-white/20 shadow-2xl transition-all duration-300 transform scale-0 opacity-0 group-hover:scale-100 group-hover:opacity-100 origin-top-right">
      <img v-if="frameData" :src="'data:image/jpeg;base64,' + frameData"
        class="w-full h-full object-cover opacity-90" />
      <div v-else
        class="w-full h-full bg-black/50 flex items-center justify-center text-xs text-eling-dark-text/50 font-mono">
        NO SIGNAL
      </div>
    </div>
  </div>
</template>
