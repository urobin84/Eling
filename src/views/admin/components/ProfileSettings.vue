<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';

interface UserProfile {
    id: number;
    username: string;
    email: string | null;
    role: string;
    created_at: string;
}

const profile = ref<UserProfile>({
    id: 1,
    username: 'admin',
    email: 'admin@eling.id',
    role: 'admin',
    created_at: new Date().toISOString()
});

const isEditing = ref(false);
const isSaving = ref(false);
const editForm = ref({
    username: '',
    email: '',
    currentPassword: '',
    newPassword: '',
    confirmPassword: ''
});

const showPasswordSection = ref(false);

function startEdit() {
    editForm.value.username = profile.value.username;
    editForm.value.email = profile.value.email || '';
    editForm.value.currentPassword = '';
    editForm.value.newPassword = '';
    editForm.value.confirmPassword = '';
    isEditing.value = true;
}

function cancelEdit() {
    isEditing.value = false;
    showPasswordSection.value = false;
}

async function saveProfile() {
    // Validate
    if (!editForm.value.username.trim()) {
        alert('Username is required');
        return;
    }

    if (showPasswordSection.value) {
        if (!editForm.value.currentPassword) {
            alert('Current password is required to change password');
            return;
        }
        if (editForm.value.newPassword !== editForm.value.confirmPassword) {
            alert('New passwords do not match');
            return;
        }
        if (editForm.value.newPassword.length < 6) {
            alert('New password must be at least 6 characters');
            return;
        }
    }

    isSaving.value = true;
    try {
        // TODO: Call backend API to update profile
        // await invoke('update_user_profile', {
        //     username: editForm.value.username,
        //     email: editForm.value.email,
        //     currentPassword: editForm.value.currentPassword,
        //     newPassword: editForm.value.newPassword
        // });

        // Simulate API call
        await new Promise(resolve => setTimeout(resolve, 500));

        profile.value.username = editForm.value.username;
        profile.value.email = editForm.value.email;

        alert('Profile updated successfully!');
        isEditing.value = false;
        showPasswordSection.value = false;
    } catch (e) {
        console.error('Failed to update profile:', e);
        alert('Failed to update profile: ' + e);
    } finally {
        isSaving.value = false;
    }
}

function formatDate(dateString: string) {
    const date = new Date(dateString);
    return date.toLocaleDateString('en-US', {
        year: 'numeric',
        month: 'long',
        day: 'numeric'
    });
}

onMounted(() => {
    // TODO: Fetch actual user profile from backend
    // const userProfile = await invoke<UserProfile>('get_current_user_profile');
    // profile.value = userProfile;
});
</script>

