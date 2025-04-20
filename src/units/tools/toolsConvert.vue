<script setup lang="ts">
import {defineProps, onMounted} from "vue";
import {SchematicsData, schematicTypeList} from "../../moduels/schematics_data.ts";
import dayjs from "dayjs";

const props = defineProps<{
  data: SchematicsData | undefined,
}>()
const parseDimensions = (sizeStr: string) => {
  const [length, width, height] = sizeStr.split(',').map(Number);
  return [`X${length}`, `Y${width}`, `Z${height}`]
};
const formatTime = (time: any) => {
  return dayjs(time).format('YYYY/MM/DD HH:mm')
}
onMounted(async()=>{
  console.log(props.data)
})
</script>

<template>
  <div v-if="props.data" class="ma-4">
    <v-card-title>蓝图基本信息</v-card-title>

    <v-card-text>
      <v-row>
        <v-col cols="6">
          <v-list density="compact">
            <v-list-item>
              <template v-slot:prepend>
                <v-icon icon="mdi-identifier"></v-icon>
              </template>
              <v-list-item-title>ID：{{ props.data.id }}</v-list-item-title>
            </v-list-item>

            <v-list-item>
              <v-list-item-title>名称：{{ props.data.name }}</v-list-item-title>
            </v-list-item>

            <v-list-item>
              <v-list-item-title>类型：{{ schematicTypeList[props.data.schematic_type as 1 | 2 | 3 | 4] }} </v-list-item-title>
            </v-list-item>

            <v-list-item>
              <v-list-item-title>
                尺寸：
                <v-chip
                    color="deep-purple"
                    variant="outlined"
                    size="small"
                    class="dimension-chip"
                >
                  <div class="d-flex align-center">
                    <v-icon icon="mdi-axis-arrow" class="mr-1"></v-icon>
                    <div class="dimension-values">
                                <span v-for="(dim, index) in parseDimensions(props.data.sizes)" :key="index">
                                  {{ dim }}
                                  <v-icon v-if="index < 2" icon="mdi-close" size="x-small" class="mx-1"></v-icon>
                                </span>
                    </div>
                  </div>
                </v-chip>
              </v-list-item-title>
            </v-list-item>
          </v-list>
        </v-col>

        <v-col cols="6">
          <v-list density="compact">
            <v-list-item>
              <v-list-item-title>
                状态：
                <v-chip :color="props.data.is_deleted ? 'error' : 'success'" size="small">
                  {{ props.data.is_deleted ? '已删除' : '正常' }}
                </v-chip>
              </v-list-item-title>
            </v-list-item>

            <v-list-item>
              <v-list-item-title>创建者：{{ props.data.user || '未知' }}</v-list-item-title>
            </v-list-item>

            <v-list-item>
              <v-list-item-title>版本：v{{ props.data.version }}
                <v-chip
                    color="orange-lighten-4"
                    size="small"
                    class="text-orange-darken-4"
                >
                  <v-icon start icon="mdi-cube"></v-icon>
                  {{ props.data.game_version }}
                </v-chip>
              </v-list-item-title>
            </v-list-item>

            <v-list-item>
              <v-list-item-title>更新时间：{{ formatTime(props.data.updated_at) }}</v-list-item-title>
            </v-list-item>
          </v-list>
        </v-col>
      </v-row>

      <v-textarea
          readonly
          :value="props.data.description"
          label="蓝图描述"
          auto-grow
          class="mt-4"
      ></v-textarea>
    </v-card-text>
  </div>
  <div v-else class="ma-4 error-card">
    <v-alert type="error">
      未选取蓝图
    </v-alert>
  </div>
</template>

<style scoped>

</style>