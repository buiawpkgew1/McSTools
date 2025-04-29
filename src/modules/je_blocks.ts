import {invoke} from "@tauri-apps/api/core";
import {ref} from "vue";

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
        throw new Error(`err: ${error}`);
    }
}