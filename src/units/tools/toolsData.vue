<script setup lang="ts">
import VueJsonPretty from "vue-json-pretty";
import {onMounted, onUnmounted, ref} from "vue";
import {cleanLargeSNBT} from "../../modules/snbt_to_json.ts";
import {fetchSchematicStr, schematic_id} from "../../modules/tools_data.ts";
import {toast} from "../../modules/others.ts";
import {data, json_data} from "../../modules/toolsData_data.ts"
const isJson = ref(false)
const isLoading = ref(false);
const collapsedDepth = ref(1);

const get_schematicStr = async (id: number) => {
  try {
    isLoading.value = true
    data.value = await fetchSchematicStr(id)
    json_data.value = await JSON.parse(cleanLargeSNBT(data.value))
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
    <div v-if="isLoading" class="loading-overlay">
      <div class="loader">
        <div class="spinner"></div>
        <p>加载结构中...</p>
      </div>
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
.spinner {
  width: 40px;
  height: 40px;
  border: 4px solid #f3f3f3;
  border-top: 4px solid #3498db;
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

.loading-overlay {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: rgba(255, 255, 255, 0.8);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 999;
}
.loader {
  text-align: center;
}
</style>