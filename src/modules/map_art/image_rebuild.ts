import {RawData, SubData} from "./map_art_data.ts";
import {clamp, colorDistance, createMapArt, hexToRgb, loadBlockImages} from "./image_utils.ts";
import {BlockStatePosList} from "./build_schematic.ts";

export class MapArtProcessor {
    private mapData: RawData[]

    private idToBlockMap: Map<string, SubData>
    private colorToIdsMap: Map<string, Set<string>>
    private MAX_CANVAS_PIXELS = 4096 * 4096;
    private MAX_DIMENSION = 2048;
    private selectedIds: Set<string>

    constructor(mapData: RawData[], blocks: string[] = []) {
        this.mapData = mapData
        this.selectedIds = new Set(blocks)
        this.idToBlockMap = new Map()
        this.colorToIdsMap = new Map()

        this._buildIndexes()
    }

    private _buildIndexes() {
        this.idToBlockMap.clear()
        this.colorToIdsMap.clear()

        for (const category of this.mapData) {
            for (const item of category.items) {
                this.idToBlockMap.set(item.id, item)

                const normalizedColor = item.average_rgb_hex.toLowerCase()
                if (!this.colorToIdsMap.has(normalizedColor)) {
                    this.colorToIdsMap.set(normalizedColor, new Set())
                }
                this.colorToIdsMap.get(normalizedColor)?.add(item.id)
            }
        }
    }

    updateBlocksData(blocks: string[] = []) {
        this.selectedIds = new Set(blocks)
        this._buildIndexes()
    }
    updateMapData(newData: RawData[]) {
        this.mapData = newData
        this._buildIndexes()
    }

    selectBlock(id: string) {
        if (this.idToBlockMap.has(id)) {
            this.selectedIds.add(id)
        }
    }

    deselectBlock(id: string) {
        this.selectedIds.delete(id)
    }

    toggleBlock(id: string) {
        if (this.selectedIds.has(id)) {
            this.deselectBlock(id)
        } else {
            this.selectBlock(id)
        }
    }

    getSelectedColorMap(): Map<string, string[]> {
        const result = new Map<string, string[]>()

        for (const id of this.selectedIds) {
            const block = this.idToBlockMap.get(id)
            if (!block) continue

            const color = block.average_rgb_hex.toLowerCase()
            const ids = result.get(color) || []
            ids.push(id)
            result.set(color, ids)
        }

        return result
    }

    getSelectedBlocks(): SubData[] {
        return Array.from(this.selectedIds)
            .map(id => this.idToBlockMap.get(id))
            .filter((item): item is SubData => !!item)
    }

    getBlocksByColor(hexColor: string): SubData[] {
        const normalized = hexColor.toLowerCase()
        const ids = this.colorToIdsMap.get(normalized) || new Set()
        return Array.from(ids)
            .map(id => this.idToBlockMap.get(id))
            .filter((item): item is SubData => !!item)
    }

    isValidBlockId(id: string): boolean {
        return this.idToBlockMap.has(id)
    }

    getCategories(): RawData[] {
        return this.mapData
    }

