<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import BaseCard from '@/components/atoms/BaseCard.vue';
import LoadingSpinner from '@/components/atoms/LoadingSpinner.vue';
import BaseButton from '@/components/atoms/BaseButton.vue';
import RecordingPlayback from './RecordingPlayback.vue';

interface ActiveSession {
    session_id: string;
    user_id: number;
    username: string;
    event_id: number | null;
    event_name: string | null;
    test_name: string;
    started_at: number;
    last_frame: string | null;
    last_update: number;
}

const sessions = ref<ActiveSession[]>([]);
const loading = ref(true);
const selectedSession = ref<ActiveSession | null>(null);
const showFullscreen = ref(false);
const showRecordingPlayback = ref(false);
const playbackSessionId = ref<string>('');
let pollInterval: number | null = null;

// Fetch active sessions
async function fetchSessions() {
    try {
        sessions.value = await invoke<ActiveSession[]>('get_active_sessions');
        loading.value = false;
    } catch (e) {
        console.error('Failed to fetch sessions:', e);
        loading.value = false;
    }
}

// Get duration in MM:SS format
function getDuration(startedAt: number): string {
    const now = Math.floor(Date.now() / 1000);
    const elapsed = now - startedAt;
    const minutes = Math.floor(elapsed / 60);
    const seconds = elapsed % 60;
    return `${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`;
}

// Check if session is active (updated in last 30 seconds)
function isSessionActive(session: ActiveSession): boolean {
    const now = Math.floor(Date.now() / 1000);
    return (now - session.last_update) < 30;
}

// Grid columns based on session count
const gridCols = computed(() => {
    const count = sessions.value.length;
    if (count <= 4) return 'grid-cols-2';
    if (count <= 9) return 'grid-cols-3';
    return 'grid-cols-4';
});

// Open fullscreen view
function openFullscreen(session: ActiveSession) {
    selectedSession.value = session;
    showFullscreen.value = true;
}

// Close fullscreen view
function closeFullscreen() {
    showFullscreen.value = false;
    selectedSession.value = null;
}

// Cleanup stale sessions
async function cleanupStaleSessions() {
    try {
        await invoke('cleanup_stale_sessions');
    } catch (e) {
        console.error('Failed to cleanup sessions:', e);
    }
}

// Open recording playback
function openRecordingPlayback(sessionId: string) {
    playbackSessionId.value = sessionId;
    showRecordingPlayback.value = true;
    showFullscreen.value = false;
}

// Close recording playback
function closeRecordingPlayback() {
    showRecordingPlayback.value = false;
    playbackSessionId.value = '';
}

onMounted(() => {
    fetchSessions();
    // Poll every 5 seconds
    pollInterval = window.setInterval(() => {
        fetchSessions();
        cleanupStaleSessions();
    }, 5000);
});

onUnmounted(() => {
    if (pollInterval) {
        clearInterval(pollInterval);
    }
});
</script>

