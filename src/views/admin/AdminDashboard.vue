<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useRouter } from 'vue-router';
import CandidateList from './components/CandidateList.vue';
import ScheduleList from './components/ScheduleList.vue';
import ToolEditor from './components/ToolEditor.vue';
import TestResults from './components/TestResults.vue';
import NotificationBell from '../../components/NotificationBell.vue';
import ThemeToggle from '../../components/ThemeToggle.vue';
import ProfileSettings from './components/ProfileSettings.vue';
import EventDetailModal from './components/EventDetailModal.vue';

// Define types based on our Rust structs
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

const users = ref<User[]>([]);
const events = ref<Event[]>([]);
const tools = ref<Tool[]>([]);
const router = useRouter();

// View State
const currentView = ref<string>('dashboard');
const showLogoutConfirm = ref(false);

// Navigation Structure
interface NavItem {
  id: string;
  label: string;
  icon: string;
  children?: NavItem[];
  collapsed?: boolean;
}

const navItems = ref<NavItem[]>([
  { id: 'dashboard', label: 'Dashboard', icon: 'M4 6h16M4 12h16M4 18h16' },
  { id: 'schedule', label: 'Schedule', icon: 'M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z' },
  { id: 'candidates', label: 'Candidates', icon: 'M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z' },
  { id: 'results', label: 'Test Results', icon: 'M9 12l2 2 4-4M7.835 4.697a3.42 3.42 0 001.946-.806 3.42 3.42 0 014.438 0 3.42 3.42 0 001.946.806 3.42 3.42 0 013.138 3.138 3.42 3.42 0 00.806 1.946 3.42 3.42 0 010 4.438 3.42 3.42 0 00-.806 1.946 3.42 3.42 0 01-3.138 3.138 3.42 3.42 0 00-1.946.806 3.42 3.42 0 01-4.438 0 3.42 3.42 0 00-1.946-.806 3.42 3.42 0 01-3.138-3.138 3.42 3.42 0 00-.806-1.946 3.42 3.42 0 010-4.438 3.42 3.42 0 00.806-1.946 3.42 3.42 0 013.138-3.138z' },
  {
    id: 'master_tools',
    label: 'Master Tools Test',
    icon: 'M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10',
    collapsed: true,
    children: []  // Will be populated dynamically from database
  },
]);

function handleUserAction(action: string, payload: any) {
  if (action === 'View') {
    // View candidate details
    console.log('View candidate:', payload);
    alert(`Viewing details for: ${payload.username}\nID: ${payload.id}\nRole: ${payload.role}`);
  } else if (action === 'Edit') {
    // Edit candidate
    console.log('Edit candidate:', payload);
    alert(`Edit functionality for: ${payload.username}\n(To be implemented)`);
  } else if (action === 'Delete') {
    // Delete candidate with confirmation
    if (confirm(`Are you sure you want to delete candidate ID: ${payload}?`)) {
      console.log('Delete candidate ID:', payload);
      // TODO: Implement actual delete via invoke
      alert(`Delete functionality for ID: ${payload}\n(To be implemented)`);
      // After successful delete, refresh data
      // fetchData();
    }
  }
}

const showCreateEventModal = ref(false);
const newEventName = ref('');
const newEventDesc = ref('');
const selectedTools = ref<number[]>([]);
const newEventDate = ref('');

// Event Detail Modal
const showEventDetailModal = ref(false);
const selectedEventId = ref<number | null>(null);

function viewEventDetails(eventId: number) {
  selectedEventId.value = eventId;
  showEventDetailModal.value = true;
}

function closeEventDetailModal() {
  showEventDetailModal.value = false;
  selectedEventId.value = null;
  fetchData(); // Refresh data after modal closes
}
const newEventTime = ref('');
const showUserDropdown = ref(false);

