<script setup lang="ts">
interface Props {
    padding?: 'sm' | 'md' | 'lg';
}

withDefaults(defineProps<Props>(), {
    padding: 'md'
});

const paddingClasses = {
    sm: 'p-3',
    md: 'p-4',
    lg: 'p-6'
};
</script>

<template>
    <div :class="['glass-panel border-black/5 dark:border-white/5', paddingClasses[padding]]">
        <slot />
    </div>
</template>
