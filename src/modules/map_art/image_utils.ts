import {SubData} from "./map_art_data.ts";
import {getBlockImg, toast} from "../others.ts";
import {invoke} from "@tauri-apps/api/core";
import {BlockStatePos} from "./schematic_data.ts";

export function hexToRgb(hex: string): { r: number; g: number; b: number } | null {
    const result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex)
    return result ? {
        r: parseInt(result[1], 16),
        g: parseInt(result[2], 16),
        b: parseInt(result[3], 16)
    } : null
}

export interface Size {
    width: number,
    height: number,
    length: number
}

export function colorDistance(
    r1: number, g1: number, b1: number,
    r2: number, g2: number, b2: number
): number {
    const dr = r1 - r2
    const dg = g1 - g2
    const db = b1 - b2
    return dr * dr + dg * dg + db * db
}

export async function loadBlockImages(blocks: SubData[]): Promise<Map<string, HTMLImageElement>> {
    const imageMap = new Map<string, HTMLImageElement>()

    await Promise.all(blocks.map(async (block) => {
        const img = new Image()
        img.src = getBlockImg(block.id)
        await new Promise((resolve, reject) => {
            img.onload = resolve
            img.onerror = reject
        })
        imageMap.set(block.id, img)
    }))

    return imageMap
}

export function clamp(value: number): number {
    return Math.max(0, Math.min(255, Math.round(value)))
}

export const createMapArt = async (
    blocks: BlockStatePos[],
    file_name: string,
    size: Size,
    schematic_type: number,
    sub_version: number
): Promise<boolean> => {
    try {
        return await invoke<boolean>(
            'create_map_art',
            {
                blocks: blocks,
                fileName: file_name,
                size: size,
                schematicType: schematic_type,
                subVersion: sub_version,
            }
        )
    } catch (error) {
        toast.error(`发生了一个错误:${error}`, {
            timeout: 3000
        });
        throw new Error(`创建地图画失败: ${error}`);
    }
}
