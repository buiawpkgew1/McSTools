import {invoke} from '@tauri-apps/api/core';

export interface SchematicsData {
    id: number,
    name: string,
    description: string,
    schematic_type: number,
    sub_type: number,
    is_deleted: boolean,
    sizes: string,
    user: string,
    is_upload: boolean,
    version: number,
    version_list: string,
    created_at: string,
    updated_at: string,
    game_version: string
}

export let schematicTypeList = {
    1: "香草结构",
    2: "投影蓝图",
    3: "创世神",
    4: "建筑小帮手",
}

interface PaginatedResult<T> {
    data: T[];
    page: number;
    page_size: number;
}

export interface SchematicsParams {
    filter?: string;
    page?: number;
    page_size?: number;
}

export const fetchSchematics = async (
    params: SchematicsParams
): Promise<PaginatedResult<SchematicsData>> => {
    try {
        const result = await invoke<PaginatedResult<SchematicsData>>(
            'get_schematics',
            {
                filter: params.filter || '',
                page: params.page || 1,
                pageSize: params.page_size || 20
            }
        );
        return {
            ...result,
            data: result.data.map(item => ({
                ...item,
            }))
        };
    } catch (error) {
        throw new Error(`获取原理图失败: ${error}`);
    }
}