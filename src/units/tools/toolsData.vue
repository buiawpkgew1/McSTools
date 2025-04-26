<script setup lang="ts">
import VueJsonPretty from "vue-json-pretty";
import {defineProps, onMounted, onUnmounted, ref, shallowRef} from "vue";
import {cleanLargeSNBT} from "../../modules/snbt_to_json.ts";
const props = defineProps<{
  data: string | undefined;
}>();


const json_data = shallowRef<JSON | undefined>()
const isJson = ref(false)
const isLoading = ref(false);
const collapsedDepth = ref(2);

const processData = async () => {
  isLoading.value = true;
  if (props.data != undefined){
    try {
      json_data.value = JSON.parse(cleanLargeSNBT(props.data))
      isJson.value = true;
    }finally {
      isLoading.value = false;
    }
  }

};
onMounted(async()=>{
  await processData()
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
          :data="props.data"
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