<script setup lang="ts">
import {onMounted} from 'vue'
import {onBeforeRouteLeave, useRouter} from "vue-router";
import {isLeaving, navigationGuard} from "../modules/navigation.ts";
import { activeTab } from "../modules/layout.ts"
import 'vue-json-pretty/lib/styles.css';
import toolsConvert from '../units/tools/toolsConvert.vue';
import toolsReplace from '../units/tools/toolsReplace.vue';
import toolsStats from  '../units/tools/toolsStats.vue';
import toolsData from '../units/tools/toolsData.vue';
import toolsSchematic from '../units/tools/toolsSchematic.vue';
import toolsHistory from '../units/tools/toolsHistory.vue'
import toolsSplit from '../units/tools/toolsSplit.vue';
import ToolsThreeD from "../units/tools/toolsThreeD.vue";

import {
  schematic_id,
  fetch_data,
  schematicData,
  schematicRequirements,
  convertData,
  uniqueBlocks,
  historyRecordData, activeTools
} from "../modules/tools_data.ts"
import {opacity} from "../modules/theme.ts";
const router = useRouter()
onBeforeRouteLeave(navigationGuard)
onMounted(async() => {
    if (schematic_id.value != undefined){
        await fetch_data(schematic_id.value)
    }
})

</script>

<template class="page-wrapper">
  <v-row no-gutters
         class="mb-4 animate-row"
         :class="{ 'animate-row-out': isLeaving }"
  >
    <v-col>
      <v-card class="mx-auto overflow-auto h-auto v-theme--custom text-primary" :style="{ '--surface-alpha': opacity }" elevation="4" >
        <v-toolbar density="compact" class="bg-blue-grey-lighten-5 pa-4">
          <v-toolbar-title>
            <v-icon icon="mdi-tools" class="mr-2"></v-icon>
            <span class="text-h5">蓝图编辑   </span>
            <v-chip
                v-if="schematic_id != undefined"
                color="orange-lighten-4"
                class="text-orange-darken-4"
            >
              <v-icon start icon="mdi-cube"></v-icon>
              蓝图ID:{{ schematic_id }}
            </v-chip>
            <v-chip
                v-else
                color="red-orange-darken-4"
            >
              <v-icon start icon="mdi-cube"></v-icon>
              未选取目标蓝图
            </v-chip>
          </v-toolbar-title>
          <v-btn variant="text" icon="mdi-cloud-upload" title="上传蓝图" @click="router.push('/home');activeTab = 'home'"/>

          <v-divider vertical inset class="mx-4"/>

          <v-tabs v-model="activeTools"
                  align-tabs="center"
                  color="blue-lighten-1"
                  :disabled="schematic_id == undefined"
          >
            <v-tab value="schematic">蓝图详情</v-tab>
            <v-tab value="history">版本管理</v-tab>
            <v-tab value="split">蓝图分割</v-tab>
            <v-tab value="replace">方块替换</v-tab>
            <v-tab value="convert">蓝图转换</v-tab>
            <v-tab value="data">源数据查看</v-tab>
            <v-tab value="stats">材料统计</v-tab>
            <v-tab value="threeD">结构预览</v-tab>
          </v-tabs>
        </v-toolbar>
        <v-window v-model="activeTools">
          <v-window-item value="schematic">
            <toolsSchematic :data="schematicData"/>
          </v-window-item>
          <v-window-item value="history">
            <toolsHistory :data="historyRecordData"/>
          </v-window-item>
          <v-window-item value="split">
            <toolsSplit />
          </v-window-item>
          <v-window-item value="replace">
            <toolsReplace
                :data="schematicRequirements"
                :blocks="uniqueBlocks"
            />
          </v-window-item>
          <v-window-item value="convert">
            <toolsConvert :data="convertData"/>
          </v-window-item>
          <v-window-item value="data" >
            <toolsData/>
          </v-window-item>
          <v-window-item value="stats">
            <toolsStats :data="schematicRequirements"/>
          </v-window-item>
          <v-window-item value="threeD">
            <tools-three-d />
          </v-window-item>
        </v-window>
      </v-card>
    </v-col>
  </v-row>

</template>

<style scoped>

</style>