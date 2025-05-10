const encode_image = async (file: File | undefined) => {
    if (!file) return;

    const ext = file.name.split('.').pop()?.toLowerCase() || 'png';
    const name = file.name.split('.').slice(0, -1).join('.');
    const arrayBuffer = await file.arrayBuffer();
    const uint8Array = new Uint8Array(arrayBuffer);

    const { width, height, image } = await new Promise<{
        width: number;
        height: number;
        image: HTMLImageElement
    }>((resolve, reject) => {
        const blob = new Blob([uint8Array], { type: file.type });
        const url = URL.createObjectURL(blob);
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
        data: uint8Array,
        width,
        height,
        image
    };
};

