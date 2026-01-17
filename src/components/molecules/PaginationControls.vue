<script setup lang="ts">
interface Props {
    currentPage: number;
    totalPages: number;
    totalItems: number;
    itemsPerPage: number;
}

const props = defineProps<Props>();

const emit = defineEmits<{
    'update:currentPage': [page: number];
}>();

const startItem = (props.currentPage - 1) * props.itemsPerPage + 1;
const endItem = Math.min(props.currentPage * props.itemsPerPage, props.totalItems);

const goToPrevious = () => {
    if (props.currentPage > 1) {
        emit('update:currentPage', props.currentPage - 1);
    }
};

const goToNext = () => {
    if (props.currentPage < props.totalPages) {
        emit('update:currentPage', props.currentPage + 1);
    }
};
</script>

<template>
    <div class="bg-black/5 dark:bg-white/5 border-t border-black/10 dark:border-white/10 px-4 py-3 flex items-center justify-between">
        <p class="text-sm text-gray-900 dark:text-white/50 font-mono">
            Showing {{ startItem }} to {{ endItem }} of {{ totalItems }} results
        </p>
        <nav class="relative z-0 inline-flex rounded-md shadow-sm -space-x-px">
            <button 
                @click="goToPrevious" 
                :disabled="currentPage === 1"
                class="relative inline-flex items-center px-2 py-2 rounded-l-md border border-black/10 dark:border-white/10 bg-black/5 dark:bg-white/5 text-sm font-medium text-gray-900 dark:text-white/70 hover:bg-white/10 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
            >
                <svg class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                    <path fill-rule="evenodd" d="M12.707 5.293a1 1 0 010 1.414L9.414 10l3.293 3.293a1 1 0 01-1.414 1.414l-4-4a1 1 0 010-1.414l4-4a1 1 0 011.414 0z" clip-rule="evenodd" />
                </svg>
            </button>
            <span class="relative inline-flex items-center px-4 py-2 border border-black/10 dark:border-white/10 bg-black/5 dark:bg-white/5 text-sm font-medium text-gray-900 dark:text-white">
                {{ currentPage }} / {{ totalPages || 1 }}
            </span>
            <button 
                @click="goToNext" 
                :disabled="currentPage >= totalPages"
                class="relative inline-flex items-center px-2 py-2 rounded-r-md border border-black/10 dark:border-white/10 bg-black/5 dark:bg-white/5 text-sm font-medium text-gray-900 dark:text-white/70 hover:bg-white/10 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
            >
                <svg class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                    <path fill-rule="evenodd" d="M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 011.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z" clip-rule="evenodd" />
                </svg>
            </button>
        </nav>
    </div>
</template>
