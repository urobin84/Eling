<script setup lang="ts">
import { ref, watch } from 'vue';
import BaseCard from '@/components/atoms/BaseCard.vue';
import EventCodeInput from '@/components/molecules/EventCodeInput.vue';

const props = defineProps<{
  error?: string;
  isJoining: boolean;
}>();

const emit = defineEmits<{
  join: [code: string];
}>();

const localCode = ref('');

// Clear code on successful join
watch(() => [props.error, props.isJoining], ([newError, joining]) => {
  if (!newError && !joining && localCode.value) {
    localCode.value = '';
  }
});
</script>

<template>
  <BaseCard padding="lg" class="relative overflow-hidden group">
    <!-- Ambient glow -->
    <div class="absolute top-0 right-0 w-64 h-64 bg-eling-emerald/10 dark:bg-eling-emerald/5 rounded-full blur-3xl -translate-y-1/2 translate-x-1/2 group-hover:bg-eling-emerald/20 dark:group-hover:bg-eling-emerald/10 transition-all duration-700"></div>

    <div class="relative z-10 text-center">
      <h2 class="text-3xl font-bold text-gray-900 dark:text-white mb-2 transition-colors">Join an Event</h2>
      <p class="text-gray-600 dark:text-gray-300 mb-8 max-w-md mx-auto transition-colors">
        Enter the 6-character access code provided by your administrator to enroll in a new assessment event.
      </p>

      <div class="max-w-sm mx-auto">
        <EventCodeInput
          v-model="localCode"
          :is-loading="isJoining"
          @submit="$emit('join', localCode)"
        />
        
        <p v-if="error" class="mt-3 text-xs text-red-600 dark:text-red-400 font-mono bg-red-50 dark:bg-red-500/10 py-2 px-3 rounded border border-red-200 dark:border-red-500/20 transition-colors">
          ⚠️ {{ error }}
        </p>
      </div>
    </div>
  </BaseCard>
</template>
