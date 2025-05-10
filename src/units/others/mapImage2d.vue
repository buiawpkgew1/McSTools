<script setup lang="ts">
import {ref} from "vue";
import {mapArtData} from "../../modules/map_art/map_art_data.ts"
import {getBlockImg} from "../../modules/others.ts";
const exportSettings = ref({
  width: 128,
  height: 128,
  dithering: true,
  maxResolution: 4096
});
const selectedBlocks = ref<string[]>([]);
const expandedCategories = ref<string[]>([])
const mapImg = ref<File>();
const previewImage = ref<string>("");

const toggleBlock = (blockId: string) => {
  const index = selectedBlocks.value.indexOf(blockId)
  if (index === -1) {
    selectedBlocks.value.push(blockId)
  } else {
    selectedBlocks.value.splice(index, 1)
  }
}

const toggleCategory = (categoryName: string) => {
  const category = mapArtData.value.find(c => c.name === categoryName)
  if (!category) return

  const allSelected = category.items.every(item =>
      selectedBlocks.value.includes(item.id)
  )

  if (allSelected) {
    selectedBlocks.value = selectedBlocks.value.filter(
        id => !category.items.some(item => item.id === id)
    )
  } else {
    const newItems = category.items
        .filter(item => !selectedBlocks.value.includes(item.id))
        .map(item => item.id)
    selectedBlocks.value = [...selectedBlocks.value, ...newItems]
  }
}

const isCategorySelected = (categoryName: string) => {
  const category = mapArtData.value.find(c => c.name === categoryName)
  return category?.items.every(item =>
      selectedBlocks.value.includes(item.id)
  ) ?? false
}

</script>

<template>
  <v-row no-gutters>
    <v-col cols="12" md="4" class="pa-4 bg-grey-lighten-3 d-flex flex-column">
      <div class="flex-grow-0">
        <v-file-input
            v-model="mapImg"
            accept="image/png, image/jpeg, image/bmp, image/jpg"
            label="选择背景文件"
            density="compact"
            prepend-icon="mdi-folder-open"
            @change=""
            class="mb-4"
        ></v-file-input>
        <v-alert
            v-if="!mapImg"
            variant="tonal"
            color="info"
            icon="mdi-information"
            class="mb-4"
        >
          请先上传源图片
        </v-alert>
        <v-img
            v-else
            :src="previewImage"
            :max-height="300"
            contain
            class="mb-4 elevation-2 rounded"
        ></v-img>
      </div>

      <div class="flex-grow-1 overflow-y-auto">
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
            class="mb-2"
        >
          <template v-slot:append>
            <span class="text-caption">px</span>
          </template>
        </v-text-field>

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
            class="mb-4"
        >
          <template v-slot:append>
            <span class="text-caption">px</span>
          </template>
        </v-text-field>

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
      </div>
      <v-card>
        <v-toolbar density="compact">
          <v-toolbar-title>方块选择器</v-toolbar-title>
        </v-toolbar>

        <v-list style="height: 80vh">
          <v-list-group
              v-for="category in mapArtData"
              :key="category.name"
              v-model="expandedCategories"
              :value="category.name"
          >
            <template v-slot:activator="{ props }">
              <v-list-item
                  v-bind="props"
                  :title="category.zh_cn"
                  class="category-header"

              >
                <template v-slot:prepend>
                  <v-checkbox
                      :model-value="isCategorySelected(category.name)"
                      color="primary"
                      hide-details
                      @click.stop="toggleCategory(category.name)"
                  ></v-checkbox>
                  <v-icon icon="mdi-cube-scan"></v-icon>
                </template>
              </v-list-item>
            </template>

            <v-list-item
                v-for="block in category.items"
                :key="block.id"
                class="block-item"
                @click="toggleBlock(block.id)"
                style="padding-inline-start: 0px !important;"
            >
              <template v-slot:prepend>
                <v-checkbox
                    :model-value="selectedBlocks.includes(block.id)"
                    color="primary"
                    hide-details
                ></v-checkbox>
              </template>
              <v-row align="start" no-gutters>
                <v-col cols="2" class="d-flex justify-center mt-2">
                  <v-avatar
                      size="28"
                      rounded="0"
                      class="mr-2"
                      style="border: 1px solid rgba(0,0,0,0.1)"
                  >
                    <v-img
                        :src="getBlockImg(block.id)"
                        :lazy-src="getBlockImg(block.id)"
                        :alt="block.id"
                        style="
                        width: 100%;
                        height: 100%;
                        object-fit: contain;
                        image-rendering: crisp-edges;
                      "
                    />
                  </v-avatar>
                </v-col>

                <v-col cols="6">
                  <div class="text-body-1 font-weight-bold">{{ block.zh_cn }}</div>
                  <div class="text-caption text-grey">{{ block.id }}</div>
                </v-col>

                <v-col cols="4">
                  <v-chip
                      class="ma-1"
                      label
                  >
                    <v-avatar
                        :color="block.average_rgb_hex"
                        size="20"
                        class="mr-1"
                    ></v-avatar>
                    <span>{{ block.average_rgb_hex }}</span>
                  </v-chip>
                </v-col>
              </v-row>
            </v-list-item>
          </v-list-group>
        </v-list>
      </v-card>
    </v-col>

    <v-col cols="12" md="8" class="pa-4 d-flex flex-column">
      <div class="preview-container elevation-3 rounded-lg">
        <div v-if="!mapImg" class="d-flex align-center justify-center">
          <v-alert
              variant="tonal"
              color="grey"
              icon="mdi-image-off-outline"
              text="等待输入图像..."
          ></v-alert>
        </div>

        <div v-else class="preview-content">
          <div class="processing-indicator" >
            <v-progress-circular indeterminate></v-progress-circular>
            <div class="mt-2">正在生成预览...</div>
          </div>

          <canvas ref="previewCanvas" class="elevation-2"></canvas>
        </div>
      </div>

      <v-btn
          color="primary"
          size="large"
          class="mt-4 align-self-end"
          :disabled="!mapImg"
          @click=""
      >
        <v-icon start>mdi-download</v-icon>
        导出地图画配置
      </v-btn>
    </v-col>
  </v-row>
</template>

<style scoped>
.preview-container {
  background: repeating-conic-gradient(#f5f5f5 0% 25%, white 0% 50%) 50% / 20px 20px;
  border: 2px dashed #ddd;
  min-height: 400px;
}

.processing-indicator {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  color: rgba(0, 0, 0, 0.6);
}

canvas {
  max-width: 100%;
  max-height: 80vh;
  object-fit: contain;
  background: white;
}

</style>