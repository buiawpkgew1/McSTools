import {invoke} from "@tauri-apps/api/core";
import {toast} from "../others.ts";
import {ref} from "vue";

export interface BlockColorData {
    low_rgb: number[];
    normal_rgb: number[];
    high_rgb: number[];
    lowest_rgb: number[];
    average_rgb: number[];
    average_rgb_hex: string;
    zh_cn: string;
}

export const mapArtData = ref<CategoryBlocks>()

export type CategoryBlocks = Record<string, Record<string, BlockColorData>>;

export async function fetchMapArtsData(
): Promise<CategoryBlocks> {
    try {
        return await invoke<CategoryBlocks>('get_map_arts')
    } catch (error) {
        toast.error(`发生了一个错误:${error}`, {
            timeout: 3000
        });
        throw new Error(`err: ${error}`);
    }
}