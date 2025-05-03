import {invoke} from "@tauri-apps/api/core";
import {ref} from "vue";
import {toast} from "./others.ts";

export const jeBlocks = ref<SubData[]>()
export interface SubData {
    zh_cn: string;
    block_name: string;
    block_id: string;
    version_map: Record<number, string>;
}

export async function fetchJeBlocks(
): Promise<SubData[]> {
    try {
        return await invoke<SubData[]>('get_je_blocks')
    } catch (error) {
        toast.error(`发生了一个错误:${error}`, {
            timeout: 3000
        });
        throw new Error(`err: ${error}`);
    }
}