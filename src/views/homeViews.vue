<script setup lang="ts">
  import bgImg from '../static/img/bg.jpg'
  import createImg from '../static/img/create.jpg'
  import lmImg from '../static/img/Litematica.jpg'
  import weImg from '../static/img/wordEdit.png'
  import beImg from '../static/img/grass_block.png'
</script>

<template class="page-wrapper">
  <v-row no-gutters class="mb-4 animate-row"
         transition="scroll-x-transition"
  >
    <v-col>
      <v-card class="mx-auto" elevation="4" hover transition="slide-x-transition">
        <v-card-title class="bg-blue-grey-lighten-5 pa-4">
          <v-icon icon="mdi-tools" class="mr-2"></v-icon>
          <span class="text-h5">蓝图工具箱</span>
        </v-card-title>

        <v-card-text class="py-4">
          <v-row class="mb-4" dense>
            <v-col cols="2">
              <div class="d-flex align-center">
                <v-icon icon="mdi-folder-multiple" color="deep-purple" class="mr-2"></v-icon>
                <div>
                  <div class="text-caption text-grey">本地蓝图总数</div>
                  <div class="text-h4">11</div>
                </div>
              </div>
            </v-col>
            <v-col cols="2">
              <div class="d-flex align-center">
                <v-icon icon="mdi-cloud-upload" size="32" color="teal" class="mr-2"></v-icon>
                <div>
                  <div class="text-caption text-grey">云端蓝图总数</div>
                  <div class="text-h4">20</div>
                </div>
              </div>
            </v-col>
          </v-row>
          <v-alert
              variant="tonal"
              color="info"
              icon="mdi-information"
              class="mt-4"
          >
            新版本 v-1.0.0 已发布！
          </v-alert>
        </v-card-text>
      </v-card>
    </v-col>
  </v-row>
  <v-row
      class="fill-height animate-row"
  >
    <v-col cols="8" class="h-75">
      <v-card class="d-flex flex-column h-100" elevation="4">
        <v-card-title class="text-h6 bg-blue-lighten-5">
          <v-icon icon="mdi-cloud-upload" class="mr-2"></v-icon>
          蓝图处理
        </v-card-title>

        <v-card-text class="flex-grow-1 d-flex flex-column align-center justify-center upload-area">
          <v-icon icon="mdi-file-cloud" size="80" class="text-blue-grey"></v-icon>
          <div class="text-body-1 my-4">拖放文件或点击上传蓝图</div>

        </v-card-text>
      </v-card>
    </v-col>

    <v-col cols="4" class="h-75">
      <v-card class="h-100" elevation="4">
        <v-card-title class="text-h6 bg-green-lighten-5">
          <v-icon icon="mdi-format-list-checks" class="mr-2"></v-icon>
          支持蓝图类型
        </v-card-title>

        <v-card-text>
          <v-list lines="two">
            <v-list-item
            >
              <template v-slot:prepend>
                <v-avatar>
                  <v-img
                      :src="createImg"
                      sizes="40"
                  ></v-img>
                </v-avatar>
              </template>
              <template v-slot:title>
                <span class="font-weight-bold">香草结构</span>
              </template>
              <template v-slot:subtitle>
                我的世界原版支持的蓝图格式，机械动力也采用了这种格式
              </template>
            </v-list-item>
            <v-list-item
            >
              <template v-slot:prepend>
                <v-avatar>
                  <v-img
                      :src="bgImg"
                      sizes="40"
                  ></v-img>
                </v-avatar>
              </template>
              <template v-slot:title>
                <span class="font-weight-bold">建筑小帮手</span>
              </template>
              <template v-slot:subtitle>
                科技包最常见的辅组建筑工具
              </template>
            </v-list-item>
            <v-list-item
            >
              <template v-slot:prepend>
                <v-avatar>
                  <v-img
                      :src="lmImg"
                      sizes="40"
                  ></v-img>
                </v-avatar>
              </template>
              <template v-slot:title>
                <span class="font-weight-bold">建筑投影</span>
              </template>
              <template v-slot:subtitle>
                生电玩家活下去的必备工具，
              </template>
            </v-list-item>
            <v-list-item
            >
              <template v-slot:prepend>
                <v-avatar>
                  <v-img
                      :src="weImg"
                      sizes="40"
                  ></v-img>
                </v-avatar>
              </template>
              <template v-slot:title>
                <span class="font-weight-bold">创世神</span>
              </template>
              <template v-slot:subtitle>
                老牌建筑工具，延用至今，新版axios也采用了这种蓝图格式
              </template>
            </v-list-item>
            <v-list-item
            >
              <template v-slot:prepend>
                <v-avatar>
                  <v-img
                      :src="beImg"
                      sizes="40"
                  ></v-img>
                </v-avatar>
              </template>
              <template v-slot:title>
                <span class="font-weight-bold">MC BE</span>
              </template>
              <template v-slot:subtitle>
                我的世界BE采用的蓝图格式，目前未完全适配
              </template>
            </v-list-item>
          </v-list>
        </v-card-text>
      </v-card>
    </v-col>
  </v-row>
</template>

<style scoped>
.upload-area {
  border: 2px dashed rgba(0, 0, 0, 0.12);
  transition: all 0.3s ease;
  cursor: pointer;
}

.upload-area:hover {
  border-color: #2196F3;
  background: rgba(33, 150, 243, 0.05);
}

.h-100 {
  height: 100%;
}

.fill-height {
  height: 100%;
}
.page-wrapper {
  overflow-x: hidden;
  overflow-y: hidden;
}

.animate-row {
  animation: rowEntrance 0.5s cubic-bezier(0.4, 0, 0.2, 1);
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