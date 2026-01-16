<script setup lang="ts">
import { ref } from 'vue';
import { useRouter } from 'vue-router';
import { invoke } from '@tauri-apps/api/core';

const router = useRouter();
const username = ref('');
const password = ref('');
const confirmPassword = ref('');
const error = ref('');
const isLoading = ref(false);

async function handleRegister() {
  if (password.value !== confirmPassword.value) {
    error.value = "PASSWORDS DO NOT MATCH DETECTED";
    return;
  }

  if (!username.value || !password.value) {
    error.value = "FIELDS CANNOT BE EMPTY";
    return;
  }

  try {
    isLoading.value = true;
    error.value = '';
    // Call backend register (Candidate role)
    await invoke('register_user', { username: username.value, password: password.value, role: 'candidate' });

    // If successful, redirect to login
    router.push('/login');
  } catch (e: any) {
    console.error("Register failed", e);
    error.value = "REGISTRATION FAILED: CHECK CONSOLE";
  } finally {
    isLoading.value = false;
  }
}
</script>

<template>
  <div class="min-h-screen flex items-center justify-center bg-eling-dark relative overflow-hidden">
    <!-- Ambient Background -->
    <div class="absolute inset-0 bg-gradient-to-bl from-[#1B3022] to-[#0D1612]"></div>
    <div class="absolute -bottom-40 -left-40 w-96 h-96 bg-eling-accent/10 rounded-full blur-3xl animate-pulse"></div>
    <div class="absolute top-20 right-20 w-64 h-64 bg-eling-secondary/10 rounded-full blur-3xl animate-pulse delay-700">
    </div>

    <!-- Register Card -->
    <div class="glass-panel w-full max-w-md p-8 relative z-10 border-eling-accent/20 flex flex-col items-center">

      <!-- Header -->
      <div class="mb-8 text-center">
        <h2 class="text-xl font-bold text-eling-light tracking-wide uppercase">New Candidate</h2>
        <p class="text-xs text-eling-light/50 font-mono mt-1">INITIALIZE PROFILE SEQUENCE</p>
      </div>

      <form class="w-full space-y-5" @submit.prevent="handleRegister">
        <div class="space-y-4">
          <div>
            <label for="username"
              class="block text-xs font-mono text-eling-accent mb-1.5 uppercase tracking-wider">Username</label>
            <input v-model="username" id="username" type="text" required
              class="input-glass w-full bg-black/20 focus:bg-black/40 border-white/10 text-center tracking-wider"
              placeholder="CHOOSE IDENTIFIER">
          </div>
          <div>
            <label for="password"
              class="block text-xs font-mono text-eling-light/70 mb-1.5 uppercase tracking-wider">Password</label>
            <input v-model="password" id="password" type="password" required
              class="input-glass w-full bg-black/20 focus:bg-black/40 border-white/10 text-center tracking-wider"
              placeholder="••••••••">
          </div>
          <div>
            <label for="confirm-password"
              class="block text-xs font-mono text-eling-light/70 mb-1.5 uppercase tracking-wider">Confirm
              Password</label>
            <input v-model="confirmPassword" id="confirm-password" type="password" required
              class="input-glass w-full bg-black/20 focus:bg-black/40 border-white/10 text-center tracking-wider"
              placeholder="••••••••">
          </div>
        </div>

        <div v-if="error"
          class="p-3 rounded-lg bg-red-500/10 border border-red-500/20 text-red-400 text-xs text-center font-mono animate-pulse">
          /!/ {{ error }}
        </div>

        <div>
          <button type="submit" :disabled="isLoading"
            class="group btn-neumorphic w-full py-3 flex justify-center items-center font-bold text-sm tracking-wide">
            <span v-if="isLoading"
              class="w-4 h-4 border-2 border-eling-dark/30 border-t-eling-dark rounded-full animate-spin mr-2"></span>
            {{ isLoading ? 'REGISTERING...' : 'CREATE PROFILE' }}
          </button>
        </div>

        <div class="text-center pt-4 border-t border-white/5">
          <router-link to="/login"
            class="text-xs font-mono text-eling-light/40 hover:text-eling-accent transition-colors">
            EXISTING CANDIDATE? ACCESS_TERMINAL
          </router-link>
        </div>
      </form>
    </div>
  </div>
</template>
