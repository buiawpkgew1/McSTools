<script setup lang="ts">

import {isLeaving} from "../modules/navigation.ts";
</script>

<template class="page-wrapper">
  <v-row no-gutters class="mb-4 animate-row" :class="{ 'animate-row-out': isLeaving }">
    <v-col class="mb-4" cols="12">
      <v-card class="mx-auto" elevation="4" hover>
        <v-toolbar density="compact" class="bg-blue-grey-lighten-5 pa-2">
          <v-toolbar-title>
            <v-icon icon="mdi-information-outline" class="mr-2"></v-icon>
            <span class="text-h6">关于本站</span>
          </v-toolbar-title>
        </v-toolbar>
        <v-card-text>
          <v-row>
            <v-col cols="3" class="d-flex align-center justify-center">
              <div class="icon-container">
                <v-img
                    class="mx-auto"
                    height="80"
                    width="80"
                src="https://bad.src/not/valid"
                style="min-width: 80px"
                >
                <template v-slot:error>
                  <div class="error-wrapper d-flex align-center justify-center h-100">
                    <v-icon
                        icon="mdi-cube-scan"
                        size="40"
                        class="app-logo"
                    />
                  </div>
                </template>
                </v-img>
              </div>
            </v-col>

            <v-col cols="9">
              <v-row>
                <v-col cols="12">
                  <div class="d-flex justify-space-between">
                    <span class="text-h6 text-blue-grey-darken-2">版本: V1.0.00</span>
                    <span class="text-h6 text-blue-grey-darken-2">发布日期: 2024-02-20</span>
                  </div>
                  <v-row align="center" justify="start">
                    <v-col cols="auto">
                      <v-btn
                          color="primary"
                          variant="outlined"
                          prepend-icon="mdi-update"
                          @click=""
                      >
                        检测更新
                      </v-btn>
                    </v-col>
                    <v-col cols="auto">
                      <v-btn
                          color="secondary"
                          variant="outlined"
                          prepend-icon="mdi-text-box"
                          @click=""
                      >
                        更新日志
                      </v-btn>
                    </v-col>
                  </v-row>
                </v-col>
                <v-col cols="12">
                  <p class="text-body-1 text-grey-darken-2">
                    软件使用tuari 后端基于rust，前端使用vue.
                    <br/>
                    分离化设计让软件性能得到保障，rust安全设计，性能更好，内存安全高效。
                  </p>
                </v-col>
                <v-col cols="12">
                  <v-row align="center" justify="start">
                    <v-col cols="auto">
                      <v-btn
                          density="default"
                          color="primary"
                          variant="outlined"
                          prepend-icon="mdi-github"
                          @click=""
                      >
                        Github
                      </v-btn>
                    </v-col>
                    <v-col cols="auto">
                      <v-btn
                          density="default"
                          color="secondary"
                          variant="outlined"
                          prepend-icon="mdi-web"
                          @click=""
                      >
                        官方网站
                      </v-btn>
                    </v-col>
                    <v-col cols="auto">
                      <v-btn
                          density="default"
                          color="secondary"
                          variant="outlined"
                          prepend-icon="mdi-comment-question-outline"
                          @click=""
                      >
                        FAQ→
                      </v-btn>
                    </v-col>
                  </v-row>
                </v-col>
                <v-col cols="12">
                  <v-card>
                    <v-toolbar density="compact" class="pa-2 justify-center">
                      <v-toolbar-title class="text-center">
                        <span class="text-h6 text-blue-grey-darken-2">蓝图站</span>
                      </v-toolbar-title>
                    </v-toolbar>

                    <v-card-text class="text-center">
                      <p class="text-body-1 text-grey-darken-2 mb-4">
                        支持多种蓝图的网站，提供私有和公开多种模式，可以在线预览蓝图
                        <br>
                        提供在线的蓝图转换功能，材料统计，可将网站中蓝图快速导入到本地
                      </p>

                      <div class="d-flex justify-center">
                        <v-btn
                            color="secondary"
                            variant="outlined"
                            prepend-icon="mdi-web"
                            class="px-6"
                        @click=""
                        >
                        前往网站→
                        </v-btn>
                      </div>
                    </v-card-text>
                  </v-card>
                </v-col>
              </v-row>
            </v-col>
          </v-row>
        </v-card-text>
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




.icon-container {
  position: relative;
  width: 100%;
  height: 100%;
  min-height: 100px;
}

.error-wrapper {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  width: 80px;
  height: 80px;
}


</style>