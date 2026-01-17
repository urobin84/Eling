<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';

interface EventDetails {
    id: number;
    event_name: string;
    description: string | null;
    event_code: string | null;
    status: string;
    max_participants: number | null;
    enrollment_deadline: string | null;
    created_at: string;
    participant_count: number;
}

interface Participant {
    user_id: number;
    username: string;
    email: string | null;
    status: string;
    enrolled_at: string;
    completed_at: string | null;
}

interface User {
    id: number;
    username: string;
    email?: string;
    role: string;
}

const props = defineProps<{
    eventId: number;
    allUsers: User[];
}>();

const emit = defineEmits(['close', 'refresh']);

// State
const eventDetails = ref<EventDetails | null>(null);
const participants = ref<Participant[]>([]);
const isLoading = ref(true);
const activeTab = ref<'overview' | 'participants'>('overview');
const showAddParticipant = ref(false);
const selectedUserId = ref<number | null>(null);
const searchQuery = ref('');
const generatingCode = ref(false);

// Computed
const availableCandidates = computed(() => {
    const participantIds = new Set(participants.value.map(p => p.user_id));
    return props.allUsers
        .filter(u => u.role === 'participant' && !participantIds.has(u.id))
        .filter(u => {
            if (!searchQuery.value) return true;
            const query = searchQuery.value.toLowerCase();
            return u.username.toLowerCase().includes(query) ||
                u.email?.toLowerCase().includes(query);
        });
});

const statusColor = (status: string) => {
    switch (status) {
        case 'enrolled': return 'bg-blue-500/20 text-blue-400 border-blue-500/30';
        case 'in_progress': return 'bg-yellow-500/20 text-yellow-400 border-yellow-500/30';
        case 'completed': return 'bg-green-500/20 text-green-400 border-green-500/30';
        case 'withdrawn': return 'bg-red-500/20 text-red-400 border-red-500/30';
        default: return 'bg-gray-500/20 text-gray-400 border-gray-500/30';
    }
};

// Methods
async function fetchEventDetails() {
    try {
        isLoading.value = true;
        const details = await invoke<EventDetails>('get_event_details', {
            eventId: props.eventId
        });
        eventDetails.value = details;
    } catch (error) {
        console.error('Failed to fetch event details:', error);
        alert('Failed to load event details');
    } finally {
        isLoading.value = false;
    }
}

async function fetchParticipants() {
    try {
        const parts = await invoke<Participant[]>('get_event_participants', {
            eventId: props.eventId
        });
        participants.value = parts;
    } catch (error) {
        console.error('Failed to fetch participants:', error);
    }
}

async function generateEventCode() {
    console.log('generateEventCode called, eventId:', props.eventId);

    if (!confirm('Generate a new event code? This will replace the existing code.')) {
        console.log('User cancelled generation');
        return;
    }

    try {
        generatingCode.value = true;
        console.log('Calling generate_event_code_cmd with eventId:', props.eventId);

        const code = await invoke<string>('generate_event_code_cmd', {
            eventId: props.eventId
        });

        console.log('Generated code:', code);

        if (eventDetails.value) {
            eventDetails.value.event_code = code;
        }
        alert(`New event code generated: ${code}`);
    } catch (error) {
        console.error('Failed to generate code:', error);
        alert(`Failed to generate event code: ${error}`);
    } finally {
        generatingCode.value = false;
    }
}

async function addParticipant() {
    if (!selectedUserId.value) return;

    try {
        await invoke('add_participant_to_event', {
            eventId: props.eventId,
            userId: selectedUserId.value
        });

        showAddParticipant.value = false;
        selectedUserId.value = null;
        searchQuery.value = '';

        await fetchParticipants();
        await fetchEventDetails();
        emit('refresh');
    } catch (error) {
        console.error('Failed to add participant:', error);
        alert('Failed to add participant');
    }
}

async function removeParticipant(userId: number, username: string) {
    if (!confirm(`Remove ${username} from this event?`)) return;

    try {
        await invoke('remove_participant_from_event', {
            eventId: props.eventId,
            userId: userId
        });

        await fetchParticipants();
        await fetchEventDetails();
        emit('refresh');
    } catch (error) {
        console.error('Failed to remove participant:', error);
        alert('Failed to remove participant');
    }
}

function copyEventCode() {
    if (eventDetails.value?.event_code) {
        navigator.clipboard.writeText(eventDetails.value.event_code);
        alert('Event code copied to clipboard!');
    }
}

onMounted(() => {
    fetchEventDetails();
    fetchParticipants();
});
</script>