// Tool categories for organized display
const toolCategories: Record<string, string[]> = {
  'Cognitive': ['TIU', 'IST', 'CFIT', 'MATRICES', 'WPT', 'GATB'],
  'Personality': ['EPPS', 'PAPI', 'DISC', 'MBTI', '16PF', 'HEXACO'],
  'Interest & Style': ['RMIB', 'RIASEC', 'MSDT'],
  'Speed Test': ['KRAEPELIN'],
  'Projective': ['WARTEGG', 'DAP', 'HTP', 'BAUM']
};

// Computed: organize tools by category for selection
const organizedTools = computed(() => {
  const result: { category: string; tools: Tool[] }[] = [];
  for (const [category, toolNames] of Object.entries(toolCategories)) {
    const categoryTools = tools.value.filter(t =>
      toolNames.some(name => t.name.toUpperCase().includes(name.toUpperCase()))
    );
    if (categoryTools.length > 0) {
      result.push({ category, tools: categoryTools });
    }
  }
  return result;
});

function toggleToolSelection(toolId: number) {
  const idx = selectedTools.value.indexOf(toolId);
  if (idx >= 0) {
    selectedTools.value.splice(idx, 1);
  } else {
    selectedTools.value.push(toolId);
  }
}

function isToolSelected(toolId: number): boolean {
  return selectedTools.value.includes(toolId);
}

function selectAllInCategory(categoryTools: Tool[]) {
  for (const tool of categoryTools) {
    if (!selectedTools.value.includes(tool.id)) {
      selectedTools.value.push(tool.id);
    }
  }
}

function deselectAllInCategory(categoryTools: Tool[]) {
  for (const tool of categoryTools) {
    const idx = selectedTools.value.indexOf(tool.id);
    if (idx >= 0) {
      selectedTools.value.splice(idx, 1);
    }
  }
}

async function fetchData() {
  try {
    const [usersData, eventsData, toolsData] = await Promise.all([
      invoke<User[]>('get_all_users'),
      invoke<Event[]>('get_events'),
      invoke<Tool[]>('get_tools')
    ]);

    users.value = usersData;
    events.value = eventsData;
    tools.value = toolsData;

    // Populate Master Tools menu dynamically
    const masterToolsItem = navItems.value.find(item => item.id === 'master_tools');
    if (masterToolsItem && toolsData.length > 0) {
      // Group by category
      const categories = new Map<string, Tool[]>();
      for (const [category, toolNames] of Object.entries(toolCategories)) {
        const categoryTools = toolsData.filter(t =>
          toolNames.some(name => t.name.toUpperCase().includes(name.toUpperCase()))
        );
        if (categoryTools.length > 0) {
          categories.set(category, categoryTools);
        }
      }

      // Build children array
      const children: NavItem[] = [];
      for (const [category, categoryTools] of categories) {
        // Add category header
        children.push({
          id: `cat_${category.toLowerCase().replace(/\s+/g, '_')}`,
          label: `── ${category} ──`,
          icon: ''
        });
        // Add tools
        for (const tool of categoryTools) {
          children.push({
            id: `tool_${tool.id}`,
            label: tool.name,
            icon: ''
          });
        }
      }

      masterToolsItem.children = children;
    }
  } catch (e) {
    console.error('Failed to fetch data:', e);
  }
}

// Helper to extract ID
function getToolId(viewId: string): number {
  return parseInt(viewId.replace('tool_', ''));
}

async function handleCreateEvent() {
  if (!newEventName.value.trim()) {
    alert('Event name is required');
    return;
  }

  if (selectedTools.value.length === 0) {
    alert('Please select at least one tool for this event');
    return;
  }

  try {
    const eventId = await invoke<number>('create_event', {
      eventName: newEventName.value,
      description: newEventDesc.value || null,
      toolIds: selectedTools.value
    });

    if (!eventId) {
      throw new Error('Failed to create event - no ID returned');
    }

    showCreateEventModal.value = false;
    newEventName.value = '';
    newEventDesc.value = '';
    selectedTools.value = [];
    newEventDate.value = '';
    newEventTime.value = '';
    await fetchData();
    alert("Event created successfully!");
  } catch (e) {
    console.error("Create Event Error:", e);
    alert('Failed to create event: ' + JSON.stringify(e));
  }
}

