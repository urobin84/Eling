<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import BaseCard from '@/components/atoms/BaseCard.vue';
import BaseButton from '@/components/atoms/BaseButton.vue';

interface RecordingMetadata {
    session_id: string;
    user_id: number;
    username: string;
    test_name: string;
    event_name: string | null;
    started_at: number;
    ended_at: number | null;
    duration: number | null;
    camera_file: string | null;
    screen_file: string | null;
    camera_size: number | null;
    screen_size: number | null;
}

const props = defineProps<{
    sessionId: string;
}>();

const emit = defineEmits<{
    close: [];
}>();

const metadata = ref<RecordingMetadata | null>(null);
const cameraVideoUrl = ref<string | null>(null);
const screenVideoUrl = ref<string | null>(null);
const loading = ref(true);
const isLoadingVideo = ref(false);
const error = ref<string>('');
const viewMode = ref<'split' | 'camera' | 'screen'>('split');

// Load recording data
async function loadRecording() {
    try {
        loading.value = true;
        error.value = '';

        // Get metadata
        metadata.value = await invoke<RecordingMetadata>('get_session_recordings', {
            sessionId: props.sessionId
        });

        // Load video data as Blobs for maximum compatibility/security
        isLoadingVideo.value = true;
        
        if (metadata.value.camera_file) {
            try {
                const cameraData = await invoke<number[]>('get_recording_data', {
                    sessionId: props.sessionId,
                    videoType: 'camera'
                });
                const blob = new Blob([new Uint8Array(cameraData)], { type: 'video/webm' });
                cameraVideoUrl.value = URL.createObjectURL(blob);
            } catch (e: any) {
                console.error('Failed to load camera video:', e);
            }
        }

        if (metadata.value.screen_file) {
            try {
                const screenData = await invoke<number[]>('get_recording_data', {
                    sessionId: props.sessionId,
                    videoType: 'screen'
                });
                const blob = new Blob([new Uint8Array(screenData)], { type: 'video/webm' });
                screenVideoUrl.value = URL.createObjectURL(blob);
            } catch (e: any) {
                console.error('Failed to load screen video:', e);
            }
        }

        loading.value = false;
    } catch (e: any) {
        error.value = e.toString();
        loading.value = false;
    } finally {
        isLoadingVideo.value = false;
    }
}

// Delete recording
async function deleteRecording() {
    if (!confirm('Are you sure you want to delete this recording? This action cannot be undone.')) {
        return;
    }

    try {
        await invoke('delete_session_recording', {
            sessionId: props.sessionId
        });
        emit('close');
    } catch (e: any) {
        error.value = `Failed to delete recording: ${e}`;
    }
}

// Format duration
function formatDuration(seconds: number | null): string {
    if (!seconds) return 'N/A';
    const mins = Math.floor(seconds / 60);
    const secs = seconds % 60;
    return `${mins}m ${secs}s`;
}

// Format file size
function formatFileSize(bytes: number | null): string {
    if (!bytes) return 'N/A';
    const mb = bytes / (1024 * 1024);
    return `${mb.toFixed(2)} MB`;
}

// Format date
function formatDate(timestamp: number): string {
    return new Date(timestamp * 1000).toLocaleString();
}

onMounted(loadRecording);

onUnmounted(() => {
    if (cameraVideoUrl.value) URL.revokeObjectURL(cameraVideoUrl.value);
    if (screenVideoUrl.value) URL.revokeObjectURL(screenVideoUrl.value);
});
</script>

