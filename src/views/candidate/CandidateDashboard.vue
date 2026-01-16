<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { useRouter } from 'vue-router';
import { invoke } from '@tauri-apps/api/core';

// Types
interface Event {
  id: number;
  event_name: string;
  description?: string;
  status: string;
  event_code?: string;
  enrollment_deadline?: string;
  created_at: string;
}

interface User {
  id: number;
  username: string;
  role: string;
}

// State
const router = useRouter();
const user = ref<User | null>(null);
const myEvents = ref<Event[]>([]);
const joinCode = ref('');
const isLoading = ref(false);
const isJoining = ref(false);
const error = ref('');

// Computed
const activeEvents = computed(() =>
  myEvents.value.filter(e => e.status !== 'completed' && e.status !== 'archived')
);

const pastEvents = computed(() =>
  myEvents.value.filter(e => e.status === 'completed' || e.status === 'archived')
);

// Lifecycle
onMounted(() => {
  console.log('DEBUG: CandidateDashboard mounted');

  // Get user from localStorage
  const sessionStr = localStorage.getItem('user_session');
  console.log('DEBUG: Session from localStorage:', sessionStr);

  if (sessionStr) {
    try {
      const session = JSON.parse(sessionStr);
      console.log('DEBUG: Parsed session:', session);
      user.value = session;
      if (user.value && user.value.id) {
        fetchMyEvents(user.value.id);
      }
    } catch (e) {
      console.error('DEBUG: Session parse error:', e);
      router.push('/login');
    }
  } else {
    console.warn('DEBUG: No session found, redirecting to login');
    router.push('/login');
  }
});

// Methods
async function fetchMyEvents(userId: number) {
  try {
    isLoading.value = true;
    const events = await invoke<Event[]>('get_my_events', { userId });
    myEvents.value = events;
  } catch (e) {
    console.error("Failed to fetch events:", e);
  } finally {
    isLoading.value = false;
  }
}

async function handleJoinEvent() {
  if (!joinCode.value || !user.value) return;

  try {
    isJoining.value = true;
    error.value = '';

    // Call backend to enroll
    await invoke('enroll_candidate_to_event', {
      eventCode: joinCode.value,
      userId: user.value.id
    });

    // Refresh list
    await fetchMyEvents(user.value.id);
    joinCode.value = '';
    alert('Successfully joined event!');
  } catch (e: any) {
    console.error("Join failed:", e);
    error.value = typeof e === 'string' ? e : "Failed to join event. Please check the code.";
  } finally {
    isJoining.value = false;
  }
}

function startAssessment(eventId: number) {
  // In future, pass eventId to test runner
  console.log("Starting assessment for event:", eventId);
  router.push('/test/run');
}

function logout() {
  localStorage.removeItem('user_session');
  window.location.href = '/login';
}

// onMounted already called above with inline function
</script>

