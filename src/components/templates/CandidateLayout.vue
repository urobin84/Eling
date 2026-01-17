<script setup lang="ts">
import CandidateNavBar from '@/components/organisms/CandidateNavBar.vue';

defineProps<{
  username: string;
}>();

defineEmits<{
  logout: [];
  info: [];
  profile: [];
}>();
</script>

<template>
  <div class="min-h-screen bg-gradient-to-br from-gray-50 to-gray-100 dark:from-eling-dark dark:to-eling-dark-bg transition-colors">
    <CandidateNavBar :username="username" @logout="$emit('logout')" @info="$emit('info')" @profile="$emit('profile')" />
    
    <main class="max-w-7xl mx-auto py-12 px-4 sm:px-6 lg:px-8">
      <slot />
    </main>
  </div>
</template>
