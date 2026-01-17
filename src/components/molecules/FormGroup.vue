<script setup lang="ts">
import BaseInput from '../atoms/BaseInput.vue';

interface Props {
    label: string;
    modelValue?: string | number;
    type?: 'text' | 'password' | 'email' | 'number' | 'date';
    placeholder?: string;
    error?: string;
    required?: boolean;
}

withDefaults(defineProps<Props>(), {
    type: 'text',
    required: false
});

const emit = defineEmits<{
    'update:modelValue': [value: string | number];
}>();
</script>

<template>
    <div class="space-y-1.5">
        <label class="block text-xs font-mono text-gray-900 dark:text-eling-dark-text/70 uppercase tracking-wider">
            {{ label }}
            <span v-if="required" class="text-red-400">*</span>
        </label>
        <BaseInput
            :model-value="modelValue"
            :type="type"
            :placeholder="placeholder"
            :required="required"
            @update:model-value="emit('update:modelValue', $event)"
        />
        <p v-if="error" class="text-xs text-red-400">{{ error }}</p>
    </div>
</template>
