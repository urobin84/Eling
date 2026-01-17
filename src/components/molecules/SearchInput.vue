<script setup lang="ts">
import BaseInput from '../atoms/BaseInput.vue';

interface Props {
    modelValue?: string;
    placeholder?: string;
}

withDefaults(defineProps<Props>(), {
    placeholder: 'Search...'
});

const emit = defineEmits<{
    'update:modelValue': [value: string];
}>();

const handleInput = (value: string | number) => {
    emit('update:modelValue', String(value));
};
</script>

<template>
    <div class="relative">
        <BaseInput
            :model-value="modelValue"
            type="search"
            :placeholder="placeholder"
            class="pl-11"
            @update:model-value="handleInput"
        />
        <svg 
            class="w-4 h-4 text-gray-500 dark:text-gray-400 absolute left-4 top-1/2 -translate-y-1/2 pointer-events-none" 
            fill="none" 
            viewBox="0 0 24 24" 
            stroke="currentColor"
        >
            <path 
                stroke-linecap="round" 
                stroke-linejoin="round" 
                stroke-width="2"
                d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" 
            />
        </svg>
    </div>
</template>
