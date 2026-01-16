<script setup lang="ts">
import { ref, computed } from 'vue';

// Types
interface Event {
    id: number;
    event_name: string;
    description?: string;
    created_at?: string;
    status: string;
    event_code?: string;
    participant_count?: number;
}

const props = defineProps<{
    events: Event[];
}>();

const emit = defineEmits(['create', 'viewDetails']);

// State
const viewMode = ref<'list' | 'calendar'>('list');
const searchQuery = ref('');
const currentPage = ref(1);
const itemsPerPage = ref(10);

// --- LIST VIEW LOGIC ---
const filteredEvents = computed(() => {
    if (!searchQuery.value) return props.events;
    const q = searchQuery.value.toLowerCase();
    return props.events.filter(e =>
        e.event_name.toLowerCase().includes(q) ||
        e.description?.toLowerCase().includes(q)
    );
});

const paginatedEvents = computed(() => {
    const start = (currentPage.value - 1) * itemsPerPage.value;
    const end = start + itemsPerPage.value;
    return filteredEvents.value.slice(start, end);
});

const totalPages = computed(() => Math.ceil(filteredEvents.value.length / itemsPerPage.value));

// --- CALENDAR VIEW LOGIC ---
const currentDate = ref(new Date());
const daysInMonth = computed(() => {
    const year = currentDate.value.getFullYear();
    const month = currentDate.value.getMonth();
    return new Date(year, month + 1, 0).getDate();
});
const firstDayOfMonth = computed(() => {
    const year = currentDate.value.getFullYear();
    const month = currentDate.value.getMonth();
    return new Date(year, month, 1).getDay();
});

function prevMonth() {
    currentDate.value = new Date(currentDate.value.getFullYear(), currentDate.value.getMonth() - 1, 1);
}

function nextMonth() {
    currentDate.value = new Date(currentDate.value.getFullYear(), currentDate.value.getMonth() + 1, 1);
}

// Helper to check if an event falls on a specific day (Mock logic for now, using created_at or just showing all)
// Since we don't have a specific 'scheduled_date' in the Event model yet, we'll map them based on created_at or just list them.
// For robust calendar, we'd need a 'date' field in the DB.
// I will simulate it by checking if created_at matches the day, or just putting them in 'Today' for demo.
function getEventsForDay(day: number) {
    // This is a placeholder. Real logic needs a 'scheduled_for' column.
    // We'll just return events created on this day.
    const year = currentDate.value.getFullYear();
    const month = currentDate.value.getMonth();
    const dateStr = `${year}-${String(month + 1).padStart(2, '0')}-${String(day).padStart(2, '0')}`;

    return props.events.filter(e => e.created_at?.startsWith(dateStr));
}
</script>

