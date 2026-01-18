import { ref, toRaw } from 'vue';
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
    const startTime = ref<number>(0);

    async function startRecording(options: RecordingOptions): Promise<boolean> {
        try {
            // CRITICAL: Request screen capture FIRST because it requires a user gesture.
            // If we await camera permission first, the user gesture might expire or be lost.
            screenStream.value = await navigator.mediaDevices.getDisplayMedia({
                video: { width: 1280, height: 720, frameRate: 30 },
                audio: false
            });

            // Then request camera permission (less strict about user gesture usually, or already allowed)
            cameraStream.value = await navigator.mediaDevices.getUserMedia({
                video: { width: 640, height: 480, frameRate: 30 },
                audio: false
            });

            // Setup screen recorder (Logic moved up since we got the stream first)
            screenRecorder.value = new MediaRecorder(screenStream.value, {
                mimeType: 'video/webm;codecs=vp8',
                videoBitsPerSecond: 2000000 // 2 Mbps
            });

            screenRecorder.value.ondataavailable = (event) => {
                if (event.data.size > 0) {
                    screenChunks.value.push(event.data);
                }
            };

            screenRecorder.value.onstop = () => {
                console.log('Screen recorder stopped');
            };

            // Setup camera recorder
            cameraRecorder.value = new MediaRecorder(cameraStream.value, {
                mimeType: 'video/webm;codecs=vp8',
                videoBitsPerSecond: 1000000 // 1 Mbps
            });

            cameraRecorder.value.ondataavailable = (event) => {
                if (event.data.size > 0) {
                    cameraChunks.value.push(event.data);
                }
            };

            cameraRecorder.value.onstop = () => {
                console.log('Camera recorder stopped');
            };

            // Start both recordings
            screenRecorder.value.start(1000);
            cameraRecorder.value.start(1000);
            startTime.value = Math.floor(Date.now() / 1000);
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

    async function stopRecording(options?: RecordingOptions): Promise<void> {
        if (!isRecording.value) return;

        try {
            // Stop recorders
            if (cameraRecorder.value && cameraRecorder.value.state !== 'inactive') {
                cameraRecorder.value.stop();
            }
            if (screenRecorder.value && screenRecorder.value.state !== 'inactive') {
                screenRecorder.value.stop();
            }

            // Wait for dataavailable and onstop to settle chunks
            console.log('Waiting for recording chunks...');
            await new Promise(resolve => setTimeout(resolve, 2000));

            // Explicitly save everything IF options provided
            if (options) {
                console.log('Saving all recordings...');
                await saveCameraRecording(options);
                await saveScreenRecording(options);
                await saveRecordingMetadata(options);
            } else {
                console.log('Stopping recording without saving (no options provided)');
            }

            isRecording.value = false;
            cleanup();
            console.log('Recording stopped successfully');
        } catch (error) {
            console.error('Failed to stop recording:', error);
            cleanup();
        }
    }

    async function saveCameraRecording(options: RecordingOptions): Promise<void> {
        const chunks = toRaw(cameraChunks.value);
        console.log(`DEBUG: saveCameraRecording called for ${options.sessionId}. Chunks: ${chunks.length}`);
        if (chunks.length === 0) {
            console.warn('DEBUG: No camera chunks to save');
            return;
        }

        try {
            const blob = new Blob(chunks, { type: 'video/webm' });
            console.log(`DEBUG: Camera Blob created, size: ${blob.size} bytes`);
            const arrayBuffer = await blob.arrayBuffer();
            const uint8Array = new Uint8Array(arrayBuffer);

            await invoke('save_camera_recording', {
                sessionId: options.sessionId,
                videoData: uint8Array
            });

            console.log('DEBUG: Camera recording saved successfully');
        } catch (error) {
            console.error('DEBUG: Failed to save camera recording:', error);
        }
    }

    async function saveScreenRecording(options: RecordingOptions): Promise<void> {
        const chunks = toRaw(screenChunks.value);
        console.log(`DEBUG: saveScreenRecording called for ${options.sessionId}. Chunks: ${chunks.length}`);
        if (chunks.length === 0) {
            console.warn('DEBUG: No screen chunks to save');
            return;
        }

        try {
            const blob = new Blob(chunks, { type: 'video/webm' });
            console.log(`DEBUG: Screen Blob created, size: ${blob.size} bytes`);
            const arrayBuffer = await blob.arrayBuffer();
            const uint8Array = new Uint8Array(arrayBuffer);

            await invoke('save_screen_recording', {
                sessionId: options.sessionId,
                videoData: uint8Array
            });

            console.log('DEBUG: Screen recording saved successfully');
        } catch (error) {
            console.error('DEBUG: Failed to save screen recording:', error);
        }
    }

    async function saveRecordingMetadata(options: RecordingOptions): Promise<void> {
        try {
            const endTime = Math.floor(Date.now() / 1000);
            const duration = endTime - startTime.value;

            const cameraRawChunks = toRaw(cameraChunks.value);
            const screenRawChunks = toRaw(screenChunks.value);

            const cameraSizeTotal = cameraRawChunks.reduce((acc, chunk) => acc + chunk.size, 0);
            const screenSizeTotal = screenRawChunks.reduce((acc, chunk) => acc + chunk.size, 0);

            await invoke('save_recording_metadata', {
                metadata: {
                    session_id: options.sessionId,
                    user_id: options.userId,
                    username: options.username,
                    test_name: options.testName,
                    event_name: options.eventName || null,
                    started_at: startTime.value,
                    ended_at: endTime,
                    duration: duration,
                    camera_file: 'camera.webm',
                    screen_file: 'screen.webm',
                    camera_size: cameraSizeTotal,
                    screen_size: screenSizeTotal
                }
            });

            console.log(`DEBUG: Recording metadata saved. Camera: ${cameraSizeTotal} bytes, Screen: ${screenSizeTotal} bytes`);
        } catch (error) {
            console.error('DEBUG: Failed to save recording metadata:', error);
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
        stopRecording,
        saveRecordingMetadata
    };
}
