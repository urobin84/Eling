<script setup lang="ts">
import BaseCard from '@/components/atoms/BaseCard.vue';
import BaseButton from '@/components/atoms/BaseButton.vue';
import BaseBadge from '@/components/atoms/BaseBadge.vue';

interface Event {
  id: number;
  event_name: string;
  description?: string;
  status: string;
  created_at: string;
  participant_status?: string;
}

defineProps<{
  event: Event;
}>();

defineEmits<{
  start: [eventId: number];
}>();
</script>

<template>
  <BaseCard padding="md" class="hover:shadow-xl dark:hover:shadow-eling-emerald/10 transition-all group active:scale-[0.98]">
    <!-- Header Row: Date & Badges -->
    <div class="flex justify-between items-start mb-4">
      <!-- Date -->
      <span class="text-xs text-gray-500 dark:text-gray-400 font-mono pt-1">
        {{ new Date(event.created_at).toLocaleDateString() }}
      </span>

      <!-- Badges -->
      <div class="flex gap-2">
        <BaseBadge
          :variant="event.status === 'published' ? 'success' : 'warning'"
        >
          {{ event.status }}
        </BaseBadge>
        <BaseBadge
          v-if="event.participant_status"
          :variant="event.participant_status === 'completed' ? 'success' : 
                   event.participant_status === 'in-progress' ? 'warning' : 'info'"
        >
          {{ event.participant_status }}
        </BaseBadge>
      </div>
    </div>

    <h3 class="text-xl font-bold text-gray-900 dark:text-white mb-2 group-hover:text-eling-emerald transition-colors">
      {{ event.event_name }}
    </h3>
    
    <p class="text-sm text-gray-600 dark:text-gray-300 mb-6 line-clamp-2 h-10 transition-colors">
      {{ event.description || 'No description provided.' }}
    </p>

    <BaseButton
      @click="$emit('start', event.id)"
      variant="primary"
      class="w-full flex items-center justify-center gap-2"
    >
      <span>START ASSESSMENT</span>
      <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7l5 5m0 0l-5 5m5-5H6" />
      </svg>
    </BaseButton>
  </BaseCard>
</template>