<template>
    <div class="fixed inset-0 z-[9999] flex items-center justify-center p-4 bg-black/50 backdrop-blur-sm"
        @click.self="emit('close')">
        <div class="bg-white dark:bg-eling-dark-surface rounded-2xl shadow-2xl max-w-4xl w-full max-h-[90vh] overflow-hidden border border-black/10 dark:border-white/10"
            @click.stop>

            <!-- Header -->
            <div class="bg-gradient-to-r from-eling-emerald to-blue-500 p-6 text-eling-dark">
                <div class="flex justify-between items-start">
                    <div class="flex-1">
                        <h2 class="text-2xl font-bold mb-2">{{ eventDetails?.event_name || 'Loading...' }}</h2>
                        <p v-if="eventDetails?.description" class="text-sm opacity-90">{{ eventDetails.description }}
                        </p>
                    </div>
                    <button @click="emit('close')" class="p-2 hover:bg-black/10 rounded-lg transition-colors">
                        <svg class="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                d="M6 18L18 6M6 6l12 12" />
                        </svg>
                    </button>
                </div>
            </div>

            <!-- Tabs -->
            <div class="border-b border-black/10 dark:border-white/10 bg-black/5 dark:bg-white/5">
                <div class="flex">
                    <button @click="activeTab = 'overview'" class="px-6 py-3 font-medium transition-all"
                        :class="activeTab === 'overview'
                            ? 'text-eling-emerald border-b-2 border-eling-emerald bg-white dark:bg-eling-dark-surface'
                            : 'text-gray-900 dark:text-eling-dark-text/50 hover:text-gray-900 dark:hover:text-eling-dark-text'">
                        Overview
                    </button>
                    <button @click="activeTab = 'participants'" class="px-6 py-3 font-medium transition-all"
                        :class="activeTab === 'participants'
                            ? 'text-eling-emerald border-b-2 border-eling-emerald bg-white dark:bg-eling-dark-surface'
                            : 'text-gray-900 dark:text-eling-dark-text/50 hover:text-gray-900 dark:hover:text-eling-dark-text'">
                        Participants ({{ eventDetails?.participant_count || 0 }})
                    </button>
                </div>
            </div>

            <!-- Content -->
            <div class="p-6 overflow-y-auto max-h-[calc(90vh-200px)]">
                <!-- Loading State -->
                <div v-if="isLoading" class="flex items-center justify-center py-12">
                    <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-eling-emerald"></div>
                </div>

                <!-- Overview Tab -->
                <div v-else-if="activeTab === 'overview'" class="space-y-6">
                    <!-- Event Code Section -->
                    <div
                        class="bg-gradient-to-br from-eling-emerald/10 to-blue-500/10 border border-eling-emerald/30 rounded-xl p-6">
                        <h3 class="text-sm font-bold text-gray-900 dark:text-eling-dark-text mb-4">Event Access Code</h3>
                        <div v-if="eventDetails?.event_code" class="flex items-center gap-4">
                            <div class="flex-1 bg-white dark:bg-eling-dark rounded-lg p-4 border-2 border-eling-emerald">
                                <div class="text-3xl font-mono font-bold text-eling-emerald text-center tracking-widest">
                                    {{ eventDetails.event_code }}
                                </div>
                            </div>
                            <button @click="copyEventCode"
                                class="px-4 py-3 bg-eling-emerald text-eling-dark rounded-lg hover:bg-eling-emerald/90 transition-all font-medium">
                                📋 Copy
                            </button>
                        </div>
                        <div v-else class="text-center py-4">
                            <p class="text-sm text-gray-900 dark:text-eling-dark-text/50 mb-4">No event code generated yet
                            </p>
                            <button @click="generateEventCode" :disabled="generatingCode"
                                class="px-6 py-3 bg-eling-emerald text-eling-dark rounded-lg hover:bg-eling-emerald/90 transition-all font-medium disabled:opacity-50">
                                {{ generatingCode ? 'Generating...' : '🔑 Generate Code' }}
                            </button>
                        </div>
                    </div>

                    <!-- Event Info -->
                    <div class="grid grid-cols-2 gap-4">
                        <div
                            class="bg-black/5 dark:bg-white/5 rounded-lg p-4 border border-black/10 dark:border-white/10">
                            <div class="text-xs text-gray-900 dark:text-eling-dark-text/50 mb-1">Status</div>
                            <div class="text-lg font-bold text-gray-900 dark:text-eling-dark-text capitalize">{{
                                eventDetails?.status }}</div>
                        </div>
                        <div
                            class="bg-black/5 dark:bg-white/5 rounded-lg p-4 border border-black/10 dark:border-white/10">
                            <div class="text-xs text-gray-900 dark:text-eling-dark-text/50 mb-1">Participants</div>
                            <div class="text-lg font-bold text-gray-900 dark:text-eling-dark-text">
                                {{ eventDetails?.participant_count }}
                                <span v-if="eventDetails?.max_participants"
                                    class="text-sm text-gray-900 dark:text-eling-dark-text/50">
                                    / {{ eventDetails.max_participants }}
                                </span>
                            </div>
                        </div>
                        <div
                            class="bg-black/5 dark:bg-white/5 rounded-lg p-4 border border-black/10 dark:border-white/10">
                            <div class="text-xs text-gray-900 dark:text-eling-dark-text/50 mb-1">Created</div>
                            <div class="text-sm font-medium text-gray-900 dark:text-eling-dark-text">
                                {{ new Date(eventDetails?.created_at || '').toLocaleDateString() }}
                            </div>
                        </div>
                        <div
                            class="bg-black/5 dark:bg-white/5 rounded-lg p-4 border border-black/10 dark:border-white/10">
                            <div class="text-xs text-gray-900 dark:text-eling-dark-text/50 mb-1">Enrollment Deadline</div>
                            <div class="text-sm font-medium text-gray-900 dark:text-eling-dark-text">
                                {{ eventDetails?.enrollment_deadline ? new
                                    Date(eventDetails.enrollment_deadline).toLocaleDateString() : 'No deadline' }}
                            </div>
                        </div>
                    </div>
                </div>

                <!-- Participants Tab -->
                <div v-else-if="activeTab === 'participants'" class="space-y-4">
                    <!-- Add Participant Button -->
                    <div class="flex justify-between items-center">
                        <h3 class="text-lg font-bold text-gray-900 dark:text-eling-dark-text">Enrolled Candidates</h3>
                        <button @click="showAddParticipant = !showAddParticipant"
                            class="px-4 py-2 bg-eling-emerald text-eling-dark rounded-lg hover:bg-eling-emerald/90 transition-all font-medium text-sm">
                            {{ showAddParticipant ? '✕ Cancel' : '➕ Add Participant' }}
                        </button>
                    </div>

                    <!-- Add Participant Form -->
                    <div v-if="showAddParticipant"
                        class="bg-black/5 dark:bg-white/5 rounded-lg p-4 border border-black/10 dark:border-white/10">
                        <input v-model="searchQuery" type="text" placeholder="Search candidates..."
                            class="input-glass w-full mb-3">

                        <div class="max-h-48 overflow-y-auto space-y-2">
                            <button v-for="user in availableCandidates" :key="user.id" @click="selectedUserId = user.id"
                                class="w-full text-left px-4 py-2 rounded-lg transition-all border"
                                :class="selectedUserId === user.id
                                    ? 'bg-eling-emerald/20 border-eling-emerald text-gray-900 dark:text-eling-dark-text'
                                    : 'bg-white dark:bg-eling-dark border-black/10 dark:border-white/10 hover:bg-black/5 dark:hover:bg-white/5 text-gray-900 dark:text-eling-dark-text'">
                                <div class="font-medium">{{ user.username }}</div>
                                <div v-if="user.email" class="text-xs text-gray-900 dark:text-eling-dark-text/50">{{
                                    user.email }}</div>
                            </button>
                            <div v-if="availableCandidates.length === 0"
                                class="text-center py-4 text-sm text-gray-900 dark:text-eling-dark-text/50">
                                No available candidates
                            </div>
                        </div>

                        <button @click="addParticipant" :disabled="!selectedUserId"
                            class="w-full mt-3 px-4 py-2 bg-eling-emerald text-eling-dark rounded-lg hover:bg-eling-emerald/90 transition-all font-medium disabled:opacity-50 disabled:cursor-not-allowed">
                            Add Selected Candidate
                        </button>
                    </div>

                    <!-- Participants List -->
                    <div class="space-y-2">
                        <div v-for="participant in participants" :key="participant.user_id"
                            class="flex items-center justify-between p-4 bg-white dark:bg-eling-dark rounded-lg border border-black/10 dark:border-white/10 hover:border-eling-emerald/30 transition-all">
                            <div class="flex-1">
                                <div class="font-medium text-gray-900 dark:text-eling-dark-text">{{ participant.username }}
                                </div>
                                <div v-if="participant.email" class="text-xs text-gray-900 dark:text-eling-dark-text/50">{{
                                    participant.email }}</div>
                                <div class="text-xs text-gray-900 dark:text-eling-dark-text/50 mt-1">
                                    Enrolled: {{ new Date(participant.enrolled_at).toLocaleDateString() }}
                                </div>
                            </div>
                            <div class="flex items-center gap-3">
                                <span class="px-2 py-1 rounded text-xs font-medium border"
                                    :class="statusColor(participant.status)">
                                    {{ participant.status }}
                                </span>
                                <button @click="removeParticipant(participant.user_id, participant.username)"
                                    class="p-2 text-red-400 hover:bg-red-500/10 rounded-lg transition-all">
                                    <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                            d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                                    </svg>
                                </button>
                            </div>
                        </div>
                        <div v-if="participants.length === 0"
                            class="text-center py-8 text-gray-900 dark:text-eling-dark-text/50">
                            No participants enrolled yet
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>
