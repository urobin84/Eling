<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import SearchableDropdown from '@/components/molecules/SearchableDropdown.vue';
import ExportSettingsModal from './ExportSettingsModal.vue';
import BaseModal from '@/components/molecules/BaseModal.vue';
import RecordingPlayback from './RecordingPlayback.vue';

interface TestResult {
    id: number;
    candidate_id: number;
    candidate_name: string;
    event_id: number;
    event_name: string;
    tool_id: number;
    tool_name: string;
    score: number;
    raw_score: number;
    percentile?: number;
    interpretation?: string;
    status: 'pending' | 'completed' | 'reviewed';
    completed_at: string;
    session_id: string;
    recording_id?: string | null;
}

interface Tool {
    id: number;
    name: string;
}

// State
const results = ref<TestResult[]>([]);
const tools = ref<Tool[]>([]);

const loading = ref(true);

// Modals
const showDeleteConfirm = ref(false);
const isDeleting = ref(false);

const statusOptions = [
    { value: 'all', label: 'All Status' },
    { value: 'pending', label: 'Pending' },
    { value: 'completed', label: 'Completed' },
    { value: 'reviewed', label: 'Reviewed' }
];

const toolOptions = computed(() => {
    return [
        { id: null, name: 'All Tools' },
        ...tools.value
    ];
});

// Filters
const searchQuery = ref('');
const selectedTool = ref<number | null>(null);
const selectedStatus = ref<string>('all');
const currentPage = ref(1);
const itemsPerPage = ref(10);

// Selected result for detail modal
const selectedResult = ref<TestResult | null>(null);
const showDetailModal = ref(false);
const showExportModal = ref(false);
const showRecordingPlayback = ref(false);
const isGeneratingReview = ref(false);
const isEditingInterpretation = ref(false);
const editBuffer = ref('');
const isSavingInterpretation = ref(false);

// Selection
const selectedIds = ref<Set<number>>(new Set());

// Computed
const filteredResults = computed(() => {
    let filtered = results.value;

    if (searchQuery.value) {
        const q = searchQuery.value.toLowerCase();
        filtered = filtered.filter(r =>
            r.candidate_name.toLowerCase().includes(q) ||
            r.event_name.toLowerCase().includes(q) ||
            r.tool_name.toLowerCase().includes(q)
        );
    }

    if (selectedTool.value) {
        filtered = filtered.filter(r => r.tool_id === selectedTool.value);
    }

    if (selectedStatus.value !== 'all') {
        filtered = filtered.filter(r => r.status === selectedStatus.value);
    }

    return filtered;
});

const paginatedResults = computed(() => {
    const start = (currentPage.value - 1) * itemsPerPage.value;
    return filteredResults.value.slice(start, start + itemsPerPage.value);
});

const totalPages = computed(() => Math.ceil(filteredResults.value.length / itemsPerPage.value));

// Statistics
const stats = computed(() => ({
    total: results.value.length,
    completed: results.value.filter(r => r.status === 'completed').length,
    reviewed: results.value.filter(r => r.status === 'reviewed').length,
    pending: results.value.filter(r => r.status === 'pending').length,
}));

// Functions
async function fetchData() {
    loading.value = true;
    try {
        // Try to load from backend
        // tools.value = await invoke('get_tools'); // We don't have get_tools yet or it's named differently
        // invoke('get_test_results') returns generic DTO
        const backendResults = await invoke<TestResult[]>('get_test_results');
        results.value = backendResults;

        // Load tools for filter if possible
        const backendTools = await invoke<Tool[]>('get_tools');
        tools.value = backendTools;
    } catch (e) {
        console.error("Failed to load results:", e);
        // Fallback to empty or error state
        results.value = [];
    } finally {
        loading.value = false;
    }
}

function viewDetail(result: TestResult) {
    selectedResult.value = result;
    showDetailModal.value = true;
}

