<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { ElMessage, ElMessageBox } from 'element-plus';
import AdminLayout from '@/components/templates/AdminLayout.vue';
import BaseModal from '@/components/molecules/BaseModal.vue';
import BaseButton from '@/components/atoms/BaseButton.vue';
import CreateEventModal from '@/components/organisms/CreateEventModal.vue';
import CreateUserModal from '@/components/organisms/CreateUserModal.vue';

// Component imports
import CandidateList from './components/CandidateList.vue';
import ScheduleList from './components/ScheduleList.vue';
import ToolEditor from './components/ToolEditor.vue';
import TestResults from './components/TestResults.vue';
import ProfileSettings from './components/ProfileSettings.vue';
import EventDetailModal from './components/EventDetailModal.vue';
import CameraMonitoring from './components/CameraMonitoring.vue';
import UserManagement from './components/UserManagement.vue';

// Types
interface User {
    id: number;
    username: string;
    role: string;
    email?: string;
}

interface Event {
    id: number;
    event_name: string;
    status: string;
}

interface Tool {
    id: number;
    name: string;
    description: string;
}

// State
const users = ref<User[]>([]);
const events = ref<Event[]>([]);
const tools = ref<Tool[]>([]);
const currentView = ref<string>('dashboard');

// Modals
const showLogoutConfirm = ref(false);
const showCreateEventModal = ref(false);
const showAddUserModal = ref(false);
const showEventDetailModal = ref(false);
const selectedEventId = ref<number | null>(null);

// Fetch data
async function fetchData() {
    try {
        users.value = await invoke<User[]>('get_all_users');
        events.value = await invoke<Event[]>('get_events');
        tools.value = await invoke<Tool[]>('get_tools');
    } catch (e) {
        console.error('Failed to fetch data:', e);
    }
}

// Event handlers
function handleLogout() {
    showLogoutConfirm.value = true;
}

function confirmLogout() {
    showLogoutConfirm.value = false;
    localStorage.removeItem('user_session');
    window.location.href = '/login';
}

function goToProfile() {
    currentView.value = 'profile';
}

function viewEventDetails(eventId: number) {
    selectedEventId.value = eventId;
    showEventDetailModal.value = true;
}

function closeEventDetailModal() {
    showEventDetailModal.value = false;
    selectedEventId.value = null;
    fetchData();
}

const isCreatingEvent = ref(false);

async function handleCreateEvent(data: { name: string; date: string; description: string; toolIds: number[] }) {
    console.log('handleCreateEvent called with data:', data);
    
    isCreatingEvent.value = true;
    
    try {
        console.log('Calling backend create_event command...');
        
        const result = await invoke('create_event', {
            name: data.name,
            eventDate: data.date,
            description: data.description,
            toolIds: data.toolIds
        });

        console.log('Backend response:', result);
        
        ElMessage({
            message: 'Event created successfully!',
            type: 'success',
            duration: 3000
        });
        
        showCreateEventModal.value = false;
        await fetchData();
    } catch (e) {
        console.error('Create Event Error:', e);
        
        ElMessage({
            message: `Failed to create event: ${e}`,
            type: 'error',
            duration: 5000
        });
    } finally {
        isCreatingEvent.value = false;
    }
}

const isDeleting = ref(false);

async function deleteEvents(ids: number[]) {
    try {
        await ElMessageBox.confirm(
            `Are you sure you want to delete ${ids.length} event(s)? This action cannot be undone.`,
            'Confirm Deletion',
            {
                confirmButtonText: 'Delete',
                cancelButtonText: 'Cancel',
                type: 'warning',
            }
        );

        isDeleting.value = true;
        await invoke('delete_events', { eventIds: ids });
        
        ElMessage.success(`Successfully deleted ${ids.length} event(s)`);
        await fetchData();
        // We need a way to tell ScheduleList to clear selection, but fetching data might be enough 
        // if ScheduleList watches events or we rely on re-rendering. 
        // For now, let's assume valid reactivity.
    } catch (error) {
        if (error !== 'cancel') {
            console.error('Failed to delete events:', error);
            ElMessage.error(`Failed to delete events: ${error}`);
        }
    } finally {
        isDeleting.value = false;
    }
}

async function handleCreateUser(data: { username: string; password: string; role: string }) {
    try {
        await invoke('register_user', {
            username: data.username,
            password: data.password,
            role: data.role
        });

        alert(`User ${data.username} created successfully!`);
        showAddUserModal.value = false;
        await fetchData();
    } catch (e) {
        console.error('Create User Error:', e);
        alert('Failed to create user: ' + e);
    }
}

