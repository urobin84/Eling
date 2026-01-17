<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed } from 'vue';

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

const props = withDefaults(defineProps<Props>(), {
    disabled: false,
    required: false
});

const emit = defineEmits<{
    'update:modelValue': [value: string | number];
}>();

const isOpen = ref(false);
const containerRef = ref<HTMLElement | null>(null);

const selectedLabel = computed(() => {
    const selected = props.options.find(opt => opt.value === props.modelValue);
    return selected ? selected.label : props.placeholder || 'Select option';
});

function toggleDropdown() {
    if (!props.disabled) {
        isOpen.value = !isOpen.value;
    }
}

function selectOption(value: string | number) {
    emit('update:modelValue', value);
    isOpen.value = false;
}

function handleClickOutside(event: MouseEvent) {
    if (containerRef.value && !containerRef.value.contains(event.target as Node)) {
        isOpen.value = false;
    }
}

onMounted(() => {
    document.addEventListener('click', handleClickOutside);
});

onUnmounted(() => {
    document.removeEventListener('click', handleClickOutside);
});
</script>

<template>
    <div ref="containerRef" class="relative">
        <!-- Trigger Button -->
        <div 
            @click="toggleDropdown"
            class="input-glass w-full py-2.5 px-3 text-sm flex items-center justify-between cursor-pointer transition-colors"
            :class="[
                disabled ? 'opacity-50 cursor-not-allowed' : 'hover:bg-black/5 dark:hover:bg-white/5',
                isOpen ? 'ring-2 ring-eling-emerald/50 border-eling-emerald' : ''
            ]"
        >
            <span :class="!modelValue && placeholder ? 'text-gray-500 dark:text-eling-dark-text/50' : 'text-gray-900 dark:text-eling-dark-text'">
                {{ selectedLabel }}
            </span>
            
            <!-- Chevron Icon -->
            <svg 
                class="w-4 h-4 text-gray-500 dark:text-eling-dark-text/50 transition-transform duration-200"
                :class="{ 'rotate-180': isOpen }"
                fill="none" 
                viewBox="0 0 24 24" 
                stroke="currentColor"
            >
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
            </svg>
        </div>

        <!-- Dropdown Menu -->
        <transition
            enter-active-class="transition duration-100 ease-out"
            enter-from-class="transform scale-95 opacity-0"
            enter-to-class="transform scale-100 opacity-100"
            leave-active-class="transition duration-75 ease-in"
            leave-from-class="transform scale-100 opacity-100"
            leave-to-class="transform scale-95 opacity-0"
        >
            <div 
                v-if="isOpen"
                class="absolute z-50 w-full mt-1 bg-white dark:bg-eling-dark-surface border border-black/10 dark:border-white/10 rounded-lg shadow-xl max-h-60 overflow-y-auto overflow-hidden"
            >
                <div class="py-1">
                    <div 
                        v-for="option in options" 
                        :key="option.value"
                        @click="selectOption(option.value)"
                        class="px-4 py-2 text-sm cursor-pointer transition-colors flex items-center justify-between"
                        :class="[
                            modelValue === option.value 
                                ? 'bg-eling-emerald/10 text-eling-emerald font-medium' 
                                : 'text-gray-700 dark:text-eling-dark-text hover:bg-black/5 dark:hover:bg-white/5'
                        ]"
                    >
                        <span>{{ option.label }}</span>
                        <svg v-if="modelValue === option.value" class="w-4 h-4 text-eling-emerald" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
                        </svg>
                    </div>
                </div>
            </div>
        </transition>
    </div>
</template>
