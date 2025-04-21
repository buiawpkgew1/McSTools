<script setup lang="ts">
import { ref } from 'vue';
import {isLeaving, navigationGuard} from "../modules/navigation.ts";
import {onBeforeRouteLeave} from "vue-router";

const opacity = ref(0.8)
const selectedTheme = ref('blue')
const backgroundImage = ref('/default-bg.jpg');
const backgroundOpacity = ref(0.9);
const layoutMode = ref('cover');

const layoutModes = [
  { label: '拉伸填充', value: 'stretch', color: 'red-darken-2' },
  { label: '平铺重复', value: 'repeat', color: 'teal-darken-2' },
  { label: '适应屏幕', value: 'contain', color: 'purple-darken-2' },
  { label: '完整覆盖', value: 'cover', color: 'blue-darken-2' },
];

const openImageFolder = () => {
  console.log('打开背景图文件夹');
};

const refreshBackground = () => {
  console.log('刷新背景图片');
};
const themes = [
  { label: '蔚蓝主题', value: 'blue', color: 'blue-darken-2', icon: 'mdi-weather-sunny' },
  { label: '深蓝之夜', value: 'dark-blue', color: 'indigo-darken-3', icon: 'mdi-moon-waning-crescent' },
  { label: '清新绿意', value: 'green', color: 'teal-darken-2', icon: 'mdi-leaf' },
  { label: '活力橙', value: 'orange', color: 'orange-darken-2', icon: 'mdi-fire' },
  { label: '菠萝黄', value: 'yellow', color: 'yellow-darken-3', icon: 'mdi-fruit-pineapple' },
  { label: '橡木棕', value: 'brown', color: 'brown-darken-3', icon: 'mdi-square-rounded' },
]

onBeforeRouteLeave(navigationGuard)

</script>

