<script setup lang="ts">
import { ref, computed } from 'vue';

import BaseModal from '../molecules/BaseModal.vue';
import BaseButton from '../atoms/BaseButton.vue';
import BaseInput from '../atoms/BaseInput.vue';
import FormGroup from '../molecules/FormGroup.vue';

interface Tool {
    id: number;
    name: string;
    description: string;
}

interface Props {
    show: boolean;
    tools: Tool[];
    loading?: boolean;
}

const props = defineProps<Props>();
const emit = defineEmits<{
    close: [];
    create: [data: { name: string; date: string; description: string; toolIds: number[] }];
}>();

const eventName = ref('');
const eventDate = ref<string>(''); // Empty string for date picker
const eventDescription = ref('');
const selectedTools = ref<number[]>([]);

// Calendar state
const showCalendar = ref(false);
const currentMonth = ref(new Date().getMonth());
const currentYear = ref(new Date().getFullYear());
const selectedHour = ref('09');
const selectedMinute = ref('00');

const currentMonthYear = computed(() => {
    const date = new Date(currentYear.value, currentMonth.value);
    return date.toLocaleDateString('id-ID', { month: 'long', year: 'numeric' });
});

const calendarDays = computed(() => {
    const days = [];
    const firstDay = new Date(currentYear.value, currentMonth.value, 1);
    const lastDay = new Date(currentYear.value, currentMonth.value + 1, 0);
    const prevLastDay = new Date(currentYear.value, currentMonth.value, 0);
    
   const firstDayOfWeek = firstDay.getDay();
    const today = new Date();
    today.setHours(0, 0, 0, 0);
    
    // Previous month days
    for (let i = firstDayOfWeek - 1; i >= 0; i--) {
        const day = prevLastDay.getDate() - i;
        const date = new Date(currentYear.value, currentMonth.value - 1, day);
        days.push({
            day,
            date,
            isCurrentMonth: false,
            isSelected: false,
            isPast: date < today,
            key: `prev-${day}`
        });
    }
    
    // Current month days
    for (let day = 1; day <= lastDay.getDate(); day++) {
        const date = new Date(currentYear.value, currentMonth.value, day);
        const dateStr = date.toISOString().split('T')[0];
        days.push({
            day,
            date,
            isCurrentMonth: true,
            isSelected: eventDate.value === dateStr,
            isPast: date < today,
            key: `curr-${day}`
        });
    }
    
    // Next month days
    const remainingDays = 42 - days.length; // 6 rows * 7 days
    for (let day = 1; day <= remainingDays; day++) {
        const date = new Date(currentYear.value, currentMonth.value + 1, day);
        days.push({
            day,
            date,
            isCurrentMonth: false,
            isSelected: false,
            isPast: date < today,
            key: `next-${day}`
        });
    }
    
    return days;
});

const toolCategories: Record<string, string[]> = {
    'Cognitive': ['TIU', 'IST', 'CFIT', 'MATRICES', 'WPT', 'GATB'],
    'Personality': ['EPPS', 'PAPI', 'DISC', 'MBTI', '16PF', 'HEXACO'],
    'Interest & Style': ['RMIB', 'RIASEC', 'MSDT'],
    'Speed Test': ['KRAEPELIN'],
    'Projective': ['WARTEGG', 'DAP', 'HTP', 'BAUM']
};

