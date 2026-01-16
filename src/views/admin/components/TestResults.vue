<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';

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
}

interface Tool {
    id: number;
    name: string;
}

// State
const results = ref<TestResult[]>([]);
const tools = ref<Tool[]>([]);
const loading = ref(true);

// Filters
const searchQuery = ref('');
const selectedTool = ref<number | null>(null);
const selectedStatus = ref<string>('all');
const currentPage = ref(1);
const itemsPerPage = ref(10);

// Selected result for detail modal
const selectedResult = ref<TestResult | null>(null);
const showDetailModal = ref(false);

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
    // TODO: Implement CSV/PDF export
    alert('Export functionality coming soon!');
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
        <div class="glass-panel p-4 flex flex-wrap items-center gap-4">
            <!-- Search -->
            <div class="relative flex-1 min-w-[200px]">
                <input v-model="searchQuery" type="text" placeholder="Search candidates, events, tools..."
                    class="input-glass w-full pl-10 bg-black/20 focus:bg-black/30 border-black/10 dark:border-white/10 py-2 text-sm">
                <svg class="w-4 h-4 text-gray-900 dark:text-white/50 absolute left-3 top-3" fill="none"
                    viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                        d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
                </svg>
            </div>

            <!-- Tool Filter -->
            <select v-model="selectedTool"
                class="input-glass bg-black/20 border-black/10 dark:border-white/10 py-2 text-sm min-w-[150px]">
                <option :value="null">All Tools</option>
                <option v-for="tool in tools" :key="tool.id" :value="tool.id">{{ tool.name }}</option>
            </select>

            <!-- Status Filter -->
            <select v-model="selectedStatus"
                class="input-glass bg-black/20 border-black/10 dark:border-white/10 py-2 text-sm min-w-[130px]">
                <option value="all">All Status</option>
                <option value="pending">Pending</option>
                <option value="completed">Completed</option>
                <option value="reviewed">Reviewed</option>
            </select>

            <!-- Export Button -->
            <button @click="exportResults" class="btn-neumorphic text-xs py-2 px-4 flex items-center">
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
                                    class="animate-spin w-8 h-8 border-2 border-eling-accent border-t-transparent rounded-full mx-auto">
                                </div>
                            </td>
                        </tr>
                        <tr v-else-if="paginatedResults.length === 0">
                            <td colspan="8" class="px-6 py-12 text-center text-gray-900 dark:text-white/30 text-sm">
                                No results found matching your criteria.
                            </td>
                        </tr>
                        <tr v-for="result in paginatedResults" :key="result.id"
                            class="hover:bg-black/5 dark:bg-white/5 transition-colors">
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
                                <span class="text-sm font-bold text-eling-accent">{{ result.tool_name }}</span>
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
                                <button @click="viewDetail(result)"
                                    class="text-xs text-cyan-400 hover:text-cyan-300 font-medium px-3 py-1 rounded hover:bg-black/5 dark:bg-white/5 transition-colors">
                                    View Detail
                                </button>
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
                <div class="relative glass-panel w-full max-w-2xl p-6 bg-eling-surface border-eling-accent/20">
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

                    <div class="bg-black/5 dark:bg-white/5 rounded-xl p-4 mb-6">
                        <div class="text-xs text-gray-900 dark:text-white/50 uppercase mb-2">Interpretation</div>
                        <div class="text-gray-900 dark:text-white">{{ selectedResult.interpretation || 'No interpretation available' }}</div>
                    </div>

                    <div class="flex justify-between items-center text-xs text-gray-900 dark:text-white/40">
                        <span>Completed: {{ selectedResult.completed_at }}</span>
                        <span class="px-2 py-1 rounded border" :class="getStatusColor(selectedResult.status)">
                            {{ selectedResult.status.toUpperCase() }}
                        </span>
                    </div>

                    <div class="mt-6 flex justify-end gap-3">
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
</template>
