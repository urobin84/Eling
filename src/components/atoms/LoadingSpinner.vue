<script setup lang="ts">
interface Props {
    size?: 'sm' | 'md' | 'lg';
}

withDefaults(defineProps<Props>(), {
    size: 'md'
});

const sizeClasses = {
    sm: 'w-6 h-6 border-2',
    md: 'w-8 h-8 border-2',
    lg: 'w-12 h-12 border-[3px]'
};
</script>

<template>
    <div 
        :class="[
            'animate-spin rounded-full border-eling-emerald border-t-transparent',
            sizeClasses[size]
        ]"
    />
</template>
