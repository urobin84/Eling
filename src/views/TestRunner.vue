<script setup lang="ts">
import { ref, onMounted, computed, onUnmounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { invoke } from '@tauri-apps/api/core';
import { useRecording } from '../composables/useRecording';

const route = useRoute();
const router = useRouter();
const { isRecording, startRecording, stopRecording } = useRecording();

// Test flow states
type TestPhase = 'welcome' | 'instructions' | 'subtest-intro' | 'testing' | 'subtest-complete' | 'completed';
const testPhase = ref<TestPhase>('welcome');

// Current indices
const currentSubtestIndex = ref(0);
const currentQuestionIndex = ref(0);

// Timer
const timeRemaining = ref(0);
const timerInterval = ref<any>(null);
const timerWarning = ref(false);

// Answers storage
const answers = ref<Record<number, string>>({});
const answeredQuestions = ref<Set<number>>(new Set());

// Event ID from route
const eventId = ref<number | null>(null);
const isLoadingTest = ref(true);

// Test data structure (will be loaded from backend)
interface TestTool {
  id: number;
  name: string;
  description: string;
  category: string;
}

interface TestData {
  tool: TestTool;
  subtests: any[];
}

const testData = ref<TestData | null>(null);

// Load test data from event packages
// Load test data from event packages
async function loadEventPackages() {
  try {
    isLoadingTest.value = true;
    console.log('TestRunner: Fetching packages for Event ID:', eventId.value);
    
    if (!eventId.value || isNaN(eventId.value)) {
        console.error('TestRunner: Invalid Event ID:', eventId.value);
        alert('Invalid Event ID');
        router.push('/dashboard');
        return;
    }

    const packages = await invoke<Array<[number, string, string]>>('get_event_packages', {
      eventId: eventId.value
    });

    console.log('Loaded packages:', packages);

    if (!packages || packages.length === 0) {
      console.error('No packages found for event');
      alert('No assessment tools found for this event');
      router.push('/dashboard');
      return;
    }

    // For now, use first package as the tool
    const [toolId, , toolCategory] = packages[0];
    
    // Fetch full structure for the tool
    const toolStructure = await invoke<any>('get_tool_structure', { toolId });
    console.log('Loaded tool structure:', toolStructure);

    // Map backend structure to frontend interface
    testData.value = {
      tool: {
        id: toolStructure.tool.id,
        name: toolStructure.tool.name,
        description: toolStructure.tool.description || `${toolCategory} assessment tool`,
        category: toolStructure.tool.category || toolCategory
      },
      subtests: toolStructure.subtests.map((s: any) => ({
        id: s.subtest.id,
        name: s.subtest.subtest_name,
        description: `Subtest ${s.subtest.sequence_order} - ${s.subtest.subtest_name}`,
        instructions: parseInstructions(s.subtest.instructions),
        time_limit: s.subtest.time_limit_seconds || 300,
        questions: s.questions.map((q: any) => ({
          id: q.id,
          text: q.question_text,
          options: parseOptions(q.options)
        }))
      }))
    };

  } catch (e) {
    console.error('Failed to load event packages:', e);
    alert('Failed to load assessment. Please try again.');
    router.push('/candidate/dashboard');
  } finally {
    isLoadingTest.value = false;
  }
}

// Helper to parse instructions
function parseInstructions(instr: any): string[] {
    if (!instr) return ['Ikuti petunjuk dengan teliti'];
    try {
        if (Array.isArray(instr)) {
            return instr.map(i => typeof i === 'string' ? i : JSON.stringify(i));
        }
        if (typeof instr === 'string') {
            // Check if it looks like JSON
            if (instr.trim().startsWith('[')) {
                return JSON.parse(instr);
            }
            return [instr];
        }
        if (typeof instr === 'object') {
            // If it's an object, try to extract values or stringify
            // Common case: { "1": "instr 1", "2": "instr 2" } or similar
            const values = Object.values(instr);
            if (values.length > 0 && values.every(v => typeof v === 'string')) {
                return values as string[];
            }
            return [JSON.stringify(instr)];
        }
        return [String(instr)];
    } catch (e) {
        return ['Instruction parse error'];
    }
}

// Helper to parse options
function parseOptions(opts: any): string[] {
    if (!opts) return [];
    try {
        if (Array.isArray(opts)) return opts.map(o => String(o));
        if (typeof opts === 'string') {
             const parsed = JSON.parse(opts);
             return Array.isArray(parsed) ? parsed : [];
        }
        if (typeof opts === 'object') {
            return Object.values(opts).map(o => String(o));
        }
        return [];
    } catch (e) {
        console.error('Failed to parse options:', opts);
        return [];
    }
}

// Computed
const currentSubtest = computed(() => testData.value?.subtests[currentSubtestIndex.value]);
const currentQuestion = computed(() => currentSubtest.value?.questions[currentQuestionIndex.value]);
const totalQuestionsInSubtest = computed(() => currentSubtest.value?.questions.length || 0);
const progressPercent = computed(() => {
  if (totalQuestionsInSubtest.value === 0) return 0;
  return Math.round(((currentQuestionIndex.value + 1) / totalQuestionsInSubtest.value) * 100);
});

const formattedTime = computed(() => {
  const mins = Math.floor(timeRemaining.value / 60);
  const secs = timeRemaining.value % 60;
  return `${mins.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
});

// Timer functions
function startTimer() {
  if (timerInterval.value) clearInterval(timerInterval.value);
  timeRemaining.value = currentSubtest.value?.time_limit || 300;
  timerWarning.value = false;

  timerInterval.value = setInterval(() => {
    timeRemaining.value--;

    // Warning when 1 minute left
    if (timeRemaining.value === 60) {
      timerWarning.value = true;
    }

    // Auto-submit when time runs out
    if (timeRemaining.value <= 0) {
      clearInterval(timerInterval.value);
      finishSubtest();
    }
  }, 1000);
}

function stopTimer() {
  if (timerInterval.value) {
    clearInterval(timerInterval.value);
    timerInterval.value = null;
  }
}

// Session ID for recording
const sessionId = ref(`session-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`);

// Navigation functions
const dbSessionId = ref<number | null>(null);

// Navigation functions
async function startTest() {
  testPhase.value = 'instructions';

  // 1. Start recording expectation IMMEDIATELY to capture user gesture
  // We don't await it yet to ensure the browser sees this as a direct response to the click
  const recordingPromise = startRecording({
    sessionId: sessionId.value, // Keep using string ID for file storage
    userId: 1, 
    username: 'Test User',
    testName: testData.value?.tool.name || 'Unknown',
    eventName: undefined
  });
  
  // 2. Create DB Session for Report (Background)
  try {
      if (eventId.value) { 
          // Parse user session
          const sessionStr = localStorage.getItem('user_session');
          let participantId = 'guest';
          if (sessionStr) {
             const s = JSON.parse(sessionStr);
             participantId = s.username;
          }

          dbSessionId.value = await invoke('create_session', {
             eventId: eventId.value,
             participantId: participantId,
             metadata: {
                 userAgent: navigator.userAgent,
                 testName: testData.value?.tool.name,
                 recordingId: sessionId.value
             }
          });
          console.log('Created DB Session:', dbSessionId.value);
      }
  } catch (e) {
      console.error('Failed to create DB session:', e);
  }

  // 3. Resolve recording
  const recordingStarted = await recordingPromise;
  
  if (!recordingStarted) {
    console.warn('Failed to start recording, but continuing with test');
  }
}

async function finishTest() {

    // 1. Stop Recording Explicitly
    if (isRecording.value) {
        // Parse user session for correct metadata
        const sessionStr = localStorage.getItem('user_session');
        let participantId = 'guest';
        let userId = 1;
        if (sessionStr) {
           const s = JSON.parse(sessionStr);
           participantId = s.username;
           userId = s.id;
        }

        await stopRecording({
            sessionId: sessionId.value,
            userId: userId,
            username: participantId,
            testName: testData.value?.tool.name || 'Unknown',
        });
    }

    // 2. Calculate Final Scores
    const result = calculateScore();
    const scores = {
        total_score: result.answered, // simplified
        raw_score: result.answered,
        max_score: result.total,
        details: answers.value
    };

    // 3. Submit to Backend
    if (dbSessionId.value) {
        try {
            await invoke('submit_test_results', {
                sessionId: dbSessionId.value,
                scores: scores,
                interpretations: null
            });
            console.log('Results submitted successfully');
        } catch (e) {
            console.error('Failed to submit results:', e);
            alert('Failed to save results. Please contact admin.');
        }
    } else {
        console.warn('No DB Session ID, results not saved to DB');
    }

    testPhase.value = 'completed';
}

function selectAnswer(questionId: number, answer: string) {
  answers.value[questionId] = answer;
  answeredQuestions.value.add(questionId);
  // Optional: Auto-advance if single choice? No, let user confirm.
}

function nextQuestion() {
  if (currentQuestionIndex.value < totalQuestionsInSubtest.value - 1) {
    currentQuestionIndex.value++;
  } else {
    finishSubtest();
  }
}

function prevQuestion() {
  if (currentQuestionIndex.value > 0) {
    currentQuestionIndex.value--;
  }
}

function goToQuestion(index: number) {
  if (index >= 0 && index < totalQuestionsInSubtest.value) {
    currentQuestionIndex.value = index;
  }
}

function finishSubtest() {
    testPhase.value = 'subtest-complete';
    stopTimer();
}

function nextSubtest() {
  if (testData.value && currentSubtestIndex.value < testData.value.subtests.length - 1) {
    currentSubtestIndex.value++;
    currentQuestionIndex.value = 0;
    testPhase.value = 'subtest-intro';
  } else {
    finishTest(); 
  }
}

function continueToInstructions() {
  testPhase.value = 'subtest-intro';
}

function startSubtest() {
  testPhase.value = 'testing';
  startTimer();
}

async function resetTest() {
  // Stop recording if active
  if (isRecording.value) {
    await stopRecording();
  }
  
  testPhase.value = 'welcome';
  currentSubtestIndex.value = 0;
  currentQuestionIndex.value = 0;
  answers.value = {};
  answeredQuestions.value.clear();
  stopTimer();
}

function backToDashboard() {
  router.push('/dashboard');
}

// Calculate score
function calculateScore() {
  if (!testData.value) return { answered: 0, total: 0 };
  return {
    answered: answeredQuestions.value.size,
    total: testData.value.subtests.reduce((acc, s) => acc + s.questions.length, 0)
  };
}

// Lifecycle
onMounted(() => {
  eventId.value = Number(route.query.eventId);
  
  if (!eventId.value || isNaN(eventId.value)) {
    console.error('No valid event ID provided');
    alert('Invalid event. Please select an event from dashboard.');
    router.push('/candidate/dashboard');
    return;
  }
  
  loadEventPackages();
});

onUnmounted(async () => {
  stopTimer();
  // Stop recording on unmount
  if (isRecording.value) {
    await stopRecording();
  }
});
</script>

<template>
  <div class="min-h-screen bg-gradient-to-br from-[#0a0f1a] to-[#1a1f2e]">
    
    <!-- LOADING SCREEN -->
    <div v-if="isLoadingTest || !testData" class="min-h-screen flex items-center justify-center p-4">
      <div class="text-center">
        <div class="inline-block animate-spin rounded-full h-16 w-16 border-4 border-cyan-500 border-t-transparent mb-4"></div>
        <div class="text-white text-xl font-semibold">Loading assessment...</div>
        <div class="text-white/60 text-sm mt-2">Please wait</div>
      </div>
    </div>

    <!-- WELCOME SCREEN -->
    <div v-else-if="testPhase === 'welcome'" class="min-h-screen flex items-center justify-center p-4">
      <div class="max-w-2xl w-full">
        <!-- Header -->
        <div class="text-center mb-8">
          <div
            class="w-20 h-20 rounded-2xl bg-gradient-to-br from-cyan-400 to-blue-600 flex items-center justify-center mx-auto mb-6 shadow-lg shadow-cyan-500/20">
            <svg class="w-10 h-10 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                d="M9.663 17h4.673M12 3v1m6.364 1.636l-.707.707M21 12h-1M4 12H3m3.343-5.657l-.707-.707m2.828 9.9a5 5 0 117.072 0l-.548.547A3.374 3.374 0 0014 18.469V19a2 2 0 11-4 0v-.531c0-.895-.356-1.754-.988-2.386l-.548-.547z" />
            </svg>
          </div>
          <h1 class="text-3xl font-bold text-white mb-2">Eling Psychotest</h1>
          <p class="text-white/60">Platform Tes Psikologi Profesional</p>
        </div>

        <!-- Test Info Card -->
        <div class="bg-white/5 backdrop-blur-xl rounded-2xl border border-white/10 p-8 mb-6">
          <div class="flex items-start gap-4 mb-6">
            <div class="w-16 h-16 rounded-xl bg-cyan-500/20 flex items-center justify-center shrink-0">
              <span class="text-2xl font-bold text-cyan-400">{{ testData.tool.name.charAt(0) }}</span>
            </div>
            <div>
              <h2 class="text-xl font-bold text-white">{{ testData.tool.name }}</h2>
              <p class="text-sm text-white/60 mt-1">{{ testData.tool.description }}</p>
            </div>
          </div>

          <!-- Test Stats -->
          <div class="grid grid-cols-3 gap-4 mb-6">
            <div class="bg-white/5 rounded-xl p-4 text-center">
              <div class="text-2xl font-bold text-blue-400">{{ testData.subtests.length }}</div>
              <div class="text-xs text-white/50">Subtest</div>
            </div>
            <div class="bg-white/5 rounded-xl p-4 text-center">
              <div class="text-2xl font-bold text-purple-400">{{testData.subtests.reduce((a, s) => a +
                s.questions.length, 0) }}</div>
              <div class="text-xs text-white/50">Soal</div>
            </div>
            <div class="bg-white/5 rounded-xl p-4 text-center">
              <div class="text-2xl font-bold text-green-400">{{Math.ceil(testData.subtests.reduce((a, s) => a +
                s.time_limit, 0) / 60) }}</div>
              <div class="text-xs text-white/50">Menit</div>
            </div>
          </div>

          <!-- Rules -->
          <div class="bg-orange-500/10 rounded-xl p-4 border border-orange-500/20 mb-6">
            <h4 class="text-sm font-bold text-orange-300 mb-2">⚠️ Peraturan Umum</h4>
            <ul class="text-xs text-orange-200/80 space-y-1">
              <li>• Pastikan koneksi internet stabil</li>
              <li>• Jangan refresh atau menutup halaman selama tes</li>
              <li>• Setiap subtest memiliki batas waktu</li>
              <li>• Jawab semua pertanyaan sebelum waktu habis</li>
            </ul>
          </div>

          <button @click="startTest"
            class="w-full py-4 rounded-xl bg-gradient-to-r from-cyan-500 to-blue-600 text-white font-bold text-lg hover:from-cyan-400 hover:to-blue-500 transition-all shadow-lg shadow-cyan-500/20">
            🚀 Mulai Tes
          </button>
        </div>
      </div>
    </div>

    <!-- GENERAL INSTRUCTIONS -->
    <div v-else-if="testPhase === 'instructions'" class="min-h-screen flex items-center justify-center p-4">
      <div class="max-w-2xl w-full">
        <div class="bg-white/5 backdrop-blur-xl rounded-2xl border border-white/10 p-8">
          <div class="text-center mb-6">
            <div class="w-16 h-16 rounded-full bg-blue-500/20 flex items-center justify-center mx-auto mb-4">
              <svg class="w-8 h-8 text-blue-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                  d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
            </div>
            <h2 class="text-2xl font-bold text-white">Petunjuk Umum</h2>
            <p class="text-white/60 mt-2">Baca dengan seksama sebelum memulai</p>
          </div>

          <div class="space-y-4 mb-8">
            <div class="flex items-start gap-4 p-4 bg-white/5 rounded-xl">
              <div class="w-8 h-8 rounded-lg bg-cyan-500/20 flex items-center justify-center shrink-0">
                <span class="text-cyan-400 font-bold">1</span>
              </div>
              <div>
                <h4 class="text-white font-medium">Baca Instruksi</h4>
                <p class="text-sm text-white/60">Setiap subtest memiliki instruksi khusus yang perlu dipahami</p>
              </div>
            </div>

            <div class="flex items-start gap-4 p-4 bg-white/5 rounded-xl">
              <div class="w-8 h-8 rounded-lg bg-cyan-500/20 flex items-center justify-center shrink-0">
                <span class="text-cyan-400 font-bold">2</span>
              </div>
              <div>
                <h4 class="text-white font-medium">Perhatikan Waktu</h4>
                <p class="text-sm text-white/60">Timer akan berjalan otomatis, jawab secepat dan setepat mungkin</p>
              </div>
            </div>

            <div class="flex items-start gap-4 p-4 bg-white/5 rounded-xl">
              <div class="w-8 h-8 rounded-lg bg-cyan-500/20 flex items-center justify-center shrink-0">
                <span class="text-cyan-400 font-bold">3</span>
              </div>
              <div>
                <h4 class="text-white font-medium">Jangan Kosongkan Jawaban</h4>
                <p class="text-sm text-white/60">Jika ragu, pilih jawaban yang menurut Anda paling tepat</p>
              </div>
            </div>

            <div class="flex items-start gap-4 p-4 bg-white/5 rounded-xl">
              <div class="w-8 h-8 rounded-lg bg-cyan-500/20 flex items-center justify-center shrink-0">
                <span class="text-cyan-400 font-bold">4</span>
              </div>
              <div>
                <h4 class="text-white font-medium">Fokus dan Konsentrasi</h4>
                <p class="text-sm text-white/60">Pastikan Anda dalam kondisi tenang dan fokus</p>
              </div>
            </div>
          </div>

          <button @click="continueToInstructions"
            class="w-full py-4 rounded-xl bg-gradient-to-r from-cyan-500 to-blue-600 text-white font-bold hover:from-cyan-400 hover:to-blue-500 transition-all">
            Lanjutkan →
          </button>
        </div>
      </div>
    </div>

    <!-- SUBTEST INTRO -->
    <div v-else-if="testPhase === 'subtest-intro'" class="min-h-screen flex items-center justify-center p-4">
      <div class="max-w-2xl w-full">
        <div class="bg-white/5 backdrop-blur-xl rounded-2xl border border-white/10 p-8">
          <!-- Progress indicator -->
          <div class="flex items-center gap-2 mb-6">
            <div v-for="(s, idx) in testData.subtests" :key="s.id" class="flex-1 h-2 rounded-full transition-all"
              :class="idx < currentSubtestIndex ? 'bg-green-500' : idx === currentSubtestIndex ? 'bg-cyan-500' : 'bg-white/10'">
            </div>
          </div>

          <div class="text-center mb-6">
            <div class="text-sm text-cyan-400 font-medium mb-2">SUBTEST {{ currentSubtestIndex + 1 }} dari {{
              testData.subtests.length }}</div>
            <h2 class="text-2xl font-bold text-white">{{ currentSubtest.name }}</h2>
            <p class="text-white/60 mt-2">{{ currentSubtest.description }}</p>
          </div>

          <!-- Stats -->
          <div class="grid grid-cols-2 gap-4 mb-6">
            <div class="bg-white/5 rounded-xl p-4 text-center">
              <div class="text-2xl font-bold text-purple-400">{{ currentSubtest.questions.length }}</div>
              <div class="text-xs text-white/50">Jumlah Soal</div>
            </div>
            <div class="bg-white/5 rounded-xl p-4 text-center">
              <div class="text-2xl font-bold text-orange-400">{{ Math.floor(currentSubtest.time_limit / 60) }}</div>
              <div class="text-xs text-white/50">Menit</div>
            </div>
          </div>

          <!-- Instructions -->
          <div class="bg-blue-500/10 rounded-xl p-4 border border-blue-500/20 mb-6">
            <h4 class="text-sm font-bold text-blue-300 mb-3">📋 Petunjuk Pengerjaan</h4>
            <ul class="space-y-2">
              <li v-for="(instruction, idx) in currentSubtest.instructions" :key="idx"
                class="flex items-start gap-2 text-sm text-blue-200/80">
                <span class="text-blue-400">•</span>
                {{ instruction }}
              </li>
            </ul>
          </div>

          <button @click="startSubtest"
            class="w-full py-4 rounded-xl bg-gradient-to-r from-green-500 to-emerald-600 text-white font-bold hover:from-green-400 hover:to-emerald-500 transition-all">
            ▶ Mulai Subtest
          </button>
        </div>
      </div>
    </div>

    <!-- TESTING PHASE -->
    <div v-else-if="testPhase === 'testing'" class="min-h-screen flex flex-col">
      <!-- Top Bar -->
      <div class="bg-white/5 backdrop-blur-xl border-b border-white/10 px-6 py-4">
        <div class="max-w-4xl mx-auto flex items-center justify-between">
          <div class="flex items-center gap-4">
            <div class="text-sm">
              <span class="text-white/50">Subtest:</span>
              <span class="text-white font-bold ml-1">{{ currentSubtest.name }}</span>
            </div>
            <div class="h-4 w-px bg-white/10"></div>
            <div class="text-sm">
              <span class="text-white/50">Soal:</span>
              <span class="text-cyan-400 font-bold ml-1">{{ currentQuestionIndex + 1 }}/{{ totalQuestionsInSubtest
                }}</span>
            </div>
          </div>

          <!-- Timer -->
          <div class="flex items-center gap-2 px-4 py-2 rounded-xl"
            :class="timerWarning ? 'bg-red-500/20 animate-pulse' : 'bg-orange-500/10'">
            <svg class="w-5 h-5" :class="timerWarning ? 'text-red-400' : 'text-orange-400'" fill="none"
              viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
            <span class="font-mono font-bold text-lg" :class="timerWarning ? 'text-red-400' : 'text-orange-400'">
              {{ formattedTime }}
            </span>
          </div>
        </div>
      </div>

      <!-- Progress Bar -->
      <div class="h-1 bg-white/10">
        <div class="h-full bg-gradient-to-r from-cyan-500 to-blue-500 transition-all duration-300"
          :style="{ width: progressPercent + '%' }"></div>
      </div>

      <!-- Question Area -->
      <div class="flex-1 flex items-center justify-center p-6">
        <div class="max-w-3xl w-full">
          <!-- Question Card -->
          <div class="bg-white/5 backdrop-blur-xl rounded-2xl border border-white/10 p-8 mb-6">
            <div class="flex items-start gap-4 mb-8">
              <div
                class="w-14 h-14 rounded-xl bg-gradient-to-br from-cyan-500 to-blue-600 flex items-center justify-center shrink-0 shadow-lg shadow-cyan-500/20">
                <span class="text-xl font-bold text-white">{{ currentQuestionIndex + 1 }}</span>
              </div>
              <p class="text-xl text-white leading-relaxed flex-1 pt-2">{{ currentQuestion?.text }}</p>
            </div>

            <!-- Options -->
            <div class="space-y-3">
              <button v-for="(opt, optIdx) in currentQuestion?.options" :key="optIdx"
                @click="selectAnswer(currentQuestion!.id, opt)"
                class="w-full text-left p-5 rounded-xl border-2 transition-all flex items-center gap-4"
                :class="answers[currentQuestion!.id] === opt
                  ? 'bg-cyan-500/10 border-cyan-500 text-white'
                  : 'bg-white/5 border-white/10 text-white/70 hover:border-white/30 hover:bg-white/10'">
                <span class="w-10 h-10 rounded-lg flex items-center justify-center text-sm font-bold"
                  :class="answers[currentQuestion!.id] === opt ? 'bg-cyan-500 text-white' : 'bg-white/10 text-white/60'">
                  {{ String.fromCharCode(65 + (optIdx as number)) }}
                </span>
                <span class="flex-1 text-lg">{{ opt }}</span>
                <svg v-if="answers[currentQuestion!.id] === opt" class="w-6 h-6 text-cyan-500" fill="currentColor"
                  viewBox="0 0 20 20">
                  <path fill-rule="evenodd"
                    d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
                    clip-rule="evenodd" />
                </svg>
              </button>
            </div>
          </div>

          <!-- Navigation -->
          <div class="flex justify-between items-center">
            <button @click="prevQuestion" :disabled="currentQuestionIndex === 0"
              class="px-6 py-3 rounded-xl text-sm font-medium transition-all disabled:opacity-30 disabled:cursor-not-allowed bg-white/5 text-white/70 hover:bg-white/10 border border-white/10">
              ← Sebelumnya
            </button>

            <!-- Quick Navigation -->
            <div class="flex gap-1">
              <button v-for="i in totalQuestionsInSubtest" :key="i" @click="goToQuestion(i - 1)"
                class="w-8 h-8 rounded text-xs font-medium transition-all" :class="currentQuestionIndex === i - 1
                  ? 'bg-cyan-500 text-white'
                  : answeredQuestions.has(currentSubtest.questions[i - 1]?.id)
                    ? 'bg-green-500/20 text-green-400 border border-green-500/30'
                    : 'bg-white/5 text-white/50 hover:bg-white/10'">
                {{ i }}
              </button>
            </div>

            <button @click="nextQuestion" class="px-6 py-3 rounded-xl text-sm font-medium transition-all" :class="currentQuestionIndex === totalQuestionsInSubtest - 1
              ? 'bg-gradient-to-r from-green-500 to-emerald-600 text-white hover:from-green-400'
              : 'bg-gradient-to-r from-cyan-500 to-blue-600 text-white hover:from-cyan-400'">
              {{ currentQuestionIndex === totalQuestionsInSubtest - 1 ? 'Selesai ✓' : 'Selanjutnya →' }}
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- SUBTEST COMPLETE -->
    <div v-else-if="testPhase === 'subtest-complete'" class="min-h-screen flex items-center justify-center p-4">
      <div class="max-w-md w-full text-center">
        <div class="bg-white/5 backdrop-blur-xl rounded-2xl border border-white/10 p-8">
          <div class="w-20 h-20 rounded-full bg-green-500/20 flex items-center justify-center mx-auto mb-6">
            <svg class="w-10 h-10 text-green-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
          </div>
          <h2 class="text-2xl font-bold text-white mb-2">Subtest Selesai!</h2>
          <p class="text-white/60 mb-6">{{ currentSubtest.name }} telah selesai dikerjakan</p>

          <div class="bg-white/5 rounded-xl p-4 mb-6">
            <div class="text-3xl font-bold text-cyan-400">
              {{currentSubtest.questions.filter((q: any) => answeredQuestions.has(q.id)).length}}/{{
                currentSubtest.questions.length }}
            </div>
            <div class="text-sm text-white/50">Soal Dijawab</div>
          </div>

          <button @click="nextSubtest"
            class="w-full py-4 rounded-xl bg-gradient-to-r from-cyan-500 to-blue-600 text-white font-bold hover:from-cyan-400 transition-all">
            {{ currentSubtestIndex < testData.subtests.length - 1 ? 'Lanjut ke Subtest Berikutnya →'
              : 'Lihat Hasil Akhir' }} </button>
        </div>
      </div>
    </div>

    <!-- COMPLETED -->
    <div v-else-if="testPhase === 'completed'" class="min-h-screen flex items-center justify-center p-4">
      <div class="max-w-2xl w-full">
        <div class="bg-white/5 backdrop-blur-xl rounded-2xl border border-white/10 p-8 text-center">
          <div
            class="w-24 h-24 rounded-full bg-gradient-to-br from-green-500 to-emerald-600 flex items-center justify-center mx-auto mb-6 shadow-lg shadow-green-500/20">
            <svg class="w-12 h-12 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
          </div>
          <h2 class="text-3xl font-bold text-white mb-2">Tes Selesai! 🎉</h2>
          <p class="text-white/60 mb-8">Terima kasih telah menyelesaikan {{ testData.tool.name }}</p>

          <!-- Summary -->
          <div class="bg-white/5 rounded-xl p-6 mb-8">
            <h4 class="text-sm font-bold text-white/50 uppercase tracking-wider mb-4">Ringkasan</h4>
            <div class="grid grid-cols-2 gap-4">
              <div class="bg-white/5 rounded-lg p-4">
                <div class="text-2xl font-bold text-cyan-400">{{ calculateScore().answered }}</div>
                <div class="text-xs text-white/50">Soal Dijawab</div>
              </div>
              <div class="bg-white/5 rounded-lg p-4">
                <div class="text-2xl font-bold text-purple-400">{{ calculateScore().total }}</div>
                <div class="text-xs text-white/50">Total Soal</div>
              </div>
            </div>
          </div>

          <!-- Per Subtest Results -->
          <div class="space-y-3 mb-8">
            <div v-for="(subtest, idx) in testData.subtests" :key="subtest.id"
              class="flex items-center justify-between p-3 bg-white/5 rounded-lg">
              <div class="flex items-center gap-3">
                <span
                  class="w-6 h-6 rounded bg-cyan-500/20 flex items-center justify-center text-xs font-bold text-cyan-400">{{
                  idx + 1 }}</span>
                <span class="text-sm text-white">{{ subtest.name }}</span>
              </div>
              <div class="flex items-center gap-2">
                <div class="w-24 h-2 bg-white/10 rounded-full overflow-hidden">
                  <div class="h-full bg-green-500"
                  :style="{ width: (subtest.questions.filter((q: any) => answeredQuestions.has(q.id)).length / subtest.questions.length * 100) + '%' }">
                  </div>
                </div>
                <span class="text-xs text-white/50 w-12 text-right">
                  {{subtest.questions.filter((q: any) => answeredQuestions.has(q.id)).length}}/{{ subtest.questions.length }}
                </span>
              </div>
            </div>
          </div>

          <div class="bg-blue-500/10 rounded-xl p-4 border border-blue-500/20 mb-6">
            <p class="text-sm text-blue-200/80">
              Hasil tes Anda akan dianalisis oleh psikolog profesional. Laporan lengkap akan tersedia dalam 1-3 hari
              kerja.
            </p>
          </div>

          <div class="flex gap-4 justify-center">
            <button @click="backToDashboard"
              class="px-8 py-3 rounded-xl bg-gradient-to-r from-cyan-500 to-blue-600 text-white font-bold hover:from-cyan-400 transition-all">
              ← Kembali ke Dashboard
            </button>
            <button @click="resetTest"
              class="px-8 py-3 rounded-xl bg-white/10 text-white font-medium hover:bg-white/20 transition-all">
              🔄 Ulang Tes
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
