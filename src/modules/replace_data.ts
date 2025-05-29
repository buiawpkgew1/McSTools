import {invoke} from "@tauri-apps/api/core";
import {toast} from "./others.ts";

export interface BlockId  {
    name: string;
}

export interface BlockDataNew  {
    id: BlockId;
    properties: Record<string, string>;
}
export interface BlockData  {
    id: string;
    properties: Record<string, string>;
}


export interface BlockPos {
    x: number,
    y: number,
    z: number,
}
export async function fetchUniqueBlocks(
    schematicId: number
): Promise<BlockData[]> {
    try {
        const response = await invoke<string>('get_unique_block', { id: schematicId });
        return JSON.parse(response) as BlockData[];
    } catch (error) {
        toast.error(`发生了一个错误:${error}`, {
            timeout: 3000
        });
        throw new Error(`err: ${error}`);
    }

}