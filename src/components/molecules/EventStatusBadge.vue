<script setup lang="ts">
import { computed } from 'vue';
import BaseBadge from '@/components/atoms/BaseBadge.vue';

const props = defineProps<{
  status: string;
}>();

const badgeVariant = computed(() => {
  switch (props.status.toLowerCase()) {
    case 'active': return 'success';
    case 'pending': return 'warning';
    case 'completed': return 'info';
    default: return 'default';
  }
});
</script>

<template>
  <BaseBadge :variant="badgeVariant" size="sm">
    {{ status }}
  </BaseBadge>
</template>
