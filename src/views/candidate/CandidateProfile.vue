<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import CandidateLayout from '@/components/templates/CandidateLayout.vue';
import BaseCard from '@/components/atoms/BaseCard.vue';
import BaseButton from '@/components/atoms/BaseButton.vue';
import BaseInput from '@/components/atoms/BaseInput.vue';
import { emitAvatarUpdate } from '@/utils/avatarEvents';

const router = useRouter();
const user = ref<any>(null);
const avatarUrl = ref<string>('');
const isUploading = ref(false);
const isSaving = ref(false);
const successMessage = ref('');

// Form fields
const username = ref('');
const email = ref('');
const fullName = ref('');

onMounted(() => {
  const sessionStr = localStorage.getItem('user_session');
  if (sessionStr) {
    try {
      user.value = JSON.parse(sessionStr);
      username.value = user.value.username || '';
      
      // Load avatar from localStorage
      const savedAvatar = localStorage.getItem(`avatar_${user.value.id}`);
      if (savedAvatar) {
        avatarUrl.value = savedAvatar;
      }
    } catch (e) {
      router.push('/login');
    }
  } else {
    router.push('/login');
  }
});

function goBack() {
  router.push('/dashboard');
}

function logout() {
  localStorage.removeItem('user_session');
  window.location.href = '/login';
}

function goToInfo() {
  router.push('/info');
}

function handleAvatarUpload(event: Event) {
  const target = event.target as HTMLInputElement;
  const file = target.files?.[0];
  
  if (file) {
    // Check file size (max 2MB)
    if (file.size > 2 * 1024 * 1024) {
      alert('File size must be less than 2MB');
      return;
    }

    // Check file type
    if (!file.type.startsWith('image/')) {
      alert('Please upload an image file');
      return;
    }

    isUploading.value = true;
    
    const reader = new FileReader();
    reader.onload = (e) => {
      avatarUrl.value = e.target?.result as string;
      // Save to localStorage
      if (user.value) {
        localStorage.setItem(`avatar_${user.value.id}`, avatarUrl.value);
        // Emit event to update navbar
        emitAvatarUpdate();
      }
      isUploading.value = false;
      successMessage.value = 'Avatar updated successfully!';
      setTimeout(() => successMessage.value = '', 3000);
    };
    reader.readAsDataURL(file);
  }
}

function removeAvatar() {
  avatarUrl.value = '';
  if (user.value) {
    localStorage.removeItem(`avatar_${user.value.id}`);
    // Emit event to update navbar
    emitAvatarUpdate();
  }
  successMessage.value = 'Avatar removed successfully!';
  setTimeout(() => successMessage.value = '', 3000);
}

async function saveProfile() {
  isSaving.value = true;
  
  try {
    // In future, call backend API to update profile
    // For now, just update localStorage
    if (user.value) {
      user.value.username = username.value;
      user.value.email = email.value;
      user.value.fullName = fullName.value;
      localStorage.setItem('user_session', JSON.stringify(user.value));
    }
    
    successMessage.value = 'Profile updated successfully!';
    setTimeout(() => successMessage.value = '', 3000);
  } catch (e) {
    console.error('Failed to save profile:', e);
  } finally {
    isSaving.value = false;
  }
}
</script>

