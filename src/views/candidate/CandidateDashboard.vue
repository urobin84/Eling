<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { useRouter } from 'vue-router';
import { invoke } from '@tauri-apps/api/core';
import CandidateLayout from '@/components/templates/CandidateLayout.vue';
import JoinEventCard from '@/components/organisms/JoinEventCard.vue';
import EventGrid from '@/components/organisms/EventGrid.vue';

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
const isLoading = ref(false);
const isJoining = ref(false);
const error = ref('');

// Computed
const activeEvents = computed(() =>
  myEvents.value.filter(e => e.status !== 'completed' && e.status !== 'archived')
);

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

async function handleJoinEvent(code: string) {
  if (!code || !user.value) return;

  try {
    isJoining.value = true;
    error.value = '';

    await invoke('enroll_candidate_to_event', {
      eventCode: code,
      userId: user.value.id
    });

    await fetchMyEvents(user.value.id);
    alert('Successfully joined event!');
  } catch (e: any) {
    console.error("Join failed:", e);
    error.value = typeof e === 'string' ? e : "Failed to join event. Please check the code.";
  } finally {
    isJoining.value = false;
  }
}

function startAssessment(eventId: number) {
  console.log("Starting assessment for event:", eventId);
  router.push('/test/run');
}

function logout() {
  localStorage.removeItem('user_session');
  window.location.href = '/login';
}

function goToInfo() {
  router.push('/info');
}

function goToProfile() {
  router.push('/profile');
}

// Lifecycle
onMounted(() => {
  console.log('DEBUG: CandidateDashboard mounted');

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
</script>

<template>
  <CandidateLayout :username="user?.username || ''" @logout="logout" @info="goToInfo" @profile="goToProfile">
    <!-- Join Event Section -->
    <section class="max-w-3xl mx-auto mb-16">
      <JoinEventCard
        :error="error"
        :is-joining="isJoining"
        @join="handleJoinEvent"
      />
    </section>

    <!-- Active Events -->
    <section>
      <EventGrid
        :events="activeEvents"
        :is-loading="isLoading"
        @start-assessment="startAssessment"
      />
    </section>
  </CandidateLayout>
</template>