<template>
    <div class="space-y-6">
        <!-- Header Controls -->
        <div class="flex flex-col sm:flex-row justify-between items-center gap-4">
            <!-- View Toggle & Search -->
            <div class="flex items-center space-x-4 w-full sm:w-auto">
                <div class="glass-panel p-1 flex space-x-1">
                    <button @click="viewMode = 'list'"
                        class="px-3 py-1.5 rounded-lg text-xs font-bold uppercase tracking-wider transition-all"
                        :class="viewMode === 'list' ? 'bg-eling-accent text-eling-dark shadow-lg' : 'text-gray-500 dark:text-gray-900 dark:text-eling-light/50 hover:text-gray-900 dark:text-eling-light'">
                        List View
                    </button>
                    <button @click="viewMode = 'calendar'"
                        class="px-3 py-1.5 rounded-lg text-xs font-bold uppercase tracking-wider transition-all"
                        :class="viewMode === 'calendar' ? 'bg-eling-accent text-eling-dark shadow-lg' : 'text-gray-500 dark:text-gray-900 dark:text-eling-light/50 hover:text-gray-900 dark:text-eling-light'">
                        Calendar
                    </button>
                </div>

                <div v-if="viewMode === 'list'" class="relative w-full sm:w-64">
                    <input v-model="searchQuery" type="text" placeholder="Search events..."
                        class="input-glass w-full pl-10 bg-black/20 focus:bg-black/30 border-black/10 dark:border-white/10 py-1.5 text-sm">
                    <svg class="w-4 h-4 text-gray-500 dark:text-gray-900 dark:text-eling-light/50 absolute left-3 top-2.5"
                        fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                            d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
                    </svg>
                </div>
            </div>

            <!-- Actions -->
            <div class="flex items-center space-x-3 w-full sm:w-auto">
                <button @click="$emit('create')"
                    class="btn-neumorphic text-xs py-2 px-4 flex items-center shadow-eling-accent/20">
                    <svg class="w-4 h-4 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
                    </svg>
                    New Assessment
                </button>
            </div>
        </div>

        <!-- LIST VIEW -->
        <div v-if="viewMode === 'list'"
            class="glass-panel overflow-hidden border-black/5 dark:border-white/5 relative fade-enter-active">
            <div class="overflow-x-auto">
                <table class="min-w-full divide-y divide-white/10">
                    <thead class="bg-black/5 dark:bg-white/5">
                        <tr>
                            <th
                                class="px-6 py-3 text-left text-xs font-mono font-medium text-gray-500 dark:text-gray-900 dark:text-eling-light/50 uppercase tracking-wider">
                                Event Name</th>
                            <th
                                class="px-6 py-3 text-left text-xs font-mono font-medium text-gray-500 dark:text-gray-900 dark:text-eling-light/50 uppercase tracking-wider">
                                Event Code</th>
                            <th
                                class="px-6 py-3 text-left text-xs font-mono font-medium text-gray-500 dark:text-gray-900 dark:text-eling-light/50 uppercase tracking-wider">
                                Participants</th>
                            <th
                                class="px-6 py-3 text-left text-xs font-mono font-medium text-gray-500 dark:text-gray-900 dark:text-eling-light/50 uppercase tracking-wider">
                                Status</th>
                            <th
                                class="px-6 py-3 text-left text-xs font-mono font-medium text-gray-500 dark:text-gray-900 dark:text-eling-light/50 uppercase tracking-wider">
                                Actions</th>
                        </tr>
                    </thead>
                    <tbody class="divide-y divide-white/5 bg-transparent">
                        <tr v-if="paginatedEvents.length === 0">
                            <td colspan="6"
                                class="px-6 py-12 text-center text-gray-900 dark:text-eling-light/30 text-sm">
                                No scheduled events found.
                            </td>
                        </tr>
                        <tr v-for="event in paginatedEvents" :key="event.id"
                            class="hover:bg-black/5 dark:bg-white/5 transition-colors">
                            <td class="px-6 py-4 whitespace-nowrap">
                                <div class="text-sm font-bold text-gray-900 dark:text-eling-light">{{ event.event_name
                                }}</div>
                                <div
                                    class="text-xs text-gray-600 dark:text-gray-900 dark:text-eling-light/40 font-mono">
                                    ID: {{ event.id }}
                                </div>
                            </td>
                            <td class="px-6 py-4 whitespace-nowrap">
                                <div v-if="event.event_code"
                                    class="inline-flex items-center px-3 py-1 rounded-lg bg-eling-accent/20 border border-eling-accent/30">
                                    <span class="text-sm font-mono font-bold text-eling-accent">{{ event.event_code
                                        }}</span>
                                </div>
                                <span v-else class="text-xs text-gray-900 dark:text-eling-light/30">No code</span>
                            </td>
                            <td class="px-6 py-4 whitespace-nowrap">
                                <div class="flex items-center gap-2">
                                    <svg class="w-4 h-4 text-gray-900 dark:text-eling-light/50" fill="none"
                                        viewBox="0 0 24 24" stroke="currentColor">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                            d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z" />
                                    </svg>
                                    <span class="text-sm font-medium text-gray-900 dark:text-eling-light">{{
                                        event.participant_count || 0 }}</span>
                                </div>
                            </td>
                            <td class="px-6 py-4 whitespace-nowrap">
                                <span
                                    class="px-2 py-0.5 rounded text-[10px] font-bold uppercase tracking-wide bg-blue-500/10 border border-blue-500/20 text-blue-300">
                                    {{ event.status }}
                                </span>
                            </td>
                            <td class="px-6 py-4 whitespace-nowrap">
                                <button @click="emit('viewDetails', event.id)"
                                    class="px-3 py-1.5 text-xs font-medium bg-eling-accent/10 text-eling-accent hover:bg-eling-accent/20 rounded-lg transition-all border border-eling-accent/30">
                                    👁️ View Details
                                </button>
                            </td>
                        </tr>
                    </tbody>
                </table>
            </div>
            <!-- Pagination Footer -->
            <div
                class="bg-black/5 dark:bg-white/5 border-t border-black/10 dark:border-white/10 px-4 py-3 flex items-center justify-between sm:px-6">
                <div class="hidden sm:flex-1 sm:flex sm:items-center sm:justify-between">
                    <p class="text-sm text-gray-500 dark:text-gray-900 dark:text-eling-light/50 font-mono">
                        Showing {{ (currentPage - 1) * itemsPerPage + 1 }} to {{ Math.min(currentPage * itemsPerPage,
                            filteredEvents.length) }} of {{ filteredEvents.length }} results
                    </p>
                    <nav class="relative z-0 inline-flex rounded-md shadow-sm -space-x-px" aria-label="Pagination">
                        <button @click="currentPage > 1 ? currentPage-- : null" :disabled="currentPage === 1"
                            class="relative inline-flex items-center px-2 py-2 rounded-l-md border border-black/10 dark:border-white/10 bg-black/5 dark:bg-white/5 text-sm font-medium text-gray-900 dark:text-gray-900 dark:text-eling-light/70 hover:bg-black/10 dark:bg-white/10 disabled:opacity-50">
                            <span class="sr-only">Previous</span>
                            <svg class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                                <path fill-rule="evenodd"
                                    d="M12.707 5.293a1 1 0 010 1.414L9.414 10l3.293 3.293a1 1 0 01-1.414 1.414l-4-4a1 1 0 010-1.414l4-4a1 1 0 011.414 0z"
                                    clip-rule="evenodd" />
                            </svg>
                        </button>
                        <span
                            class="relative inline-flex items-center px-4 py-2 border border-black/10 dark:border-white/10 bg-black/5 dark:bg-white/5 text-sm font-medium text-gray-900 dark:text-eling-light">
                            {{ currentPage }} / {{ totalPages || 1 }}
                        </span>
                        <button @click="currentPage < totalPages ? currentPage++ : null"
                            :disabled="currentPage === totalPages"
                            class="relative inline-flex items-center px-2 py-2 rounded-r-md border border-black/10 dark:border-white/10 bg-black/5 dark:bg-white/5 text-sm font-medium text-gray-900 dark:text-gray-900 dark:text-eling-light/70 hover:bg-black/10 dark:bg-white/10 disabled:opacity-50">
                            <span class="sr-only">Next</span>
                            <svg class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                                <path fill-rule="evenodd"
                                    d="M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 011.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z"
                                    clip-rule="evenodd" />
                            </svg>
                        </button>
                    </nav>
                </div>
            </div>
        </div>

        <!-- CALENDAR VIEW -->
        <div v-else class="glass-panel p-6 border-black/5 dark:border-white/5 fade-enter-active">
            <div class="flex items-center justify-between mb-6">
                <h3 class="text-xl font-bold text-gray-900 dark:text-eling-light flex items-center">
                    <span class="mr-2">{{ currentDate.toLocaleString('default', { month: 'long', year: 'numeric' })
                    }}</span>
                </h3>
                <div class="flex space-x-2">
                    <button @click="prevMonth"
                        class="p-2 rounded-full hover:bg-black/10 dark:bg-white/10 text-gray-900 dark:text-eling-light transition-colors">
                        <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
                        </svg>
                    </button>
                    <button @click="nextMonth"
                        class="p-2 rounded-full hover:bg-black/10 dark:bg-white/10 text-gray-900 dark:text-eling-light transition-colors">
                        <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
                        </svg>
                    </button>
                </div>
            </div>

            <!-- Calendar Grid -->
            <div
                class="grid grid-cols-7 gap-px bg-black/10 dark:bg-white/10 rounded-lg overflow-hidden border border-black/10 dark:border-white/10">
                <!-- Days Header -->
                <div v-for="day in ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat']" :key="day"
                    class="bg-eling-surface/50 p-2 text-center text-xs font-mono font-bold text-gray-500 dark:text-gray-900 dark:text-eling-light/50 uppercase">
                    {{ day }}
                </div>

                <!-- Empty Cells -->
                <div v-for="n in firstDayOfMonth" :key="'emp-' + n"
                    class="bg-white dark:bg-eling-surface/30 min-h-[100px]"></div>

                <!-- Days -->
                <div v-for="day in daysInMonth" :key="'day-' + day"
                    class="bg-white dark:bg-eling-surface/30 min-h-[100px] p-2 hover:bg-black/5 dark:bg-white/5 transition-colors group relative border-t border-l border-black/5 dark:border-white/5">
                    <span
                        class="text-sm font-mono text-gray-900 dark:text-gray-900 dark:text-eling-light/70 group-hover:text-eling-accent">{{
                            day }}</span>

                    <!-- Event Dots -->
                    <div class="mt-2 space-y-1">
                        <div v-for="event in getEventsForDay(day)" :key="event.id"
                            class="text-[10px] px-1.5 py-0.5 rounded bg-eling-accent/20 text-eling-accent border border-eling-accent/30 truncate">
                            {{ event.event_name }}
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped>
.fade-enter-active {
    animation: fade-in 0.3s ease-out;
}
</style>
