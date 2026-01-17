<script setup lang="ts">
import { ref, onMounted } from 'vue';
import ThemeToggle from '../ThemeToggle.vue';
import NotificationBell from '../NotificationBell.vue';
import { useAuthStore } from '../../stores/auth';

const authStore = useAuthStore();
const showUserDropdown = ref(false);

const emit = defineEmits<{
    logout: [];
    profile: [];
}>();

function handleLogout() {
    showUserDropdown.value = false;
    emit('logout');
}

function goToProfile() {
    showUserDropdown.value = false;
    emit('profile');
}

onMounted(() => {
    if (!authStore.user) {
        authStore.fetchUser();
    }
});
</script>

<template>
    <div class="sticky top-0 h-16 border-b border-black/5 dark:border-white/5 bg-white/5 backdrop-blur-sm flex items-center justify-end px-8 gap-4 z-[100]">
        <!-- Theme Toggle -->
        <ThemeToggle />

        <!-- Notification Bell -->
        <NotificationBell />

        <!-- User Profile Dropdown -->
        <div class="relative">
            <button 
                @click="showUserDropdown = !showUserDropdown"
                class="flex items-center gap-2 px-3 py-1.5 rounded-lg hover:bg-black/5 dark:hover:bg-white/5 transition-colors"
            >
                <div class="w-8 h-8 rounded-full bg-gradient-to-br from-eling-emerald to-cyan-500 overflow-hidden flex items-center justify-center text-white font-bold text-sm border border-white/20">
                    <img v-if="authStore.user?.avatar_url" :src="authStore.user.avatar_url" class="w-full h-full object-cover" />
                    <span v-else>{{ authStore.user?.username?.charAt(0).toUpperCase() || 'A' }}</span>
                </div>
                <span class="text-sm font-medium text-gray-900 dark:text-eling-dark-text">{{ authStore.user?.username || 'Admin' }}</span>

                <svg class="w-4 h-4 text-gray-500" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
                </svg>
            </button>

            <!-- Dropdown Menu -->
            <div 
                v-if="showUserDropdown"
                class="absolute right-0 mt-2 w-48 rounded-xl shadow-xl bg-white dark:bg-eling-dark-surface border border-black/10 dark:border-white/10 py-1 z-50"
            >
                <button 
                    @click="goToProfile"
                    class="w-full text-left px-4 py-2 text-sm text-gray-700 dark:text-eling-dark-text hover:bg-black/5 dark:hover:bg-white/5 transition-colors flex items-center gap-2"
                >
                    <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
                    </svg>
                    Profile Settings
                </button>
                <div class="border-t border-black/10 dark:border-white/10 my-1"></div>
                <button 
                    @click="handleLogout"
                    class="w-full text-left px-4 py-2 text-sm text-red-600 dark:text-red-400 hover:bg-red-50 dark:hover:bg-red-500/10 transition-colors flex items-center gap-2"
                >
                    <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1" />
                    </svg>
                    Logout
                </button>
            </div>
        </div>
    </div>
</template>
