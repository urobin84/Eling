import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';

interface RecordingOptions {
    sessionId: string;
    userId: number;
    username: string;
    testName: string;
    eventName?: string;
}

export function useRecording() {
    const isRecording = ref(false);
    const cameraRecorder = ref<MediaRecorder | null>(null);
    const screenRecorder = ref<MediaRecorder | null>(null);
    const cameraChunks = ref<Blob[]>([]);
    const screenChunks = ref<Blob[]>([]);
    const cameraStream = ref<MediaStream | null>(null);
    const screenStream = ref<MediaStream | null>(null);

    async function startRecording(options: RecordingOptions): Promise<boolean> {
        try {
            // Request camera permission
            cameraStream.value = await navigator.mediaDevices.getUserMedia({
                video: { width: 640, height: 480, frameRate: 30 },
                audio: false
            });

            // Request screen capture permission
            screenStream.value = await navigator.mediaDevices.getDisplayMedia({
                video: { width: 1280, height: 720, frameRate: 30 },
                audio: false
            });

            // Setup camera recorder
            cameraRecorder.value = new MediaRecorder(cameraStream.value, {
                mimeType: 'video/webm;codecs=vp9',
                videoBitsPerSecond: 1000000 // 1 Mbps
            });

            cameraRecorder.value.ondataavailable = (event) => {
                if (event.data.size > 0) {
                    cameraChunks.value.push(event.data);
                }
            };

            cameraRecorder.value.onstop = async () => {
                await saveCameraRecording(options);
            };

            // Setup screen recorder
            screenRecorder.value = new MediaRecorder(screenStream.value, {
                mimeType: 'video/webm;codecs=vp9',
                videoBitsPerSecond: 2000000 // 2 Mbps
            });

            screenRecorder.value.ondataavailable = (event) => {
                if (event.data.size > 0) {
                    screenChunks.value.push(event.data);
                }
            };

            screenRecorder.value.onstop = async () => {
                await saveScreenRecording(options);
            };

            // Start both recordings
            cameraRecorder.value.start(1000); // Collect data every second
            screenRecorder.value.start(1000);
            isRecording.value = true;

            // Register session
            await invoke('register_session', {
                sessionId: options.sessionId,
                userId: options.userId,
                username: options.username,
                eventId: null,
                eventName: options.eventName || null,
                testName: options.testName
            });

            console.log('Recording started successfully');
            return true;
        } catch (error) {
            console.error('Failed to start recording:', error);
            cleanup();
            return false;
        }
    }

    async function stopRecording(): Promise<void> {
        if (!isRecording.value) return;

        try {
            // Stop recorders
            if (cameraRecorder.value && cameraRecorder.value.state !== 'inactive') {
                cameraRecorder.value.stop();
            }
            if (screenRecorder.value && screenRecorder.value.state !== 'inactive') {
                screenRecorder.value.stop();
            }

            // Wait a bit for onstop handlers to complete
            await new Promise(resolve => setTimeout(resolve, 1000));

            cleanup();
            isRecording.value = false;
            console.log('Recording stopped successfully');
        } catch (error) {
            console.error('Failed to stop recording:', error);
            cleanup();
        }
    }

    async function saveCameraRecording(options: RecordingOptions): Promise<void> {
        if (cameraChunks.value.length === 0) return;

        try {
            const blob = new Blob(cameraChunks.value, { type: 'video/webm' });
            const arrayBuffer = await blob.arrayBuffer();
            const uint8Array = new Uint8Array(arrayBuffer);
            const base64 = btoa(String.fromCharCode(...uint8Array));

            await invoke('save_camera_recording', {
                sessionId: options.sessionId,
                videoData: base64
            });

            console.log('Camera recording saved');
        } catch (error) {
            console.error('Failed to save camera recording:', error);
        }
    }

    async function saveScreenRecording(options: RecordingOptions): Promise<void> {
        if (screenChunks.value.length === 0) return;

        try {
            const blob = new Blob(screenChunks.value, { type: 'video/webm' });
            const arrayBuffer = await blob.arrayBuffer();
            const uint8Array = new Uint8Array(arrayBuffer);
            const base64 = btoa(String.fromCharCode(...uint8Array));

            await invoke('save_screen_recording', {
                sessionId: options.sessionId,
                videoData: base64
            });

            console.log('Screen recording saved');
        } catch (error) {
            console.error('Failed to save screen recording:', error);
        }
    }

    function cleanup(): void {
        // Stop all tracks
        if (cameraStream.value) {
            cameraStream.value.getTracks().forEach(track => track.stop());
            cameraStream.value = null;
        }
        if (screenStream.value) {
            screenStream.value.getTracks().forEach(track => track.stop());
            screenStream.value = null;
        }

        // Clear chunks
        cameraChunks.value = [];
        screenChunks.value = [];

        // Clear recorders
        cameraRecorder.value = null;
        screenRecorder.value = null;
    }

    return {
        isRecording,
        startRecording,
        stopRecording
    };
}
