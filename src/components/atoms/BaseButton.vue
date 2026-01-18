<script setup lang="ts">
import { computed } from 'vue';

interface Props {
    variant?: 'primary' | 'danger' | 'ghost';
    size?: 'sm' | 'md' | 'lg';
    disabled?: boolean;
    type?: 'button' | 'submit' | 'reset';
}

const props = withDefaults(defineProps<Props>(), {
    variant: 'primary',
    size: 'md',
    disabled: false,
    type: 'button'
});

const emit = defineEmits<{
    click: [event: MouseEvent];
}>();

const buttonClasses = computed(() => {
    const base = 'btn-neumorphic font-bold whitespace-nowrap transition-all flex items-center justify-center gap-2';
    
    const variants = {
        primary: 'shadow-eling-emerald/20',
        danger: 'bg-red-500/10 text-red-400 border-red-500/20 hover:bg-red-500/20',
        ghost: 'bg-transparent border-black/10 dark:border-white/10 hover:bg-black/5 dark:hover:bg-white/5'
    };
    
    const sizes = {
        sm: 'text-xs py-1.5 px-3',
        md: 'text-sm py-2.5 px-5',
        lg: 'text-base py-3 px-6'
    };
    
    return `${base} ${variants[props.variant]} ${sizes[props.size]}`;
});
</script>

<template>
    <button 
        :class="buttonClasses"
        :disabled="disabled"
        :type="type"
        @click="emit('click', $event)"
    >
        <slot />
    </button>
</template>