function getStatusColor(status: string) {
    switch (status) {
        case 'reviewed': return 'bg-green-500/10 text-green-400 border-green-500/20';
        case 'completed': return 'bg-blue-500/10 text-blue-400 border-blue-500/20';
        case 'pending': return 'bg-yellow-500/10 text-yellow-400 border-yellow-500/20';
        default: return 'bg-gray-500/10 text-gray-400 border-gray-500/20';
    }
}

function getScoreColor(percentile: number | undefined) {
    if (!percentile) return 'text-gray-400';
    if (percentile >= 90) return 'text-purple-400';
    if (percentile >= 75) return 'text-green-400';
    if (percentile >= 50) return 'text-blue-400';
    if (percentile >= 25) return 'text-yellow-400';
    return 'text-red-400';
}

function exportResults() {
    showExportModal.value = true;
}

async function generateAiReview() {
    if (!selectedResult.value) return;

    isGeneratingReview.value = true;
    try {
        const review = await invoke('generate_ai_review', { resultId: selectedResult.value.id });
        
        // Update local state
        selectedResult.value.interpretation = review as string;
        
        // Update in list as well
        const idx = results.value.findIndex(r => r.id === selectedResult.value?.id);
        if (idx !== -1) {
            results.value[idx].interpretation = review as string;
        }

    } catch (e) {
        const err = String(e); // Ensure it's a string
        const errLower = err.toLowerCase();
        console.error(err);
        
        if (errLower.includes('connection refused') || errLower.includes('failed to connect')) {
            if (confirm("⚠️ Local AI (Ollama) is not running.\n\nWould you like to generate a MOCK interpretation to verify the UI flow?")) {
                isGeneratingReview.value = true;
                await new Promise(r => setTimeout(r, 1500));
                
                const score = selectedResult.value.raw_score || 0;
                const mockReview = `[MOCK ANALYSIS] Based on the score of ${score}, the candidate shows potential. This is a simulated response because the local AI engine is unavailable. In a real environment, Gemma 2 2B would provide a psychometric analysis here.`;
                
                selectedResult.value.interpretation = mockReview;
                
                // Update in list as well
                const idx = results.value.findIndex(r => r.id === selectedResult.value?.id);
                if (idx !== -1) {
                    results.value[idx].interpretation = mockReview;
                }
            }
        } else {
            alert("Failed to generate AI review: " + err);
        }
    } finally {
        isGeneratingReview.value = false;
    }
}

function startEditing() {
    if (!selectedResult.value) return;
    editBuffer.value = selectedResult.value.interpretation || '';
    isEditingInterpretation.value = true;
}

function cancelEditing() {
    isEditingInterpretation.value = false;
    editBuffer.value = '';
}

async function saveInterpretation() {
    if (!selectedResult.value) return;
    
    isSavingInterpretation.value = true;
    try {
        await invoke('update_test_interpretation', {
            resultId: selectedResult.value.id,
            interpretation: editBuffer.value
        });
        
        // Update local state
        selectedResult.value.interpretation = editBuffer.value;
         // Update in list
        const idx = results.value.findIndex(r => r.id === selectedResult.value?.id);
        if (idx !== -1) {
            results.value[idx].interpretation = editBuffer.value;
        }
        
        isEditingInterpretation.value = false;
    } catch (e) {
        console.error('Failed to save interpretation:', e);
        alert('Failed to save interpretation: ' + e);
    } finally {
        isSavingInterpretation.value = false;
    }
}

function toggleSelectAll(e: Event) {
    const checked = (e.target as HTMLInputElement).checked;
    if (checked) {
        paginatedResults.value.forEach(r => selectedIds.value.add(r.id));
    } else {
        selectedIds.value.clear();
    }
}

function toggleSelection(id: number) {
    if (selectedIds.value.has(id)) {
        selectedIds.value.delete(id);
    } else {
        selectedIds.value.add(id);
    }
}

