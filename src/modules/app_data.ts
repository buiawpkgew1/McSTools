import { app } from '@tauri-apps/api';
import {ref} from "vue";

export interface AppData  {
    appName: string;
    appVersion: string;
    tauriVersion: string;
}
export const appData = ref<AppData | undefined>()

export const getAppVersion = async ():Promise<AppData> => {
    try {
        const version = await app.getVersion();
        const name = await app.getName();
        const tauriVersion = await app.getTauriVersion();

        return {
            appName: name,
            appVersion: version,
            tauriVersion
        };
    } catch (error) {
        console.error('获取版本信息失败:', error);
        return {
            appName: 'Unknown',
            appVersion: '0.0.0',
            tauriVersion: 'unknown'
        };
    }
};
