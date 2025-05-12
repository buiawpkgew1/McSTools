import {ref} from "vue";

export const image_data = ref<ProcessedImage>()
export const encode_image = async (file: File | undefined): Promise<ProcessedImage> => {
    if (!file) return;

    const ext = file.name.split('.').pop()?.toLowerCase() || 'png';
    const name = file.name.split('.').slice(0, -1).join('.');
    const arrayBuffer = await file.arrayBuffer();
    const uint8Array = new Uint8Array(arrayBuffer);

    const mimeTypeMap: Record<string, string> = {
        png: 'image/png',
        jpg: 'image/jpeg',
        jpeg: 'image/jpeg',
        bmp: 'image/bmp',
        gif: 'image/gif',
        webp: 'image/webp'
    };

    const base64String = await new Promise<string>((resolve, reject) => {
        const reader = new FileReader();
        reader.onload = () => {
            const dataUrl = reader.result as string;
            const base64Data = dataUrl.split(',')[1];
            resolve(base64Data);
        };
        reader.onerror = reject;
        reader.readAsDataURL(new Blob([arrayBuffer], { type: file.type }));
    });

    const mimeType = mimeTypeMap[ext] || file.type || 'application/octet-stream';

    const { width, height, image } = await new Promise<{
        width: number;
        height: number;
        image: HTMLImageElement
    }>((resolve, reject) => {
        const url = URL.createObjectURL(new Blob([uint8Array]));
        const img = new Image();

        img.onload = () => {
            URL.revokeObjectURL(url);
            resolve({
                width: img.naturalWidth,
                height: img.naturalHeight,
                image: img
            });
        };

        img.onerror = () => {
            URL.revokeObjectURL(url);
            reject(new Error('图片加载失败'));
        };

        img.src = url;
    });
    return {
        name,
        ext,
        rawData: uint8Array,
        width,
        height,
        image,
        base64: `data:${mimeType};base64,${base64String}`
    };
};

export interface ProcessedImage {
    name: string;
    ext: string;
    rawData: Uint8Array;
    width: number;
    height: number;
    image: HTMLImageElement;
    base64: string;
}