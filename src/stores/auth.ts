import { defineStore } from 'pinia';
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';

export interface User {
    id: number;
    username: string;
    email: string | null;
    role: string;
    avatar_url?: string;
    created_at: string;
}

export const useAuthStore = defineStore('auth', () => {
    const user = ref<User | null>(null);
    const isLoading = ref(false);

    async function fetchUser(username: string = 'admin') {
        isLoading.value = true;
        try {
            const result = await invoke<User>('get_user_profile', { username });
            user.value = result;
        } catch (e) {
            console.error('Failed to fetch user:', e);
        } finally {
            isLoading.value = false;
        }
    }

    function updateUser(userData: Partial<User>) {
        if (user.value) {
            user.value = { ...user.value, ...userData };
        }
    }

    return {
        user,
        isLoading,
        fetchUser,
        updateUser
    };
});
