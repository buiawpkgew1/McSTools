<script setup lang="ts">
import bgImg from '../../static/img/bg.jpg'
import createImg from '../../static/img/create.jpg'
import lmImg from '../../static/img/Litematica.jpg'
import weImg from '../../static/img/wordEdit.png'
import beImg from '../../static/img/grass_block.png'
import {defineProps, onMounted} from "vue";
import {ConvertData} from "../../modules/convert_data.ts";
const props = defineProps<{
  data: ConvertData | undefined,
}>()

onMounted(() =>{
  console.log(props.data)
})


</script>

<template>
  <v-row>
    <v-col cols="12">
      <v-alert
          variant="tonal"
          color="info"
          icon="mdi-information"
          class="mt-4"
      >
        大型蓝图的转换耗时可能过长请耐心等待
      </v-alert>
    </v-col>
  </v-row>

  <v-container class="pa-8">
    <v-row class="conversion-flow" justify="center" no-gutters>
      <v-col cols="12" md="5">
        <v-card class="format-card elevation-3" hover>
          <v-row no-gutters align="center">
            <v-col cols="4">
              <v-img
                  :src="lmImg"
                  height="120"
                  cover
                  class="card-image"
              ></v-img>
            </v-col>

            <v-col cols="8" class="pa-4">
              <div class="meta-info">
                <div class="d-flex justify-space-between">
                  <span class="text-caption text-grey">格式类型:</span>
                  <span class="text-caption">投影蓝图格式</span>
                </div>

                <div class="d-flex justify-space-between mt-1">
                  <span class="text-caption text-grey">原始大小:</span>
                  <span class="text-caption font-weight-medium">{{ ((Number(props.data?.size) ?? 0) / 1024).toFixed(2) + ' KB' }}</span>
                </div>
                <div class="d-flex justify-space-between mt-1">
                  <span class="text-caption text-grey">版本:</span>
                  <div>
                    <v-icon
                        color="success"
                        size="16"
                        icon="mdi-tag"
                    >
                    </v-icon>
                    <span class="text-caption font-weight-medium">v{{ props.data?.version }}</span>
                  </div>

                </div>

                <div class="d-flex justify-space-between mt-1">
                  <span class="text-caption text-grey">适配版本:</span>
                  <div>
                    <v-icon
                        color="success"
                        size="16"
                        icon="mdi-check-circle"
                    >
                    </v-icon>
                    <span class="text-caption font-weight-medium">v{{ props.data?.sub_type }}</span>
                  </div>
                </div>
              </div>
              <v-divider class="my-2"></v-divider>
            </v-col>
          </v-row>

          <v-card-item>
            <v-card-title class="text-h5 font-weight-bold text-blue-darken-2">
              <v-icon icon="mdi-vector-square" size="28" class="mr-2"></v-icon>
              投影蓝图
            </v-card-title>
            <v-card-subtitle class="text-caption text-grey-darken-1">
              适配与 我的世界建筑投影蓝图 格式
            </v-card-subtitle>
          </v-card-item>
        </v-card>
      </v-col>

      <v-col cols="12" md="2" class="text-center py-4">
        <v-icon size="64" class="transition-swing">
          <svg xmlns="http://www.w3.org/2000/svg" width="128" height="128" viewBox="0 0 24 24"><path fill="#0284c7" d="m21.71 9.29l-4-4a1 1 0 0 0-1.42 1.42L18.59 9H7a1 1 0 0 0 0 2h14a1 1 0 0 0 .92-.62a1 1 0 0 0-.21-1.09M17 13H3a1 1 0 0 0-.92.62a1 1 0 0 0 .21 1.09l4 4a1 1 0 0 0 1.42 0a1 1 0 0 0 0-1.42L5.41 15H17a1 1 0 0 0 0-2"/></svg>
        </v-icon>
        <div class=" text-blue-grey-darken-1 mt-2">一键转换</div>
      </v-col>

      <v-col cols="12" md="5">
        <v-card class="format-card elevation-3" hover>
          <v-row no-gutters align="center">
            <v-col cols="4">
              <v-img
                  :src="createImg"
                  height="120"
                  cover
                  class="card-image"
              ></v-img>
            </v-col>

            <v-col cols="8" class="pa-4">
              <div class="meta-info">
                <div class="d-flex justify-space-between">
                  <span class="text-caption text-grey">格式类型:</span>
                  <span class="text-caption">Minecraft 原生格式</span>
                </div>

                <div class="d-flex justify-space-between mt-1">
                  <span class="text-caption text-grey">原始大小:</span>
                  <span class="text-caption font-weight-medium">1000kb</span>
                </div>

                <div class="d-flex justify-space-between mt-1">
                  <span class="text-caption text-grey">文件状态:</span>
                  <v-icon
                      color="success"
                      size="16"
                      icon="mdi-check-circle"
                  >
                  </v-icon>
                </div>
              </div>
              <v-divider class="my-2"></v-divider>
            </v-col>
          </v-row>

          <v-card-item>
            <v-card-title class="text-h5 font-weight-bold text-blue-darken-2">
              <v-icon icon="mdi-cube-scan" size="28" class="mr-2"></v-icon>
              香草结构
            </v-card-title>
            <v-card-subtitle class="text-caption text-grey-darken-1">
              适配与JE原版结构方块和机械动力
            </v-card-subtitle>
          </v-card-item>
        </v-card>
        <v-card class="format-card elevation-3" hover>
          <v-row no-gutters align="center">
            <v-col cols="4">
              <v-img
                  :src="weImg"
                  height="120"
                  cover
                  class="card-image"
              ></v-img>
            </v-col>

            <v-col cols="8" class="pa-4">
              <div class="meta-info">
                <div class="d-flex justify-space-between">
                  <span class="text-caption text-grey">格式类型:</span>
                  <span class="text-caption">WorldEdit 格式</span>
                </div>

                <div class="d-flex justify-space-between mt-1">
                  <span class="text-caption text-grey">原始大小:</span>
                  <span class="text-caption font-weight-medium">1000kb</span>
                </div>

                <div class="d-flex justify-space-between mt-1">
                  <span class="text-caption text-grey">文件状态:</span>
                  <v-icon
                      color="success"
                      size="16"
                      icon="mdi-check-circle"
                  >
                  </v-icon>
                </div>
              </div>
              <v-divider class="my-2"></v-divider>
            </v-col>
          </v-row>

          <v-card-item>
            <v-card-title class="text-h5 font-weight-bold text-blue-darken-2">
              <v-icon icon="mdi-cube-scan" size="28" class="mr-2"></v-icon>
              创世神
            </v-card-title>
            <v-card-subtitle class="text-caption text-grey-darken-1">
              适配与新版1.16 + 创世神模组和最新版 axios
            </v-card-subtitle>
          </v-card-item>
        </v-card>
        <v-card class="format-card elevation-3" hover>
          <v-row no-gutters align="center">
            <v-col cols="4">
              <v-img
                  :src="bgImg"
                  height="120"
                  cover
                  class="card-image"
              ></v-img>
            </v-col>

            <v-col cols="8" class="pa-4">
              <div class="meta-info">
                <div class="d-flex justify-space-between">
                  <span class="text-caption text-grey">格式类型:</span>
                  <span class="text-caption">建筑小帮手 格式</span>
                </div>

                <div class="d-flex justify-space-between mt-1">
                  <span class="text-caption text-grey">原始大小:</span>
                  <span class="text-caption font-weight-medium">1000kb</span>
                </div>

                <div class="d-flex justify-space-between mt-1">
                  <span class="text-caption text-grey">文件状态:</span>
                  <v-icon
                      color="success"
                      size="16"
                      icon="mdi-check-circle"
                  >
                  </v-icon>
                </div>
              </div>
              <v-divider class="my-2"></v-divider>
            </v-col>
          </v-row>

          <v-card-item>
            <v-card-title class="text-h5 font-weight-bold text-blue-darken-2">
              <v-icon icon="mdi-cube-scan" size="28" class="mr-2"></v-icon>
              建筑小帮手
            </v-card-title>
            <v-card-subtitle class="text-caption text-grey-darken-1">
              适配与1.12 + 建筑小帮手 3个 变种格式蓝图
            </v-card-subtitle>
          </v-card-item>
        </v-card>
        <v-card class="format-card elevation-3" hover>
          <v-row no-gutters align="center">
            <v-col cols="4">
              <v-img
                  :src="beImg"
                  height="120"
                  cover
                  class="card-image"
              ></v-img>
            </v-col>

            <v-col cols="8" class="pa-4">
              <div class="meta-info">
                <div class="d-flex justify-space-between">
                  <span class="text-caption text-grey">格式类型:</span>
                  <span class="text-caption">基岩版格式</span>
                </div>

                <div class="d-flex justify-space-between mt-1">
                  <span class="text-caption text-grey">原始大小:</span>
                  <span class="text-caption font-weight-medium">1000kb</span>
                </div>

                <div class="d-flex justify-space-between mt-1">
                  <span class="text-caption text-grey">文件状态:</span>
                  <v-icon
                      color="success"
                      size="16"
                      icon="mdi-check-circle"
                  >
                  </v-icon>
                </div>
              </div>
              <v-divider class="my-2"></v-divider>
            </v-col>
          </v-row>

          <v-card-item>
            <v-card-title class="text-h5 font-weight-bold text-blue-darken-2">
              <v-icon icon="mdi-cube-scan" size="28" class="mr-2"></v-icon>
              MC BE
            </v-card-title>
            <v-card-subtitle class="text-caption text-grey-darken-1">
              适配与1.18 + 我的世界BE原版 结构方块格式
            </v-card-subtitle>
          </v-card-item>
        </v-card>
      </v-col>
    </v-row>
  </v-container>
</template>

<style scoped>
.conversion-flow {
  min-height: 300px;
}

.format-card {
  border-radius: 12px;
  transition: transform 0.3s ease;
  overflow: hidden;
}

.format-card:hover {
  transform: translateY(-4px);
}

.card-header {
  border-radius: 12px 12px 0 0;
}

@media (max-width: 960px) {
  .conversion-flow {
    flex-direction: column;
  }

  .format-card {
    margin: 16px 0;
  }
}
</style>