function handleLogoutClick() {
  console.log("DEBUG: Logout button clicked");

  // Close dropdown first
  showUserDropdown.value = false;

  // Show custom confirmation modal instead of native confirm
  setTimeout(() => {
    showLogoutConfirm.value = true;
    console.log("DEBUG: Showing logout confirmation modal");
  }, 50);
}

function confirmLogoutAction() {
  console.log("DEBUG: User confirmed logout via modal");
  showLogoutConfirm.value = false;
  doLogout();
}

function cancelLogout() {
  console.log("DEBUG: User cancelled logout");
  showLogoutConfirm.value = false;
}

function doLogout() {
  console.log("DEBUG: Starting logout process");

  try {
    // Clear localStorage
    localStorage.removeItem('user_session');
    console.log("DEBUG: localStorage cleared");

    // Force page reload to login
    console.log("DEBUG: Redirecting to /login");
    window.location.href = '/login';

  } catch (error) {
    console.error("DEBUG: Logout error:", error);
    alert('Logout failed: ' + error);
  }
}

function goToProfile() {
  showUserDropdown.value = false;
  currentView.value = 'profile';
}

onMounted(fetchData);
</script>

<template>
  <div class="flex h-screen overflow-hidden bg-eling-light dark:bg-eling-dark transition-colors duration-300">
    <!-- Sidebar -->
    <aside
      class="w-64 bg-white dark:bg-eling-surface border-r border-black/5 dark:border-black/5 dark:border-white/5 flex flex-col transition-colors duration-300">
      <!-- Logo -->
      <div class="h-16 flex items-center px-6 border-b border-black/5 dark:border-black/5 dark:border-white/5">
        <h1 class="text-xl font-bold text-eling-accent font-sans tracking-wide">ELING</h1>
        <span
          class="ml-2 text-xs font-mono text-gray-500 dark:text-gray-900 dark:text-eling-light/50 uppercase tracking-widest pl-2">Admin</span>
      </div>

      <!-- Navigation Links -->
      <nav class="flex-1 px-4 py-6 space-y-1 overflow-y-auto">
        <template v-for="item in navItems" :key="item.id">
          <!-- Parent Item -->
          <button @click="item.children ? item.collapsed = !item.collapsed : currentView = item.id as any"
            class="w-full group flex items-center justify-between px-3 py-2 text-sm font-medium rounded-lg transition-colors text-left"
            :class="currentView === item.id ? 'bg-eling-accent/10 text-eling-accent' : 'text-gray-600 dark:text-gray-900 dark:text-eling-light/70 hover:bg-black/5 dark:hover:text-gray-900 dark:text-eling-light dark:hover:bg-black/5 dark:bg-white/5'">
            <div class="flex items-center">
              <svg class="mr-3 flex-shrink-0 h-5 w-5"
                :class="currentView === item.id ? 'text-eling-accent' : 'text-gray-900 dark:text-eling-light/50 group-hover:text-gray-900 dark:text-eling-light'"
                fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" :d="item.icon" />
              </svg>
              {{ item.label }}
            </div>
            <!-- Dropdown Icon if has children -->
            <svg v-if="item.children"
              class="w-4 h-4 text-gray-900 dark:text-eling-light/30 transition-transform duration-200"
              :class="{ 'rotate-90': !item.collapsed }" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
            </svg>
          </button>

          <!-- Children -->
          <div v-if="item.children && !item.collapsed"
            class="pl-2 space-y-0.5 mt-1 transition-all max-h-[60vh] overflow-y-auto">
            <template v-for="child in item.children" :key="child.id">
              <!-- Category Header (non-clickable) -->
              <div v-if="child.id.startsWith('cat_')"
                class="px-3 py-2 text-[10px] font-bold uppercase tracking-wider text-eling-accent/70 mt-3 first:mt-0">
                {{ child.label }}
              </div>
              <!-- Tool Item (clickable) -->
              <button v-else @click="currentView = child.id as any"
                class="w-full flex items-center px-3 py-1.5 text-xs font-medium rounded-lg transition-colors text-left"
                :class="currentView === child.id ? 'text-eling-accent bg-eling-accent/10' : 'text-gray-900 dark:text-eling-light/60 hover:text-gray-900 dark:text-eling-light hover:bg-black/5 dark:bg-white/5'">
                <span class="w-1.5 h-1.5 rounded-full mr-3 bg-current opacity-50"></span>
                {{ child.label }}
              </button>
            </template>
          </div>
        </template>
      </nav>


    </aside>

    <!-- Main Content -->
    <main class="flex-1 overflow-y-auto bg-gradient-to-tr from-[#1B3022] to-[#0D1F14]">
      <!-- Header Bar -->
      <div
        class="sticky top-0 h-16 border-b border-black/5 dark:border-white/5 bg-white/5 dark:bg-white/5 backdrop-blur-sm flex items-center justify-end px-8 gap-4 z-[100]">
        <!-- Theme Toggle -->
        <ThemeToggle />

        <!-- Notification Bell Component -->
        <NotificationBell />

        <!-- User Avatar & Dropdown -->
        <div class="relative">
          <button @click="showUserDropdown = !showUserDropdown" data-testid="user-menu-button"
            class="flex items-center gap-3 pl-4 border-l border-black/10 dark:border-white/10 hover:bg-black/5 dark:hover:bg-white/5 rounded-lg transition-colors px-3 py-2">
            <div class="text-right">
              <p class="text-xs font-medium text-gray-900 dark:text-eling-light">Administrator</p>
              <p class="text-[10px] text-gray-900 dark:text-eling-light/50">admin@eling.id</p>
            </div>
            <div
              class="w-8 h-8 rounded-full bg-eling-accent flex items-center justify-center text-eling-dark font-bold text-xs ring-2 ring-transparent hover:ring-eling-accent/50 transition-all">
              AD
            </div>
          </button>

          <!-- Dropdown Menu -->
          <div v-if="showUserDropdown"
            class="absolute right-0 mt-2 w-56 bg-white dark:bg-eling-surface rounded-xl shadow-2xl border border-black/10 dark:border-white/10 z-[9999] overflow-hidden">
            <!-- Profile Option -->
            <button @click="goToProfile"
              class="w-full px-4 py-3 text-left hover:bg-black/5 dark:hover:bg-white/5 transition-colors flex items-center gap-3 border-b border-black/5 dark:border-white/5">
              <svg class="w-5 h-5 text-gray-600 dark:text-eling-light/70" fill="none" viewBox="0 0 24 24"
                stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                  d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
              </svg>
              <div>
                <p class="text-sm font-medium text-gray-900 dark:text-eling-light">Profile</p>
                <p class="text-xs text-gray-500 dark:text-eling-light/50">View and edit profile</p>
              </div>
            </button>

            <!-- Logout Option -->
            <button @click.prevent.stop="handleLogoutClick" data-testid="logout-button"
              class="w-full px-4 py-3 text-left hover:bg-red-50 dark:hover:bg-red-500/10 transition-colors flex items-center gap-3">
              <svg class="w-5 h-5 text-red-600 dark:text-red-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                  d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1" />
              </svg>
              <div>
                <p class="text-sm font-medium text-red-600 dark:text-red-400">Logout</p>
                <p class="text-xs text-red-500 dark:text-red-400/70">Sign out of your account</p>
              </div>
            </button>
          </div>

          <!-- Backdrop -->
          <div v-if="showUserDropdown" @click="showUserDropdown = false" class="fixed inset-0 z-[9998]"></div>
        </div>
      </div>

      <div class="max-w-7xl mx-auto px-8 py-8 bg-white dark:bg-eling-dark-card rounded-3xl shadow-lg">

        <div v-if="currentView === 'dashboard'">
          <!-- Header -->
          <div class="flex justify-between items-end mb-8">
            <div>
              <h2 class="text-2xl font-bold text-gray-900 dark:text-eling-light">Dashboard Overview</h2>
              <p class="text-sm text-gray-900 dark:text-eling-light/50 mt-1">System performance and operational metrics.
              </p>
            </div>
            <span class="text-xs font-mono text-eling-accent/70 border border-eling-accent/20 px-2 py-1 rounded">
              SYSTEM STATUS: ONLINE
            </span>
          </div>

          <!-- STATISTICS CARDS -->
          <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">

            <!-- Candidates Card -->
            <div class="glass-panel p-6 border-l-4 border-l-blue-500 relative overflow-hidden group">
              <div class="absolute right-0 top-0 p-4 opacity-10 group-hover:opacity-20 transition-opacity">
                <svg class="w-24 h-24" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                    d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z" />
                </svg>
              </div>
              <h3 class="text-sm font-medium text-gray-900 dark:text-eling-light/70 uppercase tracking-wider">Total
                Candidates</h3>
              <div class="mt-2 flex items-baseline">
                <span class="text-4xl font-bold text-gray-900 dark:text-white">{{users.filter(u => u.role !==
                  'admin').length}}</span>
                <span class="ml-2 text-sm text-gray-900 dark:text-eling-light/40">Registered</span>
              </div>
              <div class="mt-4 text-xs text-blue-300 bg-blue-500/10 inline-block px-2 py-1 rounded">
                Active Personnel
              </div>
            </div>

            <!-- Schedule Card -->
            <div class="glass-panel p-6 border-l-4 border-l-green-500 relative overflow-hidden group">
              <div class="absolute right-0 top-0 p-4 opacity-10 group-hover:opacity-20 transition-opacity">
                <svg class="w-24 h-24" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                    d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
                </svg>
              </div>
              <h3 class="text-sm font-medium text-gray-900 dark:text-eling-light/70 uppercase tracking-wider">Active
                Schedules</h3>
              <div class="mt-2 flex items-baseline">
                <span class="text-4xl font-bold text-gray-900 dark:text-white">{{ events.length }}</span>
                <span class="ml-2 text-sm text-gray-900 dark:text-eling-light/40">Events</span>
              </div>
              <div class="mt-4 text-xs text-green-300 bg-green-500/10 inline-block px-2 py-1 rounded">
                Upcoming Sessions
              </div>
            </div>

            <!-- Tools Card -->
            <div class="glass-panel p-6 border-l-4 border-l-purple-500 relative overflow-hidden group">
              <div class="absolute right-0 top-0 p-4 opacity-10 group-hover:opacity-20 transition-opacity">
                <svg class="w-24 h-24" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                    d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                    d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                </svg>
              </div>
              <h3 class="text-sm font-medium text-gray-900 dark:text-eling-light/70 uppercase tracking-wider">Master
                Tools
              </h3>
              <div class="mt-2 flex items-baseline">
                <span class="text-4xl font-bold text-gray-900 dark:text-white">{{ tools.length }}</span>
                <span class="ml-2 text-sm text-gray-900 dark:text-eling-light/40">Available</span>
              </div>
              <div class="mt-4 text-xs text-purple-300 bg-purple-500/10 inline-block px-2 py-1 rounded">
                Psychological Library
              </div>
            </div>

          </div>

          <!-- Users Section -->
          <div class="mb-8">
            <div class="flex justify-between items-center mb-4 px-1">
              <h3 class="text-lg font-medium text-gray-900 dark:text-eling-light flex items-center">
                Registered Candidates
                <span
                  class="ml-3 px-2 py-0.5 bg-black/5 dark:bg-white/5 rounded-full text-xs font-mono text-gray-900 dark:text-eling-light/70">
                  {{ users.length }}
                </span>
              </h3>
            </div>
            <div class="glass-panel overflow-hidden border-black/5 dark:border-white/5">
              <ul role="list" class="divide-y divide-white/5">
                <li v-for="user in users" :key="user.id"
                  class="px-6 py-4 hover:bg-black/5 dark:bg-white/5 transition-colors">
                  <div class="flex items-center justify-between">
                    <div class="flex items-center space-x-4">
                      <div
                        class="w-10 h-10 rounded-xl bg-gradient-to-br from-eling-surface to-eling-dark border border-white/10 flex items-center justify-center text-sm font-mono text-eling-accent shadow-lg">
                        {{ user.username.substring(0, 2).toUpperCase() }}
                      </div>
                      <div>
                        <div class="text-sm font-medium text-gray-900 dark:text-eling-light">{{ user.username }}</div>
                        <div class="text-xs text-gray-900 dark:text-eling-light/40">ID: #{{ user.id }}</div>
                      </div>
                    </div>

                    <span
                      class="px-2.5 py-1 inline-flex text-xs leading-4 font-semibold tracking-wide rounded-md border"
                      :class="user.role === 'admin'
                        ? 'bg-purple-500/10 text-purple-300 border-purple-500/20'
                        : 'bg-eling-accent/10 text-eling-accent border-eling-accent/20'">
                      {{ user.role }}
                    </span>
                  </div>
                </li>
              </ul>
            </div>
          </div>

          <!-- Events Section -->
          <div>
            <div class="flex justify-between items-center mb-4 px-1">
              <h3 class="text-lg font-medium text-gray-900 dark:text-eling-light">Active Schedules</h3>
              <button @click="showCreateEventModal = true" class="btn-neumorphic text-sm py-1.5 px-4 flex items-center">
                <svg class="w-4 h-4 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
                </svg>
                New Event
              </button>
            </div>

            <div v-if="events.length === 0" class="text-center py-12 glass-panel border-dashed border-white/10">
              <div
                class="w-16 h-16 bg-black/5 dark:bg-white/5 rounded-full flex items-center justify-center mx-auto mb-4">
                <svg class="h-8 w-8 text-gray-900 dark:text-eling-light/30" fill="none" viewBox="0 0 24 24"
                  stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5"
                    d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
                </svg>
              </div>
              <p class="text-sm text-gray-900 dark:text-eling-light/50">No scheduled events found.</p>
            </div>

            <div v-else class="glass-panel overflow-hidden border-black/5 dark:border-white/5">
              <ul role="list" class="divide-y divide-white/5">
                <li v-for="event in events" :key="event.id"
                  class="px-6 py-4 hover:bg-black/5 dark:bg-white/5 transition-colors">
                  <div class="flex items-center justify-between">
                    <div>
                      <div class="text-base font-medium text-gray-900 dark:text-eling-light">{{ event.event_name }}
                      </div>
                      <div class="text-xs text-gray-900 dark:text-eling-light/40 font-mono mt-0.5">REF: {{
                        event.id.toString().padStart(6,
                          '0') }}</div>
                    </div>
                    <span
                      class="px-2.5 py-1 inline-flex text-xs leading-4 font-mono font-medium rounded bg-blue-500/10 text-blue-300 border border-blue-500/20">
                      {{ event.status }}
                    </span>
                  </div>
                </li>
              </ul>
            </div>
          </div>
        </div> <!-- End Dashboard View -->

        <!-- CANDIDATES VIEW -->
        <div v-else-if="currentView === 'candidates'">
          <div class="flex justify-between items-end mb-8">
            <div>
              <h2 class="text-2xl font-bold text-gray-900 dark:text-eling-light">Candidate Management</h2>
              <p class="text-sm text-gray-900 dark:text-eling-light/50 mt-1">Search, filter, and manage registered
                candidates.</p>
            </div>
          </div>
          <CandidateList :users="users" @delete="(id) => handleUserAction('Delete', id)"
            @edit="(u) => handleUserAction('Edit', u)" @view="(u) => handleUserAction('View', u)"
            @refresh="fetchData" />
        </div>

        <!-- SCHEDULE VIEW -->
        <div v-else-if="currentView === 'schedule'">
          <div class="flex justify-between items-end mb-8">
            <div>
              <h2 class="text-2xl font-bold text-gray-900 dark:text-eling-light">Assessment Schedule</h2>
              <p class="text-sm text-gray-900 dark:text-eling-light/50 mt-1">Manage testing sessions and timelines.</p>
            </div>
          </div>
          <ScheduleList :events="events" @create="showCreateEventModal = true" @viewDetails="viewEventDetails" />
        </div>

        <!-- TEST RESULTS VIEW -->
        <div v-else-if="currentView === 'results'">
          <div class="flex justify-between items-end mb-8">
            <div>
              <h2 class="text-2xl font-bold text-gray-900 dark:text-eling-light">Test Results</h2>
              <p class="text-sm text-gray-900 dark:text-eling-light/50 mt-1">View and manage candidate assessment
                results.
              </p>
            </div>
          </div>
          <TestResults />
        </div>

        <!-- PROFILE VIEW -->
        <div v-else-if="currentView === 'profile'">
          <ProfileSettings />
        </div>

        <!-- TOOL EDITOR VIEW -->
        <div v-else-if="currentView.startsWith('tool_')">
          <ToolEditor :toolId="getToolId(currentView)" />
        </div>

      </div>
    </main>

    <!-- Create Event Modal -->
    <div v-if="showCreateEventModal" class="fixed z-50 inset-0 overflow-y-auto" aria-labelledby="modal-title"
      role="dialog" aria-modal="true">
      <div class="flex items-center justify-center min-h-screen px-4 text-center">
        <!-- Backdrop -->
        <div class="absolute inset-0 bg-black/80 backdrop-blur-sm transition-opacity" aria-hidden="true"
          @click="showCreateEventModal = false"></div>

        <!-- Modal Panel -->
        <div
          class="inline-block glass-panel w-full max-w-2xl p-6 text-left align-middle shadow-2xl transform transition-all relative z-50 border-eling-accent/20 bg-eling-surface max-h-[90vh] overflow-y-auto">
          <div class="flex justify-between items-start mb-6">
            <div>
              <h3 class="text-lg leading-6 font-bold text-gray-900 dark:text-eling-light" id="modal-title">Create
                Assessment
                Event</h3>
              <p class="text-xs text-gray-900 dark:text-eling-light/50 mt-1">Set up a new testing session with multiple
                tools.</p>
            </div>
            <button @click="showCreateEventModal = false"
              class="text-gray-900 dark:text-eling-light/50 hover:text-gray-900 dark:text-eling-light">
              <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
              </svg>
            </button>
          </div>

          <div class="space-y-5">
            <!-- Event Info -->
            <div class="grid grid-cols-2 gap-4">
              <div>
                <label for="event-name"
                  class="block text-xs font-mono text-eling-accent mb-1.5 uppercase tracking-wider">Event Name *</label>
                <input v-model="newEventName" type="text" id="event-name"
                  class="input-glass w-full bg-black/20 focus:bg-black/40 border-white/10"
                  placeholder="e.g. BATCH_2025_Q1">
              </div>
              <div>
                <label for="event-date"
                  class="block text-xs font-mono text-gray-900 dark:text-eling-light/70 mb-1.5 uppercase tracking-wider">Event
                  Date</label>
                <input v-model="newEventDate" type="date" id="event-date"
                  class="input-glass w-full bg-black/20 focus:bg-black/40 border-white/10">
              </div>
            </div>

            <div>
              <label for="event-desc"
                class="block text-xs font-mono text-gray-900 dark:text-eling-light/70 mb-1.5 uppercase tracking-wider">Description</label>
              <textarea v-model="newEventDesc" id="event-desc" rows="2"
                class="input-glass w-full bg-black/20 focus:bg-black/40 border-white/10"
                placeholder="Optional context..."></textarea>
            </div>

            <!-- Tool Selection -->
            <div>
              <div class="flex items-center justify-between mb-3">
                <label class="block text-xs font-mono text-eling-accent uppercase tracking-wider">Select Tools for
                  Assessment</label>
                <span class="text-xs text-cyan-400 bg-cyan-500/10 px-2 py-1 rounded">{{ selectedTools.length }}
                  selected</span>
              </div>

              <div class="space-y-4 max-h-[300px] overflow-y-auto pr-2">
                <div v-for="group in organizedTools" :key="group.category"
                  class="bg-black/5 dark:bg-white/5 rounded-xl p-3">
                  <div class="flex items-center justify-between mb-2">
                    <span class="text-xs font-bold text-gray-900 dark:text-white/70 uppercase tracking-wider">{{
                      group.category }}</span>
                    <div class="flex gap-2">
                      <button @click="selectAllInCategory(group.tools)"
                        class="text-[10px] text-cyan-400 hover:text-cyan-300 px-2 py-0.5 rounded bg-black/5 dark:bg-white/5 hover:bg-white/10">
                        Select All
                      </button>
                      <button @click="deselectAllInCategory(group.tools)"
                        class="text-[10px] text-gray-900 dark:text-white/50 hover:text-gray-900 dark:text-white/70 px-2 py-0.5 rounded bg-black/5 dark:bg-white/5 hover:bg-white/10">
                        Clear
                      </button>
                    </div>
                  </div>
                  <div class="grid grid-cols-3 gap-2">
                    <button v-for="tool in group.tools" :key="tool.id" @click="toggleToolSelection(tool.id)"
                      class="px-3 py-2 rounded-lg text-xs font-medium transition-all border text-left"
                      :class="isToolSelected(tool.id)
                        ? 'bg-cyan-500/20 border-cyan-500 text-cyan-300'
                        : 'bg-black/5 dark:bg-white/5 border-white/10 text-gray-900 dark:text-white/60 hover:border-white/30 hover:bg-white/10'">
                      <div class="flex items-center gap-2">
                        <span class="w-4 h-4 rounded flex items-center justify-center text-[10px]"
                          :class="isToolSelected(tool.id) ? 'bg-cyan-500 text-gray-900 dark:text-white' : 'bg-white/10 text-gray-900 dark:text-white/30'">✓</span>
                        {{ tool.name }}
                      </div>
                    </button>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <div class="mt-8 flex flex-row-reverse gap-3">
            <button @click="handleCreateEvent" type="button" :disabled="!newEventName"
              class="btn-neumorphic text-sm flex-1 py-2.5 shadow-eling-accent/20 disabled:opacity-50 disabled:cursor-not-allowed">
              Create Event ({{ selectedTools.length }} tools)
            </button>
            <button @click="showCreateEventModal = false" type="button"
              class="px-4 py-2.5 rounded-xl border border-white/10 text-gray-900 dark:text-eling-light hover:bg-black/5 dark:bg-white/5 transition-colors text-sm w-auto font-medium">
              Cancel
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- Event Detail Modal -->
    <EventDetailModal v-if="showEventDetailModal && selectedEventId" :eventId="selectedEventId" :allUsers="users"
      @close="closeEventDetailModal" @refresh="fetchData" />

    <!-- Logout Confirmation Modal -->
    <div v-if="showLogoutConfirm"
      class="fixed inset-0 z-[10000] flex items-center justify-center bg-black/50 backdrop-blur-sm">
      <div
        class="bg-white dark:bg-eling-surface rounded-2xl shadow-2xl border border-black/10 dark:border-white/10 p-6 max-w-md w-full mx-4">
        <div class="flex items-center gap-3 mb-4">
          <div class="w-12 h-12 rounded-full bg-red-500/10 flex items-center justify-center">
            <svg class="w-6 h-6 text-red-600 dark:text-red-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1" />
            </svg>
          </div>
          <div>
            <h3 class="text-lg font-bold text-gray-900 dark:text-eling-light">Confirm Logout</h3>
            <p class="text-sm text-gray-600 dark:text-eling-light/70">Are you sure you want to log out?</p>
          </div>
        </div>

        <div class="flex gap-3 mt-6">
          <button @click="cancelLogout"
            class="flex-1 px-4 py-2.5 rounded-xl border border-black/10 dark:border-white/10 text-gray-900 dark:text-eling-light hover:bg-black/5 dark:hover:bg-white/5 transition-colors font-medium">
            Cancel
          </button>
          <button @click="confirmLogoutAction"
            class="flex-1 px-4 py-2.5 rounded-xl bg-red-600 hover:bg-red-700 text-white transition-colors font-medium">
            Logout
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
