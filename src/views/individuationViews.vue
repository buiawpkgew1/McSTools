<script setup lang="ts">
import {onMounted, ref, watch} from 'vue';
import {isLeaving, navigationGuard} from "../modules/navigation.ts";
import {onBeforeRouteLeave} from "vue-router";
import {appStore} from '../modules/store.ts';
import {saveImage} from "../modules/uploadImage.ts";
import router from "../../router";
import {useTheme} from "vuetify/framework";
import {backgroundOpacity, backgroundStr, layoutMode, opacity} from "../modules/theme.ts";


const selectedTheme = ref('grey');
const backgroundImage = ref('null');
const backgroundSize = ref<number>();
const backgroundDimensions = ref();
const backgroundName = ref();
const background = ref();
const theme = useTheme()
const layoutModes = [
  { label: '拉伸填充', value: 'stretch', color: 'red-darken-2' },
  { label: '平铺重复', value: 'repeat', color: 'teal-darken-2' },
  { label: '适应屏幕', value: 'contain', color: 'purple-darken-2' },
  { label: '完整覆盖', value: 'cover', color: 'blue-darken-2' },
];

const refreshBackground = async() => {
  await router.push({name: 'emptyRoute'});
};
const themes = [
  { label: '默认灰白', value: 'grey', color: 'bg-blue-grey-lighten-5', icon: 'mdi-weather-sunny' },
  { label: '蔚蓝主题', value: 'blue', color: 'blue-darken-2', icon: 'mdi-weather-sunny' },
  { label: '深蓝之夜', value: 'dark_blue', color: 'indigo-darken-3', icon: 'mdi-moon-waning-crescent' },
  { label: '清新绿意', value: 'green', color: 'teal-darken-2', icon: 'mdi-leaf' },
  { label: '活力橙', value: 'orange', color: 'orange-darken-2', icon: 'mdi-fire' },
  { label: '菠萝黄', value: 'yellow', color: 'yellow-darken-3', icon: 'mdi-fruit-pineapple' },
  { label: '橡木棕', value: 'brown', color: 'brown-darken-3', icon: 'mdi-square-rounded' },
]

onBeforeRouteLeave(navigationGuard)
onMounted(async () => {
  selectedTheme.value = await appStore.get('selectedTheme', 'grey');
  backgroundImage.value = await appStore.get('backgroundImage', 'null');
  backgroundSize.value = await appStore.get('backgroundSize', 0);
  backgroundDimensions.value = await appStore.get('backgroundDimensions', "null")
  backgroundName.value = await appStore.get('backgroundName', "null")
  theme.global.name.value = selectedTheme.value
});

const updateBackGround = async(file: File| undefined) =>{
  let data = await saveImage(file)
  await appStore.set('backgroundImage', data?.path);
  await appStore.set('backgroundSize', data?.size)
  await appStore.set('backgroundDimensions', data?.dimensions)
  await appStore.set('backgroundName', data?.name)
  await router.push({name: 'emptyRoute'});
}
const clearBackGround = async () => {
  await appStore.set('backgroundImage', 'null');
  await appStore.set('backgroundSize', 0)
  await appStore.set('backgroundDimensions', 'null')
  await appStore.set('backgroundName', 'null')
  await router.push({name: 'emptyRoute'});
}

watch(selectedTheme, (val) => {
      theme.global.name.value = val
      appStore.set('selectedTheme', val)
}
);
watch(backgroundImage, (val) => appStore.set('backgroundImage', val));
watch(layoutMode, (val) => appStore.set('layoutMode', val));
watch(opacity, (val) => appStore.set('opacity', val))
watch(backgroundOpacity, (val) => appStore.set('backgroundOpacity', val))
</script>

