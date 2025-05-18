import {invoke} from "@tauri-apps/api/core";
import {toast} from "./others.ts";


export async function openDev(
) {
    try {
        await invoke('open_dev')
    } catch (error) {
        toast.error(`发生了一个错误:${error}`, {
            timeout: 3000
        });
        throw new Error(`err: ${error}`);
    }
}