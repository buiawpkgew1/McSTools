import {check, Update} from '@tauri-apps/plugin-updater';
import { relaunch } from '@tauri-apps/plugin-process';
import {toast} from "./others.ts";
import {ref} from "vue";

export enum UpdateState {
    Pending,
    Downloading,
    Ready
}
export const restartDialog = ref(false);
export const updateDialog = ref(false);
export const chuckLoading = ref(false)
export const updateProgress = ref(0);
export const updateInfo = ref<Update | null>(null);
export const updateState = ref(UpdateState.Pending);
export const checkUpdate = async (auto: boolean) => {
    chuckLoading.value = true
    try {
        const update = await check();
        if (update) {
            toast.info(`发现新版本: ${update?.version} from ${update?.date} with notes ${update?.body}`, {
                timeout: 3000
            });
            updateInfo.value = update;
            updateState.value = UpdateState.Pending;
            updateDialog.value = true;
        }
        if (!auto && !update){
            toast.info(`未发现新版本`, {
                timeout: 3000
            });
        }
    } catch (error) {
        toast.error(`检查更新失败: ${error}`, {
            timeout: 3000
        });
        console.error('检查更新失败:', error);
    }
    finally {
        chuckLoading.value = false
    }
};

export const confirmUpdate = async () => {
    const delay = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));
    try {
        updateState.value = UpdateState.Downloading;
        const update = await check();

        await update.downloadAndInstall((event) => {
            if (event.event === 'Progress') {

                updateProgress.value += Math.round(
                    event.data.chunkLength
                );
            }
        });

        updateState.value = UpdateState.Ready;
        updateDialog.value = false;
        restartDialog.value = true;
        toast.info(`更新完毕即将重启`, {
            timeout: 3000
        });
        await delay(3000);
        await relaunch();
    } catch (error) {
        toast.error(`检查更新失败: ${error}`, {
            timeout: 3000
        });

        console.error('更新下载失败:', error);
        updateState.value = UpdateState.Pending;
    }
};
