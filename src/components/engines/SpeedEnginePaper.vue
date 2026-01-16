<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';

// Kraepelin Test Configuration
interface KraepelinConfig {
    columns: number;      // Total columns (typically 45-50)
    rows: number;         // Numbers per column (typically 40-50)
    timePerColumn: number; // Seconds per column (typically 60s)
    practiceColumns?: number; // Optional practice columns
}

const props = defineProps<{
    config?: KraepelinConfig;
    isPractice?: boolean;
    userId?: number;
    eventId?: number;
    toolId?: number;
}>();

const emit = defineEmits<{
    (e: 'complete', data: { answers: number[][], times: number[], stats: any }): void;
    (e: 'columnComplete', columnIndex: number, answers: number[], time: number): void;
}>();

// Default configuration
const config = computed(() => ({
    columns: props.config?.columns || 45,
    rows: props.config?.rows || 40,
    timePerColumn: props.config?.timePerColumn || 60,
    practiceColumns: props.config?.practiceColumns || 3
}));

// Test state
const testPhase = ref<'ready' | 'running' | 'complete'>('ready');
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
const inputRef = ref<HTMLInputElement | null>(null);

// Database integration state
const sessionId = ref<number | null>(null);
const isSaving = ref(false);
const saveError = ref<string | null>(null);

// Accessibility
const screenReaderMessage = ref('');

