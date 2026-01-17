<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue';

// Kraepelin Test Configuration
interface KraepelinConfig {
    columns: number;      // Total columns (typically 45-50)
    rows: number;         // Numbers per column (typically 50-60)
    timePerColumn: number; // Seconds per column (typically 60s)
    practiceColumns?: number; // Optional practice columns
}

const props = defineProps<{
    config?: KraepelinConfig;
    isPractice?: boolean;
}>();

const emit = defineEmits<{
    (e: 'complete', data: { answers: number[][], times: number[], stats: any }): void;
    (e: 'columnComplete', columnIndex: number, answers: number[], time: number): void;
}>();

// Default configuration
const config = computed(() => ({
    columns: props.config?.columns || 45,
    rows: props.config?.rows || 50,
    timePerColumn: props.config?.timePerColumn || 60,
    practiceColumns: props.config?.practiceColumns || 2
}));

// Test state
const testPhase = ref<'ready' | 'running' | 'paused' | 'complete'>('ready');
const currentColumn = ref(0);
const currentRow = ref(0);
const timeRemaining = ref(config.value.timePerColumn);
const timerInterval = ref<any>(null);

// Data
const columns = ref<number[][]>([]);
const answers = ref<(number | null)[][]>([]);
const columnTimes = ref<number[]>([]);
const columnStartTime = ref<number>(0);

// Input state
// Input state
const inputRef = ref<HTMLInputElement | null>(null);

// Generate random numbers for columns
function generateColumns() {
    const cols: number[][] = [];
    const totalCols = props.isPractice ? config.value.practiceColumns : config.value.columns;

    for (let c = 0; c < totalCols; c++) {
        const col: number[] = [];
        for (let r = 0; r < config.value.rows; r++) {
            col.push(Math.floor(Math.random() * 9) + 1); // 1-9
        }
        cols.push(col);
    }
    columns.value = cols;

    // Initialize empty answers
    answers.value = cols.map(() => Array(config.value.rows - 1).fill(null));
}

// Start test
function startTest() {
    generateColumns();
    currentColumn.value = 0;
    currentRow.value = 0;
    testPhase.value = 'running';
    columnStartTime.value = Date.now();
    startTimer();
    focusInput();
}

// Timer
function startTimer() {
    timeRemaining.value = config.value.timePerColumn;
    if (timerInterval.value) clearInterval(timerInterval.value);

    timerInterval.value = setInterval(() => {
        timeRemaining.value--;

        if (timeRemaining.value <= 0) {
            nextColumn();
        }
    }, 1000);
}

function stopTimer() {
    if (timerInterval.value) {
        clearInterval(timerInterval.value);
        timerInterval.value = null;
    }
}

// Navigation
function nextColumn() {
    stopTimer();

    // Record time for this column
    const elapsed = Math.round((Date.now() - columnStartTime.value) / 1000);
    columnTimes.value.push(elapsed);

    // Emit column complete
    const currentAnswers = answers.value[currentColumn.value].map(a => a ?? -1);
    emit('columnComplete', currentColumn.value, currentAnswers, elapsed);

    // Check if test is complete
    const totalCols = props.isPractice ? config.value.practiceColumns : config.value.columns;
    if (currentColumn.value >= totalCols - 1) {
        completeTest();
        return;
    }

    // Move to next column
    currentColumn.value++;
    currentRow.value = 0;
    columnStartTime.value = Date.now();
    startTimer();
    focusInput();
}

function completeTest() {
    testPhase.value = 'complete';
    stopTimer();

    // Calculate statistics
    const stats = calculateStats();

    // Convert answers to number[][] by replacing nulls with -1 for type safety
    const safeAnswers = answers.value.map(col => col.map(r => r ?? -1));

    emit('complete', {
        answers: safeAnswers,
        times: columnTimes.value,
        stats
    });
}

// Input handling
function handleInput(e: Event) {
    const target = e.target as HTMLInputElement;
    const value = target.value;

    // Only allow single digit
    if (value.length > 0) {
        const digit = parseInt(value.slice(-1));
        if (!isNaN(digit)) {
            submitAnswer(digit);
        }
    }
    target.value = '';
}

function handleKeyDown(e: KeyboardEvent) {
    // Quick number input
    if (e.key >= '0' && e.key <= '9') {
        e.preventDefault();
        submitAnswer(parseInt(e.key));
    }
    // Move to next column (Enter or Space when at end)
    else if (e.key === 'Enter' && currentRow.value >= config.value.rows - 2) {
        e.preventDefault();
        nextColumn();
    }
}

