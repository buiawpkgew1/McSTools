<script setup lang="ts">
import VueJsonPretty from "vue-json-pretty";
import {onMounted, onUnmounted, ref, shallowRef} from "vue";
import {cleanLargeSNBT} from "../../modules/snbt_to_json.ts";
import {fetchSchematicStr, schematic_id} from "../../modules/tools_data.ts";
import {toast} from "../../modules/others.ts";

const json_data = shallowRef<string | undefined>()
const isJson = ref(false)
const isLoading = ref(false);
const collapsedDepth = ref(1);
const data = ref()
const get_schematicStr = async (id: number) => {
  try {
    isLoading.value = true
    data.value = await fetchSchematicStr(id)
    json_data.value = JSON.parse(cleanLargeSNBT(data.value))
    isJson.value = true;
  }catch (e){
    toast.error(`源数据读取失败:${e}`, {
      timeout: 3000
    });
  }finally {
    isLoading.value = false
  }

}
onMounted(async()=>{
  await get_schematicStr(schematic_id.value)
})
onUnmounted(async()=>{
  json_data.value = undefined
  isJson.value = false
})
</script>

<template>
  <div class="data-container">
    <div v-if="isLoading" class="loader-container">
      <v-progress-circular
          indeterminate
          color="primary"
          size="64"
      ></v-progress-circular>
    </div>

    <div v-if="!isLoading" class="json-wrapper">
      <vue-json-pretty
          v-if="isJson"
          :data="json_data"
          :deep="collapsedDepth"
          :height="800"
          :item-height="24"
          virtual
          show-line
      />
      <vue-json-pretty
          v-else
          :data="data"
          :height="800"
          :item-height="24"
          virtual
          show-line
      />
    </div>
  </div>
</template>

<style scoped>
.data-container {
  position: relative;
  height: 100%;
  overflow: hidden;
}

.json-wrapper {
  height: calc(100% - 8px);
  margin: 4px;
  border: 1px solid #eee;
  border-radius: 4px;
}

.loader-container {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  z-index: 2;
}

:deep(.virtual-list) {
  contain: strict;
  will-change: transform;
}

:deep(.vjs-tree__node) {
  contain: content;
}
</style>