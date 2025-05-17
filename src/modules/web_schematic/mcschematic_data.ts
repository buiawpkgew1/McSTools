import axios from 'axios'

export interface McSchematicData {
    author: string
    authority: number
    avatarUrl: string
    belong: number
    bgType: number
    description: string
    heat: number
    imgs: string
    models: string
    name: string
    nickName: string
    size: string
    tags: string
    type: number
    updateTime: string
    uploadTime: string
    userPrivate: number
    uuid: string
}
export interface McSchematicsParams {
    filter?: string;
    page?: number;
    sort?: boolean;
}
export const fetchMcSchematics = async (
    params: McSchematicsParams
): Promise<McSchematicData[]> => {
    try {
        const pageSize = 15;
        const pageNum = params.page * pageSize;
        const response = await axios.get(
            `https://www.mcschematic.top/api/schematics?begin=${pageNum}&filter=${params.filter}&heatSort=${params.sort}&type=0&t=${Date.now()}`
        );
        console.log(response)
        if (response.status !== 200) {
            throw new Error(`API 请求失败，状态码: ${response.status}`);
        }

        return response.data.map(item => ({
            ...item
        }));

    } catch (error) {
        console.error('获取MC图纸数据失败:', error);
        return [];
    }
};