function submitAnswer(digit: number) {
    if (testPhase.value !== 'running') return;

    // Record answer
    answers.value[currentColumn.value][currentRow.value] = digit;

    // Move to next row
    if (currentRow.value < config.value.rows - 2) {
        currentRow.value++;
    }

    focusInput();
}

function focusInput() {
    setTimeout(() => {
        inputRef.value?.focus();
    }, 10);
}

// Calculate correct answer for a position
function getCorrectAnswer(colIndex: number, rowIndex: number): number {
    const col = columns.value[colIndex];
    if (!col || rowIndex >= col.length - 1) return 0;

    // Add two adjacent numbers, take last digit
    return (col[rowIndex] + col[rowIndex + 1]) % 10;
}

// Check if answer is correct
function isCorrect(colIndex: number, rowIndex: number): boolean | null {
    const answer = answers.value[colIndex]?.[rowIndex];
    if (answer === null || answer === undefined) return null;
    return answer === getCorrectAnswer(colIndex, rowIndex);
}

// Calculate statistics
function calculateStats() {
    let totalCorrect = 0;
    let totalAnswered = 0;
    const columnStats: { correct: number; total: number; accuracy: number }[] = [];

    for (let c = 0; c < answers.value.length; c++) {
        let colCorrect = 0;
        let colTotal = 0;

        for (let r = 0; r < answers.value[c].length; r++) {
            if (answers.value[c][r] !== null) {
                colTotal++;
                totalAnswered++;
                if (isCorrect(c, r)) {
                    colCorrect++;
                    totalCorrect++;
                }
            }
        }

        columnStats.push({
            correct: colCorrect,
            total: colTotal,
            accuracy: colTotal > 0 ? Math.round((colCorrect / colTotal) * 100) : 0
        });
    }

    return {
        totalCorrect,
        totalAnswered,
        totalPossible: answers.value.length * (config.value.rows - 1),
        overallAccuracy: totalAnswered > 0 ? Math.round((totalCorrect / totalAnswered) * 100) : 0,
        columnStats
    };
}

// Computed
const progress = computed(() => {
    const totalCols = props.isPractice ? config.value.practiceColumns : config.value.columns;
    return Math.round(((currentColumn.value + 1) / totalCols) * 100);
});

const currentNumbers = computed(() => {
    if (!columns.value[currentColumn.value]) return [];
    return columns.value[currentColumn.value];
});

