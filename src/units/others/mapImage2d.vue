<script setup lang="ts">
import {ref} from "vue";
const exportSettings = ref({
  width: 128,
  height: 128,
  dithering: true,
  maxResolution: 4096
});
const mapImg = ref<File>()
</script>

<template>
  <v-row>
    <v-col cols="12">
      <v-alert
          variant="tonal"
          color="red"
          icon="mdi-information"
          class="mt-4"
      >
        未实现。
      </v-alert>
    </v-col>
  </v-row>
  <v-row class="pa-4">
    <v-col cols="6">
      <v-file-input
          v-model="mapImg"
          accept="image/png, image/jpeg, image/bmp, image/jpg"
          label="选择背景文件"
          density="compact"
          placeholder="单击选择文件"
          show-size
          prepend-icon="mdi-folder-open"
          @update:model-value=""
      ></v-file-input>
    </v-col>
    <v-col cols="6">
      <v-btn
          variant="outlined"
          block
          @click=""
      >
        <v-icon left>mdi-refresh</v-icon>
        刷新
      </v-btn>
    </v-col>
    <v-col cols="12">
      <v-row align="center">
        <v-col cols="12" md="6">
          <v-text-field
              v-model.number="exportSettings.width"
              label="导出宽度"
              type="number"
              min="16"
              :max="exportSettings.maxResolution"
              step="16"
              hint="建议值为16的倍数"
              persistent-hint
              density="compact"
          >
            <template v-slot:append>
              <span class="text-caption">px</span>
            </template>
          </v-text-field>
        </v-col>

        <v-col cols="12" md="6">
          <v-text-field
              v-model.number="exportSettings.height"
              label="导出高度"
              type="number"
              min="16"
              :max="exportSettings.maxResolution"
              step="16"
              hint="建议值为16的倍数"
              persistent-hint
              density="compact"
          >
            <template v-slot:append>
              <span class="text-caption">px</span>
            </template>
          </v-text-field>
        </v-col>

        <v-col cols="12">
          <v-switch
              v-model="exportSettings.dithering"
              label="启用抖动算法"
              color="primary"
              density="compact"
              hint="通过颜色抖动提升画面细节表现"
              persistent-hint
          >
            <template v-slot:append>
              <v-tooltip location="bottom">
                <template v-slot:activator="{ props }">
                  <v-icon
                      v-bind="props"
                      icon="mdi-information-outline"
                      size="small"
                      class="ml-2"
                  ></v-icon>
                </template>
                <span>使用Floyd-Steinberg算法进行误差扩散，优化色彩过渡</span>
              </v-tooltip>
            </template>
          </v-switch>
        </v-col>
      </v-row>
    </v-col>
  </v-row>
</template>

<style scoped>

</style>