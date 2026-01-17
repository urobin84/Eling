<script setup lang="ts">
import { ref, computed } from 'vue';

// Define Props
interface User {
    id: number;
    username: string;
    role: string;
    created_at?: string;
    email?: string;
}

const props = defineProps<{
    users: User[];
}>();

const emit = defineEmits(['delete', 'edit', 'view', 'refresh']);

// State
const searchQuery = ref('');
const currentPage = ref(1);
const itemsPerPage = ref(10);
const selectedIds = ref<number[]>([]);

// Create Candidate State
const showCreateModal = ref(false);
const isLoading = ref(false);
const showSuccessModal = ref(false);
const newCandidate = ref({
    username: '',
    password: '',
    name: ''
});

// Import invoke
import { invoke } from '@tauri-apps/api/core';

async function handleCreateCandidate() {
    if (!newCandidate.value.username || !newCandidate.value.password) {
        alert("Username and password are required");
        return;
    }

    isLoading.value = true;
    try {
        await invoke('create_candidate', {
            username: newCandidate.value.username,
            password: newCandidate.value.password,
            name: newCandidate.value.name || null
        });

        // Success
        showCreateModal.value = false;
        showSuccessModal.value = true;

        // Reset form
        newCandidate.value = { username: '', password: '', name: '' };

        // Emit refresh to parent
        emit('refresh');

        // Hide success modal after delay
        setTimeout(() => {
            showSuccessModal.value = false;
        }, 2000);

    } catch (e) {
        alert("Failed to create candidate: " + e);
    } finally {
        isLoading.value = false;
    }
}

// Headers
const headers = [
    { key: 'username', label: 'CANDIDATE NAME' },
    { key: 'id', label: 'ID NUMBER' },
    { key: 'role', label: 'ROLE' },
    { key: 'actions', label: 'ACTIONS' }
];

// Computed
const filteredUsers = computed(() => {
    let result = props.users.filter(u => u.role === 'participant'); // Match DB schema role 'participant'

    if (searchQuery.value) {
        const q = searchQuery.value.toLowerCase();
        result = result.filter(u =>
            u.username.toLowerCase().includes(q) ||
            u.id.toString().includes(q)
        );
    }
    return result;
});

const totalPages = computed(() => Math.ceil(filteredUsers.value.length / itemsPerPage.value));

const paginatedUsers = computed(() => {
    const start = (currentPage.value - 1) * itemsPerPage.value;
    const end = start + itemsPerPage.value;
    return filteredUsers.value.slice(start, end);
});

const isAllSelected = computed(() => {
    return paginatedUsers.value.length > 0 && selectedIds.value.length === paginatedUsers.value.length;
});

// Methods
function toggleSelectAll() {
    if (isAllSelected.value) {
        selectedIds.value = [];
    } else {
        selectedIds.value = paginatedUsers.value.map(u => u.id);
    }
}

function toggleSelect(id: number) {
    if (selectedIds.value.includes(id)) {
        selectedIds.value = selectedIds.value.filter(x => x !== id);
    } else {
        selectedIds.value.push(id);
    }
}

function deleteSelected() {
    if (confirm(`Are you sure you want to delete ${selectedIds.value.length} users?`)) {
        // In real app, emit bulk delete
        console.log("Delete bulk:", selectedIds.value);
    }
}
</script>