    async exportSchematic(
        sourceImage: HTMLImageElement,
        file_name: string,
        schematic_type: number,
        sub_type: number,
        targetSize?: { width: number; height: number },
        rotation?: 0 | 90 | 180 | 270,
        useDithering: boolean = true,
        replaceAir: boolean = false,
        threeD: boolean = false,
        axios?: 'x' | 'y' | 'z',
    ): Promise<boolean> {
        const resizedImage = await this.resizeImage(sourceImage, targetSize, rotation)

        const { data, width, height } = this.getImageData(resizedImage)
        const colorTable = this.createColorLookupTable()
        const processedData = useDithering
            ? this.applyDithering(data, width, height, threeD, colorTable)
            : data
        const batchSize = 1000
        let blockList = new BlockStatePosList()
        let minZ = Infinity
        let maxZ = -Infinity
        for (let i = 0; i < width * height; i += batchSize) {
            let processSize = await this.processSchematic(i, Math.min(i + batchSize, width * height), {
                data: processedData,
                width,
                height,
                colorTable,
                schematic_type,
                sub_type,
                threeD,
                blockList,
                axios,
                base: {x: 0, y: 0, z: 0},
                replaceAir
            })
            console.log(processSize)
            if (processSize.minZ < minZ){
                minZ = processSize.minZ
            }
            if (processSize.maxZ > maxZ){
                maxZ = processSize.maxZ
            }
        }
        let lastZ = maxZ - minZ
        let size = axios == 'x'? {width: lastZ, height: targetSize.height, length: targetSize.width} : axios == 'y'? {width: targetSize.width, height: lastZ, length: targetSize.height} : {width: targetSize.width, height: targetSize.height, length: lastZ}
        return await createMapArt(
            blockList.elements,
            file_name,
            size,
            schematic_type,
            sub_type
        )
    }
    async generatePixelArt(
        sourceImage: HTMLImageElement,
        blockSize: number = 16,
        targetSize?: { width: number; height: number },
        useDithering: boolean = true,
        replaceAir: boolean = false,
        rotation?: 0 | 90 | 180 | 270,
    ): Promise<HTMLCanvasElement> {
        const selectedBlocks = this.getSelectedBlocks()
        if (selectedBlocks.length === 0) {
            throw new Error('未选择任何方块')
        }

        const resizedImage = await this.resizeImage(sourceImage, targetSize, rotation)

        const { data, width, height } = this.getImageData(resizedImage)
        const blockImages = await loadBlockImages(selectedBlocks)
        const colorTable = this.createColorLookupTable()
        const processedData = useDithering
            ? this.applyDithering(data, width, height, false, colorTable)
            : data
        let outputCanvas = document.createElement('canvas')
        outputCanvas.width = width * blockSize
        outputCanvas.height = height * blockSize
        if (outputCanvas.width * outputCanvas.height >= 16384 * 16384) throw new Error('原始画布过大')
        const ctx = outputCanvas.getContext('2d')
        if (!ctx) throw new Error('无法创建画布上下文')

        const batchSize = 1000
        for (let i = 0; i < width * height; i += batchSize) {
            await this.processBatch(i, Math.min(i + batchSize, width * height), {
                data: processedData,
                width,
                height,
                blockSize,
                ctx,
                colorTable,
                blockImages,
                replaceAir
            })
        }
        if (outputCanvas.width * outputCanvas.height >= 4096 * 4096){
            const scaleFactor = Math.min(
                this.MAX_DIMENSION / outputCanvas.width,
                this.MAX_DIMENSION / outputCanvas.height,
                Math.sqrt(this.MAX_CANVAS_PIXELS / outputCanvas.width * outputCanvas.height)
            );

            const targetSize = {
                width: Math.floor(outputCanvas.width * scaleFactor),
                height: Math.floor(outputCanvas.height * scaleFactor)
            };
            outputCanvas = await this.resizeImageCanvas(outputCanvas, targetSize);
        }

        return outputCanvas
    }

    private async resizeImage(
        image: HTMLImageElement,
        targetSize?: { width: number; height: number },
        rotation?: 0 | 90 | 180 | 270
    ): Promise<HTMLImageElement> {
        return new Promise((resolve) => {

            const [baseWidth, baseHeight] = rotation % 180 === 90
                ? [image.naturalHeight, image.naturalWidth]
                : [image.naturalWidth, image.naturalHeight]

            const finalWidth = targetSize?.width || baseWidth
            const finalHeight = targetSize?.height || baseHeight

            const canvas = document.createElement('canvas')
            const ctx = canvas.getContext('2d', { willReadFrequently: false })
            if (!ctx) throw new Error('无法创建临时画布')

            canvas.width = finalWidth
            canvas.height = finalHeight

            ctx.imageSmoothingQuality = 'high'
            ctx.imageSmoothingEnabled = true

            ctx.save()

            ctx.translate(canvas.width / 2, canvas.height / 2)
            ctx.rotate((rotation * Math.PI) / 180)

            const scaleX = finalWidth / baseWidth
            const scaleY = finalHeight / baseHeight
            const scale = Math.min(scaleX, scaleY)

            ctx.scale(scale, scale)
            ctx.drawImage(
                image,
                -baseWidth / 2,
                -baseHeight / 2,
                image.naturalWidth,
                image.naturalHeight
            )

            ctx.restore()

            const resizedImage = new Image()
            resizedImage.onload = () => {
                canvas.remove()
                resolve(resizedImage)
            }
            resizedImage.onerror = (err) => {
                console.error('图像加载失败:', err)
                canvas.remove()
                resolve(image)
            }
            resizedImage.src = canvas.toDataURL('image/png')
        })
    }

