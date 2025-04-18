<script setup lang="ts">
import {onMounted, ref} from "vue";
import dayjs from 'dayjs'
import {onBeforeRouteLeave} from "vue-router";
import {isLeaving, navigationGuard} from "../moduels/navigation.ts";
import {fetchSchematics, SchematicsData, schematicTypeList} from "../moduels/schematics_data.ts";

let schematics = ref<SchematicsData[]>([])
const parseDimensions = (sizeStr: string) => {
  const [length, width, height] = sizeStr.split(',').map(Number);
  return [`X${length}`, `Y${width}`, `Z${height}`]
};

const parseVersions  = (versionStr: string) => {
  return versionStr.split(',').map(Number);
};
onMounted(async() => {
  const { data, page, page_size} = await fetchSchematics({
    filter: '',
    page: 1,
    page_size: 20
  });
  console.log(data)
  schematics.value = data
})

onBeforeRouteLeave(navigationGuard)

const formatTime = (time) => {
  return dayjs(time).format('YYYY/MM/DD HH:mm')
}
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
            <v-icon icon="mdi-warehouse" class="mr-2"></v-icon>
            <span class="text-h5">蓝图仓库</span>
          </v-toolbar-title>
          <v-btn variant="text" icon="mdi-cloud-upload" title="上传蓝图"/>
        </v-toolbar>

        <v-list class="mc-blueprint-list">
          <v-list-item
              v-for="(bp) in schematics"
              :key="bp.id"
              class="py-2"
              :title="bp.name"
          >
            <template v-slot:prepend>
              <v-icon
                  icon="mdi-cube-scan"
                  size="60"
                  class="app-logo"
              />
            </template>

            <template #title>
              <div class="d-flex align-center flex-wrap">
                <span v-if="bp.schematic_type == -1" class="text-h6 text-red-lighten-1">未解析</span>
                <span class="text-h6 text-blue-darken-4">{{ bp.name }}</span>
                <div class="ms-3 d-flex align-center ga-1">
                  <v-chip
                      variant="outlined"
                      color="green-darken-2"
                      size="small"
                      class="me-2"
                  >
                    <v-icon start icon="mdi-account"></v-icon>
                    {{ bp.user }}
                  </v-chip>
                  <v-chip
                      color="deep-purple"
                      variant="outlined"
                      size="small"
                      class="dimension-chip"
                  >
                    <div class="d-flex align-center">
                      <v-icon icon="mdi-axis-arrow" class="mr-1"></v-icon>
                      <div class="dimension-values">
                        <span v-for="(dim, index) in parseDimensions(bp.sizes)" :key="index">
                          {{ dim }}
                          <v-icon v-if="index < 2" icon="mdi-close" size="x-small" class="mx-1"></v-icon>
                        </span>
                      </div>
                    </div>
                  </v-chip>
                </div>
              </div>
            </template>

            <template #subtitle>
              <div class="d-flex flex-column mt-1">
                <p class="text-caption mb-1">
                  {{ bp.description }}
                </p>

                <div class="d-flex align-center flex-wrap gap-3">
                  <div class="d-flex align-center">
                    <v-icon icon="mdi-format-list-bulleted-type" size="small" class="me-1"></v-icon>
                    <span class="text-caption">{{ schematicTypeList[bp.schematic_type as 1 | 2 | 3 | 4] }}</span>
                  </div>

                  <div class="d-flex align-center flex-wrap gap-3">
                    <div class="d-flex align-center">
                      <v-icon icon="mdi-tag" size="small" class="me-1"></v-icon>
                      <span class="text-caption">
              v{{ bp.version }}
              <v-chip size="x-small" color="green" class="ms-1">当前版本</v-chip>
            </span>
                    </div>

                    <v-menu v-if="parseVersions(bp.version_list).length > 0">
                      <template v-slot:activator="{ props }">
                        <v-btn
                            variant="text"
                            color="primary"
                            size="small"
                            v-bind="props"
                        >
                          <v-icon icon="mdi-history" class="me-1"></v-icon>
                          历史版本 ({{ parseVersions(bp.version_list).length }})
                        </v-btn>
                      </template>

                      <v-list density="compact">
                        <v-list-item
                            v-for="(version, index) in parseVersions(bp.version_list)"
                            :key="index"
                        >
                          <v-list-item-title>
                            v{{ version }}
                            <v-icon
                                v-if="version === bp.version"
                                icon="mdi-tag"
                                color="green"
                                size="small"
                                class="ms-1"
                            />
                          </v-list-item-title>
                        </v-list-item>
                      </v-list>
                    </v-menu>
                  </div>

                  <div class="d-flex align-center">
                    <v-icon icon="mdi-clock-outline" size="small" class="me-1"></v-icon>
                    <span class="text-caption">{{ formatTime(bp.updated_at) }}</span>
                  </div>
                </div>
              </div>
            </template>

            <template v-slot:append>
              <div class="d-flex flex-column align-center ga-2">
                <v-btn
                    variant="tonal"
                    color="primary"
                    prepend-icon="mdi-download"
                    size="small"
                >
                  导出
                </v-btn>
                <div class="d-flex ga-1">
                  <v-btn
                      variant="text"
                      color="grey-darken-1"
                      icon="mdi-pencil"
                      density="comfortable"
                  ></v-btn>
                  <v-btn
                      variant="text"
                      color="red-lighten-1"
                      icon="mdi-delete"
                      density="comfortable"
                  ></v-btn>
                </div>
              </div>
            </template>
          </v-list-item>
        </v-list>
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
.mc-blueprint-list {
  --v-list-item-padding: 12px;

  max-height: calc(99vh - 64px);
  overflow-y: auto;
}

</style>