<script setup lang="ts">
import SideBar from '../organisms/SideBar.vue';
import NavBar from '../organisms/NavBar.vue';

interface Props {
    currentView: string;
}

defineProps<Props>();

const emit = defineEmits<{
    'update:currentView': [view: string];
    logout: [];
    profile: [];
}>();
</script>

<template>
    <div class="flex h-screen overflow-hidden bg-eling-light-bg dark:bg-eling-dark-bg transition-colors duration-300">
        <!-- Sidebar -->
        <SideBar 
            :current-view="currentView"
            @update:current-view="emit('update:currentView', $event)"
        />

        <!-- Main Content -->
        <main class="flex-1 overflow-y-auto bg-eling-light-bg dark:bg-eling-dark-bg transition-colors duration-300">
            <!-- Header Bar -->
            <NavBar 
                @logout="emit('logout')"
                @profile="emit('profile')"
            />

            <!-- Page Content -->
            <div class="p-8">
                <slot />
            </div>
        </main>
    </div>
</template>
