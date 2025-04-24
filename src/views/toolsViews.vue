<script setup lang="ts">
import {onMounted, ref} from 'vue'
import {onBeforeRouteLeave, useRouter} from "vue-router";
import {isLeaving, navigationGuard} from "../modules/navigation.ts";
import 'vue-json-pretty/lib/styles.css';
import {RequirementStatistics} from "../modules/requirements.ts";
import { activeTab } from "../modules/layout.ts";
import toolsConvert from '../units/tools/toolsConvert.vue';
import toolsReplace from '../units/tools/toolsReplace.vue';
import toolsStats from  '../units/tools/toolsStats.vue';
import toolsData from '../units/tools/toolsData.vue';
import toolsSplit from '../units/tools/toolsSplit.vue';
import {SchematicsData} from "../modules/schematics_data.ts";
import {schematic_id, get_data, get_requirements, get_schematic_str} from "../modules/tools_data.ts"
import {opacity} from "../modules/theme.ts";
const active = ref(0)
const router = useRouter()
const schematicData = ref<SchematicsData | undefined>();
const requirementsData = ref<RequirementStatistics | undefined>();
const schematicStr = ref<string | undefined>()
onBeforeRouteLeave(navigationGuard)
onMounted(async() => {
    if (schematic_id.value != undefined){
        schematicData.value = await get_data(schematic_id.value)
        requirementsData.value = await get_requirements(schematic_id.value)
        schematicStr.value = await get_schematic_str(schematic_id.value)
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
            <span class="text-h5">蓝图工具箱   </span>
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

          <v-tabs v-model="active" align-tabs="center" color="blue-lighten-1" :disabled="schematic_id == undefined">
            <v-tab value="schematic">蓝图详情</v-tab>
            <v-tab value="split">蓝图分割</v-tab>
            <v-tab value="replace">方块替换</v-tab>
            <v-tab value="convert">蓝图转换</v-tab>
            <v-tab value="data">源数据查看</v-tab>
            <v-tab value="stats">材料统计</v-tab>
          </v-tabs>
        </v-toolbar>
        <v-window v-model="active">
          <v-window-item value="schematic">
            <toolsConvert :data="schematicData"/>
          </v-window-item>
          <v-window-item value="split">
            <toolsSplit />
          </v-window-item>
          <v-window-item value="replace">
            <toolsReplace />
          </v-window-item>
          <v-window-item value="convert">
            <v-row class="pa-4" no-gutters>
              <v-col cols="3">
                <v-card class="pa-3" elevation="2">
                  <v-select label="分割方式" :items="['垂直分层', '水平区域', '自定义范围']"/>
                  <v-range-slider label="分割范围" thumb-label min="0" max="256"/>
                  <v-btn block color="green" prepend-icon="mdi-axe">执行分割</v-btn>
                </v-card>
              </v-col>

              <v-col cols="9">
                <v-card class="h-100" elevation="2">
                  <div class="d-flex justify-center align-center h-100 text-grey">
                    3D预览区域
                  </div>
                </v-card>
              </v-col>
            </v-row>
          </v-window-item>
          <v-window-item value="data">
            <toolsData :data="schematicStr"/>
          </v-window-item>
          <v-window-item value="stats">
            <toolsStats :data="requirementsData"/>
          </v-window-item>
        </v-window>
      </v-card>
    </v-col>
  </v-row>

</template>

<style scoped>
.page-wrapper {
  overflow-x: hidden;
  overflow-y: hidden;
}

.animate-row {
  animation: rowEntrance 0.5s cubic-bezier(0.4, 0, 0.2, 1);
}
.animate-row-out {
  animation: rowOut 0.2s cubic-bezier(0.4, 0, 0.2, 1);
}

@keyframes rowOut{
  from {
    transform: translateX(0);
    opacity: 1;
  }
  to {
    transform: translateX(-500px);
    opacity: 0;
  }
}
@keyframes rowEntrance {
  from {
    opacity: 0;
    transform: translateX(-500px);
  }
  to {
    opacity: 1;
    transform: translateX(0);
  }
}
.json-container {
  height: 100%;
  border: 1px solid #e0e0e0;
  border-radius: 4px;
  margin: 8px;
  background: #f5f5f5;
  overflow: auto;
}

</style>