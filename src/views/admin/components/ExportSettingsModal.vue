<script setup lang="ts">
import { ref, computed } from 'vue';
import * as XLSX from 'xlsx';
import jsPDF from 'jspdf';
import autoTable from 'jspdf-autotable';
import SearchableDropdown from '@/components/molecules/SearchableDropdown.vue';

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

const props = defineProps<{
    show: boolean;
    results: TestResult[];
}>();

const emit = defineEmits(['close']);

// State
const format = ref<'excel' | 'pdf' | 'csv'>('excel');
const selectedEventId = ref<number | null>(null);

// Extract Unique Events from Results
const eventOptions = computed(() => {
    const eventsMap = new Map();
    eventsMap.set(null, 'All Events'); // Default option
    
    props.results.forEach(r => {
        if (!eventsMap.has(r.event_id)) {
            eventsMap.set(r.event_id, r.event_name);
        }
    });

    return Array.from(eventsMap.entries()).map(([id, name]) => ({
        id,
        name
    }));
});

// Filter Data for Export
const exportData = computed(() => {
    if (!selectedEventId.value) return props.results;
    return props.results.filter(r => r.event_id === selectedEventId.value);
});

// Preview Data (Top 5)
const previewData = computed(() => exportData.value.slice(0, 5));

function handleClose() {
    emit('close');
}

function generateFileName(ext: string) {
    const date = new Date().toISOString().split('T')[0];
    const prefix = selectedEventId.value 
        ? eventOptions.value.find(e => e.id === selectedEventId.value)?.name.replace(/\s+/g, '_') 
        : 'All_Events';
    return `Test_Results_${prefix}_${date}.${ext}`;
}

function handleExport() {
    const filename = generateFileName(format.value === 'excel' ? 'xlsx' : format.value);
    
    if (format.value === 'excel' || format.value === 'csv') {
        exportExcelOrCsv(filename);
    } else if (format.value === 'pdf') {
        exportPdf(filename);
    }
}

function exportExcelOrCsv(filename: string) {
    // Prepare data
    const data = exportData.value.map(r => ({
        'Candidate Name': r.candidate_name,
        'Candidates ID': r.candidate_id,
        'Event': r.event_name,
        'Tool': r.tool_name,
        'Score': r.score,
        'Raw Score': r.raw_score,
        'Percentile': r.percentile ? `${r.percentile}%` : '-',
        'Interpretation': r.interpretation || '-',
        'Status': r.status,
        'Completed At': r.completed_at
    }));

    const ws = XLSX.utils.json_to_sheet(data);
    
    // Adjust column widths
    const wscols = [
        { wch: 25 }, // Name
        { wch: 10 }, // ID
        { wch: 20 }, // Event
        { wch: 15 }, // Tool
        { wch: 10 }, // Score
        { wch: 10 }, // Raw
        { wch: 10 }, // Percentile
        { wch: 30 }, // Interpretation
        { wch: 15 }, // Status
        { wch: 20 }, // Date
    ];
    ws['!cols'] = wscols;

    const wb = XLSX.utils.book_new();
    XLSX.utils.book_append_sheet(wb, ws, "Test Results");

    if (format.value === 'csv') {
        XLSX.writeFile(wb, filename, { bookType: 'csv' });
    } else {
        XLSX.writeFile(wb, filename);
    }
    
    handleClose();
}

function exportPdf(filename: string) {
    const doc = new jsPDF();
    
    // Header
    doc.setFontSize(18);
    doc.text("Test Results Report", 14, 20);
    
    doc.setFontSize(10);
    doc.text(`Generated: ${new Date().toLocaleString()}`, 14, 28);
    doc.text(`Event: ${selectedEventId.value ? eventOptions.value.find(e => e.id === selectedEventId.value)?.name : 'All Events'}`, 14, 33);
    doc.text(`Total Records: ${exportData.value.length}`, 14, 38);

    // Table
    const headers = [['Candidate', 'Event', 'Tool', 'Score', 'Status']];
    const data = exportData.value.map(r => [
        r.candidate_name,
        r.event_name,
        r.tool_name,
        r.score?.toString() || '-',
        r.status.toUpperCase()
    ]);

    autoTable(doc, {
        head: headers,
        body: data,
        startY: 45,
        theme: 'grid',
        headStyles: { fillColor: [16, 185, 129] }, // Eling Emeraldish
        styles: { fontSize: 8 },
    });

    doc.save(filename);
    handleClose();
}
</script>

