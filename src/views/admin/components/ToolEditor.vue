<script setup lang="ts">
import { ref, watch, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import ChoiceEngine from '../../../components/engines/ChoiceEngine.vue';
import ManageQuestionsModal from './ManageQuestionsModal.vue';

// Types matches backend
interface Tool {
    id: number;
    name: string;
    description: string;
    tool_type: string;
    category: string;
}

interface Subtest {
    id: number;
    subtest_name: string;
    sequence_order: number;
    time_limit_seconds: number | null;
    question_count: number;
}

interface FullToolStructure {
    tool: Tool;
    subtests: {
        subtest: Subtest;
        questions: any[];
    }[];
}

const props = defineProps<{
    toolId: number;
}>();

const toolData = ref<FullToolStructure | null>(null);
const isLoading = ref(false);
const activeTab = ref<'info' | 'subtests' | 'preview'>('info');

// Create Subtest State
const showAddSubtest = ref(false);
const newSubtest = ref({
    name: '',
    timeLimit: 0,
    sequence: 1
});

// Question Management State
const selectedSubtestForEdit = ref<any>(null);

// Preview Simulation State
const previewMode = ref<'overview' | 'testing' | 'results'>('overview');
const currentSubtestIndex = ref(0);
const currentQuestionIndex = ref(0);
const previewAnswers = ref<Record<number, string>>({});
const previewStartTime = ref<number>(0);
const previewTimeRemaining = ref<number>(0);
const previewTimer = ref<any>(null);

// Computed for current question in preview
const currentSubtest = computed(() => {
    if (!toolData.value || toolData.value.subtests.length === 0) return null;
    return toolData.value.subtests[currentSubtestIndex.value];
});

const currentQuestion = computed(() => {
    if (!currentSubtest.value || currentSubtest.value.questions.length === 0) return null;
    return currentSubtest.value.questions[currentQuestionIndex.value];
});

const totalQuestionsInSubtest = computed(() => {
    return currentSubtest.value?.questions.length || 0;
});

const progressPercent = computed(() => {
    if (totalQuestionsInSubtest.value === 0) return 0;
    return Math.round(((currentQuestionIndex.value + 1) / totalQuestionsInSubtest.value) * 100);
});

// Pagination window for navigation dots
const paginationWindow = computed(() => {
    const total = totalQuestionsInSubtest.value;
    const current = currentQuestionIndex.value;
    const maxDots = 10;

    if (total <= maxDots) {
        // Show all if total is less than max
        return Array.from({ length: total }, (_, i) => i + 1);
    }

    // Calculate window
    const halfWindow = Math.floor(maxDots / 2);
    let start = Math.max(1, current + 1 - halfWindow);
    let end = Math.min(total, start + maxDots - 1);

    // Adjust if we're near the end
    if (end - start < maxDots - 1) {
        start = Math.max(1, end - maxDots + 1);
    }

    return Array.from({ length: end - start + 1 }, (_, i) => start + i);
});

// Preview functions
function startPreview() {
    previewMode.value = 'testing';
    currentSubtestIndex.value = 0;
    currentQuestionIndex.value = 0;
    previewAnswers.value = {};
    startTimer();
}

function startTimer() {
    if (previewTimer.value) clearInterval(previewTimer.value);
    const timeLimit = currentSubtest.value?.subtest.time_limit_seconds || 0;
    previewTimeRemaining.value = timeLimit;
    previewStartTime.value = Date.now();

    if (timeLimit > 0) {
        previewTimer.value = setInterval(() => {
            const elapsed = Math.floor((Date.now() - previewStartTime.value) / 1000);
            previewTimeRemaining.value = Math.max(0, timeLimit - elapsed);
            if (previewTimeRemaining.value <= 0) {
                nextSubtest();
            }
        }, 1000);
    }
}

function selectAnswer(answer: string) {
    if (currentQuestion.value) {
        previewAnswers.value[currentQuestion.value.id] = answer;
    }
}

function nextQuestion() {
    if (currentQuestionIndex.value < totalQuestionsInSubtest.value - 1) {
        currentQuestionIndex.value++;
    } else {
        nextSubtest();
    }
}

function prevQuestion() {
    if (currentQuestionIndex.value > 0) {
        currentQuestionIndex.value--;
    }
}

function nextSubtest() {
    if (previewTimer.value) clearInterval(previewTimer.value);

    if (toolData.value && currentSubtestIndex.value < toolData.value.subtests.length - 1) {
        currentSubtestIndex.value++;
        currentQuestionIndex.value = 0;
        startTimer();
    } else {
        previewMode.value = 'results';
    }
}

function resetPreview() {
    if (previewTimer.value) clearInterval(previewTimer.value);
    previewMode.value = 'overview';
    currentSubtestIndex.value = 0;
    currentQuestionIndex.value = 0;
    previewAnswers.value = {};
}

function formatTime(seconds: number): string {
    const mins = Math.floor(seconds / 60);
    const secs = seconds % 60;
    return `${mins.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
}

// Calculate score based on tool type
function calculateScore(): { correct: number; total: number; percentage: number; details: any } {
    if (!toolData.value) return { correct: 0, total: 0, percentage: 0, details: {} };

    let correct = 0;
    let total = 0;
    const details: Record<string, { correct: number; total: number }> = {};

    toolData.value.subtests.forEach(s => {
        const subtestName = s.subtest.subtest_name;
        details[subtestName] = { correct: 0, total: s.questions.length };

        s.questions.forEach(q => {
            total++;
            const userAnswer = previewAnswers.value[q.id];
            const correctAnswer = q.options?.correct;
            if (userAnswer && correctAnswer && userAnswer === correctAnswer) {
                correct++;
                details[subtestName].correct++;
            }
        });
    });

    return {
        correct,
        total,
        percentage: total > 0 ? Math.round((correct / total) * 100) : 0,
        details
    };
}


function openQuestionManager(subtestItem: any) {
    selectedSubtestForEdit.value = subtestItem;
}

async function handleRefreshAfterEdit() {
    // Reload tool data
    await fetchToolData();
    // Update the selected subtest object reference so the modal updates (if it's still open or re-opens)
    // Finding the updated subtest from new toolData
    if (toolData.value && selectedSubtestForEdit.value) {
        const updated = toolData.value.subtests.find(s => s.subtest.id === selectedSubtestForEdit.value.subtest.id);
        if (updated) {
            selectedSubtestForEdit.value = updated;
        }
    }
}

async function fetchToolData() {
    if (!props.toolId) return;
    isLoading.value = true;
    try {
        toolData.value = await invoke('get_tool_structure', { toolId: props.toolId });
    } catch (e) {
        console.error("Failed to fetch tool:", e);
        alert("Error loading tool data: " + e);
    } finally {
        isLoading.value = false;
    }
}

async function handleAddSubtest() {
    if (!props.toolId || !newSubtest.value.name) return;

    try {
        await invoke('create_subtest', {
            toolId: props.toolId,
            name: newSubtest.value.name,
            sequence: newSubtest.value.sequence,
            timeLimit: newSubtest.value.timeLimit > 0 ? newSubtest.value.timeLimit : null
        });
        showAddSubtest.value = false;
        newSubtest.value = { name: '', timeLimit: 0, sequence: (toolData.value?.subtests.length || 0) + 1 };
        await fetchToolData();
    } catch (e) {
        alert("Failed to create subtest: " + e);
    }
}

async function handleDeleteSubtest(id: number) {
    if (!confirm("Are you sure? This will delete all questions in this subtest.")) return;
    try {
        await invoke('delete_subtest', { id });
        await fetchToolData();
    } catch (e) {
        alert("Failed to delete subtest: " + e);
    }
}

// Helper functions for tool information display
function getTotalQuestions(): number {
    if (!toolData.value) return 0;
    return toolData.value.subtests.reduce((acc, s) => acc + s.questions.length, 0);
}

function getTotalTime(): number {
    if (!toolData.value) return 0;
    const totalSeconds = toolData.value.subtests.reduce((acc, s) => acc + (s.subtest.time_limit_seconds || 0), 0);
    return Math.ceil(totalSeconds / 60);
}

// Tool descriptions based on tool name
const toolDescriptions: Record<string, string> = {
    'TIU': 'Tes Intelegensia Umum (TIU) adalah tes kognitif standar yang mengukur kemampuan verbal, numerik, dan penalaran umum. Tes ini sering digunakan dalam seleksi CPNS dan perusahaan di Indonesia.',
    'IST': 'Intelligence Structure Test (IST) adalah tes inteligensi komprehensif yang mengukur 9 aspek kecerdasan termasuk kemampuan verbal, numerik, spasial, dan memori. Dikembangkan oleh Rudolf Amthauer.',
    'CFIT': 'Culture Fair Intelligence Test (CFIT) mengukur kecerdasan fluid (Gf) yang bebas dari pengaruh budaya. Menggunakan pola visual dan logika non-verbal untuk menilai kemampuan penalaran murni.',
    'MATRICES': 'Raven\'s Progressive Matrices (APM/SPM/RPM) adalah tes non-verbal yang mengukur penalaran abstrak dan kemampuan memecahkan masalah baru tanpa pengetahuan sebelumnya.',
    'WPT': 'Wonderlic Personnel Test adalah tes kognitif singkat (12 menit, 50 soal) yang mengukur kemampuan belajar, beradaptasi, dan memecahkan masalah dalam lingkungan kerja.',
    'GATB': 'General Aptitude Test Battery (GATB) mengukur 9 bakat umum termasuk kecerdasan umum, verbal, numerik, spasial, persepsi bentuk, dan koordinasi motor.',
    'EPPS': 'Edwards Personal Preference Schedule (EPPS) mengukur 15 kebutuhan psikologis berdasarkan teori Henry Murray, menggunakan format pilihan paksa berpasangan.',
    'PAPI': 'PAPI Kostick mengukur 20 dimensi kepribadian dan gaya kerja, membantu memahami preferensi dan perilaku individu dalam konteks pekerjaan.',
    'DISC': 'DISC Assessment mengukur empat dimensi perilaku: Dominance (D), Influence (I), Steadiness (S), dan Compliance (C) untuk memahami gaya komunikasi dan kerja.',
    'MBTI': 'Myers-Briggs Type Indicator mengidentifikasi 16 tipe kepribadian berdasarkan 4 dikotomi: Extraversion/Introversion, Sensing/Intuition, Thinking/Feeling, dan Judging/Perceiving.',
    '16PF': 'Sixteen Personality Factor Questionnaire mengukur 16 faktor kepribadian primer yang dikembangkan oleh Raymond Cattell untuk profiling kepribadian komprehensif.',
    'HEXACO': 'HEXACO Personality Inventory mengukur 6 dimensi kepribadian: Honesty-Humility, Emotionality, eXtraversion, Agreeableness, Conscientiousness, dan Openness.',
    'KRAEPELIN': 'Tes Kraepelin adalah tes kecepatan yang mengukur daya tahan, ketelitian, dan konsistensi kerja melalui penjumlahan angka secara kontinyu dalam waktu tertentu.',
    'RMIB': 'Rothwell Miller Interest Blank mengidentifikasi minat karir dan pekerjaan berdasarkan 12 kategori minat vokasional.',
    'RIASEC': 'Holland Interest Test (RIASEC) mengklasifikasikan minat karir ke dalam 6 tipe: Realistic, Investigative, Artistic, Social, Enterprising, dan Conventional.',
    'MSDT': 'Management Style Diagnosis Test mengidentifikasi gaya kepemimpinan dan manajemen untuk pengembangan kepemimpinan.',
};

const toolFeatures: Record<string, string[]> = {
    'TIU': ['Tes Verbal', 'Tes Numerik', 'Penalaran Logis', 'Standar CPNS'],
    'IST': ['9 Subtes Lengkap', 'Profil Inteligensi', 'Skor IQ', 'Analisis Mendalam'],
    'CFIT': ['Culture Fair', 'Non-Verbal', '4 Subtes', 'Mengukur Gf'],
    'MATRICES': ['Non-Verbal', 'Penalaran Abstrak', 'Progressive Difficulty', 'Timed Test'],
    'WPT': ['12 Menit', '50 Soal', 'Quick Assessment', 'Learning Ability'],
    'GATB': ['8 Subtes', '9 Bakat', 'Occupational Matching', 'Comprehensive'],
    'EPPS': ['15 Kebutuhan', 'Forced Choice', '225 Item', 'Ipsative Scoring'],
    'PAPI': ['20 Dimensi', 'Work Style', '90 Item', 'Role Fit'],
    'DISC': ['4 Dimensi', 'Behavior Style', 'Quick Assessment', 'Team Building'],
    'MBTI': ['16 Tipe', '4 Dikotomi', '93 Item', 'Self-Awareness'],
    '16PF': ['16 Faktor', 'Personality Profile', '185 Item', 'Research-Based'],
    'HEXACO': ['6 Dimensi', 'Modern Model', 'Cross-Cultural', 'Comprehensive'],
    'KRAEPELIN': ['Speed Test', 'Endurance', 'Accuracy', 'Work Consistency'],
    'RMIB': ['12 Kategori', 'Career Interest', 'Ranking Method', 'Vocational'],
    'RIASEC': ['6 Tipe', 'Holland Code', 'Career Match', 'Self-Directed'],
    'MSDT': ['Leadership Style', '4 Styles', 'Management Development', 'Decision Making'],
};

function getToolDescription(toolName: string): string {
    return toolDescriptions[toolName] || `${toolName} adalah alat tes psikologi profesional yang digunakan untuk mengukur aspek-aspek psikologis tertentu dalam proses seleksi dan pengembangan SDM.`;
}

function getToolFeatures(toolName: string): string[] {
    return toolFeatures[toolName] || ['Professional Assessment', 'Standardized Test', 'Reliable Results', 'Valid Measurement'];
}

function scrollToSubtest(subtestId: number) {
    const element = document.getElementById('subtest-' + subtestId);
    if (element) {
        element.scrollIntoView({ behavior: 'smooth', block: 'start' });
    }
}

watch(() => props.toolId, fetchToolData, { immediate: true });
</script>

<template>
    <div v-if="isLoading" class="flex justify-center py-12">
        <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-eling-accent"></div>
    </div>

    <div v-else-if="toolData" class="space-y-6 animate-fade-in">
        <!-- Header -->
        <div class="flex flex-col md:flex-row justify-between items-start md:items-center gap-4">
            <div>
                <div class="flex items-center gap-3">
                    <h2 class="text-2xl font-bold text-gray-900 dark:text-eling-light">{{ toolData.tool.name }}</h2>
                    <span
                        class="px-2 py-0.5 rounded text-[10px] uppercase font-bold bg-eling-accent/10 text-eling-accent border border-eling-accent/20">
                        {{ toolData.tool.category }}
                    </span>
                </div>
                <p class="text-sm text-gray-900 dark:text-eling-light/50 mt-1 font-mono">ID: #{{ toolData.tool.id }} •
                    Type: {{
                        toolData.tool.tool_type }}</p>
            </div>

            <div class="flex space-x-2 bg-black/5 dark:bg-white/5 p-1 rounded-lg">
                <button @click="activeTab = 'info'" class="px-4 py-1.5 text-xs font-medium rounded-md transition-all"
                    :class="activeTab === 'info' ? 'bg-eling-accent text-eling-dark shadow-sm' : 'text-gray-900 dark:text-eling-light/60 hover:text-gray-900 dark:text-eling-light'">
                    Information
                </button>
                <button @click="activeTab = 'subtests'"
                    class="px-4 py-1.5 text-xs font-medium rounded-md transition-all"
                    :class="activeTab === 'subtests' ? 'bg-eling-accent text-eling-dark shadow-sm' : 'text-gray-900 dark:text-eling-light/60 hover:text-gray-900 dark:text-eling-light'">
                    Subtests ({{ toolData.subtests.length }})
                </button>
            </div>
        </div>

        <!-- content -->
        <div class="glass-panel p-6 border-black/5 dark:border-white/5 min-h-[400px]">

            <!-- INFO TAB -->
            <div v-if="activeTab === 'info'" class="space-y-6">
                <!-- Tool Overview Card -->
                <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
                    <!-- Main Info -->
                    <div class="lg:col-span-2 space-y-6">
                        <!-- Hero Section -->
                        <div
                            class="bg-gradient-to-br from-eling-accent/10 to-purple-500/5 rounded-xl p-6 border border-eling-accent/20">
                            <div class="flex items-start gap-4">
                                <div
                                    class="w-16 h-16 rounded-xl bg-eling-accent/20 flex items-center justify-center shrink-0">
                                    <span class="text-2xl font-bold text-eling-accent">{{ toolData.tool.name.charAt(0)
                                        }}</span>
                                </div>
                                <div class="flex-1">
                                    <h3 class="text-xl font-bold text-gray-900 dark:text-eling-light">{{
                                        toolData.tool.name }}</h3>
                                    <p class="text-sm text-gray-900 dark:text-eling-light/60 mt-1">{{
                                        toolData.tool.description || 'No description available' }}</p>
                                    <div class="flex flex-wrap gap-2 mt-3">
                                        <span
                                            class="px-2 py-1 rounded-lg text-xs font-medium bg-eling-accent/10 text-eling-accent border border-eling-accent/20">
                                            {{ toolData.tool.category }}
                                        </span>
                                        <span
                                            class="px-2 py-1 rounded-lg text-xs font-medium bg-blue-500/10 text-blue-400 border border-blue-500/20">
                                            {{ toolData.tool.tool_type }}
                                        </span>
                                    </div>
                                </div>
                            </div>
                        </div>

                        <!-- Tool Description -->
                        <div
                            class="bg-black/5 dark:bg-white/5 rounded-xl p-5 border border-black/10 dark:border-white/10">
                            <h4
                                class="text-sm font-bold text-gray-900 dark:text-eling-light/80 mb-3 flex items-center gap-2">
                                <svg class="w-4 h-4 text-eling-accent" fill="none" viewBox="0 0 24 24"
                                    stroke="currentColor">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                        d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                                </svg>
                                About This Tool
                            </h4>
                            <p class="text-sm text-gray-900 dark:text-eling-light/60 leading-relaxed">
                                {{ getToolDescription(toolData.tool.name) }}
                            </p>
                        </div>

                        <!-- Key Features -->
                        <div
                            class="bg-black/5 dark:bg-white/5 rounded-xl p-5 border border-black/10 dark:border-white/10">
                            <h4
                                class="text-sm font-bold text-gray-900 dark:text-eling-light/80 mb-3 flex items-center gap-2">
                                <svg class="w-4 h-4 text-green-400" fill="none" viewBox="0 0 24 24"
                                    stroke="currentColor">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                        d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                                </svg>
                                Key Features
                            </h4>
                            <div class="grid grid-cols-2 gap-3">
                                <div v-for="feature in getToolFeatures(toolData.tool.name)" :key="feature"
                                    class="flex items-center gap-2 text-xs text-gray-900 dark:text-eling-light/60">
                                    <span class="w-1.5 h-1.5 rounded-full bg-green-400"></span>
                                    {{ feature }}
                                </div>
                            </div>
                        </div>
                    </div>

                    <!-- Statistics Sidebar -->
                    <div class="space-y-4">
                        <!-- Stats Cards -->
                        <div
                            class="bg-gradient-to-br from-blue-500/10 to-blue-600/5 rounded-xl p-5 border border-blue-500/20">
                            <div class="text-3xl font-bold text-blue-400">{{ toolData.subtests.length }}</div>
                            <div class="text-xs text-blue-300/70 mt-1">Subtests</div>
                        </div>

                        <div
                            class="bg-gradient-to-br from-purple-500/10 to-purple-600/5 rounded-xl p-5 border border-purple-500/20">
                            <div class="text-3xl font-bold text-purple-400">{{ getTotalQuestions() }}</div>
                            <div class="text-xs text-purple-300/70 mt-1">Total Questions</div>
                        </div>

                        <div
                            class="bg-gradient-to-br from-green-500/10 to-green-600/5 rounded-xl p-5 border border-green-500/20">
                            <div class="text-3xl font-bold text-green-400">{{ getTotalTime() }}</div>
                            <div class="text-xs text-green-300/70 mt-1">Total Time (min)</div>
                        </div>

                        <div
                            class="bg-gradient-to-br from-orange-500/10 to-orange-600/5 rounded-xl p-5 border border-orange-500/20">
                            <div class="text-xl font-bold text-orange-400">{{ toolData.tool.category }}</div>
                            <div class="text-xs text-orange-300/70 mt-1">Category</div>
                        </div>

                        <!-- Quick Actions -->
                        <div
                            class="bg-black/5 dark:bg-white/5 rounded-xl p-4 border border-black/10 dark:border-white/10 space-y-2">
                            <h5
                                class="text-xs font-bold text-gray-900 dark:text-eling-light/50 uppercase tracking-wider mb-3">
                                Quick
                                Actions</h5>
                            <button @click="activeTab = 'subtests'"
                                class="w-full text-left px-3 py-2 text-xs text-gray-900 dark:text-eling-light/70 hover:bg-black/5 dark:bg-white/5 rounded-lg transition flex items-center gap-2">
                                <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                        d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" />
                                </svg>
                                Manage Subtests
                            </button>
                            <button @click="activeTab = 'preview'"
                                class="w-full text-left px-3 py-2 text-xs text-gray-900 dark:text-eling-light/70 hover:bg-black/5 dark:bg-white/5 rounded-lg transition flex items-center gap-2">
                                <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                        d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                        d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
                                </svg>
                                Preview Test
                            </button>
                        </div>
                    </div>
                </div>
            </div>

            <!-- SUBTESTS TAB -->
            <div v-if="activeTab === 'subtests'" class="space-y-6">
                <!-- Add Subtest Form -->
                <div v-if="showAddSubtest"
                    class="p-4 bg-black/5 dark:bg-white/5 rounded-lg border border-black/10 dark:border-white/10 animate-fade-in">
                    <h4 class="text-sm font-bold text-gray-900 dark:text-eling-light mb-4">Add New Subtest</h4>
                    <div class="grid grid-cols-1 md:grid-cols-3 gap-4 mb-4">
                        <div>
                            <label class="block text-xs text-gray-900 dark:text-eling-light/50 mb-1">Name</label>
                            <input v-model="newSubtest.name" type="text" class="input-glass w-full text-sm">
                        </div>
                        <div>
                            <label class="block text-xs text-gray-900 dark:text-eling-light/50 mb-1">Sequence</label>
                            <input v-model="newSubtest.sequence" type="number" class="input-glass w-full text-sm">
                        </div>
                        <div>
                            <label class="block text-xs text-gray-900 dark:text-eling-light/50 mb-1">Time Limit
                                (Sec)</label>
                            <input v-model="newSubtest.timeLimit" type="number" class="input-glass w-full text-sm"
                                placeholder="0 = Unlimited">
                        </div>
                    </div>
                    <div class="flex justify-end gap-2">
                        <button @click="showAddSubtest = false"
                            class="px-3 py-1.5 text-xs text-gray-900 dark:text-eling-light/70 hover:bg-black/10 dark:bg-white/10 rounded">Cancel</button>
                        <button @click="handleAddSubtest" class="btn-neumorphic px-4 py-1.5 text-xs">Save
                            Subtest</button>
                    </div>
                </div>

                <div v-if="!showAddSubtest" class="flex justify-end">
                    <button @click="showAddSubtest = true" class="btn-neumorphic text-xs py-2 px-3 flex items-center">
                        <svg class="w-3 h-3 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
                        </svg>
                        Add Subtest
                    </button>
                </div>

                <!-- Subtest List -->
                <div class="space-y-2">
                    <div v-for="item in toolData.subtests" :key="item.subtest.id"
                        class="p-4 bg-black/5 dark:bg-white/5 rounded-lg border border-black/5 dark:border-white/5 hover:border-black/10 dark:border-white/10 transition-colors flex justify-between items-center group">

                        <div>
                            <div class="flex items-center gap-3">
                                <span class="text-xs font-mono text-gray-900 dark:text-eling-light/30">#{{
                                    item.subtest.sequence_order
                                    }}</span>
                                <h4 class="text-sm font-medium text-gray-900 dark:text-eling-light">{{
                                    item.subtest.subtest_name }}</h4>
                                <span v-if="item.subtest.time_limit_seconds"
                                    class="text-[10px] px-1.5 py-0.5 rounded bg-black/5 dark:bg-white/5 text-gray-900 dark:text-eling-light/50 border border-black/5 dark:border-white/5">
                                    {{ Math.floor(item.subtest.time_limit_seconds / 60) }} min
                                </span>
                                <span v-else
                                    class="text-[10px] px-1.5 py-0.5 rounded bg-black/5 dark:bg-white/5 text-gray-900 dark:text-eling-light/50 border border-black/5 dark:border-white/5">Unlimited</span>
                            </div>
                            <p class="text-xs text-gray-900 dark:text-eling-light/40 mt-1 pl-7">{{ item.questions.length
                                }} Questions
                                configured</p>
                        </div>

                        <div class="flex items-center gap-2 opacity-50 group-hover:opacity-100 transition-opacity">
                            <button @click="openQuestionManager(item)"
                                class="p-1.5 text-eling-accent hover:bg-eling-accent/10 rounded"
                                title="Manage Questions">
                                <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                        d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" />
                                </svg>
                            </button>
                            <button @click="handleDeleteSubtest(item.subtest.id)"
                                class="p-1.5 text-red-400 hover:bg-red-500/10 rounded" title="Delete Subtest">
                                <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                        d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                                </svg>
                            </button>
                        </div>
                    </div>

                    <div v-if="toolData.subtests.length === 0"
                        class="text-center py-8 border-2 border-dashed border-black/5 dark:border-white/5 rounded-lg">
                        <p class="text-sm text-gray-900 dark:text-eling-light/30">No subtests configured for this tool.
                        </p>
                    </div>
                </div>
            </div>

            <!-- Manage Questions Modal -->
            <ManageQuestionsModal v-if="selectedSubtestForEdit" :show="!!selectedSubtestForEdit"
                :subtestId="selectedSubtestForEdit.subtest.id"
                :subtestName="selectedSubtestForEdit.subtest.subtest_name"
                :existingQuestions="selectedSubtestForEdit.questions" @close="selectedSubtestForEdit = null"
                @refresh="handleRefreshAfterEdit" />


            <!-- PREVIEW TAB -->
            <div v-if="activeTab === 'preview'" class="animate-fade-in">

                <!-- OVERVIEW MODE - Before starting test -->
                <div v-if="previewMode === 'overview'" class="max-w-2xl mx-auto">
                    <div
                        class="bg-gradient-to-br from-eling-accent/10 to-purple-500/10 rounded-2xl p-8 border border-eling-accent/20 text-center">
                        <div
                            class="w-20 h-20 rounded-2xl bg-eling-accent/20 flex items-center justify-center mx-auto mb-6">
                            <span class="text-3xl font-bold text-eling-accent">{{ toolData?.tool.name.charAt(0)
                                }}</span>
                        </div>
                        <h2 class="text-2xl font-bold text-gray-900 dark:text-eling-light mb-2">{{ toolData?.tool.name
                            }}</h2>
                        <p class="text-sm text-gray-900 dark:text-eling-light/60 mb-6">{{ toolData?.tool.description ||
                            'Preview tes psikologi' }}</p>

                        <!-- Test Info -->
                        <div class="grid grid-cols-3 gap-4 mb-8">
                            <div class="bg-black/5 dark:bg-white/5 rounded-xl p-4">
                                <div class="text-2xl font-bold text-blue-400">{{ toolData?.subtests.length || 0 }}</div>
                                <div class="text-xs text-gray-900 dark:text-eling-light/50">Subtests</div>
                            </div>
                            <div class="bg-black/5 dark:bg-white/5 rounded-xl p-4">
                                <div class="text-2xl font-bold text-purple-400">{{ getTotalQuestions() }}</div>
                                <div class="text-xs text-gray-900 dark:text-eling-light/50">Questions</div>
                            </div>
                            <div class="bg-black/5 dark:bg-white/5 rounded-xl p-4">
                                <div class="text-2xl font-bold text-green-400">{{ getTotalTime() }}</div>
                                <div class="text-xs text-gray-900 dark:text-eling-light/50">Minutes</div>
                            </div>
                        </div>

                        <!-- Subtest List -->
                        <div class="text-left mb-8">
                            <h4
                                class="text-xs font-bold text-gray-900 dark:text-eling-light/50 uppercase tracking-wider mb-3">
                                Test
                                Structure</h4>
                            <div class="space-y-2">
                                <div v-for="(item, idx) in toolData?.subtests" :key="item.subtest.id"
                                    class="bg-black/5 dark:bg-white/5 rounded-lg p-3 flex items-center justify-between">
                                    <div class="flex items-center gap-3">
                                        <span
                                            class="w-6 h-6 rounded bg-eling-accent/10 flex items-center justify-center text-xs font-bold text-eling-accent">{{
                                                idx + 1 }}</span>
                                        <span class="text-sm text-gray-900 dark:text-eling-light">{{
                                            item.subtest.subtest_name }}</span>
                                    </div>
                                    <div class="flex items-center gap-2 text-xs text-gray-900 dark:text-eling-light/50">
                                        <span>{{ item.questions.length }} soal</span>
                                        <span v-if="item.subtest.time_limit_seconds" class="text-orange-400">
                                            {{ Math.floor(item.subtest.time_limit_seconds / 60) }} min
                                        </span>
                                    </div>
                                </div>
                            </div>
                        </div>

                        <button @click="startPreview" :disabled="getTotalQuestions() === 0"
                            class="btn-neumorphic px-8 py-3 text-sm font-bold disabled:opacity-50 disabled:cursor-not-allowed">
                            🚀 Mulai Preview Test
                        </button>
                    </div>
                </div>

                <!-- TESTING MODE - Question by question -->
                <div v-else-if="previewMode === 'testing' && currentSubtest && currentQuestion"
                    class="max-w-3xl mx-auto">
                    <!-- Top Bar -->
                    <div
                        class="bg-black/5 dark:bg-white/5 rounded-xl p-4 mb-6 flex items-center justify-between border border-black/10 dark:border-white/10">
                        <div class="flex items-center gap-4">
                            <div class="text-sm">
                                <span class="text-gray-900 dark:text-eling-light/50">Subtest:</span>
                                <span class="text-gray-900 dark:text-eling-light font-bold ml-1">{{
                                    currentSubtest.subtest.subtest_name
                                    }}</span>
                            </div>
                            <div class="h-4 w-px bg-black/10 dark:bg-white/10"></div>
                            <div class="text-sm">
                                <span class="text-gray-900 dark:text-eling-light/50">Question:</span>
                                <span class="text-eling-accent font-bold ml-1">{{ currentQuestionIndex + 1 }}/{{
                                    totalQuestionsInSubtest }}</span>
                            </div>
                        </div>

                        <!-- Timer -->
                        <div v-if="currentSubtest.subtest.time_limit_seconds"
                            class="flex items-center gap-2 px-4 py-2 rounded-lg"
                            :class="previewTimeRemaining < 60 ? 'bg-red-500/20 text-red-400' : 'bg-orange-500/10 text-orange-400'">
                            <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                    d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
                            </svg>
                            <span class="font-mono font-bold">{{ formatTime(previewTimeRemaining) }}</span>
                        </div>

                        <button @click="resetPreview"
                            class="text-xs text-red-400 hover:text-red-300 px-3 py-1 rounded hover:bg-red-500/10">
                            Exit Preview
                        </button>
                    </div>

                    <!-- Progress Bar -->
                    <div class="h-1 bg-black/10 dark:bg-white/10 rounded-full mb-6 overflow-hidden">
                        <div class="h-full bg-gradient-to-r from-eling-accent to-purple-500 transition-all duration-300"
                            :style="{ width: progressPercent + '%' }"></div>
                    </div>

                    <!-- Question Card -->
                    <div
                        class="bg-gradient-to-br from-white/5 to-white/[0.02] rounded-2xl p-8 border border-black/10 dark:border-white/10 mb-6">
                        <div class="flex items-start gap-4 mb-6">
                            <div
                                class="w-12 h-12 rounded-xl bg-eling-accent/20 flex items-center justify-center shrink-0">
                                <span class="text-lg font-bold text-eling-accent">{{ currentQuestionIndex + 1 }}</span>
                            </div>
                            <p class="text-lg text-gray-900 dark:text-eling-light leading-relaxed flex-1">{{
                                currentQuestion.question_text
                                }}</p>
                        </div>

                        <!-- Multiple Choice Options -->
                        <div v-if="currentQuestion.question_type === 'multiple_choice'" class="space-y-3">
                            <button v-for="(opt, optIdx) in (currentQuestion.options?.choices || [])" :key="optIdx"
                                @click="selectAnswer(opt)"
                                class="w-full text-left p-4 rounded-xl border-2 transition-all flex items-center gap-4"
                                :class="previewAnswers[currentQuestion.id] === opt
                                    ? 'bg-eling-accent/10 border-eling-accent text-gray-900 dark:text-eling-light'
                                    : 'bg-black/5 dark:bg-white/5 border-black/10 dark:border-white/10 text-gray-900 dark:text-eling-light/70 hover:border-black/20 dark:border-white/20 hover:bg-black/10 dark:bg-white/10'">
                                <span class="w-8 h-8 rounded-lg flex items-center justify-center text-sm font-bold"
                                    :class="previewAnswers[currentQuestion.id] === opt ? 'bg-eling-accent text-eling-dark' : 'bg-black/10 dark:bg-white/10'">
                                    {{ String.fromCharCode(65 + optIdx) }}
                                </span>
                                <span class="flex-1">{{ opt }}</span>
                                <svg v-if="previewAnswers[currentQuestion.id] === opt" class="w-5 h-5 text-eling-accent"
                                    fill="currentColor" viewBox="0 0 20 20">
                                    <path fill-rule="evenodd"
                                        d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
                                        clip-rule="evenodd" />
                                </svg>
                            </button>
                        </div>

                        <!-- Kraepelin Column Display - Paper-like Layout -->
                        <div v-else-if="currentQuestion.question_type === 'kraepelin_column'" class="overflow-x-auto">
                            <!-- Paper Sheet -->
                            <div
                                class="bg-white dark:bg-gray-800 border-2 border-gray-400 dark:border-gray-600 shadow-xl p-6 rounded-sm min-w-max">
                                <div class="text-center mb-4">
                                    <h4 class="text-sm font-bold text-gray-700 dark:text-eling-light mb-2">Kraepelin
                                        Test Preview</h4>
                                    <p class="text-xs text-gray-500 dark:text-eling-light/50">Showing {{ Math.min(10,
                                        currentSubtest?.questions.length || 0) }} columns (paper-like layout)</p>
                                </div>

                                <!-- Columns Grid - Horizontal Layout -->
                                <div class="flex gap-3">
                                    <div v-for="(question, qIdx) in currentSubtest?.questions.slice(0, 10)" :key="qIdx"
                                        class="flex-shrink-0">
                                        <!-- Column -->
                                        <div class="flex flex-col gap-0">
                                            <!-- Column Number -->
                                            <div class="text-center mb-1">
                                                <span class="text-xs font-mono text-gray-400 dark:text-gray-500">{{ qIdx
                                                    + 1 }}</span>
                                            </div>

                                            <!-- Numbers -->
                                            <div v-for="(num, numIdx) in (question.options?.numbers || [])"
                                                :key="numIdx">
                                                <!-- Number Cell -->
                                                <div class="w-8 h-6 flex items-center justify-center text-xs font-mono border border-gray-300 dark:border-gray-600 bg-gray-50 dark:bg-gray-700"
                                                    :class="{
                                                        'bg-blue-100 dark:bg-blue-900 border-blue-400 dark:border-blue-500 font-bold': qIdx === currentQuestionIndex && numIdx === 0,
                                                        'bg-blue-50 dark:bg-blue-900/50': qIdx === currentQuestionIndex && numIdx === 1
                                                    }">
                                                    {{ num }}
                                                </div>

                                                <!-- Answer Slot (between numbers) -->
                                                <div v-if="numIdx < (question.options?.numbers || []).length - 1"
                                                    class="w-8 h-5 flex items-center justify-center text-xs font-mono border-x border-gray-200 dark:border-gray-600 bg-white dark:bg-gray-800"
                                                    :class="{
                                                        'bg-yellow-100 dark:bg-yellow-900/30 border-yellow-400 dark:border-yellow-600': qIdx === currentQuestionIndex && numIdx === 0
                                                    }">
                                                    <!-- Empty answer slot -->
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                </div>

                                <!-- Legend -->
                                <div class="mt-6 flex items-center justify-center gap-6 text-xs">
                                    <div class="flex items-center gap-2">
                                        <div
                                            class="w-4 h-4 bg-blue-100 dark:bg-blue-900 border border-blue-400 dark:border-blue-500">
                                        </div>
                                        <span class="text-gray-600 dark:text-gray-400">Current position</span>
                                    </div>
                                    <div class="flex items-center gap-2">
                                        <div
                                            class="w-4 h-4 bg-yellow-100 dark:bg-yellow-900/30 border border-yellow-400 dark:border-yellow-600">
                                        </div>
                                        <span class="text-gray-600 dark:text-gray-400">Answer slot</span>
                                    </div>
                                </div>

                                <!-- Info -->
                                <div class="mt-4 text-center">
                                    <p class="text-xs text-gray-500 dark:text-eling-light/50">
                                        Total {{ currentSubtest?.questions.length || 0 }} columns × {{
                                            (currentQuestion.options?.numbers || []).length }} numbers each
                                    </p>
                                    <p class="text-xs text-gray-400 dark:text-gray-600 mt-1">
                                        Scroll horizontally to see more columns →
                                    </p>
                                </div>
                            </div>
                        </div>

                        <!-- Other question types placeholder -->
                        <div v-else class="text-center py-8 text-gray-900 dark:text-eling-light/50">
                            <p class="text-sm">Question type: {{ currentQuestion.question_type }}</p>
                            <p class="text-xs mt-2">Preview not available for this question type</p>
                        </div>
                    </div>

                    <!-- Navigation Buttons -->
                    <div class="flex justify-between items-center">
                        <button @click="prevQuestion" :disabled="currentQuestionIndex === 0"
                            class="px-6 py-3 rounded-xl text-sm font-medium transition-all disabled:opacity-30 disabled:cursor-not-allowed bg-black/5 dark:bg-white/5 text-gray-900 dark:text-eling-light/70 hover:bg-black/10 dark:bg-white/10">
                            ← Previous
                        </button>

                        <div class="flex gap-1">
                            <!-- Show first page indicator if not in window -->
                            <button v-if="paginationWindow[0] > 1" @click="currentQuestionIndex = 0"
                                class="w-8 h-8 rounded text-xs font-medium transition-all bg-black/5 dark:bg-white/5 text-gray-900 dark:text-eling-light/50 hover:bg-black/10 dark:bg-white/10">
                                1
                            </button>
                            <span v-if="paginationWindow[0] > 2"
                                class="flex items-center px-1 text-gray-900 dark:text-eling-light/50">...</span>

                            <!-- Pagination window -->
                            <button v-for="i in paginationWindow" :key="i" @click="currentQuestionIndex = i - 1"
                                class="w-8 h-8 rounded text-xs font-medium transition-all"
                                :class="currentQuestionIndex === i - 1
                                    ? 'bg-eling-accent text-eling-dark'
                                    : previewAnswers[currentSubtest.questions[i - 1]?.id]
                                        ? 'bg-green-500/20 text-green-400'
                                        : 'bg-black/5 dark:bg-white/5 text-gray-900 dark:text-eling-light/50 hover:bg-black/10 dark:bg-white/10'">
                                {{ i }}
                            </button>

                            <!-- Show last page indicator if not in window -->
                            <span v-if="paginationWindow[paginationWindow.length - 1] < totalQuestionsInSubtest - 1"
                                class="flex items-center px-1 text-gray-900 dark:text-eling-light/50">...</span>
                            <button v-if="paginationWindow[paginationWindow.length - 1] < totalQuestionsInSubtest"
                                @click="currentQuestionIndex = totalQuestionsInSubtest - 1"
                                class="w-8 h-8 rounded text-xs font-medium transition-all bg-black/5 dark:bg-white/5 text-gray-900 dark:text-eling-light/50 hover:bg-black/10 dark:bg-white/10">
                                {{ totalQuestionsInSubtest }}
                            </button>
                        </div>

                        <button @click="nextQuestion" class="px-6 py-3 rounded-xl text-sm font-medium transition-all"
                            :class="currentQuestionIndex === totalQuestionsInSubtest - 1
                                ? 'bg-green-500 text-gray-900 dark:text-white hover:bg-green-600'
                                : 'bg-eling-accent text-eling-dark hover:bg-eling-accent/90'">
                            {{ currentQuestionIndex === totalQuestionsInSubtest - 1 ? 'Finish Subtest →' : 'Next →' }}
                        </button>
                    </div>
                </div>

                <!-- RESULTS MODE - After completing test -->
                <div v-else-if="previewMode === 'results'" class="max-w-3xl mx-auto">
                    <div
                        class="bg-gradient-to-br from-green-500/10 to-blue-500/10 rounded-2xl p-8 border border-green-500/20 text-center mb-8">
                        <div
                            class="w-20 h-20 rounded-full bg-green-500/20 flex items-center justify-center mx-auto mb-6">
                            <svg class="w-10 h-10 text-green-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                    d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                            </svg>
                        </div>
                        <h2 class="text-2xl font-bold text-gray-900 dark:text-eling-light mb-2">Preview Completed!</h2>
                        <p class="text-sm text-gray-900 dark:text-eling-light/60 mb-6">Berikut adalah hasil preview {{
                            toolData?.tool.name
                            }}</p>

                        <!-- Score Display -->
                        <div class="bg-black/5 dark:bg-white/5 rounded-2xl p-6 mb-6">
                            <div class="text-5xl font-bold text-eling-accent mb-2">{{ calculateScore().percentage }}%
                            </div>
                            <div class="text-sm text-gray-900 dark:text-eling-light/50">{{ calculateScore().correct }} /
                                {{
                                    calculateScore().total }} Correct</div>
                        </div>
                    </div>

                    <!-- Detailed Results per Subtest -->
                    <div
                        class="bg-black/5 dark:bg-white/5 rounded-xl p-6 border border-black/10 dark:border-white/10 mb-6">
                        <h4 class="text-sm font-bold text-gray-900 dark:text-eling-light mb-4">📊 Hasil per Subtest</h4>
                        <div class="space-y-3">
                            <div v-for="(detail, subtestName) in calculateScore().details" :key="subtestName"
                                class="flex items-center justify-between p-3 bg-black/5 dark:bg-white/5 rounded-lg">
                                <span class="text-sm text-gray-900 dark:text-eling-light">{{ subtestName }}</span>
                                <div class="flex items-center gap-3">
                                    <div class="w-32 h-2 bg-black/10 dark:bg-white/10 rounded-full overflow-hidden">
                                        <div class="h-full bg-eling-accent transition-all"
                                            :style="{ width: (detail.total > 0 ? (detail.correct / detail.total) * 100 : 0) + '%' }">
                                        </div>
                                    </div>
                                    <span class="text-xs text-gray-900 dark:text-eling-light/50 w-16 text-right">{{
                                        detail.correct }}/{{
                                            detail.total }}</span>
                                </div>
                            </div>
                        </div>
                    </div>

                    <!-- Tool-specific interpretation based on category -->
                    <div
                        class="bg-black/5 dark:bg-white/5 rounded-xl p-6 border border-black/10 dark:border-white/10 mb-6">
                        <h4 class="text-sm font-bold text-gray-900 dark:text-eling-light mb-4">🎯 Interpretasi {{
                            toolData?.tool.category
                            }}</h4>
                        <div v-if="toolData?.tool.category === 'kognitif'"
                            class="text-sm text-gray-900 dark:text-eling-light/70">
                            <p class="mb-2">Berdasarkan hasil tes kognitif:</p>
                            <div class="bg-blue-500/10 rounded-lg p-4 border border-blue-500/20">
                                <div v-if="calculateScore().percentage >= 80" class="text-blue-300">
                                    <strong>Superior</strong> - Kemampuan kognitif sangat baik, di atas rata-rata.
                                </div>
                                <div v-else-if="calculateScore().percentage >= 60" class="text-green-300">
                                    <strong>Above Average</strong> - Kemampuan kognitif di atas rata-rata.
                                </div>
                                <div v-else-if="calculateScore().percentage >= 40" class="text-yellow-300">
                                    <strong>Average</strong> - Kemampuan kognitif dalam rentang normal.
                                </div>
                                <div v-else class="text-orange-300">
                                    <strong>Below Average</strong> - Perlu pengembangan kemampuan kognitif.
                                </div>
                            </div>
                        </div>
                        <div v-else-if="toolData?.tool.category === 'kepribadian'"
                            class="text-sm text-gray-900 dark:text-eling-light/70">
                            <p class="mb-2">Profil kepribadian berdasarkan respons:</p>
                            <div class="bg-purple-500/10 rounded-lg p-4 border border-purple-500/20 text-purple-300">
                                Analisis kepribadian memerlukan interpretasi lebih lanjut oleh psikolog profesional.
                            </div>
                        </div>
                        <div v-else class="text-sm text-gray-900 dark:text-eling-light/70">
                            <p>Hasil tes ini memerlukan interpretasi profesional untuk analisis lebih lanjut.</p>
                        </div>
                    </div>

                    <div class="flex justify-center gap-4">
                        <button @click="resetPreview" class="btn-neumorphic px-6 py-3 text-sm">
                            🔄 Ulangi Preview
                        </button>
                        <button @click="activeTab = 'subtests'"
                            class="px-6 py-3 rounded-xl bg-black/5 dark:bg-white/5 text-gray-900 dark:text-eling-light/70 hover:bg-black/10 dark:bg-white/10 text-sm">
                            ← Kembali ke Subtests
                        </button>
                    </div>
                </div>

                <!-- Empty State -->
                <div v-else-if="!toolData?.subtests.length" class="text-center py-16">
                    <div
                        class="w-16 h-16 rounded-2xl bg-black/5 dark:bg-white/5 flex items-center justify-center mx-auto mb-4">
                        <svg class="w-8 h-8 text-gray-900 dark:text-eling-light/30" fill="none" viewBox="0 0 24 24"
                            stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" />
                        </svg>
                    </div>
                    <h4 class="text-lg font-bold text-gray-900 dark:text-eling-light/50 mb-2">No Subtests Yet</h4>
                    <p class="text-sm text-gray-900 dark:text-eling-light/30 mb-4">Create subtests to start building
                        your test</p>
                    <button @click="activeTab = 'subtests'" class="btn-neumorphic text-xs py-2 px-4">Go to
                        Subtests</button>
                </div>
            </div>

        </div>
    </div>
</template>
