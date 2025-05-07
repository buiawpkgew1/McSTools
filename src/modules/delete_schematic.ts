import {invoke} from "@tauri-apps/api/core";
import {toast} from "./others.ts";
export const delete_schematic = async (
    id: number
): Promise<boolean> => {
    try {
        return await invoke<boolean>(
            'delete_schematic',
            {
                id: id,
            }
        )
    } catch (error) {
        toast.error(`发生了一个错误:${error}`, {
            timeout: 3000
        });
        throw new Error(`获取原理图失败: ${error}`);
    }
}