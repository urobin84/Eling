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
    // Call backend login
    // Call backend login
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

    // Save session for development convenience
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
  <div class="min-h-screen flex items-center justify-center bg-eling-dark relative overflow-hidden">
    <!-- Ambient Background -->
    <div class="absolute inset-0 bg-gradient-to-br from-[#1B3022] to-[#0D1612]"></div>
    <div class="absolute -top-40 -right-40 w-96 h-96 bg-eling-accent/10 rounded-full blur-3xl animate-pulse"></div>
    <div
      class="absolute bottom-20 left-20 w-64 h-64 bg-eling-secondary/10 rounded-full blur-3xl animate-pulse delay-700">
    </div>

    <!-- Login Card -->
    <div class="glass-panel w-full max-w-md p-8 relative z-10 border-eling-accent/20 flex flex-col items-center">

      <!-- Logo -->
      <div class="mb-8 flex flex-col items-center">
        <div
          class="w-16 h-16 rounded-2xl bg-gradient-to-tr from-eling-surface to-eling-dark border border-eling-accent/30 flex items-center justify-center shadow-lg shadow-eling-accent/10 mb-4">
          <span class="text-2xl font-bold text-eling-accent">E</span>
        </div>
        <h1 class="text-2xl font-bold text-eling-light tracking-wide">ELING</h1>
        <p class="text-xs text-eling-light/50 font-mono tracking-widest uppercase mt-1">Conscious Intelligence</p>
      </div>

      <form class="w-full space-y-6" @submit.prevent="handleLogin">
        <div class="space-y-4">
          <div>
            <label for="username"
              class="block text-xs font-mono text-eling-accent mb-1.5 uppercase tracking-wider">Username</label>
            <input v-model="username" id="username" type="text" required
              class="input-glass w-full bg-black/20 focus:bg-black/40 border-white/10 text-center tracking-wider"
              placeholder="ENTER USERNAME">
          </div>
          <div>
            <label for="password"
              class="block text-xs font-mono text-eling-light/70 mb-1.5 uppercase tracking-wider">Password</label>
            <input v-model="password" id="password" type="password" required
              class="input-glass w-full bg-black/20 focus:bg-black/40 border-white/10 text-center tracking-wider"
              placeholder="••••••••">
          </div>
        </div>

        <div v-if="error"
          class="p-3 rounded-lg bg-red-500/10 border border-red-500/20 text-red-400 text-xs text-center font-mono">
          {{ error }}
        </div>

        <div>
          <button type="submit" :disabled="isLoading"
            class="group btn-neumorphic w-full py-3 flex justify-center items-center font-bold text-sm tracking-wide">
            <span v-if="isLoading"
              class="w-4 h-4 border-2 border-eling-dark/30 border-t-eling-dark rounded-full animate-spin mr-2"></span>
            {{ isLoading ? 'AUTHENTICATING...' : 'ACCESS TERMINAL' }}
          </button>
        </div>

        <div class="text-center pt-4 border-t border-white/5">
          <router-link to="/register"
            class="text-xs font-mono text-eling-light/40 hover:text-eling-accent transition-colors">
            NO ACCOUNT? REGISTER_CANDIDATE
          </router-link>
        </div>
      </form>
    </div>

    <!-- Footer -->
    <div class="absolute bottom-6 text-[10px] font-mono text-eling-light/20">
      SECURE TERMINAL // V1.0.0
    </div>
  </div>
</template>