onMounted(fetchData);
</script>

<template>
    <AdminLayout 
        :current-view="currentView"
        @update:current-view="currentView = $event"
        @logout="handleLogout"
        @profile="goToProfile"
    >
        <!-- Dashboard View -->
        <div v-if="currentView === 'dashboard'" class="space-y-6">
            <div class="flex justify-between items-end mb-8">
                <div>
                    <h2 class="text-2xl font-bold text-gray-900 dark:text-gray-100">Dashboard Overview</h2>
                    <p class="text-sm text-gray-900 dark:text-gray-300/50 mt-1">System performance and operational metrics.</p>
                </div>
                <span class="text-xs font-mono text-eling-emerald/70 border border-eling-emerald/20 px-2 py-1 rounded">
                    SYSTEM STATUS: ONLINE
                </span>
            </div>

            <!-- Stats Cards (Gemini Design) -->
            <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
                <!-- Candidates Card -->
                <div class="glass-panel p-6 border-l-4 border-l-blue-500 relative overflow-hidden group">
                    <div class="absolute right-0 top-0 p-4 opacity-10 group-hover:opacity-20 transition-opacity">
                        <svg class="w-24 h-24" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z" />
                        </svg>
                    </div>
                    <h3 class="text-sm font-medium text-gray-900 dark:text-gray-300/70 uppercase tracking-wider">Total Candidates</h3>
                    <div class="mt-2 flex items-baseline">
                        <span class="text-4xl font-bold text-gray-900 dark:text-white">{{ users.filter(u => u.role !== 'admin').length }}</span>
                        <span class="ml-2 text-sm text-gray-900 dark:text-gray-300/40">Registered</span>
                    </div>
                    <div class="mt-4 text-xs text-blue-300 bg-blue-500/10 inline-block px-2 py-1 rounded">Active Personnel</div>
                </div>

                <!-- Schedule Card -->
                <div class="glass-panel p-6 border-l-4 border-l-green-500 relative overflow-hidden group">
                    <div class="absolute right-0 top-0 p-4 opacity-10 group-hover:opacity-20 transition-opacity">
                        <svg class="w-24 h-24" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
                        </svg>
                    </div>
                    <h3 class="text-sm font-medium text-gray-900 dark:text-gray-300/70 uppercase tracking-wider">Active Schedules</h3>
                    <div class="mt-2 flex items-baseline">
                        <span class="text-4xl font-bold text-gray-900 dark:text-white">{{ events.length }}</span>
                        <span class="ml-2 text-sm text-gray-900 dark:text-gray-300/40">Events</span>
                    </div>
                    <div class="mt-4 text-xs text-green-300 bg-green-500/10 inline-block px-2 py-1 rounded">Upcoming Sessions</div>
                </div>

                <!-- Tools Card -->
                <div class="glass-panel p-6 border-l-4 border-l-purple-500 relative overflow-hidden group">
                    <div class="absolute right-0 top-0 p-4 opacity-10 group-hover:opacity-20 transition-opacity">
                        <svg class="w-24 h-24" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
                        </svg>
                    </div>
                    <h3 class="text-sm font-medium text-gray-900 dark:text-gray-300/70 uppercase tracking-wider">Master Tools</h3>
                    <div class="mt-2 flex items-baseline">
                        <span class="text-4xl font-bold text-gray-900 dark:text-white">{{ tools.length }}</span>
                        <span class="ml-2 text-sm text-gray-900 dark:text-gray-300/40">Available</span>
                    </div>
                    <div class="mt-4 text-xs text-purple-300 bg-purple-500/10 inline-block px-2 py-1 rounded">Psychological Library</div>
                </div>
            </div>

            <!-- Quick Actions -->
            <div class="glass-panel p-5">
                <h3 class="text-xs font-bold text-gray-900 dark:text-gray-100 uppercase tracking-wider mb-3">Quick Actions</h3>
                <div class="grid grid-cols-2 md:grid-cols-4 gap-2">
                    <BaseButton size="md" @click="showCreateEventModal = true">
                        <svg class="w-5 h-5 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
                        </svg>
                        New Event
                    </BaseButton>
                    <BaseButton size="md" @click="showAddUserModal = true">
                        <svg class="w-5 h-5 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M18 9v3m0 0v3m0-3h3m-3 0h-3m-2-5a4 4 0 11-8 0 4 4 0 018 0zM3 20a6 6 0 0112 0v1H3v-1z" />
                        </svg>
                        Add User
                    </BaseButton>
                    <BaseButton size="md" @click="currentView = 'results'">
                        <svg class="w-5 h-5 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 17v-2m3 2v-4m3 4v-6m2 10H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
                        </svg>
                        Reports
                    </BaseButton>
                    <BaseButton size="md" @click="currentView = 'profile'">
                        <svg class="w-5 h-5 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
                        </svg>
                        Settings
                    </BaseButton>
                </div>
            </div>
        </div>

        <!-- Candidates View -->
        <div v-else-if="currentView === 'candidates'">
            <div class="mb-6">
                <h2 class="text-2xl font-bold text-gray-900 dark:text-gray-100">Candidate Management</h2>
                <p class="text-sm text-gray-900 dark:text-gray-300/50 mt-1">Search, filter, and manage registered candidates.</p>
            </div>
            <CandidateList :users="users" @refresh="fetchData" />
        </div>

        <!-- Schedule View -->
        <div v-else-if="currentView === 'schedule'">
            <div class="mb-6">
                <h2 class="text-2xl font-bold text-gray-900 dark:text-gray-100">Assessment Schedule</h2>
                <p class="text-sm text-gray-900 dark:text-gray-300/50 mt-1">Manage testing sessions and timelines.</p>
            </div>
            <ScheduleList 
                :events="events" 
                :loading="isDeleting"
                @create="showCreateEventModal = true" 
                @viewDetails="viewEventDetails" 
                @delete="deleteEvents" 
            />
        </div>

        <!-- Test Results View -->
        <div v-else-if="currentView === 'results'">
            <div class="mb-6">
                <h2 class="text-2xl font-bold text-gray-900 dark:text-gray-100">Test Results</h2>
                <p class="text-sm text-gray-900 dark:text-gray-300/50 mt-1">View and manage candidate assessment results.</p>
            </div>
            <TestResults />
        </div>

        <!-- Profile View -->
        <div v-else-if="currentView === 'profile'">
            <div class="mb-6">
                <h2 class="text-2xl font-bold text-gray-900 dark:text-gray-100">Profile Settings</h2>
                <p class="text-sm text-gray-900 dark:text-gray-300/50 mt-1">Manage your account settings and preferences.</p>
            </div>
            <ProfileSettings />
        </div>

        <!-- Camera Monitoring View -->
        <div v-else-if="currentView === 'monitoring'">
            <CameraMonitoring />
        </div>

        <!-- User Management View -->
        <div v-else-if="currentView === 'user_management'">
            <div class="mb-6">
                <h2 class="text-2xl font-bold text-gray-900 dark:text-gray-100">User Management</h2>
                <p class="text-sm text-gray-900 dark:text-gray-300/50 mt-1">Manage system administrators, proctors, and other users.</p>
            </div>
            <UserManagement :users="users" @refresh="fetchData" />
        </div>

        <!-- Tool Editor Views -->
        <div v-else-if="currentView.startsWith('tool_')">
            <ToolEditor :tool-id="parseInt(currentView.replace('tool_', ''))" />
        </div>

        <!-- Logout Confirmation Modal -->
        <BaseModal :show="showLogoutConfirm" title="Confirm Logout" size="md" @close="showLogoutConfirm = false">
            <div class="text-center py-4">
                <div class="w-16 h-16 rounded-full bg-red-500/10 flex items-center justify-center mx-auto mb-4">
                    <svg class="w-8 h-8 text-red-600 dark:text-red-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1" />
                    </svg>
                </div>
                <h3 class="text-lg font-bold text-gray-900 dark:text-gray-100 mb-2">Are you sure?</h3>
                <p class="text-sm text-gray-600 dark:text-gray-300/70">You will be signed out of your account.</p>
            </div>
            <template #footer>
                <div class="flex gap-3">
                    <BaseButton variant="ghost" size="md" @click="showLogoutConfirm = false" class="flex-1">
                        Cancel
                    </BaseButton>
                    <BaseButton variant="danger" size="md" @click="confirmLogout" class="flex-1">
                        Logout
                    </BaseButton>
                </div>
            </template>
        </BaseModal>

        <!-- Create Event Modal -->
        <CreateEventModal 
            :show="showCreateEventModal"
            :tools="tools"
            @close="showCreateEventModal = false"
            @create="handleCreateEvent"
        />

        <!-- Create User Modal -->
        <CreateUserModal 
            :show="showAddUserModal"
            @close="showAddUserModal = false"
            @create="handleCreateUser"
        />

        <!-- Event Detail Modal -->
        <EventDetailModal 
            v-if="showEventDetailModal && selectedEventId" 
            :eventId="selectedEventId" 
            :allUsers="users"
            @close="closeEventDetailModal" 
            @refresh="fetchData" 
        />
    </AdminLayout>
</template>
