import {BlockDataNew} from "../replace_data.ts";

export interface BlockPos  {
    x: number;
    y: number;
    z: number;
}

export interface BlockStatePos {
    pos: BlockPos,
    block: BlockDataNew,
}

export interface BlockStatePosList {
    elements: BlockStatePos[];
}