<template>
  <div class="min-h-screen bg-eling-dark">
    <!-- Navbar -->
    <nav class="glass-panel sticky top-0 z-30 border-b border-white/5">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="flex justify-between h-16">
          <div class="flex items-center gap-4">
            <div
              class="w-10 h-10 rounded-xl bg-gradient-to-br from-eling-accent to-eling-dark border border-eling-accent/30 flex items-center justify-center">
              <span class="text-xl font-bold text-eling-light">E</span>
            </div>
            <div>
              <h1 class="text-lg font-bold text-eling-light tracking-wide">ELING</h1>
              <p class="text-[10px] text-eling-light/50 font-mono tracking-widest uppercase">Candidate Portal</p>
            </div>
          </div>

          <div class="flex items-center space-x-6">
            <div class="text-right hidden sm:block">
              <span class="block text-xs text-eling-light/50 font-mono uppercase">Logged in as</span>
              <span class="block text-sm font-bold text-eling-accent">{{ user?.username }}</span>
            </div>
            <button @click="logout"
              class="px-4 py-2 rounded-xl text-xs font-bold uppercase tracking-wider text-eling-light/70 hover:text-white bg-white/5 hover:bg-white/10 border border-white/5 transition-all">
              Log Out
            </button>
          </div>
        </div>
      </div>
    </nav>

    <main class="max-w-7xl mx-auto py-12 px-4 sm:px-6 lg:px-8">
      <!-- 1. Join Event Section -->
      <section class="max-w-3xl mx-auto mb-16">
        <div class="glass-panel p-8 relative overflow-hidden group">
          <div
            class="absolute top-0 right-0 w-64 h-64 bg-eling-accent/5 rounded-full blur-3xl -translate-y-1/2 translate-x-1/2 group-hover:bg-eling-accent/10 transition-all duration-700">
          </div>

          <div class="relative z-10 text-center">
            <h2 class="text-3xl font-bold text-eling-light mb-2">Join an Event</h2>
            <p class="text-eling-light/60 mb-8 max-w-md mx-auto">Enter the 6-character access code provided by your
              administrator to enroll in a new assessment event.</p>

            <div class="max-w-sm mx-auto">
              <div class="flex gap-2">
                <input v-model="joinCode" type="text" placeholder="ENTER CODE" maxlength="6"
                  class="flex-1 input-glass bg-black/30 border-white/10 focus:border-eling-accent/50 text-center font-mono text-lg tracking-[0.2em] uppercase placeholder:normal-case placeholder:tracking-normal"
                  @keyup.enter="handleJoinEvent">
                <button @click="handleJoinEvent" :disabled="!joinCode || isJoining"
                  class="px-6 btn-neumorphic font-bold whitespace-nowrap disabled:opacity-50 disabled:cursor-not-allowed">
                  {{ isJoining ? 'JOINING...' : 'JOIN EVENT' }}
                </button>
              </div>
              <p v-if="error"
                class="mt-3 text-xs text-red-400 font-mono bg-red-500/10 py-1 px-2 rounded inline-block border border-red-500/20">
                ⚠️ {{ error }}
              </p>
            </div>
          </div>
        </div>
      </section>

      <!-- 2. Active Events List -->
      <section>
        <div class="flex items-center gap-3 mb-6">
          <h2 class="text-xl font-bold text-eling-light">My Assessments</h2>
          <span
            class="px-2 py-0.5 rounded-full bg-eling-accent/20 border border-eling-accent/30 text-eling-accent text-xs font-bold">
            {{ activeEvents.length }}
          </span>
        </div>

        <!-- Empty State -->
        <div v-if="activeEvents.length === 0 && !isLoading"
          class="text-center py-16 border border-dashed border-white/10 rounded-2xl bg-white/5">
          <div class="w-16 h-16 rounded-full bg-white/5 mx-auto flex items-center justify-center mb-4">
            <svg class="w-8 h-8 text-eling-light/30" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" />
            </svg>
          </div>
          <p class="text-eling-light/50 font-medium">No active assessments found.</p>
          <p class="text-sm text-eling-light/30 mt-1">Join an event to get started.</p>
        </div>

        <!-- Event Grid -->
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          <div v-for="event in activeEvents" :key="event.id"
            class="glass-panel p-6 border-white/5 hover:border-eling-accent/50 transition-all group active:scale-[0.98]">

            <div class="flex justify-between items-start mb-4">
              <div
                class="px-2 py-1 rounded bg-eling-accent/10 border border-eling-accent/20 text-eling-accent text-[10px] font-bold uppercase tracking-wider">
                {{ event.status }}
              </div>
              <span class="text-xs text-eling-light/40 font-mono">{{ new Date(event.created_at ||
                '').toLocaleDateString() }}</span>
            </div>

            <h3 class="text-xl font-bold text-eling-light mb-2 group-hover:text-eling-accent transition-colors">{{ event.event_name }}</h3>
            <p class="text-sm text-eling-light/60 mb-6 line-clamp-2 h-10">{{ event.description || 'No description provided.' }}</p>
            <h3 class="text-xl font-bold text-eling-light mb-2 group-hover:text-eling-accent transition-colors">{{ event.event_name }}</h3>
            <p class="text-sm text-eling-light/60 mb-6 line-clamp-2 h-10">{{ event.description || 'No description provided.' }}</p>
            <h3 class="text-xl font-bold text-eling-light mb-2 group-hover:text-eling-accent transition-colors">{{ event.event_name }}</h3>
            <p class="text-sm text-eling-light/60 mb-6 line-clamp-2 h-10">{{ event.description || 'No description provided.' }}</p>
            <h3 class="text-xl font-bold text-eling-light mb-2 group-hover:text-eling-accent transition-colors">{{ event.event_name }}</h3>
            <p class="text-sm text-eling-light/60 mb-6 line-clamp-2 h-10">{{ event.description || 'No description provided.' }}</p>

            <button @click="startAssessment(event.id)"
              class="w-full py-3 rounded-xl bg-white/5 hover:bg-eling-accent hover:text-eling-dark border border-white/10 hover:border-eling-accent transition-all font-bold text-sm tracking-wide flex items-center justify-center gap-2">
              <span>START ASSESSMENT</span>
              <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7l5 5m0 0l-5 5m5-5H6" />
              </svg>
            </button>
          </div>
        </div>
      </section>

      <!-- 3. Past Events (Optional, collapsible) -->
      <section v-if="pastEvents.length > 0" class="mt-16 pt-8 border-t border-white/5">
        <h3 class="text-sm font-bold text-eling-light/50 uppercase tracking-wider mb-6">Past Assessments</h3>
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 opacity-60">
          <div v-for="event in pastEvents" :key="event.id" class="p-6 rounded-2xl bg-black/20 border border-white/5">
            <h4 class="font-bold text-eling-light/70 mb-1">{{ event.event_name }}</h4>
            <div class="flex justify-between items-center text-xs text-eling-light/30">
              <span>{{ event.status }}</span>
              <span>{{ new Date(event.created_at || '').toLocaleDateString() }}</span>
            </div>
          </div>
        </div>
      </section>
    </main>
  </div>
</template>
