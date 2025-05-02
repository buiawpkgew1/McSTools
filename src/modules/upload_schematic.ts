import {ref} from "vue";
import {invoke} from "@tauri-apps/api/core";
import {userData} from "./user_data.ts";

export const files = ref<File[]>([]);
export const uploadStatus = ref<'idle' | 'uploading' | 'success' | 'error'>('idle');
export const uploadError = ref<string | null>(null);
export const progressTimer = ref<number | null>(null)
export const progressValue = ref(100)

export const handleUpload = async (update_id: number) => {
    if (files.value.length === 0) return;

    uploadStatus.value = 'uploading';
    uploadError.value = null;

    try {
        for (const file of files.value) {
            const arrayBuffer = await file.arrayBuffer();
            const uint8Array = new Uint8Array(arrayBuffer);

            await invoke('encode_uploaded_schematic', {
                fileName: file.name,
                data: Array.from(uint8Array),
                update: update_id != -1,
                updateId: update_id
            });
        }

        uploadStatus.value = 'success';
        userData.value.schematics += 1;
        startProgressTimer()
    } catch (err) {
        uploadStatus.value = 'error';
        uploadError.value = err instanceof Error ? err.message : '文件上传失败';
        startProgressTimer()
        console.error('上传错误:', err);
    }
};
const startProgressTimer = () => {
    const duration = 2500
    const interval = 50
    const steps = duration / interval
    let currentStep = 0

    progressTimer.value = window.setInterval(() => {
        currentStep++
        progressValue.value = 100 - (currentStep / steps) * 100

        if (currentStep >= steps) {
            uploadStatus.value = 'idle'
            files.value = []
            if (progressTimer.value) {
                window.clearInterval(progressTimer.value)
            }
        }
    }, interval)
}