<script setup lang="ts">
import { ref, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';

interface Question {
    id: number;
    subtest_id: number;
    question_text: string;
    question_type: string;
    options: any; // { choices: [...] }
    sequence_order: number;
}

const props = defineProps<{
    subtestId: number;
    subtestName: string;
    existingQuestions: Question[];
    show: boolean;
}>();

const emit = defineEmits<{
    (e: 'close'): void;
    (e: 'refresh'): void;
}>();

const newQuestion = ref({
    text: '',
    type: 'multiple_choice',
    optionsText: '', // "A, B, C, D" or "True, False"
    correctAnswer: '',
    mediaUrl: '' // For image path
});

const imagePreview = ref('');
const editingQuestionId = ref<number | null>(null);

const isSubmitting = ref(false);

const sortedQuestions = computed(() => {
    return [...props.existingQuestions].sort((a, b) => a.sequence_order - b.sequence_order);
});

function handleImageSelect(event: Event) {
    const target = event.target as HTMLInputElement;
    const file = target.files?.[0];
    if (file) {
        const reader = new FileReader();
        reader.onload = (e) => {
            const result = e.target?.result as string;
            imagePreview.value = result;
            newQuestion.value.mediaUrl = result; // Store base64
        };
        reader.readAsDataURL(file);
    }
}

function clearImage() {
    imagePreview.value = '';
    newQuestion.value.mediaUrl = '';
}

function handleEditQuestion(question: Question) {
    editingQuestionId.value = question.id;
    newQuestion.value.text = question.question_text;
    newQuestion.value.type = question.question_type;

    // Parse options
    const opts = question.options as any;
    if (opts?.choices) {
        newQuestion.value.optionsText = opts.choices.join(', ');
        newQuestion.value.correctAnswer = opts.correct || '';
    }

    // Load existing image if any
    if ((question as any).media_url) {
        newQuestion.value.mediaUrl = (question as any).media_url;
        imagePreview.value = (question as any).media_url;
    }
}

function cancelEdit() {
    editingQuestionId.value = null;
    newQuestion.value = { text: '', type: 'multiple_choice', optionsText: '', correctAnswer: '', mediaUrl: '' };
    imagePreview.value = '';
}

async function handleSaveQuestion() {
    if (!newQuestion.value.text || !newQuestion.value.optionsText) {
        alert("Please fill all fields");
        return;
    }

    isSubmitting.value = true;
    try {
        const choices = newQuestion.value.optionsText.split(',').map(s => s.trim()).filter(s => s.length > 0);
        const optionsJson = {
            choices: choices,
            correct: newQuestion.value.correctAnswer
        };

        if (editingQuestionId.value) {
            // Update existing question
            await invoke('update_question', {
                id: editingQuestionId.value,
                text: newQuestion.value.text,
                qType: newQuestion.value.type,
                options: optionsJson
            });
            cancelEdit();
        } else {
            // Create new question
            await invoke('create_question', {
                subtestId: props.subtestId,
                text: newQuestion.value.text,
                qType: newQuestion.value.type,
                options: optionsJson,
                sequence: (props.existingQuestions.length || 0) + 1
            });
            newQuestion.value = { text: '', type: 'multiple_choice', optionsText: '', correctAnswer: '', mediaUrl: '' };
            imagePreview.value = '';
        }

        emit('refresh');
    } catch (e) {
        alert("Failed to save question: " + e);
    } finally {
        isSubmitting.value = false;
    }
}

async function handleDeleteQuestion(id: number) {
    if (!confirm("Delete this question?")) return;
    try {
        await invoke('delete_question', { id });
        emit('refresh');
    } catch (e) {
        alert("Failed to delete question: " + e);
    }
}

</script>

<template>
    <Teleport to="body">
        <div v-if="show" class="fixed inset-0 z-[9999] grid place-items-center bg-gray-200/50 dark:bg-black/80 backdrop-blur-sm p-4 transition-colors duration-300">
            <div
                class="bg-eling-light-card dark:bg-eling-dark-card border border-gray-200 dark:border-white/10 rounded-xl w-full max-w-6xl max-h-[95vh] flex flex-col shadow-2xl transition-colors duration-300">

                <!-- Header -->
                <div class="p-6 border-b border-gray-200 dark:border-white/5 flex justify-between items-center bg-gray-50/50 dark:bg-white/5">
                    <div>
                        <h3 class="text-xl font-bold text-eling-light-text dark:text-eling-dark-text">Manage Questions</h3>
                        <p class="text-sm text-eling-light-subtext dark:text-eling-dark-text/50 font-mono mt-1">SUBTEST: {{ subtestName }}</p>
                    </div>
                    <button @click="$emit('close')"
                        class="p-2 hover:bg-gray-200 dark:hover:bg-white/10 rounded-full transition-colors text-eling-light-text/70 dark:text-eling-dark-text/70">
                        <svg class="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                d="M6 18L18 6M6 6l12 12" />
                        </svg>
                    </button>
                </div>

                <!-- Content -->
                <div class="flex-1 overflow-hidden flex flex-col md:flex-row">

                    <!-- List (Left) -->
                    <div class="flex-1 overflow-y-auto p-6 space-y-4 border-r border-gray-200 dark:border-white/5 bg-eling-light-bg dark:bg-transparent">
                        <h4 class="text-sm font-bold text-eling-light-text/70 dark:text-eling-dark-text/70 uppercase tracking-wider mb-4">Existing
                            Questions
                            ({{ existingQuestions.length }})</h4>

                        <div v-if="existingQuestions.length === 0"
                            class="text-center py-12 border-2 border-dashed border-gray-300 dark:border-white/5 rounded-lg">
                            <p class="text-sm text-eling-light-subtext dark:text-eling-dark-text/30">No questions yet.</p>
                        </div>

                        <div v-for="(q, idx) in sortedQuestions" :key="q.id"
                            class="p-4 bg-white dark:bg-white/5 rounded-lg border border-gray-200 dark:border-white/5 group hover:border-eling-emerald/30 transition-all shadow-sm dark:shadow-none">
                            <div class="flex justify-between items-start gap-4">
                                <div class="flex gap-3">
                                    <span
                                        class="bg-gray-100 dark:bg-black/40 px-2 py-1 rounded text-xs font-mono text-eling-light-subtext dark:text-eling-dark-text/50 h-fit">#{{
                                            idx + 1 }}</span>
                                    <div>
                                        <p class="text-sm text-eling-light-text dark:text-eling-dark-text font-medium">{{ q.question_text }}</p>
                                        <div class="flex flex-wrap gap-2 mt-2">
                                            <span v-for="opt in q.options?.choices" :key="opt"
                                                class="text-[10px] px-2 py-0.5 rounded-full bg-gray-100 dark:bg-white/5 text-eling-light-subtext dark:text-eling-dark-text/60 border border-gray-200 dark:border-white/5">
                                                {{ opt }}
                                            </span>
                                        </div>
                                    </div>
                                </div>
                                <div class="flex gap-2">
                                    <button @click="handleEditQuestion(q)"
                                        class="text-blue-500 dark:text-blue-400 hover:text-blue-600 dark:hover:text-blue-300 opacity-0 group-hover:opacity-100 transition-opacity p-1">
                                        <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                                d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
                                        </svg>
                                    </button>
                                    <button @click="handleDeleteQuestion(q.id)"
                                        class="text-red-500 dark:text-red-400 hover:text-red-600 dark:hover:text-red-300 opacity-0 group-hover:opacity-100 transition-opacity p-1">
                                        <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                                d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                                        </svg>
                                    </button>
                                </div>
                            </div>
                        </div>
                    </div>

                    <!-- Add Form (Right) -->
                    <div class="w-full md:w-1/3 bg-gray-50 dark:bg-black/20 p-6 overflow-y-auto border-l border-gray-200 dark:border-white/5">
                        <h4 class="text-sm font-bold uppercase tracking-wider mb-6 flex items-center justify-between"
                            :class="editingQuestionId ? 'text-blue-500 dark:text-blue-400' : 'text-eling-emerald'">
                            <div class="flex items-center">
                                <svg class="w-4 h-4 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                        d="M12 4v16m8-8H4" />
                                </svg>
                                {{ editingQuestionId ? 'Edit Question' : 'Add Question' }}
                            </div>
                            <button v-if="editingQuestionId" @click="cancelEdit"
                                class="text-xs px-2 py-1 bg-gray-200 dark:bg-white/10 rounded hover:bg-gray-300 dark:hover:bg-white/20 text-eling-light-text dark:text-eling-dark-text">
                                Cancel
                            </button>
                        </h4>

                        <div class="space-y-4">
                            <div>
                                <label class="block text-xs text-eling-light-subtext dark:text-eling-dark-text/50 mb-1">Question Text</label>
                                <textarea v-model="newQuestion.text" rows="3" class="input-glass w-full text-sm"
                                    placeholder="e.g. 2, 4, 8, ..."></textarea>
                            </div>

                            <div>
                                <label class="block text-xs text-eling-light-subtext dark:text-eling-dark-text/50 mb-1">Type</label>
                                <select v-model="newQuestion.type" class="input-glass w-full text-sm">
                                    <option value="multiple_choice">Multiple Choice</option>
                                    <option value="true_false">True / False</option>
                                </select>
                            </div>

                            <div>
                                <label class="block text-xs text-eling-light-subtext dark:text-eling-dark-text/50 mb-1">Choices (Comma Separated)</label>
                                <textarea v-model="newQuestion.optionsText" rows="3" class="input-glass w-full text-sm"
                                    placeholder="e.g. 16, 20, 24, 32"></textarea>
                                <p class="text-[10px] text-eling-light-subtext/70 dark:text-eling-dark-text/30 mt-1">Separate options with commas.</p>
                            </div>

                            <div>
                                <label class="block text-xs text-eling-light-subtext dark:text-eling-dark-text/50 mb-1">Correct Answer (Exact
                                    Match)</label>
                                <input v-model="newQuestion.correctAnswer" type="text"
                                    class="input-glass w-full text-sm" placeholder="e.g. 16">
                            </div>

                            <!-- Image Upload -->
                            <div>
                                <label class="block text-xs text-eling-light-subtext dark:text-eling-dark-text/50 mb-2">Image (Optional)</label>
                                
                                <div class="relative group">
                                    <input type="file" accept="image/*" @change="handleImageSelect" id="question-image"
                                        class="absolute inset-0 w-full h-full opacity-0 cursor-pointer z-10" />
                                    
                                    <div class="border-2 border-dashed border-gray-300 dark:border-white/10 rounded-xl p-4 transition-all group-hover:border-eling-emerald/50 bg-gray-50/50 dark:bg-white/5 flex flex-col items-center justify-center text-center space-y-2">
                                        <div class="p-2 rounded-full bg-eling-emerald/10 text-eling-emerald">
                                            <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" />
                                            </svg>
                                        </div>
                                        <div class="text-xs">
                                            <span class="font-medium text-eling-light-text dark:text-eling-dark-text">Click to upload</span>
                                            <span class="text-eling-light-subtext dark:text-eling-dark-text/50"> or drag and drop</span>
                                        </div>
                                        <p class="text-[10px] text-eling-light-subtext/70 dark:text-eling-dark-text/30">SVG, PNG, JPG or GIF</p>
                                    </div>
                                </div>

                                <!-- Image Preview -->
                                <div v-if="imagePreview" class="mt-3 relative group">
                                    <div class="rounded-xl overflow-hidden border border-gray-200 dark:border-white/10 bg-gray-100 dark:bg-black/40">
                                        <img :src="imagePreview" class="w-full h-40 object-contain" />
                                        
                                        <!-- Remove Overlay -->
                                        <div class="absolute inset-0 bg-black/60 opacity-0 group-hover:opacity-100 transition-opacity flex items-center justify-center backdrop-blur-sm">
                                            <button @click="clearImage" type="button"
                                                class="flex items-center gap-2 px-3 py-1.5 bg-red-500/90 text-white rounded-lg hover:bg-red-600 transition-colors text-xs font-medium">
                                                <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                                                </svg>
                                                Remove Image
                                            </button>
                                        </div>
                                    </div>
                                </div>
                            </div>

                            <div class="pt-4">
                                <button @click="handleSaveQuestion" :disabled="isSubmitting"
                                    class="btn-neumorphic w-full py-2 text-xs flex justify-center items-center">
                                    <span v-if="isSubmitting"
                                        class="animate-spin h-4 w-4 border-b-2 border-white rounded-full mr-2"></span>
                                    {{ isSubmitting ? 'Saving...' : (editingQuestionId ? 'Update Question' : 'Add Question') }}
                                </button>
                            </div>
                        </div>
                    </div>

                </div>
            </div>
        </div>
    </Teleport>
</template>
