import {invoke} from "@tauri-apps/api/core";
import {toast} from "./others.ts";
import {Requirement} from "./requirements.ts";
import {SchematicsData} from "./schematics_data.ts";
import {BlockData} from "./replace_data.ts";

export interface HistoryRecord  {
    schematic: string;
    requirements: string;
    unique_blocks: string;
}

export interface HistoryRecordData  {
    schematic: SchematicsData[];
    requirements: Requirement[];
    unique_blocks: BlockData[][];
}

export function parseHistoryRecord(record: HistoryRecord): HistoryRecordData {
    try {
        const schematic = JSON.parse(record.schematic);
        const requirements = JSON.parse(record.requirements);
        const unique_blocks = JSON.parse(record.unique_blocks);

        return {
            schematic,
            requirements,
            unique_blocks
        };
    } catch (error) {
        throw new Error(`历史记录解析失败: ${error}`);
    }
}
export async function fetchHistoryRecord(
    schematicId: number
): Promise<HistoryRecordData> {
    try {
        const response = await invoke<HistoryRecord>('get_history', { schematicId: schematicId });
        console.log(response)
        return parseHistoryRecord(response);
    } catch (error) {
        toast.error(`发生了一个错误:${error}`, {
            timeout: 3000
        });
        throw new Error(`err: ${error}`);
    }

}