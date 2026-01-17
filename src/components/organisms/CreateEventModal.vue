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
}

const props = defineProps<Props>();
const emit = defineEmits<{
    close: [];
    create: [data: { name: string; date: string; description: string; toolIds: number[] }];
}>();

const eventName = ref('');
const eventDate = ref<string | Date>(new Date().toISOString().split('T')[0]); // Default to today
const eventDescription = ref('');
const selectedTools = ref<number[]>([]);

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
    if (!eventName.value) {
        alert('Event name is required');
        return;
    }
    
    if (!eventDate.value) {
        alert('Event date is required');
        return;
    }

    // Convert Date to string if needed
    const dateStr = typeof eventDate.value === 'string' 
        ? eventDate.value 
        : (eventDate.value as Date).toISOString().split('T')[0];

    emit('create', {
        name: eventName.value,
        date: dateStr,
        description: eventDescription.value,
        toolIds: selectedTools.value
    });

    // Reset form
    eventName.value = '';
    eventDate.value = new Date().toISOString().split('T')[0];
    eventDescription.value = '';
    selectedTools.value = [];
}

function handleClose() {
    emit('close');
    // Reset form
    eventName.value = '';
    eventDate.value = new Date().toISOString().split('T')[0];
    eventDescription.value = '';
    selectedTools.value = [];
}
</script>

<template>
    <BaseModal :show="show" title="Create Assessment Event" size="2xl" @close="handleClose">
        <div class="space-y-5">
            <!-- Event Info -->
            <div class="grid grid-cols-2 gap-4">
                <FormGroup label="Event Name" required>
                    <BaseInput
                        v-model="eventName"
                        placeholder="e.g. BATCH_2025_Q1"
                    />
                </FormGroup>
                <FormGroup label="Event Date" required>
                    <input
                        v-model="eventDate"
                        type="date"
                        class="input-glass w-full"
                        :min="new Date().toISOString().split('T')[0]"
                    />
                    <p class="text-xs text-gray-500 dark:text-gray-400 mt-1">
                        📅 When will this assessment take place?
                    </p>
                </FormGroup>
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
                <BaseButton 
                    size="md" 
                    :disabled="!eventName || !eventDate"
                    @click="handleCreate"
                >
                    Create Event ({{ selectedTools.length }} tools)
                </BaseButton>
                <BaseButton 
                    variant="ghost" 
                    size="md"
                    @click="handleClose"
                >
                    Cancel
                </BaseButton>
            </div>
        </template>
    </BaseModal>
</template>
