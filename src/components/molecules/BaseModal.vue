<script setup lang="ts">
interface Props {
    show: boolean;
    title: string;
    size?: 'sm' | 'md' | 'lg' | 'xl' | '2xl' | '3xl' | '4xl';
}

withDefaults(defineProps<Props>(), {
    size: '2xl'
});

const emit = defineEmits<{
    close: [];
}>();

const sizeClasses = {
    sm: 'sm:max-w-sm',
    md: 'sm:max-w-md',
    lg: 'sm:max-w-lg',
    xl: 'sm:max-w-xl',
    '2xl': 'sm:max-w-2xl',
    '3xl': 'sm:max-w-3xl',
    '4xl': 'sm:max-w-4xl'
};
</script>

<template>
    <Teleport to="body">
        <div 
            v-if="show" 
            class="fixed z-[9999] inset-0 overflow-y-auto" 
            aria-labelledby="modal-title" 
            role="dialog" 
            aria-modal="true"
        >
            <div class="flex items-end sm:items-center justify-center min-h-screen pt-4 px-4 pb-20 text-center sm:block sm:p-0">
                <!-- Backdrop -->
                <div 
                    class="fixed inset-0 bg-black/80 backdrop-blur-sm transition-opacity" 
                    aria-hidden="true" 
                    @click="emit('close')"
                />

                <!-- Modal Panel -->
                <div 
                    :class="[
                        'inline-block align-bottom glass-panel bg-eling-light-surface text-left overflow-hidden shadow-xl transform transition-all sm:my-8 sm:align-middle sm:w-full border border-eling-emerald/20',
                        sizeClasses[size]
                    ]"
                >
                    <!-- Header -->
                    <div class="flex justify-between items-center p-6 border-b border-black/10 dark:border-white/10">
                        <h3 class="text-lg leading-6 font-bold text-gray-900 dark:text-white" id="modal-title">
                            {{ title }}
                        </h3>
                        <button 
                            @click="emit('close')" 
                            class="text-gray-400 hover:text-gray-500 dark:hover:text-white transition-colors"
                        >
                            <span class="sr-only">Close</span>
                            <svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                            </svg>
                        </button>
                    </div>

                    <!-- Content -->
                    <div class="p-6">
                        <slot />
                    </div>

                    <!-- Footer (optional) -->
                    <div v-if="$slots.footer" class="p-6 border-t border-black/10 dark:border-white/10">
                        <slot name="footer" />
                    </div>
                </div>
            </div>
        </div>
    </Teleport>
</template>