const formattedTime = computed(() => {
    const mins = Math.floor(timeRemaining.value / 60);
    const secs = timeRemaining.value % 60;
    return `${mins.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
});

onMounted(() => {
    focusInput();
});

onUnmounted(() => {
    stopTimer();
});
</script>

<template>
    <div class="min-h-screen bg-gradient-to-br from-[#0a0f1a] to-[#1a1f2e] text-white">

        <!-- READY STATE -->
        <div v-if="testPhase === 'ready'" class="min-h-screen flex items-center justify-center p-4">
            <div class="max-w-lg w-full text-center">
                <div class="bg-white/5 backdrop-blur-xl rounded-2xl border border-white/10 p-8">
                    <div
                        class="w-20 h-20 rounded-2xl bg-gradient-to-br from-orange-400 to-red-600 flex items-center justify-center mx-auto mb-6 shadow-lg">
                        <svg class="w-10 h-10 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                d="M9 7h6m0 10v-3m-3 3h.01M9 17h.01M9 14h.01M12 14h.01M15 11h.01M12 11h.01M9 11h.01M7 21h10a2 2 0 002-2V5a2 2 0 00-2-2H7a2 2 0 00-2 2v14a2 2 0 002 2z" />
                        </svg>
                    </div>

                    <h2 class="text-2xl font-bold mb-2">Tes Kraepelin</h2>
                    <p class="text-white/60 mb-6">{{ isPractice ? 'Mode Latihan' : 'Tes Utama' }}</p>

                    <!-- Instructions -->
                    <div class="bg-white/5 rounded-xl p-4 text-left mb-6">
                        <h4 class="text-sm font-bold text-orange-400 mb-3">📋 Petunjuk:</h4>
                        <ul class="text-sm text-white/70 space-y-2">
                            <li>• Jumlahkan 2 angka yang bersebelahan</li>
                            <li>• Tuliskan <strong class="text-white">angka satuan</strong> dari hasil penjumlahan</li>
                            <li>• Contoh: 7 + 8 = 15 → tulis <strong class="text-cyan-400">5</strong></li>
                            <li>• Kerjakan secepat dan seteliti mungkin</li>
                            <li>• Waktu: {{ config.timePerColumn }} detik per kolom</li>
                        </ul>
                    </div>

                    <!-- Example -->
                    <div class="bg-orange-500/10 rounded-xl p-4 border border-orange-500/20 mb-6">
                        <div class="flex justify-center items-center gap-8">
                            <div class="text-center">
                                <div class="text-3xl font-bold text-white">7</div>
                                <div class="text-xl text-orange-400">+</div>
                                <div class="text-3xl font-bold text-white">8</div>
                            </div>
                            <div class="text-3xl text-white/50">=</div>
                            <div class="text-center">
                                <div class="text-4xl font-bold text-cyan-400">5</div>
                                <div class="text-xs text-white/50">(15 → 5)</div>
                            </div>
                        </div>
                    </div>

                    <button @click="startTest"
                        class="w-full py-4 rounded-xl bg-gradient-to-r from-orange-500 to-red-600 text-white font-bold text-lg hover:from-orange-400 hover:to-red-500 transition-all shadow-lg">
                        🚀 {{ isPractice ? 'Mulai Latihan' : 'Mulai Tes' }}
                    </button>
                </div>
            </div>
        </div>

        <!-- RUNNING STATE -->
        <div v-else-if="testPhase === 'running'" class="min-h-screen flex flex-col">
            <!-- Top Bar -->
            <div class="bg-white/5 backdrop-blur-xl border-b border-white/10 px-6 py-4">
                <div class="max-w-4xl mx-auto flex items-center justify-between">
                    <div class="flex items-center gap-6">
                        <div class="text-sm">
                            <span class="text-white/50">Kolom:</span>
                            <span class="text-cyan-400 font-bold ml-1">{{ currentColumn + 1 }}/{{ isPractice ?
                                config.practiceColumns : config.columns }}</span>
                        </div>
                        <div class="h-4 w-px bg-white/10"></div>
                        <div class="text-sm">
                            <span class="text-white/50">Baris:</span>
                            <span class="text-white font-bold ml-1">{{ currentRow + 1 }}/{{ config.rows - 1 }}</span>
                        </div>
                    </div>

                    <!-- Timer -->
                    <div class="flex items-center gap-2 px-4 py-2 rounded-xl"
                        :class="timeRemaining <= 10 ? 'bg-red-500/20 animate-pulse' : 'bg-orange-500/10'">
                        <svg class="w-5 h-5" :class="timeRemaining <= 10 ? 'text-red-400' : 'text-orange-400'"
                            fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
                        </svg>
                        <span class="font-mono font-bold text-lg"
                            :class="timeRemaining <= 10 ? 'text-red-400' : 'text-orange-400'">
                            {{ formattedTime }}
                        </span>
                    </div>
                </div>
            </div>

            <!-- Progress Bar -->
            <div class="h-1 bg-white/10">
                <div class="h-full bg-gradient-to-r from-orange-500 to-red-500 transition-all duration-300"
                    :style="{ width: progress + '%' }"></div>
            </div>

            <!-- Main Content -->
            <div class="flex-1 flex items-center justify-center p-6">
                <div class="flex gap-8 items-start">
                    <!-- Number Column -->
                    <div class="bg-white/5 backdrop-blur-xl rounded-2xl border border-white/10 p-6">
                        <div class="space-y-1">
                            <div v-for="(num, idx) in currentNumbers" :key="idx" class="flex items-center gap-4">
                                <!-- Number -->
                                <div class="w-16 h-12 flex items-center justify-center text-3xl font-bold rounded-lg transition-all"
                                    :class="idx === currentRow ? 'bg-cyan-500/20 text-cyan-400 scale-110' :
                                        idx === currentRow + 1 ? 'bg-cyan-500/10 text-cyan-300' :
                                            'text-white/70'">
                                    {{ num }}
                                </div>

                                <!-- Answer slot (between numbers) -->
                                <div v-if="idx < currentNumbers.length - 1"
                                    class="w-12 h-10 flex items-center justify-center rounded-lg border-2 text-xl font-bold transition-all"
                                    :class="idx === currentRow ? 'border-orange-500 bg-orange-500/20 text-orange-400 animate-pulse' :
                                        answers[currentColumn][idx] !== null ?
                                            (isCorrect(currentColumn, idx) ? 'border-green-500/50 bg-green-500/10 text-green-400' : 'border-red-500/50 bg-red-500/10 text-red-400') :
                                            'border-white/10 text-white/30'">
                                    {{ answers[currentColumn][idx] ?? '?' }}
                                </div>
                            </div>
                        </div>
                    </div>

                    <!-- Input Area -->
                    <div class="bg-white/5 backdrop-blur-xl rounded-2xl border border-white/10 p-6 w-64">
                        <div class="text-center mb-4">
                            <div class="text-sm text-white/50 mb-2">Hasil Penjumlahan</div>
                            <div class="text-4xl font-bold text-white mb-1">
                                {{ currentNumbers[currentRow] }} + {{ currentNumbers[currentRow + 1] }}
                            </div>
                            <div class="text-sm text-white/50">= {{ currentNumbers[currentRow] +
                                currentNumbers[currentRow + 1] }} → ketik <span class="text-cyan-400 font-bold">{{
                                    (currentNumbers[currentRow] + currentNumbers[currentRow + 1]) % 10 }}</span></div>
                        </div>

                        <input ref="inputRef" type="number" inputmode="numeric"
                            class="w-full h-20 text-center text-4xl font-bold rounded-xl bg-white/10 border-2 border-orange-500 focus:border-cyan-500 focus:outline-none text-white"
                            @input="handleInput" @keydown="handleKeyDown" min="0" max="9" autofocus>

                        <p class="text-xs text-center text-white/40 mt-3">Ketik angka 0-9 untuk menjawab</p>

                        <button @click="nextColumn"
                            class="w-full mt-4 py-3 rounded-xl bg-white/10 text-white/70 hover:bg-white/20 hover:text-white font-medium transition-all text-sm">
                            Kolom Berikutnya →
                        </button>
                    </div>
                </div>
            </div>
        </div>

        <!-- COMPLETE STATE -->
        <div v-else-if="testPhase === 'complete'" class="min-h-screen flex items-center justify-center p-4">
            <div class="max-w-2xl w-full">
                <div class="bg-white/5 backdrop-blur-xl rounded-2xl border border-white/10 p-8 text-center">
                    <div
                        class="w-20 h-20 rounded-full bg-gradient-to-br from-green-400 to-emerald-600 flex items-center justify-center mx-auto mb-6 shadow-lg">
                        <svg class="w-10 h-10 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                        </svg>
                    </div>

                    <h2 class="text-2xl font-bold mb-2">{{ isPractice ? 'Latihan Selesai!' : 'Tes Selesai!' }}</h2>
                    <p class="text-white/60 mb-6">
                        {{ isPractice ? 'Anda siap untuk tes utama' : 'Terima kasih telah menyelesaikan tes Kraepelin' }}
                    </p>

                    <!-- Stats Summary -->
                    <div class="grid grid-cols-3 gap-4 mb-6">
                        <div class="bg-white/5 rounded-xl p-4">
                            <div class="text-2xl font-bold text-cyan-400">{{ calculateStats().totalAnswered }}</div>
                            <div class="text-xs text-white/50">Dijawab</div>
                        </div>
                        <div class="bg-white/5 rounded-xl p-4">
                            <div class="text-2xl font-bold text-green-400">{{ calculateStats().totalCorrect }}</div>
                            <div class="text-xs text-white/50">Benar</div>
                        </div>
                        <div class="bg-white/5 rounded-xl p-4">
                            <div class="text-2xl font-bold text-purple-400">{{ calculateStats().overallAccuracy }}%
                            </div>
                            <div class="text-xs text-white/50">Akurasi</div>
                        </div>
                    </div>

                    <div class="bg-blue-500/10 rounded-xl p-4 border border-blue-500/20 mb-6">
                        <p class="text-sm text-blue-200/80">
                            Hasil tes akan dianalisis untuk mengukur kecepatan kerja, ketelitian, dan daya tahan
                            konsentrasi Anda.
                        </p>
                    </div>

                    <button @click="testPhase = 'ready'"
                        class="px-8 py-3 rounded-xl bg-white/10 text-white font-medium hover:bg-white/20 transition-all">
                        {{ isPractice ? 'Ulangi Latihan' : 'Kembali' }}
                    </button>
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped>
input[type="number"]::-webkit-outer-spin-button,
input[type="number"]::-webkit-inner-spin-button {
    -webkit-appearance: none;
    margin: 0;
}

input[type="number"] {
    -moz-appearance: textfield;
}
</style>
