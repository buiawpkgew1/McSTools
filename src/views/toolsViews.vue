<script setup lang="ts">
import { ref } from 'vue'
import {onBeforeRouteLeave} from "vue-router";
import {isLeaving, navigationGuard} from "../moduels/navigation.ts";
const activeTab = ref(0)
const blueprintData = ref({})
const materialStats = ref([])
const replacementRules = ref([])

const sampleMaterials = [
  { name: '橡木木板', id: 'minecraft:oak_planks', count: 1520 },
  { name: '圆石', id: 'minecraft:cobblestone', count: 845 },
  { name: '红石粉', id: 'minecraft:redstone', count: 328 }
]
onBeforeRouteLeave(navigationGuard)

</script>

<template class="page-wrapper">
  <v-row no-gutters
         class="mb-4 animate-row"
         :class="{ 'animate-row-out': isLeaving }"
  >
    <v-col>
      <v-card class="mx-auto" elevation="4" style="height: 99vh">
        <v-toolbar density="compact" class="bg-blue-grey-lighten-5 pa-4">
          <v-toolbar-title>
            <v-icon icon="mdi-tools" class="mr-2"></v-icon>
            <span class="text-h5">蓝图工具箱</span>
          </v-toolbar-title>
          <v-btn variant="text" icon="mdi-cloud-upload" title="上传蓝图"/>
          <v-btn variant="text" icon="mdi-content-save" title="保存配置"/>

          <v-divider vertical inset class="mx-4"/>

          <v-tabs v-model="activeTab" align-tabs="center" color="blue-lighten-1">
            <v-tab value="schem">蓝图详情</v-tab>
            <v-tab value="split">蓝图分割</v-tab>
            <v-tab value="replace">方块替换</v-tab>
            <v-tab value="convert">蓝图转换</v-tab>
            <v-tab value="data">源数据查看</v-tab>
            <v-tab value="stats">材料统计</v-tab>
          </v-tabs>
        </v-toolbar>
        <v-window v-model="activeTab">
          <v-window-item value="split">
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
          <v-window-item value="replace">
            <v-row class="pa-4" no-gutters>
              <v-col cols="4">
                <v-card class="pa-3" elevation="2">
                  <div class="d-flex align-center mb-4">
                    <v-icon icon="mdi-magnify" class="mr-2"/>
                    <v-combobox label="查找方块" :items="['橡木木板', '圆石', '红石粉']"/>
                  </div>

                  <v-divider class="my-4"/>

                  <div class="d-flex align-center">
                    <v-icon icon="mdi-arrow-right" class="mx-2"/>
                    <v-combobox label="替换为" :items="['云杉木板', '石砖', '红石块']"/>
                  </div>

                  <v-checkbox label="全局替换" density="compact"/>
                  <v-btn block color="orange" prepend-icon="mdi-swap-horizontal">执行替换</v-btn>
                </v-card>
              </v-col>

              <v-col cols="8">
                <v-card class="h-100" elevation="2">
                  <v-table density="compact">
                    <thead>
                    <tr>
                      <th>原方块</th>
                      <th>新方块</th>
                      <th>数量</th>
                      <th>操作</th>
                    </tr>
                    </thead>
                    <tbody>
                    <tr v-for="(rule, index) in replacementRules" :key="index">
                      <td>{{ rule.original }}</td>
                      <td>{{ rule.replacement }}</td>
                      <td>{{ rule.count }}</td>
                      <td><v-btn icon="mdi-delete" variant="text" color="red"/></td>
                    </tr>
                    </tbody>
                  </v-table>
                </v-card>
              </v-col>
            </v-row>
          </v-window-item>
          <v-window-item value="data">
            <v-card class="ma-4" elevation="2">
              <v-card-text>
                <v-textarea
                    auto-grow
                    readonly
                    variant="outlined"
                    value="{/* 示例NBT数据 */}"
                    rows="15"
                />
              </v-card-text>
            </v-card>
          </v-window-item>
          <v-window-item value="stats">
            <v-card class="ma-4" elevation="2">
              <v-table density="compact">
                <thead>
                <tr>
                  <th>材料名称</th>
                  <th>id</th>
                  <th>数量</th>
                  <th>占比</th>
                </tr>
                </thead>
                <tbody>
                <tr v-for="(item, index) in sampleMaterials" :key="index">
                  <td>{{ item.name }}</td>
                  <td class="text-caption">{{ item.id }}</td>
                  <td>{{ item.count }}</td>
                  <td>
                    <v-progress-linear
                        :model-value="(item.count / 3000) * 100"
                        height="20"
                        color="green"
                        rounded
                    >
                      <template v-slot:default="{ value }">
                        <strong>{{ Math.ceil(value) }}%</strong>
                      </template>
                    </v-progress-linear>
                  </td>
                </tr>
                </tbody>
              </v-table>
            </v-card>
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
</style>