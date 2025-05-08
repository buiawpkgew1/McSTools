import { useToast } from "vue-toastification";
import {open} from "@tauri-apps/plugin-shell";
export const toast = useToast();

export const getBlockIcon = (blockId: string) => {
    const block = blockId.split(':');
    return new URL(`../assets/icon/icon-exports-x32/${block[0]}__${block[1]}.png`, import.meta.url).href
};
export const openLink = async (url: string) => {
    try {
        await open(url)
    } catch (err) {
        console.error('打开链接失败:', err)
    }
}