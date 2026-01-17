<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import BaseCard from '@/components/atoms/BaseCard.vue';
import BaseButton from '@/components/atoms/BaseButton.vue';
import BaseInput from '@/components/atoms/BaseInput.vue';

const serverStatus = ref<any>(null);
const serverPort = ref(8080);
const localIPs = ref<string[]>([]);
const isLoading = ref(false);
const message = ref('');
const messageType = ref<'success' | 'error'>('success');

onMounted(async () => {
  await loadServerStatus();
  await loadLocalIPs();
});

async function loadServerStatus() {
  try {
    serverStatus.value = await invoke('get_server_status');
  } catch (e) {
    console.error('Failed to load server status:', e);
  }
}

async function loadLocalIPs() {
  try {
    localIPs.value = await invoke('get_local_ips');
  } catch (e) {
    console.error('Failed to load local IPs:', e);
  }
}

async function startServer() {
  isLoading.value = true;
  message.value = '';
  
  try {
    const result = await invoke('start_api_server', { port: serverPort.value });
    message.value = result as string;
    messageType.value = 'success';
    await loadServerStatus();
  } catch (e: any) {
    message.value = e.toString();
    messageType.value = 'error';
  } finally {
    isLoading.value = false;
  }
}

async function stopServer() {
  isLoading.value = true;
  message.value = '';
  
  try {
    const result = await invoke('stop_api_server');
    message.value = result as string;
    messageType.value = 'success';
    await loadServerStatus();
  } catch (e: any) {
    message.value = e.toString();
    messageType.value = 'error';
  } finally {
    isLoading.value = false;
  }
}

function copyToClipboard(text: string) {
  navigator.clipboard.writeText(text);
  message.value = 'Copied to clipboard!';
  messageType.value = 'success';
  setTimeout(() => message.value = '', 2000);
}
</script>

<template>
  <BaseCard padding="lg">
    <div class="flex items-center justify-between mb-6">
      <div>
        <h2 class="text-xl font-bold text-gray-900 dark:text-white">API Server Control</h2>
        <p class="text-sm text-gray-600 dark:text-gray-400">Manage REST API server for receiving candidate data</p>
      </div>
      <div v-if="serverStatus" class="flex items-center gap-2">
        <div :class="[
          'w-3 h-3 rounded-full',
          serverStatus.running ? 'bg-green-500 animate-pulse' : 'bg-gray-400'
        ]"></div>
        <span class="text-sm font-medium" :class="serverStatus.running ? 'text-green-600 dark:text-green-400' : 'text-gray-500'">
          {{ serverStatus.running ? 'Running' : 'Stopped' }}
        </span>
      </div>
    </div>

    <!-- Message -->
    <div v-if="message" :class="[
      'mb-4 p-3 rounded-lg text-sm',
      messageType === 'success' ? 'bg-green-50 dark:bg-green-500/10 text-green-800 dark:text-green-200 border border-green-200 dark:border-green-500/20' : 'bg-red-50 dark:bg-red-500/10 text-red-800 dark:text-red-200 border border-red-200 dark:border-red-500/20'
    ]">
      {{ message }}
    </div>

    <!-- Server Configuration -->
    <div class="space-y-4">
      <!-- Port Configuration -->
      <div>
        <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
          Server Port
        </label>
        <BaseInput
          v-model.number="serverPort"
          type="number"
          :disabled="serverStatus?.running"
          placeholder="8080"
        />
        <p class="text-xs text-gray-500 dark:text-gray-400 mt-1">
          Default: 8080. Change only if port is already in use.
        </p>
      </div>

      <!-- Server URLs -->
      <div v-if="serverStatus?.running">
        <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
          Server URLs
        </label>
        <div class="space-y-2">
          <div v-for="ip in localIPs" :key="ip" class="flex items-center gap-2">
            <code class="flex-1 px-3 py-2 bg-gray-100 dark:bg-gray-800 rounded-lg text-sm font-mono">
              http://{{ ip }}:{{ serverStatus.port }}
            </code>
            <BaseButton @click="copyToClipboard(`http://${ip}:${serverStatus.port}`)" variant="ghost" size="sm">
              <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" />
              </svg>
            </BaseButton>
          </div>
        </div>
        <p class="text-xs text-gray-500 dark:text-gray-400 mt-2">
          Share these URLs with candidate computers on the same network.
        </p>
      </div>

      <!-- Control Buttons -->
      <div class="flex gap-3 pt-4 border-t border-gray-200 dark:border-gray-700">
        <BaseButton
          v-if="!serverStatus?.running"
          @click="startServer"
          variant="primary"
          :disabled="isLoading"
          class="flex-1"
        >
          <svg v-if="!isLoading" class="w-4 h-4 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14.752 11.168l-3.197-2.132A1 1 0 0010 9.87v4.263a1 1 0 001.555.832l3.197-2.132a1 1 0 000-1.664z" />
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
          </svg>
          <svg v-else class="animate-spin w-4 h-4 mr-2" fill="none" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
          </svg>
          {{ isLoading ? 'Starting...' : 'Start Server' }}
        </BaseButton>
        <BaseButton
          v-else
          @click="stopServer"
          variant="ghost"
          :disabled="isLoading"
          class="flex-1 text-red-600 dark:text-red-400"
        >
          <svg class="w-4 h-4 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 10a1 1 0 011-1h4a1 1 0 011 1v4a1 1 0 01-1 1h-4a1 1 0 01-1-1v-4z" />
          </svg>
          Stop Server
        </BaseButton>
      </div>
    </div>

    <!-- Info Box -->
    <div class="mt-6 p-4 bg-blue-50 dark:bg-blue-500/10 border border-blue-200 dark:border-blue-500/20 rounded-lg">
      <div class="flex gap-3">
        <svg class="w-5 h-5 text-blue-600 dark:text-blue-400 flex-shrink-0 mt-0.5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
        </svg>
        <div class="text-sm text-blue-800 dark:text-blue-200">
          <p class="font-semibold mb-1">How it works:</p>
          <ul class="list-disc list-inside space-y-1">
            <li>Start the API server on this admin computer</li>
            <li>Share the server URL with candidate computers</li>
            <li>Candidates configure the URL in their app settings</li>
            <li>Test results will sync automatically after completion</li>
          </ul>
        </div>
      </div>
    </div>
  </BaseCard>
</template>
