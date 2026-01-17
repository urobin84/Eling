<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue';

const props = defineProps({
  modelValue: {
    type: [String, Number, Object, null] as any,
    default: null
  },
  options: {
    type: Array as () => any[],
    required: true
  },
  placeholder: {
    type: String,
    default: 'Select option'
  },
  labelKey: {
    type: String,
    default: 'label'
  },
  valueKey: {
    type: String,
    default: 'value'
  },
  searchable: {
    type: Boolean,
    default: true
  }
});

const emit = defineEmits(['update:modelValue']);

const isOpen = ref(false);
const searchQuery = ref('');
const dropdownRef = ref<HTMLElement | null>(null);

// Close dropdown when clicking outside
function handleClickOutside(event: MouseEvent) {
  if (dropdownRef.value && !dropdownRef.value.contains(event.target as Node)) {
    isOpen.value = false;
  }
}

onMounted(() => {
  document.addEventListener('click', handleClickOutside);
});

onUnmounted(() => {
  document.removeEventListener('click', handleClickOutside);
});

// Computed Selected Label
const selectedLabel = computed(() => {
  if (props.modelValue === null || props.modelValue === undefined || props.modelValue === 'all') { // Handle 'all' explicitly if needed or rely on parent
      // Check if there is an option that explicitly matches the "all" value, otherwise return placeholder
      const allOption = props.options.find(opt => opt[props.valueKey] === props.modelValue);
      return allOption ? allOption[props.labelKey] : props.placeholder;
  }
  
  const selected = props.options.find(opt => opt[props.valueKey] === props.modelValue);
  return selected ? selected[props.labelKey] : props.placeholder;
});

// Filtered Options
const filteredOptions = computed(() => {
  if (!props.searchable || !searchQuery.value) {
    return props.options;
  }
  const query = searchQuery.value.toLowerCase();
  return props.options.filter(opt => 
    String(opt[props.labelKey]).toLowerCase().includes(query)
  );
});

function selectOption(option: any) {
  emit('update:modelValue', option[props.valueKey]);
  isOpen.value = false;
  searchQuery.value = '';
}

function toggleDropdown() {
  isOpen.value = !isOpen.value;
  if (isOpen.value) {
      searchQuery.value = ''; // Reset search on open
  }
}
</script>

<template>
  <div ref="dropdownRef" class="relative group min-w-[160px]">
    <!-- Trigger Button -->
    <button 
      @click="toggleDropdown" 
      type="button"
      class="w-full flex items-center justify-between font-sans text-sm transition-all duration-200 outline-none
             rounded-full px-5 py-2.5 shadow-sm hover:shadow-md
             bg-[#F0F4F9] dark:bg-[#1E1F20] border border-transparent dark:border-white/10 text-gray-700 dark:text-gray-200
             hover:border-eling-emerald/30 focus:ring-2 focus:ring-eling-emerald/50"
      :class="{ 'ring-2 ring-eling-emerald/50 border-eling-emerald': isOpen }"
    >
      <span class="truncate pr-2">{{ selectedLabel }}</span>
      <svg 
        class="w-4 h-4 text-gray-400 dark:text-gray-500 transition-transform duration-200" 
        :class="{ 'rotate-180': isOpen }"
        fill="none" viewBox="0 0 24 24" stroke="currentColor"
      >
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
      </svg>
    </button>

    <!-- Dropdown Menu -->
    <div 
      v-if="isOpen" 
      class="absolute z-[100] mt-2 w-full min-w-[200px] max-h-60 overflow-hidden rounded-2xl shadow-xl 
             bg-white dark:bg-[#1E1F20] border border-gray-100 dark:border-white/10
             flex flex-col animate-in fade-in zoom-in-95 duration-100 origin-top"
    >
      <!-- Search Input -->
      <div v-if="searchable" class="p-2 border-b border-gray-100 dark:border-white/5 bg-gray-50/50 dark:bg-white/5">
        <div class="relative">
             <svg class="absolute left-3 top-2.5 w-3.5 h-3.5 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
             </svg>
             <input 
                v-model="searchQuery"
                type="text" 
                placeholder="Search..." 
                class="w-full pl-8 pr-3 py-1.5 text-xs rounded-lg 
                       bg-white dark:bg-black/20 border border-gray-200 dark:border-white/10 
                       text-gray-900 dark:text-white focus:outline-none focus:border-eling-emerald focus:ring-1 focus:ring-eling-emerald"
                @click.stop
             />
        </div>
      </div>

      <!-- Options List -->
      <div class="overflow-y-auto overflow-x-hidden flex-1 p-1 custom-scrollbar">
        <ul v-if="filteredOptions.length > 0" class="space-y-0.5">
          <li v-for="(option, index) in filteredOptions" :key="index">
            <button 
              @click="selectOption(option)"
              class="w-full text-left px-3 py-2 text-sm rounded-lg transition-colors flex items-center justify-between group"
              :class="option[valueKey] === modelValue 
                ? 'bg-eling-emerald/10 text-eling-emerald font-medium' 
                : 'text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-white/5'"
            >
              <span>{{ option[labelKey] }}</span>
              <svg v-if="option[valueKey] === modelValue" class="w-4 h-4 text-eling-emerald" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
              </svg>
            </button>
          </li>
        </ul>
        <div v-else class="px-4 py-8 text-center text-xs text-gray-400 dark:text-gray-500">
           No options found.
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.custom-scrollbar::-webkit-scrollbar {
  width: 4px;
}
.custom-scrollbar::-webkit-scrollbar-track {
  background: transparent;
}
.custom-scrollbar::-webkit-scrollbar-thumb {
  background: rgba(156, 163, 175, 0.3);
  border-radius: 4px;
}
.custom-scrollbar::-webkit-scrollbar-thumb:hover {
  background: rgba(156, 163, 175, 0.5);
}
</style>