const organizedTools = computed(() => {
    const result: { category: string; tools: Tool[] }[] = [];
    const usedToolIds = new Set<number>();

    // 1. Categorized Tools
    for (const [category, toolNames] of Object.entries(toolCategories)) {
        const categoryTools = props.tools.filter(t => 
            toolNames.some(name => t.name.toUpperCase().includes(name.toUpperCase()))
        );
        
        if (categoryTools.length > 0) {
            categoryTools.forEach(t => usedToolIds.add(t.id));
            result.push({ category, tools: categoryTools });
        }
    }

    // 2. Uncategorized (Other) Tools
    const otherTools = props.tools.filter(t => !usedToolIds.has(t.id));
    if (otherTools.length > 0) {
        result.push({ category: 'Other', tools: otherTools });
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

// Disable past dates for event date picker
function disabledPastDates(time: Date): boolean {
    return time.getTime() < Date.now() - 8.64e7; // 8.64e7 = milliseconds in a day
}

function prevMonth() {
    if (currentMonth.value === 0) {
        currentMonth.value = 11;
        currentYear.value--;
    } else {
        currentMonth.value--;
    }
}

function nextMonth() {
    if (currentMonth.value === 11) {
        currentMonth.value = 0;
        currentYear.value++;
    } else {
        currentMonth.value++;
    }
}

function selectDate(dateObj: any) {
    if (dateObj.isPast) return;
    const dateStr = dateObj.date.toISOString().split('T')[0];
    eventDate.value = `${dateStr} ${selectedHour.value}:${selectedMinute.value}`;
    console.log('Date selected, eventDate updated to:', eventDate.value);
}

function confirmDateTime() {
    console.log('Confirm clicked, eventDate is:', eventDate.value);
    showCalendar.value = false;
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

function handleCreate() {
    // Validate event name
    if (!eventName.value || eventName.value.trim() === '') {
        alert('Event name is required');
        return;
    }
    
    // Validate event date
    if (!eventDate.value || eventDate.value.trim() === '') {
        alert('Event date and time are required');
        return;
    }

    console.log('DEBUG: CreateEventModal.vue - Emitting create event. Tool IDs:', selectedTools.value);
    
    emit('create', {
        name: eventName.value,
        date: eventDate.value, // Already in format "YYYY-MM-DD HH:mm"
        description: eventDescription.value,
        toolIds: selectedTools.value
    });

    // Reset form
    eventName.value = '';
    eventDate.value = '';
    eventDescription.value = '';
    selectedTools.value = [];
}

function handleClose() {
    emit('close');
    // Reset form
    eventName.value = '';
    eventDate.value = '';
    eventDescription.value = '';
    selectedTools.value = [];
}

</script>

<template>
    <BaseModal :show="show" title="Create Assessment Event" size="2xl" @close="handleClose">
        <!-- Loading Overlay -->
        <div v-if="loading" class="absolute inset-0 bg-black/50 backdrop-blur-sm z-[100001] flex items-center justify-center rounded-3xl">
            <div class="text-center">
                <div class="inline-block animate-spin rounded-full h-12 w-12 border-4 border-eling-emerald border-t-transparent mb-4"></div>
                <div class="text-white font-semibold">Creating event...</div>
            </div>
        </div>
        
        <div class="space-y-5">
            <!-- Event Info -->
            <div class="grid grid-cols-2 gap-4">
                <!-- Event Name - Direct input without FormGroup -->
                <div class="space-y-1.5">
                    <label class="block text-xs font-mono text-gray-900 dark:text-eling-dark-text/70 uppercase tracking-wider">
                        Event Name
                        <span class="text-red-400">*</span>
                    </label>
                    <input
                        v-model="eventName"
                        type="text"
                        placeholder="e.g. BATCH_2025_Q1"
                        class="input-glass w-full py-2.5 text-sm"
                    />
                </div>
                
                <!-- Custom Date Picker (not using FormGroup because it forces BaseInput) -->
                <div class="space-y-1.5">
                    <label class="block text-xs font-mono text-gray-900 dark:text-eling-dark-text/70 uppercase tracking-wider">
                        Event Date
                        <span class="text-red-400">*</span>
                    </label>
                    <div class="relative">
                        <input
                            :value="eventDate || 'Pilih tanggal...'"
                            readonly
                            @click="showCalendar = !showCalendar"
                            class="w-full px-4 py-2.5 rounded-full border border-transparent bg-gray-100 dark:bg-gray-800
                                   focus:outline-none focus:ring-2 focus:ring-eling-emerald focus:bg-white dark:focus:bg-gray-900
                                   transition-all text-sm text-gray-900 dark:text-white cursor-pointer
                                   hover:bg-white dark:hover:bg-gray-900"
                            placeholder="Pilih tanggal..."
                        />
                        
                        <!-- Custom Mini Calendar -->
                        <div 
                            v-if="showCalendar"
                            class="absolute z-[9999] mt-2 p-4 rounded-xl border border-gray-200 dark:border-gray-700 
                                   bg-white dark:bg-gray-800 shadow-2xl w-[280px] left-0"
                        >
                            <div class="flex justify-between items-center mb-3">
                                <button @click="prevMonth" type="button" 
                                        class="p-1 hover:bg-gray-100 dark:hover:bg-gray-700 rounded transition-colors">
                                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7"/>
                                    </svg>
                                </button>
                                <div class="font-semibold text-sm text-gray-900 dark:text-white">
                                    {{ currentMonthYear }}
                                </div>
                                <button @click="nextMonth" type="button" 
                                        class="p-1 hover:bg-gray-100 dark:hover:bg-gray-700 rounded transition-colors">
                                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"/>
                                    </svg>
                                </button>
                            </div>
                            
                            <!-- Days of week -->
                            <div class="grid grid-cols-7 gap-1 mb-2">
                                <div v-for="day in ['S', 'M', 'T', 'W', 'T', 'F', 'S']" :key="day" 
                                     class="text-center text-xs font-semibold text-gray-500 dark:text-gray-400">
                                    {{ day }}
                                </div>
                            </div>
                            
                            <!-- Calendar days -->
                            <div class="grid grid-cols-7 gap-1">
                                <button
                                    v-for="date in calendarDays"
                                    :key="date.key"
                                    @click="selectDate(date)"
                                    type="button"
                                    :disabled="date.isPast"
                                    :class="[
                                        'aspect-square text-sm rounded-lg transition-all',
                                        date.isCurrentMonth ? 'text-gray-900 dark:text-white' : 'text-gray-300 dark:text-gray-600',
                                        date.isSelected ? 'bg-eling-emerald text-white font-semibold' : '',
                                        !date.isSelected && date.isCurrentMonth && !date.isPast ? 'hover:bg-gray-100 dark:hover:bg-gray-700' : '',
                                        date.isPast ? 'opacity-30 cursor-not-allowed' : 'cursor-pointer'
                                    ]"
                                >
                                    {{ date.day }}
                                </button>
                            </div>
                            
                            <!-- Time Picker -->
                            <div class="mt-4 pt-4 border-t border-gray-200 dark:border-gray-700">
                                <div class="text-xs font-semibold text-gray-700 dark:text-gray-300 mb-2">Pilih Waktu</div>
                                <div class="flex items-center gap-2 justify-center">
                                    <select 
                                        v-model="selectedHour"
                                        class="px-3 py-2 rounded-lg bg-gray-100 dark:bg-gray-700 text-sm focus:outline-none focus:ring-2 focus:ring-eling-emerald"
                                    >
                                        <option v-for="h in 24" :key="h-1" :value="String(h-1).padStart(2, '0')">
                                            {{ String(h-1).padStart(2, '0') }}
                                        </option>
                                    </select>
                                    <span class="text-lg font-semibold">:</span>
                                    <select 
                                        v-model="selectedMinute"
                                        class="px-3 py-2 rounded-lg bg-gray-100 dark:bg-gray-700 text-sm focus:outline-none focus:ring-2 focus:ring-eling-emerald"
                                    >
                                        <option value="00">00</option>
                                        <option value="15">15</option>
                                        <option value="30">30</option>
                                        <option value="45">45</option>
                                    </select>
                                </div>
                            </div>
                            
                            <!-- Confirm Button -->
                            <button
                                @click="confirmDateTime"
                                type="button"
                                class="mt-3 w-full py-2 bg-eling-emerald text-white rounded-lg hover:bg-eling-emerald/90 transition-colors font-medium text-sm"
                            >
                                Konfirmasi
                            </button>
                        </div>
                    </div>

                </div>
            </div>

            <FormGroup label="Description">
                <textarea
                    v-model="eventDescription"
                    rows="2"
                    class="input-glass w-full py-2.5 text-sm"
                    placeholder="Optional context..."
                />
            </FormGroup>

            <!-- Tool Selection -->
            <div>
                <div class="flex items-center justify-between mb-3">
                    <label class="block text-xs font-mono text-eling-emerald uppercase tracking-wider">
                        Select Tools for Assessment
                    </label>
                    <span class="text-xs text-cyan-400 bg-cyan-500/10 px-2 py-1 rounded">
                        {{ selectedTools.length }} selected
                    </span>
                </div>

                <div class="space-y-4 max-h-[300px] overflow-y-auto pr-2">
                    <div 
                        v-for="group in organizedTools" 
                        :key="group.category"
                        class="bg-black/5 dark:bg-white/5 rounded-xl p-3"
                    >
                        <div class="flex items-center justify-between mb-2">
                            <span class="text-xs font-bold text-gray-900 dark:text-white/70 uppercase tracking-wider">
                                {{ group.category }}
                            </span>
                            <div class="flex gap-2">
                                <button 
                                    @click="selectAllInCategory(group.tools)"
                                    class="text-[10px] text-cyan-400 hover:text-cyan-300 px-2 py-0.5 rounded bg-black/5 dark:bg-white/5 hover:bg-white/10"
                                >
                                    Select All
                                </button>
                                <button 
                                    @click="deselectAllInCategory(group.tools)"
                                    class="text-[10px] text-gray-900 dark:text-white/50 hover:text-white/70 px-2 py-0.5 rounded bg-black/5 dark:bg-white/5 hover:bg-white/10"
                                >
                                    Clear
                                </button>
                            </div>
                        </div>
                        <div class="grid grid-cols-3 gap-2">
                            <button 
                                v-for="tool in group.tools" 
                                :key="tool.id" 
                                @click="toggleToolSelection(tool.id)"
                                class="px-3 py-2 rounded-lg text-xs font-medium transition-all border text-left"
                                :class="isToolSelected(tool.id)
                                    ? 'bg-cyan-500/20 border-cyan-500 text-cyan-300'
                                    : 'bg-black/5 dark:bg-white/5 border-white/10 text-gray-900 dark:text-white/60 hover:border-white/30 hover:bg-white/10'"
                            >
                                <div class="flex items-center gap-2">
                                    <span 
                                        class="w-4 h-4 rounded flex items-center justify-center text-[10px]"
                                        :class="isToolSelected(tool.id) ? 'bg-cyan-500 text-white' : 'bg-white/10 text-white/30'"
                                    >
                                        ✓
                                    </span>
                                    {{ tool.name }}
                                </div>
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        </div>

        <template #footer>
            <div class="flex flex-row-reverse gap-3">
                <button
                    type="button"
                    :disabled="!eventName || !eventDate"
                    @click="handleCreate"
                    class="btn-neumorphic text-sm py-2.5 px-5 font-bold disabled:opacity-50 disabled:cursor-not-allowed"
                >
                    Create Event ({{ selectedTools.length }} tools)
                </button>
                <button
                    type="button"
                    @click="handleClose"
                    class="btn-neumorphic bg-transparent border-black/10 dark:border-white/10 hover:bg-black/5 dark:hover:bg-white/5 text-sm py-2.5 px-5 font-bold"
                >
                    Cancel
                </button>
            </div>
        </template>
    </BaseModal>
</template>

<style scoped>
/* Glassmorphism styling for Element Plus Date Picker */
:deep(.el-input__wrapper) {
    background-color: transparent !important;
    box-shadow: none !important;
    border: 1px solid rgba(255, 255, 255, 0.2);
    backdrop-filter: blur(10px);
    transition: all 0.3s ease;
}

:deep(.el-input__wrapper:hover) {
    border-color: rgba(16, 185, 129, 0.5);
    box-shadow: 0 0 0 1px rgba(16, 185, 129, 0.1);
}

:deep(.el-input__wrapper.is-focus) {
    border-color: rgba(16, 185, 129, 0.8);
    box-shadow: 0 0 0 2px rgba(16, 185, 129, 0.2);
}

:deep(.el-input__inner) {
    color: inherit;
}

:deep(.el-input__inner::placeholder) {
    color: rgba(156, 163, 175, 0.7);
}

/* Dark mode adjustments */
html.dark :deep(.el-input__wrapper) {
    border-color: rgba(255, 255, 255, 0.15);
    background: rgba(255, 255, 255, 0.05);
}

html.dark :deep(.el-input__wrapper:hover) {
    border-color: rgba(16, 185, 129, 0.5);
}
</style>
