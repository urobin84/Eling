<script setup lang="ts">
import { ref, onMounted } from 'vue';

const isDark = ref(true);

function toggleTheme() {
    isDark.value = !isDark.value;
    updateTheme();
}

function updateTheme() {
    if (isDark.value) {
        document.documentElement.classList.add('dark');
        localStorage.setItem('theme', 'dark');
    } else {
        document.documentElement.classList.remove('dark');
        localStorage.setItem('theme', 'light');
    }
}

onMounted(() => {
    // Check local storage or system preference
    const savedTheme = localStorage.getItem('theme');
    if (savedTheme) {
        isDark.value = savedTheme === 'dark';
    } else {
        // Default to dark since app started as dark
        isDark.value = true;
    }
    updateTheme();
});
</script>

<template>
    <button @click="toggleTheme"
        class="p-2 rounded-full transition-all duration-300 hover:bg-black/5 dark:hover:bg-white/10 text-eling-light-text dark:text-eling-dark-text"
        :title="isDark ? 'Switch to Light Mode' : 'Switch to Dark Mode'">
        <!-- Sun Icon (for Dark Mode -> show Sun to switch to light) -->
        <svg v-if="isDark" xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24"
            stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z" />
        </svg>

        <!-- Moon Icon (for Light Mode -> show Moon to switch to dark) -->
        <svg v-else xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24"
            stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z" />
        </svg>
    </button>
</template>