<template  class="page-wrapper">
  <v-row no-gutters
         class="mb-4 animate-row"
         :class="{ 'animate-row-out': isLeaving }"
  >
    <v-col class="mb-4" cols="12">
        <v-card class="mx-auto v-theme--custom text-primary " :style="{ '--surface-alpha': opacity }" elevation="4" hover>
          <v-toolbar density="compact" class="bg-blue-grey-lighten-5 pa-4">
            <v-toolbar-title>
              <v-icon icon="mdi-palette" class="mr-2"></v-icon>
              <span class="text-h5">个性化设置</span>
            </v-toolbar-title>
          </v-toolbar>

          <v-card-text class="pa-6">
            <v-card variant="flat" class="mb-3 bg-blue-grey-lighten-5">
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
                      <v-radio-group
                          v-model="selectedTheme"
                          inline
                          hide-details
                      >
                        <v-radio
                            v-for="theme in themes"
                            :key="theme.value"
                            :color="theme.color"
                            :v-model="selectedTheme"
                            :value="theme.value"
                        >
                          <template v-slot:label>
                            <v-chip
                                :color="theme.color"
                                :variant="selectedTheme === theme.value ? 'elevated' : 'outlined'"
                                class="ma-1"
                            >
                              <v-icon left :icon="theme.icon"></v-icon>
                              {{ theme.label }}
                            </v-chip>
                          </template>
                        </v-radio>
                      </v-radio-group>
                    </div>
                  </v-col>
                </v-row>
              </v-card-text>
            </v-card>
          </v-card-text>
        </v-card>
    </v-col>
    <v-col class="mb-4" cols="12">
      <v-card class="mx-auto" :style="{ '--surface-alpha': opacity }" elevation="4" hover>
        <v-toolbar density="compact" class="pa-2">
          <v-toolbar-title>
            <v-icon icon="mdi-image" class="mr-2"></v-icon>
            <span class="text-h7">背景设置</span>
          </v-toolbar-title>
        </v-toolbar>

        <v-card-text class="pa-4">
          <v-row class="preview-layout mb-4" v-if="backgroundImage != 'null'">
            <v-col cols="6" class="image-column">
              <v-img
                  :aspect-ratio="16/9"
                  :style="{
                    backgroundColor: `rgba(255,255,255, ${1 - backgroundOpacity})`,
                    backgroundImage: `url(${backgroundStr})`,
                    backgroundSize: 'cover',
                    backgroundPosition: 'center',
                    backgroundAttachment: 'fixed',
                  }"
                  contain
                  transition="fade-transition"
                  class="image-preview rounded-lg"
              >
                <template #default>
                  <div class="opacity-control pa-2">
                    <v-slider
                        v-model="backgroundOpacity"
                        :max="1"
                        color="primary"
                        thumb-label
                    >
                      <template #prepend>
                        <v-icon>mdi-opacity</v-icon>
                      </template>
                    </v-slider>
                  </div>
                </template>
              </v-img>
            </v-col>

            <v-col cols="6" class="info-column">
              <v-card class="h-100" :style="{ '--surface-alpha': opacity }" elevation="2">
                <v-card-title class="d-flex align-center">
                  <v-icon left>mdi-information</v-icon>
                  图片信息
                </v-card-title>

                <v-divider></v-divider>

                <v-card-text>
                  <v-list density="compact" >
                    <v-list-item>
                      <template #prepend>
                        <v-icon>mdi-file</v-icon>
                      </template>
                      <v-list-item-title>文件名</v-list-item-title>
                      <v-list-item-subtitle>{{ backgroundName }}</v-list-item-subtitle>
                    </v-list-item>

                    <v-list-item>
                      <template #prepend>
                        <v-icon>mdi-weight</v-icon>
                      </template>
                      <v-list-item-title>文件大小</v-list-item-title>
                      <v-list-item-subtitle>{{ backgroundSize?.toFixed(2) }} kb</v-list-item-subtitle>
                    </v-list-item>

                    <v-list-item>
                      <template #prepend>
                        <v-icon>mdi-arrow-expand</v-icon>
                      </template>
                      <v-list-item-title>分辨率</v-list-item-title>
                      <v-list-item-subtitle>{{ backgroundDimensions }}</v-list-item-subtitle>
                    </v-list-item>
                  </v-list>
                </v-card-text>

                <v-card-actions class="justify-end">
                  <v-btn
                      color="error"
                      variant="flat"
                      @click="clearBackGround"
                  >
                    <v-icon left>mdi-delete</v-icon>
                    清除背景
                  </v-btn>
                </v-card-actions>
              </v-card>
            </v-col>
          </v-row>

          <v-row class="mb-4">
            <v-col cols="6">
              <v-file-input
                  v-model="background"
                  accept="image/png, image/jpeg, image/bmp"
                  label="选择背景文件"
                  density="compact"
                  placeholder="单击选择文件"
                  show-size
                  prepend-icon="mdi-folder-open"
                  @update:model-value="updateBackGround(background)"
              ></v-file-input>
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
          <v-card variant="flat" class="bg-blue-grey-lighten-5 mb-3">
            <v-card-text class="pa-6">
              <v-row align="center" >
                <v-col cols="2" class="d-flex align-center">
                  <v-icon icon="mdi-opacity" class="mr-2"></v-icon>
                  <span>不透明度</span>
                </v-col>
                <v-col cols="10">
                  <div class="d-flex align-center">
                    <v-slider
                        v-model="backgroundOpacity"
                        :max="1"
                        :min="0.2"
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
                      {{ Math.round(backgroundOpacity * 100) }}%
                    </v-chip>
                  </div>
                </v-col>
              </v-row>
            </v-card-text>
          </v-card>

          <v-card variant="flat" class="bg-blue-grey-lighten-5 mb-2">
            <v-card-text class="pa-6">
              <v-row align="center">
                <v-col cols="2" class="d-flex align-center">
                  <v-icon icon="mdi-image-area" class="mr-2"></v-icon>
                  <span>布局方式</span>
                </v-col>
                <v-col cols="10">
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




.preview-layout {
  height: 300px;
}

.image-column, .info-column {
  flex: 1 1 auto;
  min-width: 0;
  padding: 8px;
}

.image-preview {
  height: 300px;
  display: flex;
  flex-direction: column;
}

.opacity-control {
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  background: linear-gradient(to top, rgba(0, 0, 0, 0.7), transparent);
}

.info-column {
  height: 100%;
}
</style>