<template>
    <div class="space-y-6">
        <!-- Header Controls -->
        <div class="flex flex-col sm:flex-row justify-between items-center gap-4">
            <!-- Search -->
            <div class="relative w-full sm:w-96">
                <input v-model="searchQuery" type="text" placeholder="Search candidates..."
                    class="input-glass w-full pl-11 py-2.5 text-sm">
                <svg class="w-4 h-4 text-gray-500 dark:text-gray-400 absolute left-4 top-1/2 -translate-y-1/2 pointer-events-none" fill="none"
                    viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                        d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
                </svg>
            </div>

            <!-- Actions -->
            <div class="flex items-center space-x-3 w-full sm:w-auto">
                <button @click="showCreateModal = true"
                    class="btn-neumorphic text-sm py-2.5 px-5 flex items-center shadow-eling-emerald/20 font-bold whitespace-nowrap">
                    <svg class="w-5 h-5 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
                    </svg>
                    New Candidate
                </button>

                <div v-if="selectedIds.length > 0" class="flex items-center space-x-2 animate-fade-in">
                    <span class="text-xs text-gray-900 dark:text-eling-dark-text/70 font-mono">{{ selectedIds.length }}
                        Selected</span>
                    <button @click="deleteSelected"
                        class="px-3 py-2 rounded-lg bg-red-500/10 text-red-400 border border-red-500/20 hover:bg-red-500/20 text-xs font-bold uppercase tracking-wider transition-colors">
                        Delete
                    </button>
                </div>

                <!-- Per Page Removed -->
            </div>
        </div>

        <!-- Table -->
        <div class="glass-panel overflow-hidden border-black/5 dark:border-white/5 relative">
            <div class="overflow-x-auto">
                <table class="min-w-full divide-y divide-white/10">
                    <thead class="bg-black/5 dark:bg-white/5">
                        <tr>
                            <th scope="col" class="px-6 py-3 text-left">
                                <input type="checkbox" :checked="isAllSelected" @change="toggleSelectAll"
                                    class="rounded border-black/20 dark:border-white/20 bg-white/10 text-eling-emerald focus:ring-eling-emerald">
                            </th>
                            <th v-for="head in headers" :key="head.key" scope="col"
                                class="px-6 py-3 text-left text-xs font-mono font-medium text-gray-900 dark:text-eling-dark-text/50 uppercase tracking-wider">
                                {{ head.label }}
                            </th>
                        </tr>
                    </thead>
                    <tbody class="divide-y divide-white/5 bg-transparent">
                        <tr v-if="paginatedUsers.length === 0">
                            <td colspan="5"
                                class="px-6 py-12 text-center text-gray-900 dark:text-eling-dark-text/30 text-sm">
                                No candidates found matching your query.
                            </td>
                        </tr>
                        <tr v-for="user in paginatedUsers" :key="user.id"
                            class="hover:bg-black/5 dark:bg-white/5 transition-colors group"
                            :class="{ 'bg-eling-emerald/5': selectedIds.includes(user.id) }">
                            <td class="px-6 py-4 whitespace-nowrap">
                                <input type="checkbox" :checked="selectedIds.includes(user.id)"
                                    @change="toggleSelect(user.id)"
                                    class="rounded border-black/20 dark:border-white/20 bg-white/10 text-eling-emerald focus:ring-eling-emerald">
                            </td>
                            <td class="px-6 py-4 whitespace-nowrap">
                                <div class="flex items-center">
                                    <div
                                        class="w-8 h-8 rounded-full bg-eling-light-surface border border-black/10 dark:border-white/10 flex items-center justify-center text-xs font-bold text-gray-900 dark:text-eling-dark-text mr-3">
                                        {{ user.username.charAt(0).toUpperCase() }}
                                    </div>
                                    <div class="text-sm font-medium text-gray-900 dark:text-eling-dark-text">{{
                                        user.username }}</div>
                                </div>
                            </td>
                            <td
                                class="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-eling-dark-text/60 font-mono">
                                #{{ user.id.toString().padStart(4, '0') }}
                            </td>
                            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-eling-dark-text/60">
                                <span
                                    class="px-2 py-0.5 rounded text-[10px] font-bold uppercase tracking-wide bg-black/5 dark:bg-white/5 border border-black/10 dark:border-white/10 text-gray-900 dark:text-eling-dark-text/50">
                                    {{ user.role }}
                                </span>
                            </td>
                            <td class="px-6 py-4 whitespace-nowrap text-sm font-medium">
                                <div
                                    class="flex items-center space-x-3 opacity-0 group-hover:opacity-100 transition-opacity">
                                    <button @click="$emit('view', user)"
                                        class="text-eling-emerald hover:text-white transition-colors"
                                        title="View Details">
                                        <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                                d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                                d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
                                        </svg>
                                    </button>
                                    <button @click="$emit('edit', user)"
                                        class="text-blue-400 hover:text-white transition-colors" title="Edit">
                                        <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                                d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
                                        </svg>
                                    </button>
                                    <button @click="$emit('delete', user.id)"
                                        class="text-red-400 hover:text-red-200 transition-colors" title="Delete">
                                        <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                                d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                                        </svg>
                                    </button>
                                </div>
                            </td>
                        </tr>
                    </tbody>
                </table>
            </div>

            <!-- Pagination Footer -->
            <div
                class="bg-black/5 dark:bg-white/5 border-t border-black/10 dark:border-white/10 px-4 py-3 flex items-center justify-between sm:px-6">
                <div class="hidden sm:flex-1 sm:flex sm:items-center sm:justify-between">
                    <div class="flex items-center gap-3">
                        <select v-model="itemsPerPage"
                            class="input-glass w-16 py-1 px-2 text-xs h-8 focus:ring-0">
                            <option :value="5">5</option>
                            <option :value="10">10</option>
                            <option :value="25">25</option>
                            <option :value="50">50</option>
                        </select>
                        <p class="text-sm text-gray-900 dark:text-eling-dark-text/50 font-mono">
                            Showing <span class="font-medium text-gray-900 dark:text-eling-dark-text">{{ (currentPage - 1) *
                                itemsPerPage + 1
                                }}</span> to <span class="font-medium text-gray-900 dark:text-eling-dark-text">{{
                                    Math.min(currentPage *
                                        itemsPerPage, filteredUsers.length) }}</span> of <span
                                class="font-medium text-gray-900 dark:text-eling-dark-text">{{ filteredUsers.length
                                }}</span> results
                        </p>
                    </div>
                    <div>
                        <nav class="relative z-0 inline-flex rounded-md shadow-sm -space-x-px" aria-label="Pagination">
                            <button @click="currentPage > 1 ? currentPage-- : null" :disabled="currentPage === 1"
                                class="relative inline-flex items-center px-2 py-2 rounded-l-md border border-black/10 dark:border-white/10 bg-black/5 dark:bg-white/5 text-sm font-medium text-gray-900 dark:text-eling-dark-text/70 hover:bg-white/10 disabled:opacity-50">
                                <span class="sr-only">Previous</span>
                                <svg class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                                    <path fill-rule="evenodd"
                                        d="M12.707 5.293a1 1 0 010 1.414L9.414 10l3.293 3.293a1 1 0 01-1.414 1.414l-4-4a1 1 0 010-1.414l4-4a1 1 0 011.414 0z"
                                        clip-rule="evenodd" />
                                </svg>
                            </button>
                            <button v-for="p in totalPages" :key="p" @click="currentPage = p"
                                class="relative inline-flex items-center px-4 py-2 border border-black/10 dark:border-white/10 text-sm font-medium"
                                :class="p === currentPage ? 'bg-eling-emerald text-eling-dark font-bold' : 'bg-black/5 dark:bg-white/5 text-gray-900 dark:text-eling-dark-text/70 hover:bg-white/10'">
                                {{ p }}
                            </button>
                            <button @click="currentPage < totalPages ? currentPage++ : null"
                                :disabled="currentPage === totalPages"
                                class="relative inline-flex items-center px-2 py-2 rounded-r-md border border-black/10 dark:border-white/10 bg-black/5 dark:bg-white/5 text-sm font-medium text-gray-900 dark:text-eling-dark-text/70 hover:bg-white/10 disabled:opacity-50">
                                <span class="sr-only">Next</span>
                                <svg class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                                    <path fill-rule="evenodd"
                                        d="M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 011.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z"
                                        clip-rule="evenodd" />
                                </svg>
                            </button>
                        </nav>
                    </div>
                </div>
            </div>
        </div>

        <!-- Create Modal -->
        <div v-if="showCreateModal" class="fixed inset-0 z-50 flex items-center justify-center px-4">
            <div class="absolute inset-0 bg-black/80 backdrop-blur-sm" @click="showCreateModal = false"></div>
            <div
                class="glass-panel w-full max-w-md p-6 relative z-10 bg-eling-light-surface border-eling-emerald/20 shadow-2xl transform transition-all">
                <h3 class="text-lg font-bold text-gray-900 dark:text-eling-dark-text mb-1">New Candidate</h3>
                <p class="text-xs text-gray-900 dark:text-eling-dark-text/50 mb-6">Register a new examinee into the system.
                </p>

                <div class="space-y-4">
                    <div>
                        <label class="block text-xs font-mono text-eling-emerald mb-1 uppercase">Username
                            (Required)</label>
                        <input v-model="newCandidate.username" type="text"
                            class="input-glass w-full">
                    </div>
                    <div>
                        <label
                            class="block text-xs font-mono text-gray-900 dark:text-eling-dark-text/70 mb-1 uppercase">Full
                            Name
                            (Optional)</label>
                        <input v-model="newCandidate.name" type="text"
                            class="input-glass w-full">
                    </div>
                    <div>
                        <label
                            class="block text-xs font-mono text-gray-900 dark:text-eling-dark-text/70 mb-1 uppercase">Password
                            (Required)</label>
                        <input v-model="newCandidate.password" type="password"
                            class="input-glass w-full">
                    </div>
                </div>

                <div class="mt-8 flex justify-end space-x-3">
                    <button @click="showCreateModal = false"
                        class="px-4 py-2 rounded-lg text-sm text-gray-900 dark:text-eling-dark-text/70 hover:bg-black/5 dark:bg-white/5 transition-colors">Cancel</button>
                    <button @click="handleCreateCandidate" class="btn-neumorphic px-6 py-2 text-sm font-bold">Create
                        Profile</button>
                </div>
            </div>
        </div>

        <!-- Loading Overlay -->
        <div v-if="isLoading"
            class="fixed inset-0 z-[60] flex items-center justify-center bg-black/60 backdrop-blur-sm">
            <div class="glass-panel p-8 flex flex-col items-center">
                <div
                    class="w-12 h-12 border-4 border-eling-emerald/30 border-t-eling-emerald rounded-full animate-spin mb-4">
                </div>
                <p class="text-gray-900 dark:text-eling-dark-text font-mono text-sm animate-pulse">Processing...</p>
            </div>
        </div>

        <!-- Success Modal -->
        <div v-if="showSuccessModal" class="fixed inset-0 z-[60] flex items-center justify-center px-4">
            <div class="absolute inset-0 bg-black/60 backdrop-blur-sm"></div>
            <div
                class="glass-panel p-8 flex flex-col items-center text-center animate-bounce-in relative z-10 border-eling-emerald/50 shadow-[0_0_50px_rgba(0,230,118,0.2)]">
                <div
                    class="w-16 h-16 rounded-full bg-eling-emerald/10 flex items-center justify-center mb-4 text-eling-emerald border border-eling-emerald/20">
                    <svg class="w-8 h-8" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M5 13l4 4L19 7" />
                    </svg>
                </div>
                <h3 class="text-xl font-bold text-white mb-2">Success!</h3>
                <p class="text-gray-900 dark:text-eling-dark-text/70 text-sm">Candidate profile created successfully.</p>
            </div>
        </div>
    </div>
</template>
