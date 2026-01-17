<script setup lang="ts">
interface Props {
    variant?: 'success' | 'warning' | 'error' | 'info' | 'neutral';
    size?: 'sm' | 'md';
}

withDefaults(defineProps<Props>(), {
    variant: 'neutral',
    size: 'md'
});

const variantClasses = {
    success: 'bg-green-500/10 text-green-400 border-green-500/20',
    warning: 'bg-yellow-500/10 text-yellow-400 border-yellow-500/20',
    error: 'bg-red-500/10 text-red-400 border-red-500/20',
    info: 'bg-blue-500/10 text-blue-400 border-blue-500/20',
    neutral: 'bg-gray-500/10 text-gray-400 border-gray-500/20'
};

const sizeClasses = {
    sm: 'px-1.5 py-0.5 text-[10px]',
    md: 'px-2 py-1 text-xs'
};
</script>

<template>
    <span 
        :class="[
            'inline-block rounded font-bold uppercase tracking-wide border',
            variantClasses[variant],
            sizeClasses[size]
        ]"
    >
        <slot />
    </span>
</template>
