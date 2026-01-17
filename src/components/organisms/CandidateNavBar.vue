<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import ThemeToggle from '@/components/ThemeToggle.vue';
import { onAvatarUpdate } from '@/utils/avatarEvents';

const props = defineProps<{
  username: string;
}>();

const emit = defineEmits(['logout', 'profile', 'info']);

const showUserDropdown = ref(false);
const avatarUrl = ref<string>('');

function loadAvatar() {
  const sessionStr = localStorage.getItem('user_session');
  if (sessionStr) {
    try {
      const session = JSON.parse(sessionStr);
      const savedAvatar = localStorage.getItem(`avatar_${session.id}`);
      avatarUrl.value = savedAvatar || '';
    } catch (e) {
      console.error('Failed to load avatar:', e);
    }
  }
}

onMounted(() => {
  loadAvatar();
  
  // Listen for avatar updates
  const cleanup = onAvatarUpdate(() => {
    loadAvatar();
  });
  
  onUnmounted(cleanup);
});

function handleLogout() {
  showUserDropdown.value = false;
  emit('logout');
}

function goToProfile() {
  showUserDropdown.value = false;
  emit('profile');
}

function goToInfo() {
  showUserDropdown.value = false;
  emit('info');
}
</script>

<template>
  <nav class="bg-white/90 dark:bg-eling-dark-surface/90 backdrop-blur-xl sticky top-0 z-30 border-b border-gray-200 dark:border-white/10 shadow-sm transition-colors">
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
      <div class="flex justify-between h-16">
        <!-- Logo -->
        <div class="flex items-center gap-4">
          <div class="w-10 h-10 rounded-xl bg-gradient-to-br from-eling-emerald to-emerald-600 border border-eling-emerald/30 flex items-center justify-center shadow-lg">
            <span class="text-xl font-bold text-white">E</span>
          </div>
          <div>
            <h1 class="text-lg font-bold text-gray-900 dark:text-white tracking-wide transition-colors">ELING</h1>
            <p class="text-[10px] text-gray-500 dark:text-gray-400 font-mono tracking-widest uppercase transition-colors">Candidate Portal</p>
          </div>
        </div>

        <!-- Theme Toggle + User Avatar Dropdown -->
        <div class="flex items-center gap-4">
          <!-- Theme Toggle -->
          <ThemeToggle />
          
          <!-- User Profile Dropdown -->
          <div class="relative">
            <button 
              @click="showUserDropdown = !showUserDropdown"
              class="flex items-center gap-2 px-3 py-1.5 rounded-lg hover:bg-black/5 dark:hover:bg-white/5 transition-colors"
            >
              <!-- Avatar with photo or initial -->
              <div v-if="avatarUrl" class="w-8 h-8 rounded-full overflow-hidden border-2 border-eling-emerald/30">
                <img :src="avatarUrl" alt="Avatar" class="w-full h-full object-cover" />
              </div>
              <div v-else class="w-8 h-8 rounded-full bg-gradient-to-br from-eling-emerald to-cyan-500 flex items-center justify-center text-white font-bold text-sm">
                {{ username.charAt(0).toUpperCase() }}
              </div>
              <span class="text-sm font-medium text-gray-900 dark:text-white hidden sm:block">{{ username }}</span>
              <svg class="w-4 h-4 text-gray-500 dark:text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
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
              <button 
                @click="goToInfo"
                class="w-full text-left px-4 py-2 text-sm text-gray-700 dark:text-eling-dark-text hover:bg-black/5 dark:hover:bg-white/5 transition-colors flex items-center gap-2"
              >
                <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                </svg>
                Info Aplikasi
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
      </div>
    </div>
  </nav>
</template>