<template>
    <div v-if="show" class="fixed z-[9999] inset-0 overflow-y-auto" aria-labelledby="modal-title" role="dialog" aria-modal="true">
        <div class="flex items-end sm:items-center justify-center min-h-screen pt-4 px-4 pb-20 text-center sm:block sm:p-0">
            <!-- Backdrop -->
            <div class="fixed inset-0 bg-black/80 backdrop-blur-sm transition-opacity" aria-hidden="true" @click="handleClose"></div>

            <!-- Modal Panel -->
            <div class="inline-block align-bottom glass-panel bg-eling-light-surface text-left overflow-hidden shadow-xl transform transition-all sm:my-8 sm:align-middle sm:max-w-4xl sm:w-full border border-eling-emerald/20">
                <div class="flex justify-between items-center p-6 border-b border-black/10 dark:border-white/10">
                    <h3 class="text-lg leading-6 font-bold text-gray-900 dark:text-white" id="modal-title">Export Data</h3>
                    <button @click="handleClose" class="text-gray-400 hover:text-gray-500 dark:hover:text-white">
                        <span class="sr-only">Close</span>
                        <svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                        </svg>
                    </button>
                </div>

                <div class="p-6">
                    <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
                        <!-- Settings Column -->
                        <div class="space-y-6">
                            <!-- Format Selection -->
                            <div>
                                <label class="block text-xs font-mono text-eling-emerald mb-3 uppercase tracking-wider">Export Format</label>
                                <div class="space-y-3">
                                    <div @click="format = 'excel'" 
                                        class="cursor-pointer p-4 rounded-xl border transition-all flex items-center gap-4 group"
                                        :class="format === 'excel' ? 'bg-green-500/10 border-green-500 shadow-sm' : 'bg-black/5 dark:bg-white/5 border-transparent hover:bg-black/10 dark:hover:bg-white/10'">
                                        <div class="w-10 h-10 rounded-lg flex items-center justify-center"
                                            :class="format === 'excel' ? 'bg-green-500 text-white' : 'bg-gray-200 dark:bg-gray-700 text-gray-500 dark:text-gray-400'">
                                            <svg class="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
                                            </svg>
                                        </div>
                                        <div>
                                            <div class="font-bold text-sm text-gray-900 dark:text-white group-hover:text-green-600 dark:group-hover:text-green-400 transition-colors">Excel (.xlsx)</div>
                                            <div class="text-xs text-gray-500 dark:text-white/50">Best for analysis and editing</div>
                                        </div>
                                        <div class="ml-auto">
                                            <div class="w-4 h-4 rounded-full border flex items-center justify-center"
                                                :class="format === 'excel' ? 'border-green-500' : 'border-gray-400'">
                                                <div v-if="format === 'excel'" class="w-2 h-2 rounded-full bg-green-500"></div>
                                            </div>
                                        </div>
                                    </div>

                                    <div @click="format = 'pdf'" 
                                        class="cursor-pointer p-4 rounded-xl border transition-all flex items-center gap-4 group"
                                        :class="format === 'pdf' ? 'bg-red-500/10 border-red-500 shadow-sm' : 'bg-black/5 dark:bg-white/5 border-transparent hover:bg-black/10 dark:hover:bg-white/10'">
                                        <div class="w-10 h-10 rounded-lg flex items-center justify-center"
                                            :class="format === 'pdf' ? 'bg-red-500 text-white' : 'bg-gray-200 dark:bg-gray-700 text-gray-500 dark:text-gray-400'">
                                            <svg class="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 21h10a2 2 0 002-2V9.414a1 1 0 00-.293-.707l-5.414-5.414A1 1 0 0012.586 3H7a2 2 0 00-2 2v14a2 2 0 002 2z" />
                                            </svg>
                                        </div>
                                        <div>
                                            <div class="font-bold text-sm text-gray-900 dark:text-white group-hover:text-red-500 transition-colors">PDF Document</div>
                                            <div class="text-xs text-gray-500 dark:text-white/50">Best for printing and sharing</div>
                                        </div>
                                        <div class="ml-auto">
                                            <div class="w-4 h-4 rounded-full border flex items-center justify-center"
                                                :class="format === 'pdf' ? 'border-red-500' : 'border-gray-400'">
                                                <div v-if="format === 'pdf'" class="w-2 h-2 rounded-full bg-red-500"></div>
                                            </div>
                                        </div>
                                    </div>

                                    <div @click="format = 'csv'" 
                                        class="cursor-pointer p-4 rounded-xl border transition-all flex items-center gap-4 group"
                                        :class="format === 'csv' ? 'bg-blue-500/10 border-blue-500 shadow-sm' : 'bg-black/5 dark:bg-white/5 border-transparent hover:bg-black/10 dark:hover:bg-white/10'">
                                        <div class="w-10 h-10 rounded-lg flex items-center justify-center"
                                            :class="format === 'csv' ? 'bg-blue-500 text-white' : 'bg-gray-200 dark:bg-gray-700 text-gray-500 dark:text-gray-400'">
                                            <svg class="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 17v-2m3 2v-4m3 4v-6m2 10H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
                                            </svg>
                                        </div>
                                        <div>
                                            <div class="font-bold text-sm text-gray-900 dark:text-white group-hover:text-blue-500 transition-colors">CSV (.csv)</div>
                                            <div class="text-xs text-gray-500 dark:text-white/50">Best for database imports</div>
                                        </div>
                                        <div class="ml-auto">
                                            <div class="w-4 h-4 rounded-full border flex items-center justify-center"
                                                :class="format === 'csv' ? 'border-blue-500' : 'border-gray-400'">
                                                <div v-if="format === 'csv'" class="w-2 h-2 rounded-full bg-blue-500"></div>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>

                            <!-- Filter Selection -->
                            <div>
                                <label class="block text-xs font-mono text-eling-emerald mb-3 uppercase tracking-wider">Filter Data</label>
                                <SearchableDropdown
                                    v-model="selectedEventId"
                                    :options="eventOptions"
                                    label-key="name"
                                    value-key="id"
                                    placeholder="All Events"
                                    class="w-full"
                                />
                                <p class="text-[10px] text-gray-500 dark:text-white/40 mt-2">
                                    {{ exportData.length }} records will be exported.
                                </p>
                            </div>
                        </div>

                        <!-- Preview Column -->
                        <div class="lg:col-span-2 flex flex-col h-full">
                            <label class="block text-xs font-mono text-eling-emerald mb-3 uppercase tracking-wider flex justify-between">
                                <span>Data Preview (First 5 Rows)</span>
                                <span class="text-gray-400 normal-case">{{ format.toUpperCase() }} Format</span>
                            </label>
                            
                            <div class="flex-1 bg-black/5 dark:bg-white/5 rounded-xl border border-black/10 dark:border-white/10 p-1 overflow-hidden flex flex-col">
                                <div class="overflow-x-auto">
                                    <table class="min-w-full divide-y divide-black/10 dark:divide-white/10">
                                        <thead class="bg-black/5 dark:bg-white/5">
                                            <tr>
                                                <th class="px-3 py-2 text-left text-[10px] font-bold text-gray-500 uppercase">Candidate</th>
                                                <th class="px-3 py-2 text-left text-[10px] font-bold text-gray-500 uppercase">Event</th>
                                                <th class="px-3 py-2 text-left text-[10px] font-bold text-gray-500 uppercase">Tool</th>
                                                <th class="px-3 py-2 text-right text-[10px] font-bold text-gray-500 uppercase">Score</th>
                                                <th class="px-3 py-2 text-center text-[10px] font-bold text-gray-500 uppercase">Status</th>
                                            </tr>
                                        </thead>
                                        <tbody class="divide-y divide-black/5 dark:divide-white/5">
                                            <tr v-if="previewData.length === 0">
                                                <td colspan="5" class="px-3 py-8 text-center text-xs text-gray-500">No data available for export</td>
                                            </tr>
                                            <tr v-for="row in previewData" :key="row.id" class="hover:bg-white/5">
                                                <td class="px-3 py-2 text-xs text-gray-900 dark:text-white">{{ row.candidate_name }}</td>
                                                <td class="px-3 py-2 text-xs text-gray-500">{{ row.event_name }}</td>
                                                <td class="px-3 py-2 text-xs text-gray-500">{{ row.tool_name }}</td>
                                                <td class="px-3 py-2 text-xs text-right font-mono">{{ row.score || '-' }}</td>
                                                <td class="px-3 py-2 text-center">
                                                    <span class="inline-block px-1.5 py-0.5 rounded text-[10px] uppercase font-bold bg-gray-100 dark:bg-white/10 text-gray-500 dark:text-white/70">
                                                        {{ row.status }}
                                                    </span>
                                                </td>
                                            </tr>
                                        </tbody>
                                    </table>
                                </div>
                                <div class="mt-auto p-2 bg-black/5 dark:bg-white/5 text-[10px] text-center text-gray-500 border-t border-black/5 dark:border-white/5">
                                    Displaying 5 of {{ exportData.length }} records
                                </div>
                            </div>
                        </div>
                    </div>

                    <div class="mt-8 flex justify-end gap-3 pt-6 border-t border-black/10 dark:border-white/10">
                        <button @click="handleClose"
                            class="px-4 py-2.5 rounded-xl border border-black/10 dark:border-white/10 text-gray-700 dark:text-text-primary hover:bg-black/5 dark:bg-white/5 transition-colors text-sm font-medium">
                            Cancel
                        </button>
                        <button @click="handleExport"
                            class="btn-neumorphic px-6 py-2.5 text-sm font-bold flex items-center gap-2"
                            :disabled="exportData.length === 0">
                            <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" />
                            </svg>
                            Export {{ format === 'excel' ? 'Excel' : (format === 'csv' ? 'CSV' : 'PDF') }}
                        </button>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>
