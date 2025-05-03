import {fetchSchematic, SchematicsData} from "./schematics_data.ts";
import {fetchRequirementsWithStats, RequirementStatistics} from "./requirements.ts";
import {ref} from "vue";
import {invoke} from "@tauri-apps/api/core";
import {ConvertData, fetchConvertData} from "./convert_data.ts";
import {BlockData, fetchUniqueBlocks} from "./replace_data.ts";
import {toast} from "./others.ts";

export let schematic_id = ref<number | undefined>();
export let schematicData = ref<SchematicsData | undefined>();
export let convertData = ref<ConvertData | undefined>();
export let schematicRequirements = ref<RequirementStatistics | undefined>();
export let schematicStr = ref<string | undefined>();
export let uniqueBlocks = ref<BlockData[] | undefined>();

export const clear_tools = () =>{
    schematic_id.value = undefined
    schematicData.value = undefined
    schematicRequirements.value = undefined
    convertData.value = undefined
    schematicStr.value = undefined
    uniqueBlocks.value = undefined
}

export const fetch_data = async (id: number) => {
    schematic_id.value = id
    schematicData.value = await fetchSchematic(id)
    convertData.value = await fetchConvertData(id)
    schematicRequirements.value = await fetchRequirementsWithStats(id)
    schematicStr.value = await fetchSchematicStr(id)
    uniqueBlocks.value = await fetchUniqueBlocks(id)
}

export const get_data = async (id: number) => {
    if (id == schematic_id.value){
        return schematicData.value
    }
    clear_tools()
    await fetch_data(id)
    return schematicData.value
}

export const get_requirements = async (id: number) => {
    if (id == schematic_id.value){
        return schematicRequirements.value
    }
    clear_tools()
    await fetch_data(id)
    return schematicRequirements.value
}
export async function fetchSchematicStr(
    schematicId: number
): Promise<string> {
    try {
        return await invoke<string>('get_schematic_data', {id: schematicId})
    } catch (error) {
        toast.error(`发生了一个错误:${error}`, {
            timeout: 3000
        });
        throw new Error(`err: ${error}`);
    }

}