<script setup lang="ts">
import BaseBadge from '@/components/atoms/BaseBadge.vue';
import LoadingSpinner from '@/components/atoms/LoadingSpinner.vue';
import EventCard from './EventCard.vue';

interface Event {
  id: number;
  event_name: string;
  description?: string;
  status: string;
  created_at: string;
}

defineProps<{
  events: Event[];
  isLoading: boolean;
}>();

defineEmits<{
  startAssessment: [eventId: number];
}>();
</script>

<template>
  <div>
    <!-- Header -->
    <div class="flex items-center gap-3 mb-6">
      <h2 class="text-xl font-bold text-gray-900 dark:text-white transition-colors">My Assessments</h2>
      <BaseBadge variant="success">{{ events.length }}</BaseBadge>
    </div>

    <!-- Loading State -->
    <div v-if="isLoading" class="flex justify-center py-20">
      <LoadingSpinner size="lg" />
    </div>

    <!-- Empty State -->
    <div v-else-if="events.length === 0" class="text-center py-16 border-2 border-dashed border-gray-300 dark:border-white/20 rounded-2xl bg-white/50 dark:bg-white/5 transition-colors">
      <div class="w-16 h-16 rounded-full bg-gray-100 dark:bg-white/10 mx-auto flex items-center justify-center mb-4 transition-colors">
        <svg class="w-8 h-8 text-gray-400 dark:text-gray-500" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" />
        </svg>
      </div>
      <p class="text-gray-600 dark:text-gray-300 font-medium transition-colors">No active assessments found.</p>
      <p class="text-sm text-gray-500 dark:text-gray-400 mt-1 transition-colors">Join an event to get started.</p>
    </div>

    <!-- Event Grid -->
    <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      <EventCard
        v-for="event in events"
        :key="event.id"
        :event="event"
        @start="$emit('startAssessment', $event)"
      />
    </div>
  </div>
</template>
