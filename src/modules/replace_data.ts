import {invoke} from "@tauri-apps/api/core";

export interface BlockData  {
    id: string;
    properties: Record<string, string>;
}

export async function fetchUniqueBlocks(
    schematicId: number
): Promise<BlockData[]> {
    try {
        const response = await invoke<string>('get_unique_block', { id: schematicId });
        return JSON.parse(response) as BlockData[];
    } catch (error) {
        throw new Error(`err: ${error}`);
    }

}