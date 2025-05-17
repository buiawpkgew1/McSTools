import {invoke} from "@tauri-apps/api/core";

export interface SearchParams {
    filter: string;
    page: number;
    sort: 'up' | 'down';
}
export interface CMSchematicData {
    id: number
    img: string
    name: string
    time: string
    size: string
    desc: string
    author: string
    stress: number
    download: number
    type: string
    versions: {
        type: string
        versions: string[]
    }[]
    tags: string[]
}
export interface SearchResult {
    success: boolean;
    data?: string | CMSchematicData[];
    error?: string;
}
export interface SearchConfig {
    parseHTML?: boolean;
    onError?: (error: string) => void;
}
export async function performSearch(
    params: SearchParams,
    config: SearchConfig = {}
): Promise<SearchResult> {
    const defaultConfig = {
        parseHTML: false,
        ...config
    };

    try {
        const result = await invoke<SearchResult>('perform_search', {
            filter: params.filter,
            page: params.page,
            sort: params.sort
        });

        if (!result.success) {
            throw new Error(result.error || 'Unknown server error');
        }

        let processedData: string | CMSchematicData[] = result.data;
        if (defaultConfig.parseHTML && processedData) {
            processedData = parseSearchResults(processedData as string);
        }

        return {
            ...result,
            data: processedData
        };
    } catch (error) {
        const errorMessage = (error as Error).toString();

        if (defaultConfig.onError) {
            defaultConfig.onError(errorMessage);
        }

        return {
            success: false,
            error: errorMessage
        };
    }
}

function parseSearchResults(html: string): CMSchematicData[] {
    try {
        const parser = new DOMParser();
        const doc = parser.parseFromString(html, 'text/html');

        return Array.from(doc.querySelectorAll<HTMLAnchorElement>('a.list_result')).map(item => {
            const id = parseInt(item.href.match(/\/detail\/(\d+)\//)?.[1] || '0');

            const timeElement = item.querySelector('time[datetime]');

            const baseData = {
                id,
                img: `https://www.creativemechanicserver.com${item.querySelector('img')?.getAttribute('src') || ''}`,
                name: item.querySelector('h2.b.title.oh')?.textContent?.trim() || '',
                time: timeElement?.textContent?.trim() || '',
                size: item.querySelector('.size')?.textContent?.replace('尺寸：', '')?.trim() || '',
                desc: item.querySelector('.desc')?.textContent?.trim() || '',
                author: item.querySelector('.author')?.textContent?.replace('作者：', '')?.trim() || '',
                stress: parseInt(item.querySelector('.stress')?.textContent?.replace('应力：', '') || '0'),
                download: parseInt(item.querySelector('.download')?.textContent?.match(/\d+/)?.[0] || '0'),
                type: item.querySelector('.function').querySelector('.c_box')?.textContent?.trim() || '',
                versions: Array.from(item.querySelectorAll('.f.version .tip_box')).map(ver => ({
                    type: ver.querySelector('.t')?.textContent?.trim(),
                    versions: Array.from(ver.querySelectorAll('.tip div')).map(d => d.textContent?.trim())
                })),
                tags: Array.from(item.querySelectorAll('.io .tag')).map(t => t.textContent?.trim())
            };
            return {
                ...baseData,
            } as CMSchematicData;
        });
    } catch (e) {
        console.error('解析失败:', e);
        return [];
    }
}