<template>
    <div class="fixed inset-0 bg-black/90 z-[9999] flex items-center justify-center p-4">
        <div class="max-w-7xl w-full h-[90vh] flex flex-col">
            <!-- Header -->
            <div class="flex items-center justify-between mb-4">
                <div>
                    <h2 class="text-2xl font-bold text-white">Recording Playback</h2>
                    <p v-if="metadata" class="text-sm text-white/60 mt-1">
                        {{ metadata.username }} - {{ metadata.test_name }}
                    </p>
                </div>
                <button 
                    @click="emit('close')"
                    class="p-2 rounded-lg hover:bg-white/10 transition-colors"
                >
                    <svg class="w-6 h-6 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                    </svg>
                </button>
            </div>

            <!-- Loading State -->
            <div v-if="loading" class="flex-1 flex items-center justify-center">
                <div class="text-white">Loading recording...</div>
            </div>

            <!-- Error State -->
            <div v-else-if="error" class="flex-1 flex items-center justify-center">
                <BaseCard padding="lg">
                    <div class="text-center">
                        <svg class="w-16 h-16 text-red-400 mx-auto mb-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                        </svg>
                        <h3 class="text-lg font-bold text-gray-900 dark:text-eling-dark-text mb-2">Error Loading Recording</h3>
                        <p class="text-sm text-gray-600 dark:text-eling-dark-text/70">{{ error }}</p>
                    </div>
                </BaseCard>
            </div>

            <!-- Video Player -->
            <div v-else class="flex-1 flex flex-col gap-4">
                <!-- View Mode Toggle -->
                <div class="flex gap-2">
                    <button
                        @click="viewMode = 'split'"
                        class="px-4 py-2 rounded-lg text-sm font-medium transition-colors"
                        :class="viewMode === 'split' ? 'bg-eling-emerald text-white' : 'bg-white/10 text-white/70 hover:bg-white/20'"
                    >
                        Split View
                    </button>
                    <button
                        @click="viewMode = 'camera'"
                        class="px-4 py-2 rounded-lg text-sm font-medium transition-colors"
                        :class="viewMode === 'camera' ? 'bg-eling-emerald text-white' : 'bg-white/10 text-white/70 hover:bg-white/20'"
                    >
                        Camera Only
                    </button>
                    <button
                        @click="viewMode = 'screen'"
                        class="px-4 py-2 rounded-lg text-sm font-medium transition-colors"
                        :class="viewMode === 'screen' ? 'bg-eling-emerald text-white' : 'bg-white/10 text-white/70 hover:bg-white/20'"
                    >
                        Screen Only
                    </button>
                </div>

                <!-- Videos -->
                <div class="flex-1 grid gap-4" :class="{
                    'grid-cols-2': viewMode === 'split',
                    'grid-cols-1': viewMode !== 'split'
                }">
                    <!-- Camera Video -->
                    <div v-if="viewMode === 'split' || viewMode === 'camera'" class="bg-black rounded-lg overflow-hidden flex flex-col">
                        <div class="bg-white/5 px-4 py-2 border-b border-white/10">
                            <span class="text-sm font-medium text-white">Camera Feed</span>
                        </div>
                        <div class="flex-1 flex items-center justify-center">
                            <video 
                                v-if="cameraVideoUrl"
                                controls
                                preload="metadata"
                                playsinline
                                crossorigin="anonymous"
                                class="w-full h-full object-contain"
                            >
                                <source :src="cameraVideoUrl" type="video/webm">
                                Your browser does not support the video tag.
                            </video>
                            <div v-else class="text-white/50">No camera recording</div>
                        </div>
                    </div>

                    <!-- Screen Video -->
                    <div v-if="viewMode === 'split' || viewMode === 'screen'" class="bg-black rounded-lg overflow-hidden flex flex-col">
                        <div class="bg-white/5 px-4 py-2 border-b border-white/10">
                            <span class="text-sm font-medium text-white">Screen Recording</span>
                        </div>
                        <div class="flex-1 flex items-center justify-center">
                            <video 
                                v-if="screenVideoUrl"
                                controls
                                preload="metadata"
                                playsinline
                                crossorigin="anonymous"
                                class="w-full h-full object-contain"
                            >
                                <source :src="screenVideoUrl" type="video/webm">
                                Your browser does not support the video tag.
                            </video>
                            <div v-else class="text-white/50">No screen recording</div>
                        </div>
                    </div>
                </div>

                <!-- Metadata & Actions -->
                <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
                    <BaseCard padding="sm">
                        <div class="text-xs text-gray-600 dark:text-eling-dark-text/50 mb-1">Started</div>
                        <div class="text-sm font-bold text-gray-900 dark:text-eling-dark-text">
                            {{ metadata ? formatDate(metadata.started_at) : 'N/A' }}
                        </div>
                    </BaseCard>
                    <BaseCard padding="sm">
                        <div class="text-xs text-gray-600 dark:text-eling-dark-text/50 mb-1">Duration</div>
                        <div class="text-sm font-bold text-gray-900 dark:text-eling-dark-text">
                            {{ metadata ? formatDuration(metadata.duration) : 'N/A' }}
                        </div>
                    </BaseCard>
                    <BaseCard padding="sm">
                        <div class="text-xs text-gray-600 dark:text-eling-dark-text/50 mb-1">Camera Size</div>
                        <div class="text-sm font-bold text-gray-900 dark:text-eling-dark-text">
                            {{ metadata ? formatFileSize(metadata.camera_size) : 'N/A' }}
                        </div>
                    </BaseCard>
                    <BaseCard padding="sm">
                        <div class="text-xs text-gray-600 dark:text-eling-dark-text/50 mb-1">Screen Size</div>
                        <div class="text-sm font-bold text-gray-900 dark:text-eling-dark-text">
                            {{ metadata ? formatFileSize(metadata.screen_size) : 'N/A' }}
                        </div>
                    </BaseCard>
                </div>

                <!-- Actions -->
                <div class="flex gap-2 justify-end">
                    <BaseButton variant="danger" @click="deleteRecording">
                        Delete Recording
                    </BaseButton>
                    <BaseButton variant="ghost" @click="emit('close')">
                        Close
                    </BaseButton>
                </div>
            </div>
        </div>
    </div>
</template>