<template>
    <div class="max-w-4xl mx-auto">
        <!-- Header -->
        <div class="mb-8">
            <h2 class="text-2xl font-bold text-gray-900 dark:text-eling-dark-text">Profile Settings</h2>
            <p class="text-sm text-gray-600 dark:text-eling-dark-text/60 mt-1">Manage your account information and
                preferences</p>
        </div>

        <!-- Profile Card -->
        <div class="glass-panel p-8 border-black/5 dark:border-white/5">
            <!-- Avatar Section -->
            <div class="flex items-start gap-6 mb-8 pb-8 border-b border-black/10 dark:border-white/10">
                <div
                    class="w-24 h-24 rounded-full bg-gradient-to-br from-eling-emerald to-green-400 flex items-center justify-center text-eling-dark font-bold text-3xl shadow-lg">
                    {{ profile.username.charAt(0).toUpperCase() }}
                </div>
                <div class="flex-1">
                    <h3 class="text-xl font-bold text-gray-900 dark:text-eling-dark-text">{{ profile.username }}</h3>
                    <p class="text-sm text-gray-600 dark:text-eling-dark-text/60 mt-1">{{ profile.email }}</p>
                    <div class="flex items-center gap-2 mt-3">
                        <span
                            class="px-3 py-1 rounded-full text-xs font-bold uppercase tracking-wide bg-eling-emerald/10 text-eling-emerald border border-eling-emerald/20">
                            {{ profile.role }}
                        </span>
                        <span class="text-xs text-gray-500 dark:text-eling-dark-text/40">
                            Member since {{ formatDate(profile.created_at) }}
                        </span>
                    </div>
                </div>
                <button v-if="!isEditing" @click="startEdit"
                    class="btn-neumorphic px-4 py-2 text-sm flex items-center gap-2">
                    <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                            d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
                    </svg>
                    Edit Profile
                </button>
            </div>

            <!-- View Mode -->
            <div v-if="!isEditing" class="space-y-6">
                <!-- Account Information -->
                <div>
                    <h4 class="text-sm font-bold text-gray-900 dark:text-eling-dark-text/80 mb-4 uppercase tracking-wider">
                        Account Information</h4>
                    <div class="grid grid-cols-2 gap-6">
                        <div
                            class="bg-black/5 dark:bg-white/5 rounded-xl p-4 border border-black/10 dark:border-white/10">
                            <p class="text-xs text-gray-500 dark:text-eling-dark-text/50 mb-1">Username</p>
                            <p class="text-sm font-medium text-gray-900 dark:text-eling-dark-text">{{ profile.username }}
                            </p>
                        </div>
                        <div
                            class="bg-black/5 dark:bg-white/5 rounded-xl p-4 border border-black/10 dark:border-white/10">
                            <p class="text-xs text-gray-500 dark:text-eling-dark-text/50 mb-1">Email</p>
                            <p class="text-sm font-medium text-gray-900 dark:text-eling-dark-text">{{ profile.email || 'Not set' }}</p>
                        </div>
                        <div
                            class="bg-black/5 dark:bg-white/5 rounded-xl p-4 border border-black/10 dark:border-white/10">
                            <p class="text-xs text-gray-500 dark:text-eling-dark-text/50 mb-1">Role</p>
                            <p class="text-sm font-medium text-gray-900 dark:text-eling-dark-text capitalize">{{
                                profile.role }}</p>
                        </div>
                        <div
                            class="bg-black/5 dark:bg-white/5 rounded-xl p-4 border border-black/10 dark:border-white/10">
                            <p class="text-xs text-gray-500 dark:text-eling-dark-text/50 mb-1">User ID</p>
                            <p class="text-sm font-medium text-gray-900 dark:text-eling-dark-text font-mono">#{{
                                profile.id.toString().padStart(4, '0') }}</p>
                        </div>
                    </div>
                </div>

                <!-- Security -->
                <div>
                    <h4 class="text-sm font-bold text-gray-900 dark:text-eling-dark-text/80 mb-4 uppercase tracking-wider">
                        Security</h4>
                    <div class="bg-black/5 dark:bg-white/5 rounded-xl p-4 border border-black/10 dark:border-white/10">
                        <div class="flex items-center justify-between">
                            <div>
                                <p class="text-sm font-medium text-gray-900 dark:text-eling-dark-text">Password</p>
                                <p class="text-xs text-gray-500 dark:text-eling-dark-text/50 mt-1">Last changed: Never</p>
                            </div>
                            <span class="text-xs text-gray-900 dark:text-eling-dark-text/60 font-mono">••••••••</span>
                        </div>
                    </div>
                </div>
            </div>

            <!-- Edit Mode -->
            <div v-else class="space-y-6">
                <!-- Basic Information -->
                <div>
                    <h4 class="text-sm font-bold text-gray-900 dark:text-eling-dark-text/80 mb-4 uppercase tracking-wider">
                        Basic Information</h4>
                    <div class="space-y-4">
                        <div>
                            <label class="block text-xs font-medium text-gray-700 dark:text-eling-dark-text/70 mb-2">
                                Username *
                            </label>
                            <input v-model="editForm.username" type="text"
                                class="input-glass w-full"
                                placeholder="Enter username">
                        </div>
                        <div>
                            <label class="block text-xs font-medium text-gray-700 dark:text-eling-dark-text/70 mb-2">
                                Email
                            </label>
                            <input v-model="editForm.email" type="email"
                                class="input-glass w-full"
                                placeholder="Enter email address">
                        </div>
                    </div>
                </div>

                <!-- Change Password Section -->
                <div>
                    <div class="flex items-center justify-between mb-4">
                        <h4 class="text-sm font-bold text-gray-900 dark:text-eling-dark-text/80 uppercase tracking-wider">
                            Change Password</h4>
                        <button @click="showPasswordSection = !showPasswordSection"
                            class="text-xs text-eling-emerald hover:text-eling-emerald/80 font-medium">
                            {{ showPasswordSection ? 'Cancel' : 'Change Password' }}
                        </button>
                    </div>

                    <div v-if="showPasswordSection" class="space-y-4 animate-fade-in">
                        <div>
                            <label class="block text-xs font-medium text-gray-700 dark:text-eling-dark-text/70 mb-2">
                                Current Password *
                            </label>
                            <input v-model="editForm.currentPassword" type="password"
                                class="input-glass w-full"
                                placeholder="Enter current password">
                        </div>
                        <div>
                            <label class="block text-xs font-medium text-gray-700 dark:text-eling-dark-text/70 mb-2">
                                New Password *
                            </label>
                            <input v-model="editForm.newPassword" type="password"
                                class="input-glass w-full"
                                placeholder="Enter new password (min. 6 characters)">
                        </div>
                        <div>
                            <label class="block text-xs font-medium text-gray-700 dark:text-eling-dark-text/70 mb-2">
                                Confirm New Password *
                            </label>
                            <input v-model="editForm.confirmPassword" type="password"
                                class="input-glass w-full"
                                placeholder="Confirm new password">
                        </div>
                    </div>
                </div>

                <!-- Action Buttons -->
                <div class="flex justify-end gap-3 pt-4 border-t border-black/10 dark:border-white/10">
                    <button @click="cancelEdit" :disabled="isSaving"
                        class="px-6 py-2 rounded-lg text-sm text-gray-700 dark:text-eling-dark-text/70 hover:bg-black/5 dark:hover:bg-white/5 transition-colors disabled:opacity-50">
                        Cancel
                    </button>
                    <button @click="saveProfile" :disabled="isSaving"
                        class="btn-neumorphic px-6 py-2 text-sm font-medium disabled:opacity-50 flex items-center gap-2">
                        <svg v-if="isSaving" class="animate-spin h-4 w-4" fill="none" viewBox="0 0 24 24">
                            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4">
                            </circle>
                            <path class="opacity-75" fill="currentColor"
                                d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z">
                            </path>
                        </svg>
                        {{ isSaving ? 'Saving...' : 'Save Changes' }}
                    </button>
                </div>
            </div>
        </div>
    </div>
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
