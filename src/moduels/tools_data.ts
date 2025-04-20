import {fetchSchematic, SchematicsData} from "./schematics_data.ts";
import {fetchRequirementsWithStats, RequirementStatistics} from "./requirements.ts";
import {ref} from "vue";

export let schematic_id = ref<number | undefined>();
export let schematic_data = ref<SchematicsData | undefined>();
export let schematic_requirements = ref<RequirementStatistics | undefined>();

export const clear_tools = () =>{
    schematic_id.value = undefined
    schematic_data.value = undefined
    schematic_requirements.value = undefined
}

export const fetch_data = async (id: number) => {
    schematic_id.value = id
    schematic_data.value = await fetchSchematic(id)
    schematic_requirements.value = await fetchRequirementsWithStats(id)
}

export const get_data = async (id: number) => {
    if (id == schematic_id.value){
        return schematic_data.value
    }
    clear_tools()
    await fetch_data(id)
    return schematic_data.value
}

export const get_requirements = async (id: number) => {
    if (id == schematic_id.value){
        return schematic_requirements.value
    }
    clear_tools()
    await fetch_data(id)
    return schematic_requirements.value
}