function announceToScreenReader(message: string) {
    screenReaderMessage.value = message;
    setTimeout(() => {
        screenReaderMessage.value = '';
    }, 1000);
}

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
async function startTest() {
    generateColumns();
    currentColumn.value = 0;
    currentRow.value = 0;
    testPhase.value = 'running';
    columnStartTime.value = Date.now();

    // Create session in database (only for non-practice)
    if (!props.isPractice && props.userId && props.toolId) {
        try {
            sessionId.value = await invoke<number>('create_kraepelin_session', {
                userId: props.userId,
                eventId: props.eventId || null,
                toolId: props.toolId
            });
            console.log('Session created:', sessionId.value);
            announceToScreenReader('Test started');
        } catch (error) {
            console.error('Failed to create session:', error);
            saveError.value = 'Failed to create session';
            // Continue anyway - we'll retry on save
        }
    }

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
async function nextColumn() {
    stopTimer();

    // Record time for this column
    const elapsed = Math.round((Date.now() - columnStartTime.value) / 1000);
    columnTimes.value.push(elapsed);

    // Calculate stats for this column
    const currentAnswers = answers.value[currentColumn.value].map(a => a ?? -1);
    let correctCount = 0;
    let totalQuestions = 0;

    answers.value[currentColumn.value].forEach((answer, rowIdx) => {
        if (answer !== null) {
            totalQuestions++;
            if (isCorrect(currentColumn.value, rowIdx)) {
                correctCount++;
            }
        }
    });

    // Save to database (only for non-practice)
    if (!props.isPractice && sessionId.value) {
        try {
            isSaving.value = true;
            await invoke('save_kraepelin_column', {
                sessionId: sessionId.value,
                columnIndex: currentColumn.value,
                answers: currentAnswers,
                correctCount,
                totalQuestions,
                timeTaken: elapsed
            });
            saveError.value = null;
        } catch (error) {
            console.error('Failed to save column:', error);
            saveError.value = `Failed to save column ${currentColumn.value + 1}`;
            // Continue anyway - data is still in memory
        } finally {
            isSaving.value = false;
        }
    }

    // Emit column complete
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
    announceToScreenReader(`Column ${currentColumn.value + 1} of ${totalCols}`);
    startTimer();
    focusInput();
}

async function completeTest() {
    testPhase.value = 'complete';
    stopTimer();

    // Calculate statistics
    const stats = calculateStats();

    // Save final results to database (only for non-practice)
    if (!props.isPractice && sessionId.value) {
        try {
            isSaving.value = true;
            await invoke('complete_kraepelin_session', {
                sessionId: sessionId.value,
                totalCorrect: stats.totalCorrect,
                totalAnswered: stats.totalAnswered,
                accuracy: stats.overallAccuracy
            });
            announceToScreenReader('Test completed successfully');
            saveError.value = null;
        } catch (error) {
            console.error('Failed to complete session:', error);
            saveError.value = 'Failed to save final results';
        } finally {
            isSaving.value = false;
        }
    }

    // Convert answers to number[][] by replacing nulls with -1 for type safety
    const safeAnswers = answers.value.map(col => col.map(r => r ?? -1));

    emit('complete', {
        answers: safeAnswers,
        times: columnTimes.value,
        stats
    });
}

// Input handling
function handleKeyDown(e: KeyboardEvent) {
    if (testPhase.value !== 'running') return;

    // Number keys
    if (e.key >= '0' && e.key <= '9') {
        e.preventDefault();
        submitAnswer(parseInt(e.key));
    }
    // Enter to move to next column
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
    } else {
        // Auto move to next column when finished
        nextColumn();
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
    if (answer === null) return null;

    return answer === getCorrectAnswer(colIndex, rowIndex);
}

// Calculate statistics
function calculateStats() {
    let totalAnswered = 0;
    let totalCorrect = 0;

    answers.value.forEach((col, colIdx) => {
        col.forEach((answer, rowIdx) => {
            if (answer !== null) {
                totalAnswered++;
                if (isCorrect(colIdx, rowIdx)) {
                    totalCorrect++;
                }
            }
        });
    });

    return {
        totalAnswered,
        totalCorrect,
        overallAccuracy: totalAnswered > 0 ? Math.round((totalCorrect / totalAnswered) * 100) : 0
    };
}

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
    <div class="min-h-screen bg-white text-gray-900">
        <!-- Screen Reader Announcements -->
        <div aria-live="polite" aria-atomic="true" class="sr-only">
            {{ screenReaderMessage }}
        </div>

        <!-- Saving Status Indicator -->
        <div v-if="isSaving"
            class="fixed top-4 right-4 z-50 bg-blue-500 text-white px-4 py-2 rounded-lg shadow-lg flex items-center gap-2">
            <svg class="animate-spin h-5 w-5" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                <path class="opacity-75" fill="currentColor"
                    d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z">
                </path>
            </svg>
            <span>Menyimpan...</span>
        </div>

        <!-- Error Indicator -->
        <div v-if="saveError" class="fixed top-4 right-4 z-50 bg-red-500 text-white px-4 py-2 rounded-lg shadow-lg">
            {{ saveError }}
        </div>

        <!-- READY STATE -->
        <div v-if="testPhase === 'ready'" class="min-h-screen flex items-center justify-center p-4">
            <div class="max-w-lg w-full text-center">
                <div class="bg-white border-2 border-gray-300 rounded-lg p-8 shadow-lg">
                    <h2 class="text-2xl font-bold mb-2">Tes Kraepelin</h2>
                    <p class="text-gray-600 mb-6">{{ isPractice ? 'Mode Latihan' : 'Tes Utama' }}</p>

                    <!-- Instructions -->
                    <div class="bg-gray-50 border border-gray-200 rounded-lg p-4 text-left mb-6">
                        <h4 class="text-sm font-bold text-gray-700 mb-3">📋 Petunjuk:</h4>
                        <ul class="text-sm text-gray-600 space-y-2">
                            <li>• Jumlahkan 2 angka yang bersebelahan (atas-bawah)</li>
                            <li>• Tuliskan <strong>angka satuan</strong> dari hasil penjumlahan</li>
                            <li>• Contoh: 7 + 8 = 15 → tulis <strong class="text-blue-600">5</strong></li>
                            <li>• Kerjakan secepat dan seteliti mungkin</li>
                            <li>• Waktu: {{ config.timePerColumn }} detik per kolom</li>
                        </ul>
                    </div>

                    <button @click="startTest"
                        class="w-full py-4 rounded-lg bg-blue-600 text-white font-bold text-lg hover:bg-blue-700 transition-all shadow-md">
                        🚀 {{ isPractice ? 'Mulai Latihan' : 'Mulai Tes' }}
                    </button>
                </div>
            </div>
        </div>

        <!-- RUNNING STATE - Paper-like Layout -->
        <div v-else-if="testPhase === 'running'" class="min-h-screen flex flex-col bg-gray-50">
            <!-- Top Bar -->
            <div class="bg-white border-b-2 border-gray-300 px-6 py-3 shadow-sm">
                <div class="max-w-7xl mx-auto flex items-center justify-between">
                    <div class="flex items-center gap-6">
                        <div class="text-sm font-mono">
                            <span class="text-gray-500">Kolom:</span>
                            <span class="text-blue-600 font-bold ml-1">{{ currentColumn + 1 }}/{{ isPractice ?
                                config.practiceColumns : config.columns }}</span>
                        </div>
                        <div class="h-4 w-px bg-gray-300"></div>
                        <div class="text-sm font-mono">
                            <span class="text-gray-500">Baris:</span>
                            <span class="text-gray-900 font-bold ml-1">{{ currentRow + 1 }}/{{ config.rows - 1 }}</span>
                        </div>
                    </div>

                    <!-- Timer -->
                    <div class="flex items-center gap-2 px-4 py-2 rounded-lg font-mono text-lg font-bold"
                        :class="timeRemaining <= 10 ? 'bg-red-100 text-red-600 animate-pulse' : 'bg-blue-100 text-blue-600'">
                        ⏱ {{ formattedTime }}
                    </div>
                </div>
            </div>

            <!-- Main Content - Paper Layout -->
            <div class="flex-1 overflow-auto p-6">
                <div class="max-w-7xl mx-auto">
                    <!-- Paper Sheet -->
                    <div class="bg-white border-2 border-gray-400 shadow-2xl p-8 rounded-sm">
                        <!-- Columns Grid -->
                        <div class="flex gap-4 overflow-x-auto pb-4">
                            <div v-for="(col, colIdx) in columns" :key="colIdx" class="flex-shrink-0">
                                <!-- Column -->
                                <div class="flex flex-col gap-0.5">
                                    <!-- Column Number -->
                                    <div class="text-center mb-1">
                                        <span class="text-xs font-mono text-gray-400">{{ colIdx + 1 }}</span>
                                    </div>

                                    <!-- Numbers and Answer Slots -->
                                    <div v-for="(num, rowIdx) in col" :key="rowIdx">
                                        <!-- Number -->
                                        <div class="w-8 h-6 flex items-center justify-center text-sm font-mono border border-gray-300"
                                            :class="{
                                                'bg-blue-100 border-blue-400 font-bold': colIdx === currentColumn && rowIdx === currentRow,
                                                'bg-blue-50': colIdx === currentColumn && rowIdx === currentRow + 1,
                                                'bg-gray-50': colIdx < currentColumn
                                            }">
                                            {{ num }}
                                        </div>

                                        <!-- Answer Slot (between numbers) -->
                                        <div v-if="rowIdx < col.length - 1"
                                            class="w-8 h-5 flex items-center justify-center text-xs font-mono border-x border-gray-200"
                                            :class="{
                                                'bg-yellow-100 border-yellow-400 animate-pulse': colIdx === currentColumn && rowIdx === currentRow,
                                                'bg-green-50 text-green-700': answers[colIdx][rowIdx] !== null && isCorrect(colIdx, rowIdx),
                                                'bg-red-50 text-red-700': answers[colIdx][rowIdx] !== null && !isCorrect(colIdx, rowIdx),
                                                'bg-white': answers[colIdx][rowIdx] === null
                                            }">
                                            {{ answers[colIdx][rowIdx] ?? '' }}
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>

                    <!-- Input Helper -->
                    <div class="mt-6 bg-white border-2 border-gray-300 rounded-lg p-4 max-w-md mx-auto">
                        <div class="text-center">
                            <div class="text-sm text-gray-500 mb-2">Soal Saat Ini:</div>
                            <div class="text-3xl font-bold text-gray-900 mb-2">
                                {{ columns[currentColumn]?.[currentRow] }} + {{ columns[currentColumn]?.[currentRow + 1]
                                }}
                            </div>
                            <div class="text-sm text-gray-500 mb-4">
                                = {{ (columns[currentColumn]?.[currentRow] || 0) + (columns[currentColumn]?.[currentRow
                                    + 1] || 0) }}
                                → ketik <span class="text-blue-600 font-bold">{{ ((columns[currentColumn]?.[currentRow]
                                    || 0) + (columns[currentColumn]?.[currentRow + 1] || 0)) % 10 }}</span>
                            </div>

                            <!-- Hidden input for keyboard capture -->
                            <input ref="inputRef" @keydown="handleKeyDown" type="text" inputmode="numeric"
                                class="w-full px-4 py-3 text-center text-2xl font-mono border-2 border-blue-400 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                                placeholder="Ketik angka 0-9" autocomplete="off">
                        </div>
                    </div>
                </div>
            </div>
        </div>

        <!-- COMPLETE STATE -->
        <div v-else-if="testPhase === 'complete'" class="min-h-screen flex items-center justify-center p-4 bg-gray-50">
            <div class="max-w-2xl w-full">
                <div class="bg-white border-2 border-gray-300 rounded-lg p-8 text-center shadow-lg">
                    <div class="w-20 h-20 rounded-full bg-green-100 flex items-center justify-center mx-auto mb-6">
                        <svg class="w-10 h-10 text-green-600" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                        </svg>
                    </div>

                    <h2 class="text-2xl font-bold text-gray-900 mb-2">Tes Selesai! 🎉</h2>
                    <p class="text-gray-600 mb-8">{{ isPractice ? 'Latihan selesai' : 'Hasil tes telah direkam' }}</p>

                    <!-- Results -->
                    <div class="grid grid-cols-3 gap-4 mb-8">
                        <div class="bg-gray-50 border border-gray-200 rounded-lg p-4">
                            <div class="text-2xl font-bold text-blue-600">{{ calculateStats().totalAnswered }}</div>
                            <div class="text-xs text-gray-500">Total Jawaban</div>
                        </div>
                        <div class="bg-gray-50 border border-gray-200 rounded-lg p-4">
                            <div class="text-2xl font-bold text-green-600">{{ calculateStats().totalCorrect }}</div>
                            <div class="text-xs text-gray-500">Jawaban Benar</div>
                        </div>
                        <div class="bg-gray-50 border border-gray-200 rounded-lg p-4">
                            <div class="text-2xl font-bold text-purple-600">{{ calculateStats().overallAccuracy }}%
                            </div>
                            <div class="text-xs text-gray-500">Akurasi</div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped>
/* Ensure paper-like appearance */
input:focus {
    outline: none;
}

/* Screen reader only - visually hidden but accessible */
.sr-only {
    position: absolute;
    width: 1px;
    height: 1px;
    padding: 0;
    margin: -1px;
    overflow: hidden;
    clip: rect(0, 0, 0, 0);
    white-space: nowrap;
    border-width: 0;
}
</style>
