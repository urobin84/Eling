<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';

interface Notification {
    id: number;
    title: string;
    message: string;
    type: string;
    related_session_id?: number;
    related_user_id?: number;
    is_read: boolean;
    created_at: string;
}

const notifications = ref<Notification[]>([]);
const showDropdown = ref(false);
const isLoading = ref(false);

let pollInterval: number | null = null;

async function fetchNotifications() {
    try {
        const data = await invoke<Notification[]>('get_notifications');
        notifications.value = data;
    } catch (e) {
        console.error('Failed to fetch notifications:', e);
    }
}

async function markAsRead(notificationId: number) {
    try {
        await invoke('mark_notification_read', { notificationId });
        await fetchNotifications();
    } catch (e) {
        console.error('Failed to mark notification as read:', e);
    }
}

async function markAllAsRead() {
    isLoading.value = true;
    try {
        await invoke('mark_all_notifications_read');
        await fetchNotifications();
        showDropdown.value = false;
    } catch (e) {
        console.error('Failed to mark all as read:', e);
    } finally {
        isLoading.value = false;
    }
}

function getNotificationIcon(type: string) {
    switch (type) {
        case 'test_completed':
            return 'M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z';
        case 'session_started':
            return 'M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z';
        case 'alert':
            return 'M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z';
        default:
            return 'M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z';
    }
}

function formatTime(dateString: string) {
    const date = new Date(dateString);
    const now = new Date();
    const diff = now.getTime() - date.getTime();
    const minutes = Math.floor(diff / 60000);
    const hours = Math.floor(minutes / 60);
    const days = Math.floor(hours / 24);

    if (minutes < 1) return 'Just now';
    if (minutes < 60) return `${minutes}m ago`;
    if (hours < 24) return `${hours}h ago`;
    return `${days}d ago`;
}

onMounted(() => {
    fetchNotifications();
    // Poll every 30 seconds
    pollInterval = window.setInterval(fetchNotifications, 30000);
});

onUnmounted(() => {
    if (pollInterval) {
        clearInterval(pollInterval);
    }
});
</script>

<template>
    <div class="relative">
        <!-- Bell Button -->
        <button @click="showDropdown = !showDropdown"
            class="relative p-2 rounded-lg hover:bg-black/5 dark:hover:bg-white/5 transition-colors group">
            <svg class="w-5 h-5 text-gray-900 dark:text-eling-light/70 group-hover:text-gray-900 dark:group-hover:text-eling-light"
                fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                    d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9" />
            </svg>
            <!-- Badge -->
            <span v-if="notifications.length > 0"
                class="absolute top-1 right-1 min-w-[18px] h-[18px] bg-red-500 text-white text-[10px] font-bold rounded-full flex items-center justify-center px-1 border-2 border-white dark:border-eling-dark">
                {{ notifications.length > 9 ? '9+' : notifications.length }}
            </span>
        </button>

        <!-- Dropdown -->
        <div v-if="showDropdown"
            class="absolute right-0 mt-2 w-96 bg-white dark:bg-eling-surface rounded-xl shadow-2xl border border-black/10 dark:border-white/10 z-[9999] overflow-hidden">
            <!-- Header -->
            <div class="flex items-center justify-between px-4 py-3 border-b border-black/10 dark:border-white/10">
                <h3 class="text-sm font-bold text-gray-900 dark:text-eling-light">Notifications</h3>
                <button v-if="notifications.length > 0" @click="markAllAsRead" :disabled="isLoading"
                    class="text-xs text-eling-accent hover:text-eling-accent/80 font-medium disabled:opacity-50">
                    Mark all as read
                </button>
            </div>

            <!-- Notifications List -->
            <div class="max-h-96 overflow-y-auto">
                <div v-if="notifications.length === 0" class="px-4 py-12 text-center">
                    <svg class="w-12 h-12 mx-auto text-gray-400 dark:text-eling-light/20 mb-3" fill="none"
                        viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                            d="M20 13V6a2 2 0 00-2-2H6a2 2 0 00-2 2v7m16 0v5a2 2 0 01-2 2H6a2 2 0 01-2-2v-5m16 0h-2.586a1 1 0 00-.707.293l-2.414 2.414a1 1 0 01-.707.293h-3.172a1 1 0 01-.707-.293l-2.414-2.414A1 1 0 006.586 13H4" />
                    </svg>
                    <p class="text-sm text-gray-500 dark:text-eling-light/50">No new notifications</p>
                </div>

                <div v-for="notification in notifications" :key="notification.id" @click="markAsRead(notification.id)"
                    class="px-4 py-3 hover:bg-black/5 dark:hover:bg-white/5 cursor-pointer transition-colors border-b border-black/5 dark:border-white/5 last:border-b-0">
                    <div class="flex items-start gap-3">
                        <!-- Icon -->
                        <div
                            class="flex-shrink-0 w-8 h-8 rounded-full bg-eling-accent/10 flex items-center justify-center">
                            <svg class="w-4 h-4 text-eling-accent" fill="none" viewBox="0 0 24 24"
                                stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                    :d="getNotificationIcon(notification.type)" />
                            </svg>
                        </div>

                        <!-- Content -->
                        <div class="flex-1 min-w-0">
                            <p class="text-sm font-medium text-gray-900 dark:text-eling-light">{{ notification.title }}
                            </p>
                            <p class="text-xs text-gray-600 dark:text-eling-light/60 mt-0.5">{{ notification.message }}
                            </p>
                            <p class="text-xs text-gray-500 dark:text-eling-light/40 mt-1">{{
                                formatTime(notification.created_at) }}</p>
                        </div>

                        <!-- Unread indicator -->
                        <div class="flex-shrink-0">
                            <span class="w-2 h-2 bg-eling-accent rounded-full block"></span>
                        </div>
                    </div>
                </div>
            </div>
        </div>

        <!-- Backdrop -->
        <div v-if="showDropdown" @click="showDropdown = false" class="fixed inset-0 z-[9998]"></div>
    </div>
</template>
