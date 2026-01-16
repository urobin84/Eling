<script setup lang="ts">
import { ref } from 'vue';
import { useRouter } from 'vue-router';
import { invoke } from '@tauri-apps/api/core';

const router = useRouter();
const username = ref('');
const password = ref('');
const error = ref('');
const isLoading = ref(false);

async function handleLogin() {
  if (!username.value || !password.value) {
    error.value = "Please enter both username and password";
    return;
  }

  try {
    isLoading.value = true;
    error.value = '';
    let invokeFn = invoke;

    // Fallback if imported invoke is undefined (Tauri V2 edge case)
    if (typeof invokeFn === 'undefined') {
      console.warn('Imported invoke is undefined, trying global fallback...');
      // @ts-ignore
      if (window.__TAURI__ && window.__TAURI__.core) {
        // @ts-ignore
        invokeFn = window.__TAURI__.core.invoke;
      } else if (window.__TAURI__) {
        // @ts-ignore
        invokeFn = window.__TAURI__.invoke;
      } else {
        alert("Error: Environment Tauri tidak terdeteksi.\nMohon gunakan Jendela Aplikasi Tauri, jangan buka di Browser (Chrome/Safari)!");
        throw new Error('Tauri invoke not found. Are you running in a browser?');
      }
    }

    const user = await invokeFn<any>('login_user', { username: username.value, password: password.value });
    console.log('DEBUG: Login successful', { id: user.id, role: user.role, username: user.username });

    // Save session
    const sessionData = {
      id: user.id,
      username: username.value,
      role: user.role
    };
    console.log('DEBUG: Saving session to localStorage', sessionData);
    localStorage.setItem('user_session', JSON.stringify(sessionData));

    // Redirect based on role
    if (user.role === 'admin') {
      console.log('DEBUG: Redirecting to /admin');
      window.location.href = '/admin';
    } else {
      console.log('DEBUG: Redirecting to /dashboard');
      window.location.href = '/dashboard';
    }
  } catch (e: any) {
    console.error("DEBUG: Login failed", e);
    error.value = "Invalid credentials. Please try again.";
  } finally {
    isLoading.value = false;
  }
}
</script>

<template>
  <div
    class="min-h-screen flex items-center justify-center bg-gradient-to-br from-[#F8FAFD] to-[#EDF2FA] dark:from-[#131314] dark:to-[#1E1F20] relative overflow-hidden transition-colors duration-300">
    <!-- Ambient Background Orbs (Gemini Style) -->
    <div class="absolute -top-40 -right-40 w-96 h-96 bg-eling-emerald/10 rounded-full blur-3xl animate-pulse"></div>
    <div class="absolute bottom-20 left-20 w-64 h-64 bg-eling-blue/10 rounded-full blur-3xl animate-pulse"
      style="animation-delay: 700ms;"></div>
    <div
      class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[600px] h-[600px] bg-eling-purple/5 rounded-full blur-3xl">
    </div>

    <!-- Login Card -->
    <div class="glass-panel w-full max-w-md p-8 relative z-10 flex flex-col items-center">
      <!-- Logo with Gemini Gradient -->
      <div class="mb-8 text-center">
        <div class="w-20 h-20 mx-auto mb-4 rounded-2xl bg-gemini-gradient flex items-center justify-center shadow-lg">
          <span class="text-3xl font-bold text-white">E</span>
        </div>
        <h1 class="text-3xl font-bold bg-clip-text text-transparent bg-gemini-gradient">ELING</h1>
        <p class="text-sm text-gray-600 dark:text-gray-400 mt-2">Kesadaran Penuh dalam Setiap Interaksi</p>
      </div>

      <!-- Error Message -->
      <div v-if="error"
        class="w-full mb-4 p-3 bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-xl text-red-600 dark:text-red-400 text-sm">
        {{ error }}
      </div>

      <!-- Login Form -->
      <form @submit.prevent="handleLogin" class="w-full space-y-4">
        <!-- Username Input -->
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Username</label>
          <input v-model="username" type="text" required class="input-glass w-full" placeholder="Enter your username" />
        </div>

        <!-- Password Input -->
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Password</label>
          <input v-model="password" type="password" required class="input-glass w-full"
            placeholder="Enter your password" />
        </div>

        <!-- Login Button -->
        <button type="submit" :disabled="isLoading"
          class="btn-neumorphic w-full py-3 text-base font-bold disabled:opacity-50 disabled:cursor-not-allowed">
          <span v-if="!isLoading">Login</span>
          <span v-else class="flex items-center justify-center gap-2">
            <svg class="animate-spin h-5 w-5" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
              <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
              <path class="opacity-75" fill="currentColor"
                d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z">
              </path>
            </svg>
            Logging in...
          </span>
        </button>
      </form>

      <!-- Footer -->
      <div class="mt-6 text-center text-sm text-gray-500 dark:text-gray-400">
        <p>Default credentials:</p>
        <p class="font-mono text-xs mt-1">admin / password123</p>
      </div>

      <!-- Eling Indicator -->
      <div class="camera-indicator mt-6">
        <div class="dot"></div>
        <span class="text text-xs">System Active</span>
      </div>
    </div>
  </div>
</template>