<template>
  <CandidateLayout :username="user?.username || ''" @logout="logout" @info="goToInfo">
    <div class="max-w-4xl mx-auto">
      <!-- Header -->
      <div class="mb-8">
        <button @click="goBack" class="flex items-center gap-2 text-gray-600 dark:text-gray-400 hover:text-eling-emerald transition-colors mb-4">
          <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18" />
          </svg>
          Back to Dashboard
        </button>
        <h1 class="text-3xl font-bold text-gray-900 dark:text-white mb-2">Profile Settings</h1>
        <p class="text-gray-600 dark:text-gray-300">Manage your account information and avatar</p>
      </div>

      <!-- Success Message -->
      <div v-if="successMessage" class="mb-6 p-4 bg-green-50 dark:bg-green-500/10 border border-green-200 dark:border-green-500/20 rounded-xl flex items-center gap-3 animate-fade-in">
        <svg class="w-5 h-5 text-green-600 dark:text-green-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
        </svg>
        <span class="text-green-800 dark:text-green-200 font-medium">{{ successMessage }}</span>
      </div>

      <div class="grid md:grid-cols-3 gap-6">
        <!-- Avatar Section -->
        <div class="md:col-span-1">
          <BaseCard padding="lg">
            <h2 class="text-lg font-bold text-gray-900 dark:text-white mb-4">Profile Picture</h2>
            
            <div class="flex flex-col items-center">
              <!-- Avatar Display -->
              <div class="relative group mb-4">
                <div v-if="avatarUrl" class="w-32 h-32 rounded-full overflow-hidden border-4 border-gray-200 dark:border-gray-700 shadow-lg">
                  <img :src="avatarUrl" alt="Avatar" class="w-full h-full object-cover" />
                </div>
                <div v-else class="w-32 h-32 rounded-full bg-gradient-to-br from-eling-emerald to-cyan-500 flex items-center justify-center text-white text-4xl font-bold border-4 border-gray-200 dark:border-gray-700 shadow-lg">
                  {{ username.charAt(0).toUpperCase() }}
                </div>
                
                <!-- Upload Overlay -->
                <label class="absolute inset-0 flex items-center justify-center bg-black/50 rounded-full opacity-0 group-hover:opacity-100 transition-opacity cursor-pointer">
                  <svg class="w-8 h-8 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 9a2 2 0 012-2h.93a2 2 0 001.664-.89l.812-1.22A2 2 0 0110.07 4h3.86a2 2 0 011.664.89l.812 1.22A2 2 0 0018.07 7H19a2 2 0 012 2v9a2 2 0 01-2 2H5a2 2 0 01-2-2V9z" />
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 13a3 3 0 11-6 0 3 3 0 016 0z" />
                  </svg>
                  <input type="file" accept="image/*" @change="handleAvatarUpload" class="hidden" />
                </label>
              </div>

              <p class="text-sm text-gray-600 dark:text-gray-400 text-center mb-4">
                Click to upload a new photo<br />
                <span class="text-xs">Max size: 2MB</span>
              </p>

              <div class="flex gap-2 w-full">
                <label class="flex-1">
                  <BaseButton variant="ghost" size="sm" class="w-full">
                    <svg class="w-4 h-4 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-8l-4-4m0 0L8 8m4-4v12" />
                    </svg>
                    Upload
                  </BaseButton>
                  <input type="file" accept="image/*" @change="handleAvatarUpload" class="hidden" />
                </label>
                <BaseButton v-if="avatarUrl" @click="removeAvatar" variant="ghost" size="sm" class="text-red-600 dark:text-red-400">
                  Remove
                </BaseButton>
              </div>
            </div>
          </BaseCard>
        </div>

        <!-- Profile Information -->
        <div class="md:col-span-2">
          <BaseCard padding="lg">
            <h2 class="text-lg font-bold text-gray-900 dark:text-white mb-6">Personal Information</h2>
            
            <form @submit.prevent="saveProfile" class="space-y-6">
              <!-- Username -->
              <div>
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                  Username
                </label>
                <BaseInput
                  v-model="username"
                  type="text"
                  placeholder="Enter username"
                  required
                />
              </div>

              <!-- Full Name -->
              <div>
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                  Full Name
                </label>
                <BaseInput
                  v-model="fullName"
                  type="text"
                  placeholder="Enter your full name"
                />
              </div>

              <!-- Email -->
              <div>
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                  Email
                </label>
                <BaseInput
                  v-model="email"
                  type="email"
                  placeholder="Enter email address"
                />
              </div>

              <!-- Role (Read-only) -->
              <div>
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                  Role
                </label>
                <div class="px-4 py-3 bg-gray-100 dark:bg-gray-800 rounded-xl text-gray-600 dark:text-gray-400">
                  {{ user?.role || 'Candidate' }}
                </div>
              </div>

              <!-- Save Button -->
              <div class="flex justify-end gap-3 pt-4 border-t border-gray-200 dark:border-gray-700">
                <BaseButton @click="goBack" variant="ghost">
                  Cancel
                </BaseButton>
                <BaseButton type="submit" variant="primary" :disabled="isSaving">
                  <svg v-if="isSaving" class="animate-spin w-4 h-4 mr-2" fill="none" viewBox="0 0 24 24">
                    <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                    <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                  </svg>
                  {{ isSaving ? 'Saving...' : 'Save Changes' }}
                </BaseButton>
              </div>
            </form>
          </BaseCard>
        </div>
      </div>
    </div>
  </CandidateLayout>
</template>

<style scoped>
@keyframes fade-in {
  from {
    opacity: 0;
    transform: translateY(-10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.animate-fade-in {
  animation: fade-in 0.3s ease-out;
}
</style>