// function toggleSelectAll... (keep existing)
// function toggleSelection... (keep existing)

function deleteSelected() {
    if (selectedIds.value.size === 0) return;
    showDeleteConfirm.value = true;
}

function deleteSingleResult(id: number) {
    selectedIds.value.clear();
    selectedIds.value.add(id);
    showDeleteConfirm.value = true;
}

async function confirmDelete() {
    isDeleting.value = true; // Use this to toggle loading state in modal
    try {
        await invoke('delete_test_results', { resultIds: Array.from(selectedIds.value) });
        
        // Success feedback (optional, maybe ElMessage if available, or just clear)
        selectedIds.value.clear();
        showDeleteConfirm.value = false;
        await fetchData(); 
    } catch (e) {
        console.error('Failed to delete results:', e);
        alert('Failed to delete results'); // Or specialized error modal
    } finally {
        isDeleting.value = false;
    }
}

onMounted(fetchData);
</script>

<template>
    <div class="space-y-6">
        <!-- Header Stats -->
        <div class="grid grid-cols-4 gap-4">
            <div class="glass-panel p-4 border-l-4 border-l-blue-500">
                <div class="text-2xl font-bold text-gray-900 dark:text-white">{{ stats.total }}</div>
                <div class="text-xs text-gray-900 dark:text-white/50 uppercase tracking-wider">Total Results</div>
            </div>
            <div class="glass-panel p-4 border-l-4 border-l-green-500">
                <div class="text-2xl font-bold text-green-400">{{ stats.reviewed }}</div>
                <div class="text-xs text-gray-900 dark:text-white/50 uppercase tracking-wider">Reviewed</div>
            </div>
            <div class="glass-panel p-4 border-l-4 border-l-cyan-500">
                <div class="text-2xl font-bold text-cyan-400">{{ stats.completed }}</div>
                <div class="text-xs text-gray-900 dark:text-white/50 uppercase tracking-wider">Completed</div>
            </div>
            <div class="glass-panel p-4 border-l-4 border-l-yellow-500">
                <div class="text-2xl font-bold text-yellow-400">{{ stats.pending }}</div>
                <div class="text-xs text-gray-900 dark:text-white/50 uppercase tracking-wider">Pending Review</div>
            </div>
        </div>

        <!-- Filters -->
        <div class="glass-panel p-4 flex flex-wrap items-center gap-4 relative z-20">
            <!-- Search -->
            <div class="relative flex-1 min-w-[200px]">
                <input v-model="searchQuery" type="text" placeholder="Search candidates, events, tools..."
                    class="input-search w-full pl-11 py-2.5 text-sm">
                <svg class="w-4 h-4 text-gray-500 dark:text-gray-400 absolute left-4 top-1/2 -translate-y-1/2 pointer-events-none" fill="none"
                    viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                        d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
                </svg>
            </div>

            <!-- Tool Filter -->
            <SearchableDropdown
                v-model="selectedTool"
                :options="toolOptions"
                label-key="name"
                value-key="id"
                placeholder="All Tools"
                class="min-w-[180px]"
            />

            <!-- Status Filter -->
            <SearchableDropdown 
                v-model="selectedStatus"
                :options="statusOptions"
                label-key="label"
                value-key="value"
                placeholder="All Status"
                :searchable="false"
                class="min-w-[160px]"
            />

            <!-- Bulk Actions -->
            <button v-if="selectedIds.size > 0" 
                @click="deleteSelected" 
                :disabled="isDeleting"
                class="btn-neumorphic text-sm py-2.5 px-5 flex items-center shadow-red-500/20 text-red-500 hover:text-red-600 font-bold whitespace-nowrap">
                <svg v-if="isDeleting" class="animate-spin -ml-1 mr-2 h-4 w-4" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"><circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle><path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path></svg>
                <svg v-else class="w-4 h-4 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                </svg>
                Delete ({{ selectedIds.size }})
            </button>

            <!-- Export Button -->
            <button @click="exportResults" class="btn-neumorphic text-sm py-2.5 px-5 flex items-center shadow-eling-emerald/20 font-bold whitespace-nowrap">
                <svg class="w-4 h-4 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                        d="M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
                </svg>
                Export
            </button>
        </div>

        <!-- Results Table -->
        <div class="glass-panel overflow-hidden border-black/5 dark:border-white/5">
            <div class="overflow-x-auto">
                <table class="min-w-full divide-y divide-black/10 dark:divide-white/10">
                    <thead class="bg-black/5 dark:bg-white/5">
                        <tr>
                            <th class="px-6 py-3 w-10">
                                <input type="checkbox" 
                                    :checked="selectedIds.size > 0 && selectedIds.size === paginatedResults.length"
                                    :indeterminate="selectedIds.size > 0 && selectedIds.size < paginatedResults.length"
                                    @change="toggleSelectAll"
                                    class="rounded border-gray-300 text-eling-emerald focus:ring-eling-emerald cursor-pointer"
                                >
                            </th>
                            <th
                                class="px-6 py-3 text-left text-xs font-mono font-medium text-gray-900 dark:text-white/50 uppercase tracking-wider">
                                Candidate
                            </th>
                            <th
                                class="px-6 py-3 text-left text-xs font-mono font-medium text-gray-900 dark:text-white/50 uppercase tracking-wider">
                                Event
                            </th>
                            <th
                                class="px-6 py-3 text-left text-xs font-mono font-medium text-gray-900 dark:text-white/50 uppercase tracking-wider">
                                Tool
                            </th>
                            <th
                                class="px-6 py-3 text-center text-xs font-mono font-medium text-gray-900 dark:text-white/50 uppercase tracking-wider">
                                Score
                            </th>
                            <th
                                class="px-6 py-3 text-center text-xs font-mono font-medium text-gray-900 dark:text-white/50 uppercase tracking-wider">
                                Percentile
                            </th>
                            <th
                                class="px-6 py-3 text-left text-xs font-mono font-medium text-gray-900 dark:text-white/50 uppercase tracking-wider">
                                Interpretation
                            </th>
                            <th
                                class="px-6 py-3 text-center text-xs font-mono font-medium text-gray-900 dark:text-white/50 uppercase tracking-wider">
                                Status
                            </th>
                            <th
                                class="px-6 py-3 text-center text-xs font-mono font-medium text-gray-900 dark:text-white/50 uppercase tracking-wider">
                                Action
                            </th>
                        </tr>
                    </thead>
                    <tbody class="divide-y divide-black/5 dark:divide-white/5">
                        <tr v-if="loading">
                            <td colspan="8" class="px-6 py-12 text-center">
                                <div
                                    class="animate-spin w-8 h-8 border-2 border-eling-emerald border-t-transparent rounded-full mx-auto">
                                </div>
                            </td>
                        </tr>
                        <tr v-else-if="paginatedResults.length === 0">
                            <td colspan="8" class="px-6 py-12 text-center text-gray-900 dark:text-white/30 text-sm">
                                No results found matching your criteria.
                            </td>
                        </tr>
                        <tr v-for="result in paginatedResults" :key="result.id"
                            class="hover:bg-black/5 dark:bg-white/5 transition-colors"
                            :class="{'bg-blue-50/50 dark:bg-blue-900/10': selectedIds.has(result.id)}">
                            <td class="px-6 py-4 whitespace-nowrap">
                                <input type="checkbox" 
                                    :checked="selectedIds.has(result.id)" 
                                    @change="toggleSelection(result.id)"
                                    class="rounded border-gray-300 text-eling-emerald focus:ring-eling-emerald cursor-pointer"
                                >
                            </td>
                            <td class="px-6 py-4 whitespace-nowrap">
                                <div class="flex items-center gap-3">
                                    <div
                                        class="w-8 h-8 rounded-lg bg-gradient-to-br from-cyan-500/20 to-blue-500/20 flex items-center justify-center text-xs font-bold text-cyan-400">
                                        {{result.candidate_name.split(' ').map(n => n[0]).join('').slice(0, 2)}}
                                    </div>
                                    <div>
                                        <div class="text-sm font-medium text-gray-900 dark:text-white">{{
                                            result.candidate_name }}</div>
                                        <div class="text-xs text-gray-900 dark:text-white/40">ID: #{{
                                            result.candidate_id }}</div>
                                    </div>
                                </div>
                            </td>
                            <td class="px-6 py-4 whitespace-nowrap">
                                <div
                                    class="text-xs font-mono text-gray-900 dark:text-white/70 bg-black/5 dark:bg-white/5 px-2 py-1 rounded inline-block">
                                    {{ result.event_name }}
                                </div>
                            </td>
                            <td class="px-6 py-4 whitespace-nowrap">
                                <span class="text-sm font-bold text-eling-emerald">{{ result.tool_name }}</span>
                            </td>
                            <td class="px-6 py-4 whitespace-nowrap text-center">
                                <div class="text-lg font-bold text-gray-900 dark:text-white">{{ result.score || '-' }}
                                </div>
                                <div class="text-[10px] text-gray-900 dark:text-white/40">Raw: {{ result.raw_score }}
                                </div>
                            </td>
                            <td class="px-6 py-4 whitespace-nowrap text-center">
                                <span class="text-lg font-bold" :class="getScoreColor(result.percentile)">
                                    {{ result.percentile ? result.percentile + '%' : '-' }}
                                </span>
                            </td>
                            <td class="px-6 py-4">
                                <div class="text-sm text-gray-900 dark:text-white/70 max-w-[200px] truncate"
                                    :title="result.interpretation">
                                    {{ result.interpretation || '-' }}
                                </div>
                            </td>
                            <td class="px-6 py-4 whitespace-nowrap text-center">
                                <span class="px-2 py-1 rounded text-[10px] font-bold uppercase tracking-wide border"
                                    :class="getStatusColor(result.status)">
                                    {{ result.status }}
                                </span>
                            </td>
                            <td class="px-6 py-4 whitespace-nowrap text-center">
                                <div class="flex items-center justify-center gap-2">
                                    <button @click="viewDetail(result)"
                                        title="View Detail"
                                        class="p-2 rounded-lg text-cyan-400 hover:bg-cyan-500/10 transition-colors">
                                        <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
                                        </svg>
                                    </button>
                                    <button @click="deleteSingleResult(result.id)"
                                        title="Delete Result"
                                        class="p-2 rounded-lg text-red-500 hover:bg-red-500/10 transition-colors">
                                        <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                                        </svg>
                                    </button>
                                </div>
                            </td>
                        </tr>
                    </tbody>
                </table>
            </div>

            <!-- Pagination -->
            <div
                class="bg-black/5 dark:bg-white/5 border-t border-black/10 dark:border-white/10 px-4 py-3 flex items-center justify-between">
                <p class="text-sm text-gray-900 dark:text-white/50 font-mono">
                    Showing {{ (currentPage - 1) * itemsPerPage + 1 }} to {{ Math.min(currentPage * itemsPerPage,
                        filteredResults.length) }} of {{ filteredResults.length }} results
                </p>
                <nav class="relative z-0 inline-flex rounded-md shadow-sm -space-x-px">
                    <button @click="currentPage > 1 ? currentPage-- : null" :disabled="currentPage === 1"
                        class="relative inline-flex items-center px-2 py-2 rounded-l-md border border-black/10 dark:border-white/10 bg-black/5 dark:bg-white/5 text-sm font-medium text-gray-900 dark:text-white/70 hover:bg-white/10 disabled:opacity-50">
                        <svg class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                            <path fill-rule="evenodd"
                                d="M12.707 5.293a1 1 0 010 1.414L9.414 10l3.293 3.293a1 1 0 01-1.414 1.414l-4-4a1 1 0 010-1.414l4-4a1 1 0 011.414 0z"
                                clip-rule="evenodd" />
                        </svg>
                    </button>
                    <span
                        class="relative inline-flex items-center px-4 py-2 border border-black/10 dark:border-white/10 bg-black/5 dark:bg-white/5 text-sm font-medium text-gray-900 dark:text-white">
                        {{ currentPage }} / {{ totalPages || 1 }}
                    </span>
                    <button @click="currentPage < totalPages ? currentPage++ : null"
                        :disabled="currentPage >= totalPages"
                        class="relative inline-flex items-center px-2 py-2 rounded-r-md border border-black/10 dark:border-white/10 bg-black/5 dark:bg-white/5 text-sm font-medium text-gray-900 dark:text-white/70 hover:bg-white/10 disabled:opacity-50">
                        <svg class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                            <path fill-rule="evenodd"
                                d="M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 011.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z"
                                clip-rule="evenodd" />
                        </svg>
                    </button>
                </nav>
            </div>
        </div>

        <!-- Detail Modal -->
        <div v-if="showDetailModal && selectedResult" class="fixed z-50 inset-0 overflow-y-auto">
            <div class="flex items-center justify-center min-h-screen px-4">
                <!-- Backdrop -->
                <div class="absolute inset-0 bg-black/80 backdrop-blur-sm" @click="showDetailModal = false"></div>

                <!-- Modal -->
                <div class="relative glass-panel w-full max-w-2xl p-6 bg-eling-light-surface border-eling-emerald/20">
                    <div class="flex justify-between items-start mb-6">
                        <div>
                            <h3 class="text-lg font-bold text-gray-900 dark:text-white">Test Result Detail</h3>
                            <p class="text-xs text-gray-900 dark:text-white/50 mt-1">{{ selectedResult.candidate_name }}
                                - {{ selectedResult.tool_name }}</p>
                        </div>
                        <button @click="showDetailModal = false"
                            class="text-gray-900 dark:text-white/50 hover:text-gray-900 dark:text-white">
                            <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                    d="M6 18L18 6M6 6l12 12" />
                            </svg>
                        </button>
                    </div>

                    <div class="grid grid-cols-2 gap-4 mb-6">
                        <div class="bg-black/5 dark:bg-white/5 rounded-xl p-4">
                            <div class="text-xs text-gray-900 dark:text-white/50 uppercase mb-1">Candidate</div>
                            <div class="text-lg font-bold text-gray-900 dark:text-white">{{
                                selectedResult.candidate_name }}</div>
                        </div>
                        <div class="bg-black/5 dark:bg-white/5 rounded-xl p-4">
                            <div class="text-xs text-gray-900 dark:text-white/50 uppercase mb-1">Event</div>
                            <div class="text-lg font-bold text-cyan-400">{{ selectedResult.event_name }}</div>
                        </div>
                    </div>

                    <div class="grid grid-cols-3 gap-4 mb-6">
                        <div
                            class="bg-gradient-to-br from-blue-500/10 to-blue-500/5 rounded-xl p-4 text-center border border-blue-500/20">
                            <div class="text-3xl font-bold text-blue-400">{{ selectedResult.score || '-' }}</div>
                            <div class="text-xs text-gray-900 dark:text-white/50 uppercase mt-1">Score</div>
                        </div>
                        <div
                            class="bg-gradient-to-br from-green-500/10 to-green-500/5 rounded-xl p-4 text-center border border-green-500/20">
                            <div class="text-3xl font-bold text-green-400">{{ selectedResult.raw_score }}</div>
                            <div class="text-xs text-gray-900 dark:text-white/50 uppercase mt-1">Raw Score</div>
                        </div>
                        <div
                            class="bg-gradient-to-br from-purple-500/10 to-purple-500/5 rounded-xl p-4 text-center border border-purple-500/20">
                            <div class="text-3xl font-bold text-purple-400">{{ selectedResult.percentile ?
                                selectedResult.percentile + '%' : '-' }}</div>
                            <div class="text-xs text-gray-900 dark:text-white/50 uppercase mt-1">Percentile</div>
                        </div>
                    </div>

                    <div class="bg-black/5 dark:bg-white/5 rounded-xl p-4 mb-6 relative group">
                        <div class="flex justify-between items-center mb-2">
                            <div class="text-xs text-gray-900 dark:text-white/50 uppercase">Interpretation</div>
                            <div class="flex items-center gap-2">
                                <button v-if="selectedResult.interpretation && !isEditingInterpretation" @click="startEditing"
                                    class="text-[10px] text-blue-400 hover:text-blue-300 transition-colors opacity-0 group-hover:opacity-100 flex items-center">
                                    <svg class="w-3 h-3 mr-1" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
                                    </svg>
                                    Edit
                                </button>
                                <button v-if="selectedResult.interpretation && !isEditingInterpretation" @click="generateAiReview" 
                                    :disabled="isGeneratingReview"
                                    class="text-[10px] text-eling-emerald hover:text-white transition-colors opacity-0 group-hover:opacity-100 flex items-center">
                                    <span v-if="isGeneratingReview" class="animate-spin mr-1">⟳</span>
                                    {{ isGeneratingReview ? 'Regenerating...' : 'Regenerate' }}
                                </button>
                            </div>
                        </div>
                        
                        <div v-if="selectedResult.interpretation">
                            <div v-if="!isEditingInterpretation" class="max-h-60 overflow-y-auto pr-2 custom-scrollbar" @dblclick="startEditing">
                                <div class="text-gray-900 dark:text-white text-sm leading-relaxed whitespace-pre-line">
                                    {{ selectedResult.interpretation }}
                                </div>
                            </div>
                            <div v-else class="flex flex-col gap-2">
                                <textarea 
                                    v-model="editBuffer"
                                    class="w-full h-60 bg-black/10 dark:bg-white/5 border border-white/10 rounded-lg p-3 text-sm text-gray-900 dark:text-white focus:ring-2 focus:ring-eling-emerald focus:border-transparent resize-none leading-relaxed custom-scrollbar"
                                    placeholder="Enter interpretation..."
                                ></textarea>
                                <div class="flex justify-end gap-2 text-xs">
                                    <button @click="cancelEditing" class="px-3 py-1.5 rounded hover:bg-white/10 text-gray-500 hover:text-gray-300 transition">Cancel</button>
                                    <button @click="saveInterpretation" :disabled="isSavingInterpretation" 
                                        class="px-3 py-1.5 rounded bg-eling-emerald text-white hover:bg-eling-emerald/90 transition flex items-center">
                                        <span v-if="isSavingInterpretation" class="animate-spin mr-1 text-[8px]">⟳</span>
                                        Save Changes
                                    </button>
                                </div>
                            </div>
                        </div>
                        <div v-else class="flex flex-col items-center justify-center py-6">
                            <p class="text-sm text-gray-500 dark:text-gray-400 mb-4">No interpretation available yet.</p>
                            <button @click="generateAiReview" :disabled="isGeneratingReview"
                                class="btn-neumorphic text-sm py-2 px-6 flex items-center shadow-lg hover:shadow-xl transform hover:-translate-y-0.5 transition-all">
                                <svg v-if="!isGeneratingReview" class="w-5 h-5 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
                                </svg>
                                <svg v-else class="animate-spin -ml-1 mr-3 h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                                    <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                                    <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                                </svg>
                                {{ isGeneratingReview ? 'Analyzing Scores...' : 'Generate AI Analysis (Gemma 2 2B)' }}
                            </button>
                            <p class="text-[10px] text-gray-400 mt-2 font-mono">Powered by Local LLM</p>
                        </div>
                    </div>

                    <div class="flex justify-between items-center text-xs text-gray-900 dark:text-white/40">
                        <span>Completed: {{ selectedResult.completed_at }}</span>
                        <span class="px-2 py-1 rounded border" :class="getStatusColor(selectedResult.status)">
                            {{ selectedResult.status.toUpperCase() }}
                        </span>
                    </div>

                    <div class="mt-6 flex justify-between items-center gap-3">
                        <div class="flex gap-2">
                             <button v-if="selectedResult.session_id" 
                                @click="showRecordingPlayback = true"
                                class="btn-neumorphic text-sm py-2 px-4 shadow-blue-500/20 text-blue-500 font-bold flex items-center">
                                <svg class="w-4 h-4 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 10l4.553-2.276A1 1 0 0121 8.618v6.764a1 1 0 01-1.447.894L15 14M5 18h8a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z" />
                                </svg>
                                View Recording
                            </button>
                        </div>
                        <div class="flex gap-2">
                            <button @click="showDetailModal = false"
                                class="px-4 py-2 rounded-xl border border-black/10 dark:border-white/10 text-gray-900 dark:text-white hover:bg-black/5 dark:bg-white/5 transition-colors text-sm">
                                Close
                            </button>
                            <button class="btn-neumorphic text-sm py-2 px-4">
                                Mark as Reviewed
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        </div>

        <!-- Recording Playback Modal -->
        <RecordingPlayback 
            v-if="showRecordingPlayback && selectedResult" 
            :session-id="selectedResult.recording_id || selectedResult.session_id" 
            @close="showRecordingPlayback = false" 
        />

        <!-- Export Settings Modal -->
        <ExportSettingsModal 
            :show="showExportModal" 
            :results="results"
            @close="showExportModal = false" 
        />
        
        <!-- Delete Confirmation Modal -->
        <BaseModal :show="showDeleteConfirm" title="Delete Results" size="md" @close="!isDeleting ? showDeleteConfirm = false : null">
            <div class="text-center p-4">
                <div class="w-16 h-16 bg-red-500/10 rounded-full flex items-center justify-center mx-auto mb-4">
                    <svg class="w-8 h-8 text-red-500" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                    </svg>
                </div>
                
                <h4 v-if="!isDeleting" class="text-lg font-bold text-gray-900 dark:text-white mb-2">
                    Are you sure?
                </h4>
                <h4 v-else class="text-lg font-bold text-gray-900 dark:text-white mb-2">
                    Deleting...
                </h4>

                <p v-if="!isDeleting" class="text-gray-600 dark:text-gray-300 mb-6">
                    You are about to permanently delete <span class="font-bold text-red-500">{{ selectedIds.size }}</span> test results. This action cannot be undone.
                </p>
                <div v-else class="flex justify-center mb-6">
                    <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-red-500"></div>
                </div>

                <div class="flex gap-3 justify-center">
                    <button 
                        v-if="!isDeleting"
                        @click="showDeleteConfirm = false"
                        class="px-4 py-2 rounded-lg border border-black/10 dark:border-white/10 hover:bg-black/5 dark:bg-white/5 transition-colors font-medium text-gray-700 dark:text-gray-200"
                    >
                        Cancel
                    </button>
                    <button 
                        @click="confirmDelete" 
                        :disabled="isDeleting"
                        class="px-4 py-2 rounded-lg bg-red-500 text-white hover:bg-red-600 transition-colors font-bold flex items-center shadow-lg shadow-red-500/30"
                    >
                        <span v-if="isDeleting">Deleting...</span>
                        <span v-else>Yes, Delete</span>
                    </button>
                </div>
            </div>
        </BaseModal>
    </div>
</template>
