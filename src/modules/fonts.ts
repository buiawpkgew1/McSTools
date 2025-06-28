import {BaseDirectory, exists, mkdir, readFile, writeFile} from "@tauri-apps/plugin-fs";
import {path} from "@tauri-apps/api";
import {toast} from "./others.ts";

export const saveFont = async (file: File | undefined) => {
    if (!file) return;

    await mkdir("fonts", {
        baseDir: BaseDirectory.AppData,
        recursive: true
    });

    const ext = file.name.split('.').pop()?.toLowerCase() || 'ttf';
    const filename = `font.${ext}`;

    const arrayBuffer = await file.arrayBuffer();
    const uint8Array = new Uint8Array(arrayBuffer);

    await writeFile(
        `fonts/${filename}`,
        uint8Array,
        { baseDir: BaseDirectory.AppData }
    );

    const fullPath = await getFullPath(filename);

    return {
        success: true,
        path: fullPath,
        name: filename,
        size: uint8Array.length / 1024,
    };
};

const getFullPath = async (filename: string) => {
    return await path.join(
        'fonts',
        filename
    );
};
const getMimeType = (path: string): string => {
    const ext = path?.split('.').pop()?.toLowerCase() || '';
    switch (ext) {
        case 'png': return 'image/png';
        case 'jpg':
        case 'jpeg': return 'image/jpeg';
        case 'webp': return 'image/webp';

        case 'ttf': return 'font/ttf';
        case 'otf': return 'font/otf';
        case 'woff': return 'font/woff';
        case 'woff2': return 'font/woff2';
        case 'eot': return 'application/vnd.ms-fontobject';

        default: return 'application/octet-stream';
    }
}

export const getFontUrl = async (path: string) => {
    try {
        const fileExists = await exists(path, {
            baseDir: BaseDirectory.AppData
        });

        if (!fileExists) return null;

        const buffer = await readFile(path, {
            baseDir: BaseDirectory.AppData
        });
        const blob = new Blob([buffer], { type: getMimeType(path) });
        return URL.createObjectURL(blob);

    } catch (error) {
        console.error('字体加载失败:', error);
        toast.error(`发生了一个错误:${error}`, {
            timeout: 3000
        });
        return null;
    }
};

export const getFontBase64Url = async (path: string) => {
    try {
        const fileExists = await exists(path, {
            baseDir: BaseDirectory.AppData
        });

        if (!fileExists) return null;

        const buffer = await readFile(path, {
            baseDir: BaseDirectory.AppData
        });

        const base64String = arrayBufferToBase64(buffer);

        return `data:${getMimeType(path)};base64,${base64String}`;

    } catch (error) {
        console.error('字体加载失败:', error);
        toast.error(`发生了一个错误:${error}`, {
            timeout: 3000
        });
        return null;
    }
};

const arrayBufferToBase64 = (buffer: Uint8Array) => {
    let binary = '';
    const bytes = new Uint8Array(buffer);
    const len = bytes.byteLength;

    for (let i = 0; i < len; i++) {
        binary += String.fromCharCode(bytes[i]);
    }

    return window.btoa(binary);
};