    private async resizeImageCanvas(
        originalCanvas: HTMLCanvasElement,
        targetSize?: { width: number; height: number }
    ): Promise<HTMLCanvasElement> {
        if (!targetSize ||
            (targetSize.width === originalCanvas.width &&
                targetSize.height === originalCanvas.height)) {
            return originalCanvas.cloneNode(true) as HTMLCanvasElement;
        }

        const sourceCanvas = document.createElement('canvas');
        const sourceCtx = sourceCanvas.getContext('2d');
        if (!sourceCtx) throw new Error('Failed to create source canvas context');

        sourceCanvas.width = originalCanvas.width;
        sourceCanvas.height = originalCanvas.height;
        sourceCtx.drawImage(originalCanvas, 0, 0);

        const targetCanvas = document.createElement('canvas');
        targetCanvas.width = targetSize.width;
        targetCanvas.height = targetSize.height;

        const targetCtx = targetCanvas.getContext('2d', {
            willReadFrequently: false
        });
        if (!targetCtx) throw new Error('Failed to create target canvas context');

        targetCtx.imageSmoothingEnabled = true;
        targetCtx.imageSmoothingQuality = 'high';

        targetCtx.drawImage(
            sourceCanvas,
            0, 0, sourceCanvas.width, sourceCanvas.height,
            0, 0, targetSize.width, targetSize.height
        );

        sourceCanvas.width = 0;
        sourceCanvas.height = 0;

        return targetCanvas;
    }

    private getImageData(image: HTMLImageElement): ImageData {
        const canvas = document.createElement('canvas')
        const ctx = canvas.getContext('2d')
        if (!ctx) throw new Error('无法创建临时画布')

        canvas.width = image.naturalWidth
        canvas.height = image.naturalHeight
        ctx.drawImage(image, 0, 0)
        return ctx.getImageData(0, 0, canvas.width, canvas.height)
    }

    private createColorLookupTable(): Array<{
        rgb: { r: number; g: number; b: number }
        blockId: string
    }> {
        const table: Array<{
            rgb: { r: number; g: number; b: number }
            blockId: string
        }> = []

        const colorMap = this.getSelectedColorMap()
        for (const [hex, blockIds] of colorMap) {
            const rgb = hexToRgb(hex)
            if (rgb && blockIds.length > 0) {
                table.push({ rgb, blockId: blockIds[0] })
            }
        }

        return table
    }