<template>
    <div class="space-y-6">
        <!-- Header -->
        <div>
            <h2 class="text-2xl font-bold text-gray-900 dark:text-eling-dark-text">Camera Monitoring</h2>
            <p class="text-sm text-gray-900 dark:text-eling-dark-text/50 mt-1">
                Real-time surveillance of active test sessions
            </p>
        </div>

        <!-- Loading State -->
        <div v-if="loading" class="flex items-center justify-center py-20">
            <LoadingSpinner size="lg" />
        </div>

        <!-- Empty State -->
        <div v-else-if="sessions.length === 0" class="text-center py-20">
            <BaseCard padding="lg">
                <div class="flex flex-col items-center justify-center">
                    <svg class="w-16 h-16 text-gray-400 dark:text-eling-dark-text/30 mb-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 10l4.553-2.276A1 1 0 0121 8.618v6.764a1 1 0 01-1.447.894L15 14M5 18h8a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z" />
                    </svg>
                    <h3 class="text-lg font-bold text-gray-900 dark:text-eling-dark-text mb-2">No Active Sessions</h3>
                    <p class="text-sm text-gray-600 dark:text-eling-dark-text/70">
                        Camera feeds will appear here when candidates start their tests
                    </p>
                </div>
            </BaseCard>
        </div>

        <!-- Session Grid -->
        <div v-else class="grid gap-4" :class="gridCols">
            <BaseCard 
                v-for="session in sessions" 
                :key="session.session_id"
                padding="sm"
                class="cursor-pointer hover:ring-2 hover:ring-eling-emerald transition-all"
                @click="openFullscreen(session)"
            >
                <div class="space-y-2">
                    <!-- Camera Feed -->
                    <div class="relative aspect-video bg-black/90 rounded-lg overflow-hidden">
                        <img 
                            v-if="session.last_frame" 
                            :src="'data:image/jpeg;base64,' + session.last_frame"
                            class="w-full h-full object-cover"
                            alt="Camera feed"
                        />
                        <div v-else class="w-full h-full flex items-center justify-center">
                            <span class="text-xs text-white/50 font-mono">NO SIGNAL</span>
                        </div>

                        <!-- Status Indicator -->
                        <div class="absolute top-2 right-2 flex items-center gap-1 px-2 py-1 rounded-full glass-panel">
                            <span 
                                class="w-2 h-2 rounded-full"
                                :class="isSessionActive(session) ? 'bg-green-500 animate-pulse' : 'bg-red-500'"
                            ></span>
                            <span class="text-[10px] font-mono text-white">
                                {{ isSessionActive(session) ? 'LIVE' : 'OFFLINE' }}
                            </span>
                        </div>

                        <!-- Duration -->
                        <div class="absolute bottom-2 left-2 px-2 py-1 rounded glass-panel">
                            <span class="text-xs font-mono text-white">
                                {{ getDuration(session.started_at) }}
                            </span>
                        </div>
                    </div>

                    <!-- Session Info -->
                    <div class="space-y-1">
                        <div class="flex items-center justify-between">
                            <span class="text-sm font-bold text-gray-900 dark:text-eling-dark-text">
                                {{ session.username }}
                            </span>
                            <span class="text-xs text-gray-500 dark:text-eling-dark-text/50">
                                ID: {{ session.user_id }}
                            </span>
                        </div>
                        <div class="text-xs text-gray-600 dark:text-eling-dark-text/70">
                            {{ session.test_name }}
                        </div>
                        <div v-if="session.event_name" class="text-xs text-eling-emerald">
                            {{ session.event_name }}
                        </div>
                    </div>
                </div>
            </BaseCard>
        </div>

        <!-- Fullscreen Modal -->
        <div 
            v-if="showFullscreen && selectedSession"
            class="fixed inset-0 bg-black/90 z-[9999] flex items-center justify-center p-8"
            @click.self="closeFullscreen"
        >
            <div class="max-w-4xl w-full space-y-4">
                <!-- Close Button -->
                <div class="flex justify-between items-center">
                    <h3 class="text-xl font-bold text-white">{{ selectedSession.username }}</h3>
                    <button 
                        @click="closeFullscreen"
                        class="p-2 rounded-lg hover:bg-white/10 transition-colors"
                    >
                        <svg class="w-6 h-6 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                        </svg>
                    </button>
                </div>

                <!-- Large Feed -->
                <div class="relative aspect-video bg-black rounded-lg overflow-hidden">
                    <img 
                        v-if="selectedSession.last_frame" 
                        :src="'data:image/jpeg;base64,' + selectedSession.last_frame"
                        class="w-full h-full object-contain"
                        alt="Camera feed"
                    />
                    <div v-else class="w-full h-full flex items-center justify-center">
                        <span class="text-lg text-white/50 font-mono">NO SIGNAL</span>
                    </div>

                    <!-- Status Overlay -->
                    <div class="absolute top-4 right-4 flex items-center gap-2 px-3 py-2 rounded-lg glass-panel">
                        <span 
                            class="w-3 h-3 rounded-full"
                            :class="isSessionActive(selectedSession) ? 'bg-green-500 animate-pulse' : 'bg-red-500'"
                        ></span>
                        <span class="text-sm font-mono text-white font-bold">
                            {{ isSessionActive(selectedSession) ? 'LIVE' : 'OFFLINE' }}
                        </span>
                    </div>
                </div>

                <!-- Session Details -->
                <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
                    <div class="glass-panel p-4 rounded-lg">
                        <div class="text-xs text-white/50 mb-1">Candidate</div>
                        <div class="text-sm font-bold text-white">{{ selectedSession.username }}</div>
                    </div>
                    <div class="glass-panel p-4 rounded-lg">
                        <div class="text-xs text-white/50 mb-1">Test</div>
                        <div class="text-sm font-bold text-white">{{ selectedSession.test_name }}</div>
                    </div>
                    <div class="glass-panel p-4 rounded-lg">
                        <div class="text-xs text-white/50 mb-1">Event</div>
                        <div class="text-sm font-bold text-white">{{ selectedSession.event_name || 'N/A' }}</div>
                    </div>
                    <div class="glass-panel p-4 rounded-lg">
                        <div class="text-xs text-white/50 mb-1">Duration</div>
                        <div class="text-sm font-bold text-white font-mono">{{ getDuration(selectedSession.started_at) }}</div>
                    </div>
                </div>

                <!-- View Recording Button -->
                <div class="flex justify-center mt-4">
                    <BaseButton 
                        variant="primary" 
                        @click="openRecordingPlayback(selectedSession.session_id)"
                    >
                        📹 View Saved Recording
                    </BaseButton>
                </div>
            </div>
        </div>

        <!-- Recording Playback Modal -->
        <RecordingPlayback 
            v-if="showRecordingPlayback"
            :session-id="playbackSessionId"
            @close="closeRecordingPlayback"
        />
    </div>
</template>
