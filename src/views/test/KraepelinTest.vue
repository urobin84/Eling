<script setup lang="ts">
import { ref } from 'vue';
import SpeedEnginePaper from '../../components/engines/SpeedEnginePaper.vue';

const testPhase = ref<'intro' | 'practice' | 'testing' | 'complete'>('intro');
const practiceComplete = ref(false);
const testResults = ref<any>(null);

function startPractice() {
    testPhase.value = 'practice';
}

function startTest() {
    testPhase.value = 'testing';
}

function handlePracticeComplete(data: any) {
    console.log('Practice complete:', data);
    practiceComplete.value = true;
    testPhase.value = 'intro';
}

function handleTestComplete(data: any) {
    console.log('Test complete:', data);
    testResults.value = data;
    testPhase.value = 'complete';
}
</script>

<template>
    <div class="min-h-screen bg-gradient-to-br from-[#0a0f1a] to-[#1a1f2e]">

        <!-- INTRO -->
        <div v-if="testPhase === 'intro'" class="min-h-screen flex items-center justify-center p-4">
            <div class="max-w-2xl w-full">
                <div class="bg-white/5 backdrop-blur-xl rounded-2xl border border-white/10 p-8 text-center">
                    <!-- Header -->
                    <div
                        class="w-24 h-24 rounded-2xl bg-gradient-to-br from-orange-400 to-red-600 flex items-center justify-center mx-auto mb-6 shadow-lg shadow-orange-500/20">
                        <svg class="w-12 h-12 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                d="M9 7h6m0 10v-3m-3 3h.01M9 17h.01M9 14h.01M12 14h.01M15 11h.01M12 11h.01M9 11h.01M7 21h10a2 2 0 002-2V5a2 2 0 00-2-2H7a2 2 0 00-2 2v14a2 2 0 002 2z" />
                        </svg>
                    </div>

                    <h1 class="text-3xl font-bold text-white mb-2">Tes Kraepelin</h1>
                    <p class="text-white/60 mb-8">Tes Kecepatan dan Ketelitian Kerja</p>

                    <!-- Info Cards -->
                    <div class="grid grid-cols-3 gap-4 mb-8">
                        <div class="bg-white/5 rounded-xl p-4">
                            <div class="text-2xl font-bold text-cyan-400">45</div>
                            <div class="text-xs text-white/50">Kolom</div>
                        </div>
                        <div class="bg-white/5 rounded-xl p-4">
                            <div class="text-2xl font-bold text-orange-400">60</div>
                            <div class="text-xs text-white/50">Detik/Kolom</div>
                        </div>
                        <div class="bg-white/5 rounded-xl p-4">
                            <div class="text-2xl font-bold text-purple-400">~45</div>
                            <div class="text-xs text-white/50">Menit Total</div>
                        </div>
                    </div>

                    <!-- Instructions -->
                    <div class="bg-white/5 rounded-xl p-6 text-left mb-8">
                        <h3 class="text-sm font-bold text-orange-400 mb-4">📋 PETUNJUK PENGERJAAN</h3>
                        <div class="space-y-4 text-sm text-white/80">
                            <div class="flex gap-3">
                                <span
                                    class="w-6 h-6 rounded bg-orange-500/20 flex items-center justify-center text-xs font-bold text-orange-400">1</span>
                                <p>Anda akan melihat deretan angka yang tersusun <strong
                                        class="text-white">vertikal</strong> (dari atas ke bawah).</p>
                            </div>
                            <div class="flex gap-3">
                                <span
                                    class="w-6 h-6 rounded bg-orange-500/20 flex items-center justify-center text-xs font-bold text-orange-400">2</span>
                                <p>Jumlahkan <strong class="text-white">2 angka yang bersebelahan</strong> (atas-bawah).
                                </p>
                            </div>
                            <div class="flex gap-3">
                                <span
                                    class="w-6 h-6 rounded bg-orange-500/20 flex items-center justify-center text-xs font-bold text-orange-400">3</span>
                                <p>Tuliskan hanya <strong class="text-cyan-400">angka satuan</strong> dari hasil
                                    penjumlahan.</p>
                            </div>
                            <div class="flex gap-3">
                                <span
                                    class="w-6 h-6 rounded bg-orange-500/20 flex items-center justify-center text-xs font-bold text-orange-400">4</span>
                                <p>Kerjakan secepat dan seteliti mungkin!</p>
                            </div>
                        </div>

                        <!-- Example -->
                        <div class="mt-6 p-4 bg-orange-500/10 rounded-xl border border-orange-500/20">
                            <div class="text-xs text-orange-400 font-bold mb-2">CONTOH:</div>
                            <div class="flex items-center justify-center gap-6">
                                <div class="text-center">
                                    <div class="text-2xl font-bold text-white">7</div>
                                    <div class="text-lg text-orange-400">+</div>
                                    <div class="text-2xl font-bold text-white">8</div>
                                </div>
                                <div class="text-2xl text-white/50">=</div>
                                <div class="text-center">
                                    <div class="text-3xl font-bold text-cyan-400">5</div>
                                    <div class="text-xs text-white/50">(7+8=15, tulis 5)</div>
                                </div>
                            </div>
                        </div>
                    </div>

                    <!-- Buttons -->
                    <div class="space-y-3">
                        <button v-if="!practiceComplete" @click="startPractice"
                            class="w-full py-4 rounded-xl bg-white/10 text-white font-bold hover:bg-white/20 transition-all border border-white/10">
                            📝 Latihan Dulu (3 Kolom)
                        </button>
                        <button v-else @click="startPractice"
                            class="w-full py-3 rounded-xl bg-white/5 text-white/70 hover:bg-white/10 transition-all text-sm">
                            ↺ Ulangi Latihan
                        </button>

                        <button @click="startTest"
                            class="w-full py-4 rounded-xl bg-gradient-to-r from-orange-500 to-red-600 text-white font-bold text-lg hover:from-orange-400 hover:to-red-500 transition-all shadow-lg shadow-orange-500/20"
                            :class="{ 'opacity-50 cursor-not-allowed': !practiceComplete }">
                            🚀 Mulai Tes Kraepelin
                        </button>

                        <p v-if="!practiceComplete" class="text-xs text-white/40 text-center">
                            * Disarankan latihan terlebih dahulu
                        </p>
                    </div>
                </div>
            </div>
        </div>

        <!-- PRACTICE MODE -->
        <SpeedEnginePaper v-else-if="testPhase === 'practice'" :isPractice="true"
            :config="{ columns: 45, rows: 40, timePerColumn: 60, practiceColumns: 3 }"
            @complete="handlePracticeComplete" />

        <!-- TESTING MODE -->
        <SpeedEnginePaper v-else-if="testPhase === 'testing'" :isPractice="false"
            :config="{ columns: 45, rows: 40, timePerColumn: 60 }" @complete="handleTestComplete" />

        <!-- COMPLETE -->
        <div v-else-if="testPhase === 'complete'" class="min-h-screen flex items-center justify-center p-4">
            <div class="max-w-3xl w-full">
                <div class="bg-white/5 backdrop-blur-xl rounded-2xl border border-white/10 p-8 text-center">
                    <div
                        class="w-20 h-20 rounded-full bg-gradient-to-br from-green-400 to-emerald-600 flex items-center justify-center mx-auto mb-6 shadow-lg">
                        <svg class="w-10 h-10 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                        </svg>
                    </div>

                    <h2 class="text-2xl font-bold text-white mb-2">Tes Kraepelin Selesai! 🎉</h2>
                    <p class="text-white/60 mb-8">Hasil tes Anda telah direkam</p>

                    <!-- Results -->
                    <div v-if="testResults" class="grid grid-cols-4 gap-4 mb-8">
                        <div class="bg-white/5 rounded-xl p-4">
                            <div class="text-2xl font-bold text-cyan-400">{{ testResults.stats.totalAnswered }}</div>
                            <div class="text-xs text-white/50">Total Jawaban</div>
                        </div>
                        <div class="bg-white/5 rounded-xl p-4">
                            <div class="text-2xl font-bold text-green-400">{{ testResults.stats.totalCorrect }}</div>
                            <div class="text-xs text-white/50">Jawaban Benar</div>
                        </div>
                        <div class="bg-white/5 rounded-xl p-4">
                            <div class="text-2xl font-bold text-purple-400">{{ testResults.stats.overallAccuracy }}%
                            </div>
                            <div class="text-xs text-white/50">Akurasi</div>
                        </div>
                        <div class="bg-white/5 rounded-xl p-4">
                            <div class="text-2xl font-bold text-orange-400">{{ testResults.times.length }}</div>
                            <div class="text-xs text-white/50">Kolom Selesai</div>
                        </div>
                    </div>

                    <div class="bg-blue-500/10 rounded-xl p-4 border border-blue-500/20 mb-6">
                        <h4 class="text-sm font-bold text-blue-300 mb-2">📊 Aspek yang Diukur</h4>
                        <div class="grid grid-cols-2 gap-2 text-xs text-blue-200/70">
                            <div>• Kecepatan Kerja</div>
                            <div>• Ketelitian</div>
                            <div>• Daya Tahan Konsentrasi</div>
                            <div>• Konsistensi Performa</div>
                        </div>
                    </div>

                    <button @click="testPhase = 'intro'"
                        class="px-8 py-3 rounded-xl bg-white/10 text-white font-medium hover:bg-white/20 transition-all">
                        Kembali ke Beranda
                    </button>
                </div>
            </div>
        </div>
    </div>
</template>
