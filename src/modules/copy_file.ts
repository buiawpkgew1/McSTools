import {invoke} from "@tauri-apps/api/core";
import {save} from "@tauri-apps/plugin-dialog";
import {toast} from "./others.ts";
import { appConfigDir } from '@tauri-apps/api/path';
import { openPath } from '@tauri-apps/plugin-opener';
const getExtensions = (type: number): string[] => {
    switch (type) {
        case 1: return ['nbt'];
        case 2: return ['litematic'];
        case 3: return ['schem'];
        case 4: return ['json'];
        case 5: return ['mcstruct'];
        default: return ['unknown'];
    }
};

export const copySchematic = async (id: number, sub: number, version: number, type: number) => {
    try {
        const path = await save({
            filters: [
                {
                    name: 'Schematic File',
                    extensions: getExtensions(type),
                },
            ],
        });

        if (!path) {
            toast.error(`未选择目标目录`, { timeout: 3000 });
            return;
        }

        const result = await invoke('copy_schematic', {
            id: id,
            sub: sub,
            version: version,
            vType: type,
            target: path
        });

        if (result) {
            toast.success(`复制成功！`, { timeout: 3000 });
        }
    } catch (error) {
        console.error('复制失败:', error);
        toast.error(`复制失败: ${error}`, { timeout: 3000 });
    }
};

export const openData = async () => {
    try {
        const appConfigDirPath = await appConfigDir();
        await openPath(appConfigDirPath);
    } catch (error) {
        console.error('打开失败:', error);
        toast.error(`打开失败: ${error}`, { timeout: 3000 });
    }
};