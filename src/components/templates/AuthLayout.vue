<script setup lang="ts">
import ThemeToggle from '../ThemeToggle.vue';
</script>

<template>
    <div class="min-h-screen bg-gradient-to-br from-eling-light-bg via-white to-eling-emerald/5 dark:from-eling-dark-bg dark:via-eling-dark-surface dark:to-eling-emerald/5 flex items-center justify-center p-4 transition-colors duration-300">
        <!-- Theme Toggle (Top Right) -->
        <div class="absolute top-4 right-4">
            <ThemeToggle />
        </div>

        <!-- Auth Card -->
        <div class="w-full max-w-md">
            <slot />
        </div>
    </div>
</template>
