<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';

interface NavItem {
    id: string;
    label: string;
    icon: string;
    children?: NavItem[];
    collapsed?: boolean;
}

interface Tool {
    id: number;
    name: string;
    description: string;
}

interface Props {
    currentView: string;
}

defineProps<Props>();
const emit = defineEmits<{
    'update:currentView': [view: string];
}>();

const tools = ref<Tool[]>([]);

const navItems = ref<NavItem[]>([
    { id: 'dashboard', label: 'Dashboard', icon: 'M4 6h16M4 12h16M4 18h16' },
    { id: 'schedule', label: 'Schedule', icon: 'M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z' },
    { id: 'candidates', label: 'Candidates', icon: 'M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z' },
    { id: 'results', label: 'Test Results', icon: 'M9 12l2 2 4-4M7.835 4.697a3.42 3.42 0 001.946-.806 3.42 3.42 0 014.438 0 3.42 3.42 0 001.946.806 3.42 3.42 0 013.138 3.138 3.42 3.42 0 00.806 1.946 3.42 3.42 0 010 4.438 3.42 3.42 0 00-.806 1.946 3.42 3.42 0 01-3.138 3.138 3.42 3.42 0 00-1.946.806 3.42 3.42 0 01-4.438 0 3.42 3.42 0 00-1.946-.806 3.42 3.42 0 01-3.138-3.138 3.42 3.42 0 00-.806-1.946 3.42 3.42 0 010-4.438 3.42 3.42 0 00.806-1.946 3.42 3.42 0 013.138-3.138z' },
    { id: 'monitoring', label: 'Monitoring', icon: 'M15 10l4.553-2.276A1 1 0 0121 8.618v6.764a1 1 0 01-1.447.894L15 14M5 18h8a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z' },
    { id: 'user_management', label: 'User Management', icon: 'M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z' },
    {
        id: 'master_tools',
        label: 'Master Tools Test',
        icon: 'M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10',
        collapsed: true,
        children: []
    },
]);

const toolCategories: Record<string, string[]> = {
    'Cognitive': ['TIU', 'IST', 'CFIT', 'MATRICES', 'WPT', 'GATB'],
    'Personality': ['EPPS', 'PAPI', 'DISC', 'MBTI', '16PF', 'HEXACO'],
    'Interest & Style': ['RMIB', 'RIASEC', 'MSDT'],
    'Speed Test': ['KRAEPELIN'],
    'Projective': ['WARTEGG', 'DAP', 'HTP', 'BAUM']
};

async function loadTools() {
    try {
        tools.value = await invoke<Tool[]>('get_tools');
        
        // Populate Master Tools submenu
        const masterToolsItem = navItems.value.find(item => item.id === 'master_tools');
        if (masterToolsItem && tools.value.length > 0) {
            masterToolsItem.children = [];
            
            for (const [category, toolNames] of Object.entries(toolCategories)) {
                const categoryTools = tools.value.filter(t =>
                    toolNames.some(name => t.name.toUpperCase().includes(name.toUpperCase()))
                );
                
                if (categoryTools.length > 0) {
                    masterToolsItem.children.push({
                        id: `cat_${category}`,
                        label: category,
                        icon: ''
                    });
                    
                    categoryTools.forEach(tool => {
                        masterToolsItem.children!.push({
                            id: `tool_${tool.id}`,
                            label: tool.name,
                            icon: ''
                        });
                    });
                }
            }
        }
    } catch (e) {
        console.error('Failed to load tools:', e);
    }
}

function handleNavClick(itemId: string) {
    emit('update:currentView', itemId);
}

function toggleCollapse(item: NavItem) {
    if (item.children) {
        item.collapsed = !item.collapsed;
    } else {
        handleNavClick(item.id);
    }
}

onMounted(loadTools);
</script>

<template>
    <aside class="w-64 bg-white dark:bg-eling-dark-surface border-r border-black/5 dark:border-white/5 flex flex-col transition-colors duration-300">
        <!-- Logo -->
        <div class="h-16 flex items-center px-6 border-b border-black/5 dark:border-white/5">
            <h1 class="text-xl font-bold text-eling-emerald font-sans tracking-wide">ELING</h1>
            <span class="ml-2 text-xs font-mono text-gray-500 dark:text-eling-dark-text/50 uppercase tracking-widest pl-2">Admin</span>
        </div>

        <!-- Navigation Links -->
        <nav class="flex-1 px-4 py-6 space-y-1 overflow-y-auto">
            <template v-for="item in navItems" :key="item.id">
                <!-- Parent Item -->
                <button 
                    @click="toggleCollapse(item)"
                    class="w-full group flex items-center justify-between px-3 py-2 text-sm font-medium rounded-lg transition-colors text-left"
                    :class="currentView === item.id ? 'bg-eling-emerald/10 text-eling-emerald' : 'text-gray-700 dark:text-eling-dark-text/90 hover:bg-black/5 dark:hover:bg-white/5 hover:text-gray-900 dark:hover:text-eling-dark-text'"
                >
                    <div class="flex items-center">
                        <svg 
                            class="mr-3 flex-shrink-0 h-5 w-5"
                            :class="currentView === item.id ? 'text-eling-emerald' : 'text-gray-600 dark:text-eling-dark-text/80 group-hover:text-gray-900 dark:group-hover:text-eling-dark-text'"
                            fill="none" 
                            viewBox="0 0 24 24" 
                            stroke="currentColor"
                        >
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" :d="item.icon" />
                        </svg>
                        {{ item.label }}
                    </div>
                    <!-- Dropdown Icon if has children -->
                    <svg 
                        v-if="item.children"
                        class="w-4 h-4 text-gray-500 dark:text-eling-dark-text/60 transition-transform duration-200"
                        :class="{ 'rotate-90': !item.collapsed }" 
                        fill="none" 
                        viewBox="0 0 24 24" 
                        stroke="currentColor"
                    >
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
                    </svg>
                </button>

                <div 
                    v-if="item.children && !item.collapsed"
                    class="pl-2 space-y-0.5 mt-1 transition-all max-h-[60vh] overflow-y-auto"
                >
                    <template v-for="child in item.children" :key="child.id">
                        <!-- Category Header (non-clickable) -->
                        <div 
                            v-if="child.id.startsWith('cat_')"
                            class="px-3 py-2 text-[10px] font-bold uppercase tracking-wider text-eling-emerald mt-3 first:mt-0"
                        >
                            {{ child.label }}
                        </div>
                        <!-- Tool Item (clickable) -->
                        <button 
                            v-else 
                            @click="handleNavClick(child.id)"
                            class="w-full flex items-center px-3 py-1.5 text-xs font-medium rounded-lg transition-colors text-left"
                            :class="currentView === child.id ? 'text-eling-emerald bg-eling-emerald/10' : 'text-gray-700 dark:text-eling-dark-text/90 hover:text-gray-900 dark:hover:text-eling-dark-text hover:bg-black/5 dark:hover:bg-white/5'"
                        >
                            <span class="w-1.5 h-1.5 rounded-full mr-3 bg-current opacity-70"></span>
                            {{ child.label }}
                        </button>
                    </template>
                </div>
            </template>
        </nav>
    </aside>
</template>
