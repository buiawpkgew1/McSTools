<script setup lang="ts">
import {onMounted, reactive, ref} from "vue";
import {mapArtData} from "../../modules/map_art/map_art_data.ts"
import {getBlockImg, toast} from "../../modules/others.ts";
import {encode_image, image_data} from "../../modules/map_art/encode_image.ts";
import {MapArtProcessor} from "../../modules/map_art/image_rebuild.ts";
const exportSettings = reactive({
  width: 128,
  height: 128,
  dithering: true,
  maxResolution: 4096
});
const hasImage = ref(false)
const previewCanvas = ref<HTMLCanvasElement | null>(null)
const resize = ref(1)
const isProcessing = ref(false)
const selectedBlocks = ref<string[]>([]);
const expandedCategories = ref<string[]>([])
const imageBuild = ref<MapArtProcessor>();
const mapImg = ref<File>();
const previewImage = ref<string>("");
const blocksLoaded = ref(false);
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
const refreshImage = async () => {
  try {
    isProcessing.value = true
    hasImage.value = true
    exportSettings.height = image_data.value.height
    exportSettings.width = image_data.value.width
    imageBuild.value.updateBlocksData(selectedBlocks.value)
    await updateSize()
    const resultCanvas = await imageBuild.value.generatePixelArt(image_data.value.image, 16, {width: exportSettings.width, height:exportSettings.height}, exportSettings.dithering);
    const ctx = previewCanvas.value.getContext('2d')
    if (!ctx) return

    previewCanvas.value.width = resultCanvas.width
    previewCanvas.value.height = resultCanvas.height

    ctx.drawImage(resultCanvas, 0, 0)
  }catch (error) {
    console.log(error)
    hasImage.value = false
    mapImg.value = undefined
    toast.error(`像素画生成失败:${error}`, {
      timeout: 3000
    });
  } finally {
    isProcessing.value = false
  }
}
const uploadImage = async(file: File | undefined) => {
  try {
    isProcessing.value = true
    hasImage.value = true
    image_data.value = await encode_image(file);
    exportSettings.height = image_data.value.height
    exportSettings.width = image_data.value.width
    imageBuild.value.updateBlocksData(selectedBlocks.value)
    await updateSize()
    const resultCanvas = await imageBuild.value.generatePixelArt(image_data.value.image, 16, {width: exportSettings.width, height:exportSettings.height}, exportSettings.dithering);
    const ctx = previewCanvas.value.getContext('2d')
    if (!ctx) return

    previewCanvas.value.width = resultCanvas.width
    previewCanvas.value.height = resultCanvas.height

    ctx.drawImage(resultCanvas, 0, 0)
  } catch (error) {
    hasImage.value = false
    mapImg.value = undefined
    toast.error(`像素画生成失败:${error}`, {
      timeout: 3000
    });
  } finally {
    isProcessing.value = false
  }
}
const updateSize = async() => {
  exportSettings.width = image_data.value.width * resize.value;
  exportSettings.height = image_data.value.height * resize.value
}
onMounted(async () => {
  setTimeout(() => {
    blocksLoaded.value = true;
  }, 100);
  toggleCategory("wool")
  imageBuild.value = new MapArtProcessor(mapArtData.value, selectedBlocks.value)

})

</script>

<template>
  <v-row no-gutters>
    <v-col cols="12" md="4" class="pa-4 bg-grey-lighten-3 d-flex flex-column">
      <v-row v-if="image_data != undefined">
        <v-col cols="12" class="image-column" >
          <v-img
              :aspect-ratio="16/9"
              :style="{
                    backgroundImage: `url(${image_data.base64})`,
                    backgroundSize: 'cover',
                    backgroundPosition: 'center',
                    backgroundAttachment: 'fixed',
                  }"
              contain
              transition="fade-transition"
              class="image-preview rounded-lg"
          >
          </v-img>
          <v-list density="compact" >
            <v-list-item>
              <template #prepend>
                <v-icon>mdi-file</v-icon>
              </template>
              <v-list-item-title>文件名</v-list-item-title>
              <v-list-item-subtitle>{{ `${image_data.name}.${image_data.ext}` }}</v-list-item-subtitle>
            </v-list-item>

            <v-list-item>
              <template #prepend>
                <v-icon>mdi-arrow-expand</v-icon>
              </template>
              <v-list-item-title>分辨率</v-list-item-title>
              <v-list-item-subtitle>{{ `${exportSettings.width} x ${image_data.height}` }}</v-list-item-subtitle>
            </v-list-item>
          </v-list>
        </v-col>
        <v-col cols="12" class="d-flex justify-center">
          <v-btn-toggle
              v-model="resize"
              color="primary"
              mandatory
              @update:model-value="updateSize"
          >
            <v-btn :value="1" size="large" class="px-6">
              <span class="font-weight-bold">1</span>
            </v-btn>
            <v-btn :value="1/2" size="large" class="px-6">
              <span class="font-weight-bold">1/2</span>
            </v-btn>
            <v-btn :value="1/4" size="large" class="px-6">
              <span class="font-weight-bold">1/4</span>
            </v-btn>
            <v-btn :value="1/8" size="large" class="px-6">
              <span class="font-weight-bold">1/8</span>
            </v-btn>
          </v-btn-toggle>
        </v-col>
      </v-row>

      <div class="flex-grow-0">
        <v-file-input
            v-model="mapImg"
            accept="image/png, image/jpeg, image/bmp, image/jpg"
            label="选择图片文件"
            density="compact"
            prepend-icon="mdi-folder-open"
            @update:model-value="uploadImage(mapImg)"
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
            hint="建议和原尺寸保持同比例"
            persistent-hint
            density="compact"
            class="mb-2"
        >
        </v-text-field>

        <v-text-field
            v-model.number="exportSettings.height"
            label="导出高度"
            type="number"
            min="16"
            :max="exportSettings.maxResolution"
            step="16"
            hint="建议和原尺寸保持同比例"
            persistent-hint
            density="compact"
            class="mb-4"
        >
        </v-text-field>

        <v-switch
            class="ml-4"
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
      <v-btn
          variant="outlined"
          block
          color="blue"
          @click="refreshImage"
      >
        <v-icon left>mdi-refresh</v-icon>
        刷新
      </v-btn>
      <v-card v-if="blocksLoaded">
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
                      @click="toggleCategory(category.name)"
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
        <div v-if="isProcessing" class="processing-overlay">
          <v-progress-circular
              indeterminate
              size="64"
              color="primary"
          ></v-progress-circular>
          <div class="text-caption mt-2">正在处理图像...</div>
        </div>

        <div class="preview-content">
          <canvas
              ref="previewCanvas"
              class="pixel-canvas"
              :style="{
          opacity: isProcessing ? 0.5 : 1,
          cursor: isProcessing ? 'wait' : 'default'
        }"
          ></canvas>
        </div>

        <div v-if="!hasImage"  class="d-flex align-center justify-center">
          <v-alert
              variant="tonal"
              color="grey"
              icon="mdi-image-off-outline"
              text="等待输入图像..."
          ></v-alert>
        </div>
      </div>

    </v-col>
  </v-row>
</template>

<style scoped>
.preview-container {
  position: relative;
  height: 66vh;
  background: repeating-conic-gradient(#f5f5f5 0% 25%, white 0% 50%) 50% / 20px 20px;
}

.processing-overlay {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  background: rgba(255, 255, 255, 0.8);
  z-index: 2;
}

.empty-state {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  text-align: center;
}

.pixel-canvas {
  width: 100%;
  height: 100%;
  object-fit: contain;
  transition: opacity 0.3s ease;
}
</style>