<template  class="page-wrapper">
  <v-row no-gutters
         class="mb-4 animate-row"
         :class="{ 'animate-row-out': isLeaving }"
  >
    <v-col class="mb-4" cols="12">
        <v-card class="mx-auto" elevation="4" hover>
          <v-toolbar density="compact" class="bg-blue-grey-lighten-5 pa-4">
            <v-toolbar-title>
              <v-icon icon="mdi-palette" class="mr-2 text-blue-darken-2"></v-icon>
              <span class="text-h5">个性化设置</span>
            </v-toolbar-title>
          </v-toolbar>

          <v-card-text class="pa-6">
            <v-card variant="flat" class="mb-6 bg-blue-grey-lighten-5">
              <v-card-text>
                <v-row align="center">
                  <v-col cols="2">
                    <div class="d-flex align-center">
                      <v-icon icon="mdi-opacity" class="mr-2"></v-icon>
                      <span class="text-subtitle-1">不透明度</span>
                    </div>
                  </v-col>
                  <v-col cols="10">
                    <div class="d-flex align-center">
                      <v-slider
                          v-model="opacity"
                          :max="1"
                          :min="0.2"
                          :step="0.1"
                          thumb-label
                          track-color="blue-grey-lighten-3"
                          color="blue-darken-2"
                          class="mr-4"
                      ></v-slider>
                      <v-chip
                          color="blue-darken-2"
                          label
                          class="text-white"
                      >
                        {{ Math.round(opacity * 100) }}%
                      </v-chip>
                    </div>
                  </v-col>
                </v-row>
              </v-card-text>
            </v-card>
            <v-card variant="flat" class="bg-blue-grey-lighten-5">
              <v-card-text>
                <v-row align="center" justify="center">
                  <v-col cols="2">
                    <div class="d-flex align-center">
                      <v-icon icon="mdi-theme-light-dark" class="mr-2"></v-icon>
                      <span class="text-subtitle-1">主题配色</span>
                    </div>
                  </v-col>
                  <v-col cols="10">
                    <div class="d-flex flex-wrap gap-3">
                      <v-radio
                          v-for="theme in themes"
                          :key="theme.value"
                          v-model="selectedTheme"
                          :value="theme.value"
                          hide-details
                      >
                        <template v-slot:label>
                          <v-chip
                              :color="theme.color"
                              label
                              :variant="selectedTheme === theme.value ? 'elevated' : 'outlined'"
                              class="ma-2"
                          >
                            <v-icon left :icon="theme.icon"></v-icon>
                            {{ theme.label }}
                          </v-chip>
                        </template>
                      </v-radio>
                    </div>
                  </v-col>
                </v-row>
              </v-card-text>
            </v-card>
          </v-card-text>
        </v-card>
    </v-col>
    <v-col class="mb-4" cols="12">
      <v-card class="mx-auto" elevation="4" hover>
        <v-toolbar density="compact" class="pa-2">
          <v-toolbar-title>
            <v-icon icon="mdi-image" class="mr-2 text-blue-darken-3"></v-icon>
            <span class="text-h6 text-blue-darken-3">背景设置</span>
          </v-toolbar-title>
        </v-toolbar>

        <v-card-text class="pa-4">
          <div
              class="preview-container mb-6 rounded-lg"
              :style="{
              backgroundImage: `url(${backgroundImage})`,
              opacity: backgroundOpacity,
              }"
          ></div>

          <v-row class="mb-4">
            <v-col cols="6">
              <v-btn
                  variant="tonal"
                  block
                  @click="openImageFolder"
              >
                <v-icon left>mdi-folder-open</v-icon>
                选择背景图
              </v-btn>
            </v-col>
            <v-col cols="6">
              <v-btn
                  variant="outlined"
                  block
                  @click="refreshBackground"
              >
                <v-icon left>mdi-refresh</v-icon>
                刷新背景
              </v-btn>
            </v-col>
          </v-row>

          <v-row align="center" class="mb-4">
            <v-col cols="2" class="d-flex align-center">
              <v-icon icon="mdi-opacity" class="mr-2"></v-icon>
              <span>透明度</span>
            </v-col>
            <v-col cols="10">
              <div class="d-flex align-center">
                <v-slider
                    v-model="opacity"
                    :max="1"
                    :min="0.2"
                    :step="0.1"
                    thumb-label
                    track-color="blue-grey-lighten-3"
                    color="blue-darken-2"
                    class="mr-4"
                ></v-slider>
                <v-chip
                    color="blue-darken-2"
                    label
                    class="text-white"
                >
                  {{ Math.round(opacity * 100) }}%
                </v-chip>
              </div>
            </v-col>
          </v-row>

          <!-- 布局方式设置 -->
          <v-row align="center">
            <v-col cols="3" class="d-flex align-center">
              <v-icon icon="mdi-image-area" class="mr-2"></v-icon>
              <span>布局方式</span>
            </v-col>
            <v-col cols="9">
              <v-radio-group
                  v-model="layoutMode"
                  inline
                  hide-details
              >
                <v-radio
                    v-for="mode in layoutModes"
                    :key="mode.value"
                    :value="mode.value"
                    :color="mode.color"
                >
                  <template v-slot:label>
                    <v-chip
                        :color="mode.color"
                        :variant="layoutMode === mode.value ? 'elevated' : 'outlined'"
                        class="ma-1"
                    >
                      {{ mode.label }}
                    </v-chip>
                  </template>
                </v-radio>
              </v-radio-group>
            </v-col>
          </v-row>
        </v-card-text>
      </v-card>
    </v-col>
  </v-row>
</template>

<style scoped>
.settings-container {
  background-color: #f0f8ff;
  padding: 24px;
  border-radius: 8px;
  max-width: 500px;
  margin: 20px auto;
}

.setting-section {
  background: white;
  padding: 16px;
  border-radius: 6px;
  margin-bottom: 20px;
  box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}

.setting-section h2 {
  color: #2c3e50;
  font-size: 16px;
  margin-bottom: 12px;
}

.button-group {
  display: flex;
  gap: 10px;
  margin-top: 12px;
}

button {
  background: #4CAF50;
  color: white;
  border: none;
  padding: 8px 16px;
  border-radius: 4px;
  cursor: pointer;
  transition: background 0.3s;
}

button:hover {
  background: #45a049;
}

.checkbox {
  display: flex;
  align-items: center;
  gap: 8px;
  color: #666;
}

.radio-group {
  display: flex;
  gap: 12px;
}

.radio-group label {
  padding: 6px 12px;
  border-radius: 4px;
  border: 1px solid #ddd;
  cursor: pointer;
  transition: all 0.3s;
}

.radio-group label.selected {
  background: #4CAF50;
  color: white;
  border-color: #4CAF50;
}
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
.v-card:hover {
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1) !important;
}

.v-chip--selected {
  transform: scale(1.05);
  transition: all 0.3s ease;
}
</style>