<script setup lang="ts">
import { computed } from 'vue';
import EventStatusBadge from './EventStatusBadge.vue';

const props = defineProps<{
  status: string;
  date: string;
}>();

const formattedDate = computed(() => {
  return new Date(props.date).toLocaleDateString();
});
</script>

<template>
  <div class="flex justify-between items-start">
    <EventStatusBadge :status="status" />
    <span class="text-xs text-gray-500 dark:text-gray-400 font-mono transition-colors">{{ formattedDate }}</span>
  </div>
</template>
