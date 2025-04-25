import {invoke} from "@tauri-apps/api/core";
type SchematicType = "Create" | "Litematic" | "Bg" | "We" | "Be";

interface Target {
    has: boolean;
    size: string;
    version: number;
}

export interface ConvertData {
    schematic_type: SchematicType;
    schematic_type_id: number;
    sub_type: number;
    version: number;
    size: string;
    schematics: Record<SchematicType, Record<number, Target>>;
}

export const fetchConvertData = async (id: number):Promise<ConvertData> => {
    try {
        return await invoke('get_schematic_convert_data', {
            id: id,
        })
    } catch (err) {
        throw new Error(` ${err}`);
    }
}