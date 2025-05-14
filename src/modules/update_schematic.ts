import {invoke} from "@tauri-apps/api/core";
import {toast} from "./others.ts";

export const update_schematic_name = async (
    id: number,
    name: string,
    description: string
): Promise<boolean> => {
    try {
        return await invoke<boolean>(
            'update_schematic_name_description',
            {
                schematicId: id,
                name: name,
                description: description,
            }
        )
    } catch (error) {
        toast.error(`发生了一个错误:${error}`, {
            timeout: 3000
        });
        throw new Error(`发生了一个错误: ${error}`);
    }
}