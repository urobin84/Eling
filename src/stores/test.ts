import { defineStore } from 'pinia';
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';

export interface Question {
    id: number;
    text: string; // mapped from question_text
    type: string; // mapped from question_type
    options: string[] | null;
}

export interface TestTool {
    id: number;
    name: string;
    type: string;
}

export const useTestStore = defineStore('test', () => {
    // State
    const sessionId = ref<number | null>(null);
    const currentTool = ref<TestTool | null>(null);
    const currentQuestions = ref<Question[]>([]);
    const answers = ref<Map<number, string>>(new Map());
    const tools = ref<TestTool[]>([]);
    const sessionStatus = ref<string>('idle');

    // Actions
    async function loadTools() {
        try {
            const result = await invoke<any[]>('get_tools');
            tools.value = result.map(t => ({
                id: t.id,
                name: t.name,
                type: t.tool_type
            }));
        } catch (e) {
            console.error('Failed to load tools:', e);
        }
    }

    async function startSession(eventId: number, participantId: string) {
        try {
            const id = await invoke<number>('create_session', {
                eventId,
                participantId,
                metadata: null
            });
            sessionId.value = id;
            sessionStatus.value = 'active';
            return true;
        } catch (e) {
            console.error('Failed to start session:', e);
            return false;
        }
    }

    function setAnswer(questionId: number, answer: string) {
        answers.value.set(questionId, answer);
    }

    return {
        sessionId,
        currentTool,
        currentQuestions,
        answers,
        tools,
        sessionStatus,
        loadTools,
        startSession,
        setAnswer
    };
});