    private async processSchematic(
        start: number,
        end: number,
        context: {
            data: Uint8ClampedArray
            width: number
            height: number
            colorTable: Array<{ rgb: { r: number; g: number; b: number }, blockId: string }>
            schematic_type: number,
            sub_type: number,
            threeD: boolean,
            blockList: BlockStatePosList
            axios?: 'x' | 'y' | 'z',
            base?: { x: number; y: number; z: number },
            flipX?: boolean,
            flipY?: boolean
            replaceAir: boolean
        }
    ) {
        let {
            width,
            height,
            axios = 'y',
            base = { x: 0, y: 0, z: 0 },
            flipX = false,
            flipY = false
        } = context;
        let maxZ = -Infinity
        let minZ = Infinity
        let lastZ = base.z;
        for (let i = start; i < end; i++) {
            const rawX = i % width;
            const rawY = Math.floor(i / width);
            if (rawX == 0) lastZ = base.z;
            let imageX = rawX, imageY = rawY;
            switch(axios.toLowerCase()) {
                case 'x':
                    imageY = height - rawY - 1;
                    break;
                case 'y':
                    break;
                case 'z':
                    imageY = height - rawY - 1;
                    break;
            }

            imageX = flipX ? width - imageX - 1 : imageX;
            imageY = flipY ? height - imageY - 1 : imageY;

            let x3d: number, y3d: number, z3d: number;

            switch(axios.toLowerCase()) {
                case 'x':
                    x3d = base.x;
                    y3d = base.y + imageY;
                    z3d = base.z + imageX;
                    break;
                case 'y':
                    x3d = base.x + imageX;
                    y3d = base.y;
                    z3d = base.z + imageY;
                    break;
                case 'z':
                    x3d = base.x + imageX;
                    y3d = base.y + imageY;
                    z3d = base.z;
                    break;
            }

            const index = i * 4;
            const [r, g, b] = context.data.slice(index, index + 3);

            let minDistance = Infinity;
            let closestBlockId = '';
            if (context.data[index + 3] === 0 && context.replaceAir) {
                context.blockList.addBlockByPos(x3d, y3d, z3d, 'air');
                continue;
            }
            const threeDLayers = [
                { brightness: 255, zOffset: 1 },
                { brightness: 180, zOffset: -1 },
                { brightness: 220, zOffset: 0 }

            ];
            let tempZ = 0
            if(context.threeD){
                for (const layer of threeDLayers) {
                    for (const entry of context.colorTable) {
                        const adjustedColor = {
                            r: Math.round(entry.rgb.r * (layer.brightness / 255)),
                            g: Math.round(entry.rgb.g * (layer.brightness / 255)),
                            b: Math.round(entry.rgb.b * (layer.brightness / 255))
                        };
                        const distance = colorDistance(
                            r, g, b,
                            adjustedColor.r,
                            adjustedColor.g,
                            adjustedColor.b
                        );
                        if (distance < minDistance) {
                            minDistance = distance;
                            closestBlockId = entry.blockId;
                            tempZ = layer.zOffset;
                        }
                    }
                }
            }else {
                for (const entry of context.colorTable) {
                    const distance = colorDistance(r, g, b, entry.rgb.r, entry.rgb.g, entry.rgb.b);
                    if (distance < minDistance) {
                        minDistance = distance;
                        closestBlockId = entry.blockId;
                    }
                }
            }
            lastZ = lastZ + tempZ;
            if (lastZ < minZ){
                minZ = lastZ
            }
            if (lastZ > maxZ){
                maxZ = lastZ
            }
            if (closestBlockId) {
                switch(axios.toLowerCase()) {
                    case 'x':
                        context.blockList.addBlockByPos(lastZ, y3d, z3d, closestBlockId);
                        break;
                    case 'y':
                        context.blockList.addBlockByPos(x3d, lastZ, z3d, closestBlockId);
                        break;
                    case 'z':
                        context.blockList.addBlockByPos(x3d, y3d, lastZ, closestBlockId);
                        break;
                }
            }
        }
        return({minZ, maxZ})
    }
    private async processBatch(
        start: number,
        end: number,
        context: {
            data: Uint8ClampedArray
            width: number
            height: number
            blockSize: number
            ctx: CanvasRenderingContext2D
            colorTable: Array<{ rgb: { r: number; g: number; b: number }, blockId: string }>
            blockImages: Map<string, HTMLImageElement>
            replaceAir: boolean
        }
    ) {
        for (let i = start; i < end; i++) {
            const x = i % context.width
            const y = Math.floor(i / context.width)
            const index = i * 4

            const r = context.data[index]
            const g = context.data[index + 1]
            const b = context.data[index + 2]


            let minDistance = Infinity
            let closestBlockId = ''
            for (const entry of context.colorTable) {
                const distance = colorDistance(
                    r, g, b,
                    entry.rgb.r, entry.rgb.g, entry.rgb.b
                )
                if (distance < minDistance) {
                    minDistance = distance
                    closestBlockId = entry.blockId
                }
            }
            if (context.data[index + 3] == 0 && context.replaceAir) {
                context.ctx.fillStyle = 'rgba(255, 255, 255, 0.5)';
                context.ctx.fillRect(
                    x * context.blockSize,
                    y * context.blockSize,
                    context.blockSize,
                    context.blockSize
                );
            }else {
                if (closestBlockId) {
                    const img = context.blockImages.get(closestBlockId)
                    if (img) {
                        context.ctx.drawImage(
                            img,
                            x * context.blockSize,
                            y * context.blockSize,
                            context.blockSize,
                            context.blockSize
                        )
                    }
                }
            }

        }
    }

