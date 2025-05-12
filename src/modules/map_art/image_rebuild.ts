import {RawData, SubData} from "./map_art_data.ts";
import {colorDistance, hexToRgb, loadBlockImages, clamp} from "./image_utils.ts";

export class MapArtProcessor {
    private mapData: RawData[]

    private idToBlockMap: Map<string, SubData>
    private colorToIdsMap: Map<string, Set<string>>

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

    async generatePixelArt(
        sourceImage: HTMLImageElement,
        blockSize: number = 16,
        targetSize?: { width: number; height: number },
        useDithering: boolean = true
    ): Promise<HTMLCanvasElement> {
        const selectedBlocks = this.getSelectedBlocks()
        if (selectedBlocks.length === 0) {
            throw new Error('未选择任何方块')
        }

        const resizedImage = await this.resizeImage(sourceImage, targetSize)

        const { data, width, height } = this.getImageData(resizedImage)
        const blockImages = await loadBlockImages(selectedBlocks)
        const colorTable = this.createColorLookupTable()
        const processedData = useDithering
            ? this.applyDithering(data, width, height, colorTable)
            : data
        const outputCanvas = document.createElement('canvas')
        outputCanvas.width = width * blockSize
        outputCanvas.height = height * blockSize
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
                blockImages
            })
        }

        return outputCanvas
    }

    private async resizeImage(
        image: HTMLImageElement,
        targetSize?: { width: number; height: number }
    ): Promise<HTMLImageElement> {
        return new Promise((resolve) => {
            if (!targetSize ||
                (targetSize.width === image.naturalWidth &&
                    targetSize.height === image.naturalHeight)) {
                resolve(image)
                return
            }

            const canvas = document.createElement('canvas')
            const ctx = canvas.getContext('2d', { willReadFrequently: false })
            if (!ctx) throw new Error('无法创建临时画布')

            canvas.width = targetSize.width
            canvas.height = targetSize.height

            ctx.imageSmoothingQuality = 'high'
            ctx.imageSmoothingEnabled = true

            ctx.drawImage(
                image,
                0, 0, image.naturalWidth, image.naturalHeight,
                0, 0, targetSize.width, targetSize.height
            )

            const resizedImage = new Image()
            resizedImage.onload = () => {
                canvas.remove()
                resolve(resizedImage)
            }
            resizedImage.src = canvas.toDataURL('image/png')
        })
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

    private applyDithering(
        data: Uint8ClampedArray,
        width: number,
        height: number,
        colorTable: Array<{ rgb: { r: number; g: number; b: number }, blockId: string }>
    ): Uint8ClampedArray {
        const buffer = new Uint8ClampedArray(data)

        for (let y = 0; y < height; y++) {
            for (let x = 0; x < width; x++) {
                const idx = (y * width + x) * 4

                const oldR = buffer[idx]
                const oldG = buffer[idx + 1]
                const oldB = buffer[idx + 2]

                const nearest = this.findNearestColor(oldR, oldG, oldB, colorTable)

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
        colorTable: Array<{ rgb: { r: number; g: number; b: number }, blockId: string }>
    ): { r: number; g: number; b: number; blockId: string } {
        let minDistance = Infinity
        let nearestEntry = colorTable[0]

        for (const entry of colorTable) {
            const distance = this.colorDistance(
                r, g, b,
                entry.rgb.r, entry.rgb.g, entry.rgb.b
            )
            if (distance < minDistance) {
                minDistance = distance
                nearestEntry = entry
            }
        }

        return {
            r: nearestEntry.rgb.r,
            g: nearestEntry.rgb.g,
            b: nearestEntry.rgb.b,
            blockId: nearestEntry.blockId
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