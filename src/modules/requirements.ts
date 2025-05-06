import {invoke} from "@tauri-apps/api/core";
import {toast} from "./others.ts";
export interface Requirement {
    id: string,
    zh_cn: string,
    num: number
}

export interface RequirementStatistic extends Requirement {
    count: number;
    percentage: number;
}

export interface RequirementStatistics {
    total: number;
    items: RequirementStatistic[];
}
export function parseRequirements(jsonStr: string): Requirement[] {
    const rawData = JSON.parse(jsonStr) as Record<string, Requirement>;
    return Object.values(rawData).map(block => ({
        id: block.id,
        zh_cn: block.zh_cn,
        num: block.num
    }));
}

export function parseRequirementsList(jsonStr: string): Requirement[][] {
    const rawData = JSON.parse(jsonStr) as Record<string, Requirement>[];
    return rawData.map(data =>
        Object.values(data).map(block => ({
            id: block.id,
            zh_cn: block.zh_cn,
            num: block.num
        }))
    );
}
export function calculateStatistics(requirements: Requirement[]): RequirementStatistics {
    const total = requirements.reduce((sum, req) => sum + req.num, 0);

    if (total === 0) {
        return {
            total: 0,
            items: []
        };
    }

    const items = requirements.map(req => {
        const count = req.num;
        return {
            ...req,
            count,
            percentage: Number(((count / total) * 100).toFixed(3))
        };
    });
    return {
        total,
        items
    };
}

export async function fetchRequirementsWithStats(
    schematicId: number
): Promise<RequirementStatistics> {
    try {
        const response = await invoke<string>('get_schematic_requirements', { id: schematicId });
        const requirements = parseRequirements(response);
        return calculateStatistics(requirements);
    } catch (error) {
        toast.error(`发生了一个错误:${error}`, {
            timeout: 3000
        });
        throw new Error(`err: ${error}`);
    }

}