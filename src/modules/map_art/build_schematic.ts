import { BlockDataNew } from "../replace_data.ts";
import { BlockPos, BlockStatePos } from "./schematic_data.ts"

export class BlockStatePosList {
    elements: BlockStatePos[] = [];

    static create(): BlockStatePosList {
        return new BlockStatePosList();
    }

    addBlockByName(pos: BlockPos, name: string): this {
        this.elements.push({
            pos: { ...pos },
            block: { id:{name: `minecraft:${name}`} , properties: {} }
        });
        return this;
    }

    addBlockByNameXY(x:number, y:number, name: string): this {
        this.elements.push({
            pos: { x:x, y:y, z:0 },
            block: { id:{name: `minecraft:${name}`} , properties: {} }
        });
        return this;
    }

    addBlockByNameXZ(x:number, z:number, name: string): this {
        this.elements.push({
            pos: { x:x, y:0, z:z },
            block: { id:{name: `minecraft:${name}`} , properties: {} }
        });
        return this;
    }
    addBlockByNameYZ(y:number, z:number, name: string): this {
        this.elements.push({
            pos: { x:0, y:y, z:z },
            block: { id:{name: `minecraft:${name}`} , properties: {} }
        });
        return this;
    }

    addBlock(pos: BlockPos, block: BlockDataNew): this {
        this.elements.push({
            pos: { ...pos },
            block: { ...block, properties: { ...block.properties } }
        });
        return this;
    }

    addBlocks(blocks: BlockStatePos[]): this {
        blocks.forEach(b => this.addBlock(b.pos, b.block));
        return this;
    }

    removeBlock(pos: BlockPos): boolean {
        const index = this.elements.findIndex(e =>
            e.pos.x === pos.x &&
            e.pos.y === pos.y &&
            e.pos.z === pos.z
        );
        if (index > -1) {
            this.elements.splice(index, 1);
            return true;
        }
        return false;
    }

    serialize(): string {
        return JSON.stringify(this.elements);
    }
}