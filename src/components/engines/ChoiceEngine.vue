<script setup lang="ts">
import { ref } from 'vue';

interface Question {
  id: number;
  text: string;
  options: string[] | null;
}

defineProps<{
  question: Question;
}>();

const emit = defineEmits<{
  (e: 'answer', value: string): void;
}>();

const selectedOption = ref<string | null>(null);

function selectOption(option: string) {
  selectedOption.value = option;
  emit('answer', option);
}
</script>

<template>
  <div class="p-6 bg-white rounded-lg shadow-md max-w-2xl mx-auto">
    <h3 class="text-xl font-semibold mb-6 text-gray-800">{{ question.text }}</h3>
    
    <div class="space-y-3">
      <button
        v-for="(option, index) in question.options"
        :key="index"
        @click="selectOption(option)"
        :class="[
          'w-full text-left p-4 rounded-lg border-2 transition-all duration-200',
          selectedOption === option
            ? 'border-blue-500 bg-blue-50 text-blue-700'
            : 'border-gray-200 hover:border-gray-300 hover:bg-gray-50'
        ]"
      >
        <div class="flex items-center">
          <span class="w-8 h-8 flex items-center justify-center rounded-full border-2 mr-4"
            :class="selectedOption === option ? 'border-blue-500 bg-blue-500 text-white' : 'border-gray-300 text-gray-500'"
          >
            {{ String.fromCharCode(65 + index) }}
          </span>
          <span class="text-lg">{{ option }}</span>
        </div>
      </button>
    </div>
  </div>
</template>
