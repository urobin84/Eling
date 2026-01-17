<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import BaseCard from '@/components/atoms/BaseCard.vue';
import BaseButton from '@/components/atoms/BaseButton.vue';
import BaseInput from '@/components/atoms/BaseInput.vue';

const serverUrl = ref('');
const isConnected = ref(false);
const isTesting = ref(false);
const isSaving = ref(false);
const message = ref('');
const messageType = ref<'success' | 'error' | 'info'>('info');
const syncQueueStatus = ref<any>(null);

onMounted(async () => {
  await loadServerUrl();
  await loadSyncQueueStatus();
});

async function loadServerUrl() {
  try {
    const url = await invoke('get_server_url');
    if (url) {
      serverUrl.value = url as string;
    }
  } catch (e) {
    console.error('Failed to load server URL:', e);
  }
}

async function loadSyncQueueStatus() {
  try {
    syncQueueStatus.value = await invoke('get_sync_queue_status');
  } catch (e) {
    console.error('Failed to load sync queue status:', e);
  }
}

async function testConnection() {
  if (!serverUrl.value) {
    message.value = 'Please enter a server URL';
    messageType.value = 'error';
    return;
  }

  isTesting.value = true;
  message.value = '';

  try {
    // Save URL first
    await invoke('set_server_url', { url: serverUrl.value });
    
    // Test connection
    const connected = await invoke('test_server_connection');
    isConnected.value = connected as boolean;
    
    if (connected) {
      message.value = '✓ Connected successfully!';
      messageType.value = 'success';
    } else {
      message.value = '✗ Connection failed';
      messageType.value = 'error';
    }
  } catch (e: any) {
    message.value = `✗ ${e.toString()}`;
    messageType.value = 'error';
    isConnected.value = false;
  } finally {
    isTesting.value = false;
  }
}

async function saveConfiguration() {
  if (!serverUrl.value) {
    message.value = 'Please enter a server URL';
    messageType.value = 'error';
    return;
  }

  isSaving.value = true;
  message.value = '';

  try {
    await invoke('set_server_url', { url: serverUrl.value });
    message.value = '✓ Configuration saved';
    messageType.value = 'success';
  } catch (e: any) {
    message.value = `✗ ${e.toString()}`;
    messageType.value = 'error';
  } finally {
    isSaving.value = false;
  }
}

async function refreshQueueStatus() {
  await loadSyncQueueStatus();
}
</script>

<template>
  <BaseCard padding="lg">
    <div class="mb-6">
      <h2 class="text-xl font-bold text-gray-900 dark:text-white mb-2">Server Connection</h2>
      <p class="text-sm text-gray-600 dark:text-gray-400">Configure connection to admin server for data synchronization</p>
    </div>

    <!-- Message -->
    <div v-if="message" :class="[
      'mb-4 p-3 rounded-lg text-sm font-medium',
      messageType === 'success' ? 'bg-green-50 dark:bg-green-500/10 text-green-800 dark:text-green-200 border border-green-200 dark:border-green-500/20' : 
      messageType === 'error' ? 'bg-red-50 dark:bg-red-500/10 text-red-800 dark:text-red-200 border border-red-200 dark:border-red-500/20' :
      'bg-blue-50 dark:bg-blue-500/10 text-blue-800 dark:text-blue-200 border border-blue-200 dark:border-blue-500/20'
    ]">
      {{ message }}
    </div>

    <div class="space-y-6">
      <!-- Server URL Input -->
      <div>
        <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
          Admin Server URL
        </label>
        <div class="flex gap-2">
          <BaseInput
            v-model="serverUrl"
            type="text"
            placeholder="http://192.168.1.100:8080"
            class="flex-1"
          />
          <BaseButton
            @click="testConnection"
            variant="ghost"
            :disabled="isTesting || !serverUrl"
          >
            <svg v-if="!isTesting" class="w-4 h-4 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
            <svg v-else class="animate-spin w-4 h-4 mr-2" fill="none" viewBox="0 0 24 24">
              <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
              <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
            </svg>
            {{ isTesting ? 'Testing...' : 'Test' }}
          </BaseButton>
        </div>
        <p class="text-xs text-gray-500 dark:text-gray-400 mt-1">
          Enter the URL provided by your administrator (e.g., http://192.168.1.100:8080)
        </p>
      </div>

      <!-- Connection Status -->
      <div v-if="isConnected" class="p-4 bg-green-50 dark:bg-green-500/10 border border-green-200 dark:border-green-500/20 rounded-lg">
        <div class="flex items-center gap-2">
          <div class="w-3 h-3 rounded-full bg-green-500 animate-pulse"></div>
          <span class="text-sm font-medium text-green-800 dark:text-green-200">Connected to server</span>
        </div>
      </div>

      <!-- Sync Queue Status -->
      <div v-if="syncQueueStatus" class="p-4 bg-gray-50 dark:bg-gray-800 rounded-lg">
        <div class="flex items-center justify-between mb-2">
          <h3 class="text-sm font-semibold text-gray-900 dark:text-white">Sync Queue</h3>
          <BaseButton @click="refreshQueueStatus" variant="ghost" size="sm">
            <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
            </svg>
          </BaseButton>
        </div>
        <div class="flex items-center gap-4">
          <div>
            <div class="text-2xl font-bold text-gray-900 dark:text-white">{{ syncQueueStatus.pending_count }}</div>
            <div class="text-xs text-gray-600 dark:text-gray-400">Pending items</div>
          </div>
        </div>
      </div>

      <!-- Save Button -->
      <div class="flex gap-3 pt-4 border-t border-gray-200 dark:border-gray-700">
        <BaseButton
          @click="saveConfiguration"
          variant="primary"
          :disabled="isSaving || !serverUrl"
          class="flex-1"
        >
          <svg v-if="!isSaving" class="w-4 h-4 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
          </svg>
          <svg v-else class="animate-spin w-4 h-4 mr-2" fill="none" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
          </svg>
          {{ isSaving ? 'Saving...' : 'Save Configuration' }}
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
          <p class="font-semibold mb-1">How sync works:</p>
          <ul class="list-disc list-inside space-y-1">
            <li>Test results are saved locally first</li>
            <li>Data syncs to server automatically after test completion</li>
            <li>If connection fails, data is queued for retry</li>
            <li>Background worker retries every 30 seconds</li>
          </ul>
        </div>
      </div>
    </div>
  </BaseCard>
</template>
