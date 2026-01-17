<script setup lang="ts">
import BaseInput from '@/components/atoms/BaseInput.vue';
import BaseButton from '@/components/atoms/BaseButton.vue';

defineProps<{
  modelValue: string;
  isLoading: boolean;
}>();

defineEmits<{
  'update:modelValue': [value: string];
  submit: [];
}>();
</script>

<template>
  <div class="flex gap-2">
    <BaseInput
      :model-value="modelValue"
      @update:model-value="$emit('update:modelValue', $event)"
      type="text"
      placeholder="ENTER CODE"
      maxlength="6"
      class="flex-1 text-center font-mono text-lg tracking-[0.2em] uppercase"
      @keyup.enter="$emit('submit')"
    />
    <BaseButton
      @click="$emit('submit')"
      :disabled="!modelValue || isLoading"
      variant="primary"
      class="whitespace-nowrap"
    >
      {{ isLoading ? 'JOINING...' : 'JOIN EVENT' }}
    </BaseButton>
  </div>
</template>
