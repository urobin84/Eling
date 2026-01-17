<script setup lang="ts">
interface Option {
    label: string;
    value: string | number;
}

interface Props {
    modelValue?: string | number;
    options: Option[];
    placeholder?: string;
    disabled?: boolean;
    required?: boolean;
}

withDefaults(defineProps<Props>(), {
    disabled: false,
    required: false
});

const emit = defineEmits<{
    'update:modelValue': [value: string | number];
}>();

const handleChange = (event: Event) => {
    const target = event.target as HTMLSelectElement;
    emit('update:modelValue', target.value);
};
</script>

<template>
    <div class="relative">
        <select
            :value="modelValue"
            :disabled="disabled"
            :required="required"
            @change="handleChange"
            class="input-glass w-full py-2.5 text-sm appearance-none pr-10 cursor-pointer"
        >
            <option v-if="placeholder" value="" disabled selected>{{ placeholder }}</option>
            <option 
                v-for="option in options" 
                :key="option.value" 
                :value="option.value"
                class="bg-eling-light-card dark:bg-eling-dark-card text-eling-light-text dark:text-eling-dark-text"
            >
                {{ option.label }}
            </option>
        </select>
        <!-- Custom Chevron Icon -->
        <div class="absolute inset-y-0 right-0 flex items-center px-3 pointer-events-none text-eling-light-subtext dark:text-eling-dark-text/50">
             <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
            </svg>
        </div>
    </div>
</template>
