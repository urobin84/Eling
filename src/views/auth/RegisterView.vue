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
  <div class="min-h-screen flex items-center justify-center bg-gradient-to-br from-gray-50 to-gray-100 relative overflow-hidden">
    <!-- Ambient Background -->
    <div class="absolute inset-0 bg-gradient-to-br from-blue-50 via-emerald-50 to-cyan-50"></div>
    <div class="absolute -top-40 -right-40 w-96 h-96 bg-eling-emerald/20 rounded-full blur-3xl animate-pulse"></div>
    <div class="absolute bottom-20 left-20 w-64 h-64 bg-blue-400/20 rounded-full blur-3xl animate-pulse delay-700"></div>

    <!-- Register Card -->
    <div class="bg-white/90 backdrop-blur-xl w-full max-w-md p-8 relative z-10 rounded-3xl shadow-2xl border border-gray-200/50 flex flex-col items-center">

      <!-- Header -->
      <div class="mb-8 text-center">
        <h1 class="text-2xl font-bold text-gray-900 tracking-wide mb-1">NEW CANDIDATE</h1>
        <p class="text-xs text-gray-500 font-mono tracking-widest uppercase">Initialize Profile Sequence</p>
      </div>

      <form class="w-full space-y-5" @submit.prevent="handleRegister">
        <div class="space-y-4">
          <div>
            <label for="username"
              class="block text-xs font-mono text-gray-600 mb-1.5 uppercase tracking-wider">Username</label>
            <input v-model="username" id="username" type="text" required
              class="input-glass w-full text-center tracking-wider"
              placeholder="CHOOSE IDENTIFIER">
          </div>
          <div>
            <label for="password"
              class="block text-xs font-mono text-gray-600 mb-1.5 uppercase tracking-wider">Password</label>
            <input v-model="password" id="password" type="password" required
              class="input-glass w-full text-center tracking-wider"
              placeholder="••••••••">
          </div>
          <div>
            <label for="confirm-password"
              class="block text-xs font-mono text-gray-600 mb-1.5 uppercase tracking-wider">Confirm
              Password</label>
            <input v-model="confirmPassword" id="confirm-password" type="password" required
              class="input-glass w-full text-center tracking-wider"
              placeholder="••••••••">
          </div>
        </div>

        <!-- Error Message -->
        <div v-if="error" class="p-3 rounded-lg bg-red-50 border border-red-200">
          <p class="text-sm text-red-600 text-center">{{ error }}</p>
        </div>

        <div>
          <button type="submit" :disabled="isLoading"
            class="group btn-neumorphic w-full py-3 flex justify-center items-center font-bold text-sm tracking-wide">
            <span v-if="isLoading"
              class="w-4 h-4 border-2 border-eling-dark/30 border-t-eling-dark rounded-full animate-spin mr-2"></span>
            {{ isLoading ? 'REGISTERING...' : 'CREATE PROFILE' }}
          </button>
        </div>

        <!-- Footer -->
        <div class="mt-6 text-center">
          <p class="text-xs text-gray-500 font-mono">
            Existing candidate?
            <router-link to="/login" class="text-eling-emerald hover:underline ml-1 font-semibold">Access Terminal</router-link>
          </p>
        </div>
      </form>
    </div>
  </div>
</template>
