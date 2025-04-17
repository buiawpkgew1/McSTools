<script setup lang="ts">
import {ref} from "vue";
import dayjs from 'dayjs'
import {NavigationGuard, onBeforeRouteLeave} from "vue-router";
const leaveTimer = ref<number>(0)
const isLeaving = ref(false)

const minecraftBlueprints = ref([
  {
    id: 'BP001',
    title: '自动南瓜农场',
    description: '全自动收割系统，支持1.20+版本',
    author: 'guapi',
    type: '创世神',
    gameVersion: '1.20.1',
    modDependencies: [],
    uploadTime: '2024-03-15T14:30:00',
    thumbnail: 'https://example.com/path/to/thumbnail1.jpg'
  },
  {
    id: 'BP002',
    title: '小黑塔',
    description: '测试内容',
    author: 'guapi',
    type: '小帮手',
    gameVersion: '1.19.4',
    modDependencies: ['OptiFine', 'WorldEdit'],
    uploadTime: '2024-03-14T09:45:00',
    thumbnail: 'https://example.com/path/to/thumbnail2.jpg'
  },
  {
    id: 'BP002',
    title: '',
    description: '',
    author: 'guapi',
    type: '',
    gameVersion: '',
    modDependencies: [],
    uploadTime: '2024-03-14T09:45:00',
    thumbnail: 'https://example.com/path/to/thumbnail2.jpg'
  }
])
const navigationGuard: NavigationGuard = (to, from, next) => {
  isLeaving.value = true

  leaveTimer.value = window.setTimeout(() => {
    next()
  }, 200)

  const handler = () => {
    window.clearTimeout(leaveTimer.value)
    next()
  }

  document.addEventListener('animationend', handler, { once: true })
}

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
              v-for="(bp, index) in minecraftBlueprints"
              :key="bp.id"
              class="py-2"
              :title="bp.title"
          >
            <!-- 左侧缩略图 -->
            <template v-slot:prepend>
              <v-icon
                  icon="mdi-cube-scan"
                  size="60"
                  class="app-logo"
              />
            </template>

            <template #title>
              <div class="d-flex align-center flex-wrap">
                <span v-if="bp.type.length == 0" class="text-h6 text-red-lighten-1">未解析</span>
                <span class="text-h6 text-blue-darken-4">{{ bp.title }}</span>
                <div class="ms-3 d-flex align-center">
                  <v-chip
                      variant="outlined"
                      color="green-darken-2"
                      size="small"
                      class="me-2"
                  >
                    <v-icon start icon="mdi-account"></v-icon>
                    {{ bp.author }}
                  </v-chip>
                  <v-chip
                      color="orange-lighten-4"
                      size="small"
                      class="text-orange-darken-4"
                  >
                    <v-icon start icon="mdi-cube"></v-icon>
                    {{ bp.gameVersion }}
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
                    <span class="text-caption">{{ bp.type }}</span>
                  </div>

                  <div v-if="bp.modDependencies" class="d-flex align-center">
                    <v-icon icon="mdi-puzzle" size="small" class="me-1"></v-icon>
                    <span class="text-caption">
                      {{ bp.modDependencies.join(', ') }}
                    </span>
                  </div>

                  <div class="d-flex align-center">
                    <v-icon icon="mdi-clock-outline" size="small" class="me-1"></v-icon>
                    <span class="text-caption">{{ formatTime(bp.uploadTime) }}</span>
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