    private applyDithering(
        data: Uint8ClampedArray,
        width: number,
        height: number,
        threeD: boolean = false,
        colorTable: Array<{ rgb: { r: number; g: number; b: number }, blockId: string }>
    ): Uint8ClampedArray {
        const buffer = new Uint8ClampedArray(data)

        for (let y = 0; y < height; y++) {
            for (let x = 0; x < width; x++) {
                const idx = (y * width + x) * 4

                const oldR = buffer[idx]
                const oldG = buffer[idx + 1]
                const oldB = buffer[idx + 2]

                const nearest = this.findNearestColor(oldR, oldG, oldB, threeD, colorTable)

                buffer[idx] = nearest.r
                buffer[idx + 1] = nearest.g
                buffer[idx + 2] = nearest.b

                const errR = oldR - nearest.r
                const errG = oldG - nearest.g
                const errB = oldB - nearest.b

                if (x < width - 1) {
                    this.diffuseError(buffer, idx + 4, errR, errG, errB, 7/16)
                }
                if (y < height - 1) {
                    if (x > 0) {
                        this.diffuseError(buffer, idx + width * 4 - 4, errR, errG, errB, 3/16)
                    }
                    this.diffuseError(buffer, idx + width * 4, errR, errG, errB, 5/16)
                    if (x < width - 1) {
                        this.diffuseError(buffer, idx + width * 4 + 4, errR, errG, errB, 1/16)
                    }
                }
            }
        }
        return buffer
    }

    private diffuseError(
        buffer: Uint8ClampedArray,
        targetIdx: number,
        errR: number,
        errG: number,
        errB: number,
        factor: number
    ) {
        buffer[targetIdx] = clamp(buffer[targetIdx] + errR * factor)
        buffer[targetIdx + 1] = clamp(buffer[targetIdx + 1] + errG * factor)
        buffer[targetIdx + 2] = clamp(buffer[targetIdx + 2] + errB * factor)
    }


    private findNearestColor(
        r: number,
        g: number,
        b: number,
        threeD: boolean = false,
        colorTable: Array<{ rgb: { r: number; g: number; b: number }, blockId: string }>
    ): { r: number; g: number; b: number; blockId: string } {
        let minDistance = Infinity
        let nearestEntry:{ r: number; g: number; b: number }
        let blockId :string
        const threeDLayers = [
            { brightness: 255 },
            { brightness: 180 },
            { brightness: 220 }

        ];
        if (threeD){
            threeDLayers.forEach(layer => {
                for (const entry of colorTable) {
                    const adjustedColor = {
                        r: Math.round(entry.rgb.r * (layer.brightness / 255)),
                        g: Math.round(entry.rgb.g * (layer.brightness / 255)),
                        b: Math.round(entry.rgb.b * (layer.brightness / 255))
                    };
                    const distance = this.colorDistance(
                        r, g, b,
                        adjustedColor.r, adjustedColor.g, adjustedColor.b
                    )
                    if (distance < minDistance) {
                        minDistance = distance
                        nearestEntry = adjustedColor
                        blockId = entry.blockId
                    }
                }
            })
        }else {
            for (const entry of colorTable) {
                const distance = this.colorDistance(
                    r, g, b,
                    entry.rgb.r, entry.rgb.g, entry.rgb.b
                )
                if (distance < minDistance) {
                    minDistance = distance
                    nearestEntry = {
                        r: entry.rgb.r,
                        g: entry.rgb.g,
                        b: entry.rgb.b,
                    }
                    blockId = entry.blockId
                }
            }
        }

        return {
            r: nearestEntry.r,
            g: nearestEntry.g,
            b: nearestEntry.b,
            blockId: blockId
        }
    }

    private colorDistance(
        r1: number, g1: number, b1: number,
        r2: number, g2: number, b2: number
    ): number {
        const dr = r1 - r2
        const dg = g1 - g2
        const db = b1 - b2
        return dr * dr + dg * dg + db * db
    }
}