<script setup lang="ts">
import bgImg from '../static/img/bg.jpg'
import createImg from '../static/img/create.jpg'
import lmImg from '../static/img/Litematica.jpg'
import weImg from '../static/img/wordEdit.png'
import beImg from '../static/img/grass_block.png'
import {onMounted, ref, onBeforeUnmount} from "vue";
import {invoke} from '@tauri-apps/api/core';
import {opacity} from "../modules/theme.ts";
import {onBeforeRouteLeave} from 'vue-router'
import {
  files,
  handleUpload,
  progressTimer,
  progressValue,
  uploadError,
  uploadStatus
} from "../modules/upload_schematic.ts";
import {UserData} from "../modules/user_data.ts";
import {isLeaving, navigationGuard} from "../modules/navigation.ts";
import { createTimeManager } from '../modules/time_data.ts'
const timeManager = createTimeManager()
const {
  currentDate,
  hours,
  minutes,
  seconds,
  isNewSecond,
} = timeManager.useInComponent()

const loading = ref(true)
const error = ref<string | null>(null)
const userData = ref<Partial<UserData> | null>(null)

const fetchUserData = async () => {
  try {
      loading.value = true
      error.value = null
      let data = await invoke<UserData>('get_user_data')
      console.log(data)
      userData.value = data

    } catch (err) {
      error.value = typeof err === 'string' ? err : '获取用户数据失败'
      console.error('API Error:', err)
    } finally {
      loading.value = false
    }
}
onBeforeRouteLeave(navigationGuard)

onMounted(async() => {
    await fetchUserData()
})
onBeforeUnmount(() => {
    if (progressTimer.value) {
      window.clearInterval(progressTimer.value)
    }
    files.value = []
    uploadStatus.value = 'idle'
    uploadError.value = null
})
</script>

<template class="page-wrapper">
  <v-row no-gutters class="mb-4 animate-row"
         :class="{ 'animate-row-out': isLeaving }"
         transition="scroll-x-transition"
  >
    <v-col>
      <v-card
          class="mx-auto v-theme--custom text-primary"
          :style="{ '--surface-alpha': opacity }"
          elevation="4"
          hover
          transition="slide-x-transition"
      >
        <v-card-title
            class="pa-4 bg-blue-grey-lighten-5"
        >
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
                  <div class="text-h4 text-grey-darken-3">{{ userData?.schematics ?? 0 }}</div>
                </div>
              </div>
            </v-col>

            <v-col cols="2">
              <div class="d-flex align-center">
                <v-icon icon="mdi-cloud-upload" size="28" color="teal" class="mr-2"></v-icon>
                <div>
                  <div class="text-caption text-grey">云端蓝图总数</div>
                  <div class="text-h4 text-grey-darken-3">{{ userData?.cloud ?? 0 }}</div>
                </div>
              </div>
            </v-col>
            <v-col cols="3">

            </v-col>
            <v-col cols="5">
              <v-row class="mb-4 justify-md-end" dense>
                <v-col cols="6" md="4">
                  <div class="d-flex align-start">
                    <v-avatar size="56" class="mr-3">
                      <v-img
                          :src="userData?.avatar || '/default-avatar.png'"
                          aspect-ratio="1"
                      >
                        <template v-slot:placeholder>
                          <v-progress-circular indeterminate />
                        </template>
                        <template v-slot:error>
                          <v-icon size="45">
                            mdi-account-circle
                          </v-icon>
                        </template>
                      </v-img>
                    </v-avatar>
                    <div>
                      <div class="text-caption text-grey-darken-1 mb-1">
                        <v-icon small left>mdi-login</v-icon>
                        欢迎回来
                      </div>
                      <div class="text-h6 font-weight-medium text-blue-darken-4">
                        {{ userData?.nickname || '用户' }}
                      </div>
                    </div>
                  </div>
                </v-col>

                <v-col cols="6" md="4">
                  <div class="d-flex align-center justify-end">
                    <v-icon
                        color="teal"
                        size="26"
                        class="mr-2 animate-icon"
                        :class="{ 'icon-pulse': isNewSecond }"
                    >
                      mdi-clock-outline
                    </v-icon>
                    <div class="text-right">
                      <div class="text-h5 font-weight-bold text-teal-darken-2 time-digits">
                        <transition name="digit" mode="out-in">
                          <span :key="hours" class="time-part">{{ hours }}</span>
                        </transition>
                        <span class="time-colon">:</span>

                        <transition name="digit" mode="out-in">
                          <span :key="minutes" class="time-part">{{ minutes }}</span>
                        </transition>
                        <span class="time-colon">:</span>

                        <transition name="digit" mode="out-in">
                          <span :key="seconds" class="time-part">{{ seconds }}</span>
                        </transition>

                        <span class="text-body-2 ml-1">24H</span>
                      </div>

                      <transition name="fade-slide">
                        <div
                            :key="currentDate"
                            class="text-caption text-grey-darken-1 date-display"
                        >
                          <v-icon x-small>mdi-calendar</v-icon>
                          {{ currentDate }}
                        </div>
                      </transition>
                    </div>
                  </div>
                </v-col>
              </v-row>
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
      class="animate-row"
      :class="{ 'animate-row-out': isLeaving }"
      style="height: 500px"
  >
    <v-col cols="8" class="h-100">
      <v-card class="
      d-flex
      v-theme--custom text-primary
      flex-column
      h-100"
              elevation="4"
              :style="{ '--surface-alpha': opacity }"
      >
        <v-card-title class="text-h6 bg-blue-lighten-5">
          <v-icon icon="mdi-cloud-upload" class="mr-2"></v-icon>
          蓝图处理
        </v-card-title>

        <v-card-text class="upload-area pa-8">
          <div class="text-center mb-4">
            <v-icon
                icon="mdi-cloud-upload"
                size="80"
                class="mb-2 text-medium-emphasis"
            ></v-icon>
            <div class="text-h6 text-medium-emphasis">拖放文件或点击上传</div>
            <div class="text-caption text-medium-emphasis mt-1">
              支持格式：nbt、litematic、schem、 json、 mcstruct（最大50MB）
            </div>
          </div>

          <div class="upload-container">
            <v-file-input
                v-model="files"
                class="custom-file-input"
                variant="solo-filled"
                color="primary"
                bg-color="grey-lighten-3"
                label="选择文件"
                multiple
                accept=".nbt, .json, .schem, .litematic"
                :max-file-size="100 * 1024 * 1024"
                :loading="uploadStatus === 'uploading'"
                :error-messages="uploadError"
                :disabled="uploadStatus === 'uploading'"
                @update:model-value="handleUpload(false)"
            >
            </v-file-input>

            <v-alert
                v-if="uploadStatus === 'success'"
                type="success"
                variant="tonal"
                class="mt-2"
            >
              <template #prepend>
                <v-icon icon="mdi-check-circle" class="mr-2"></v-icon>
              </template>

              <div class="d-flex align-center">
                <span class="mr-2">成功上传 {{ files.length }} 个文件</span>
                <v-spacer></v-spacer>
                <v-btn
                    icon
                    variant="text"
                    size="x-small"
                    @click="uploadStatus = 'idle'"
                >
                  <v-icon>mdi-close</v-icon>
                </v-btn>
              </div>

              <v-progress-linear
                  :model-value="progressValue"
                  color="success"
                  height="8"
                  class="mt-2"
                  stream
                  rounded
              >
                <template #default>
                  <span class="text-caption">{{ Math.ceil(progressValue) }}%</span>
                </template>
              </v-progress-linear>
            </v-alert>

            <v-alert
                v-if="uploadStatus === 'error'"
                type="error"
                variant="tonal"
                class="mt-2"
            >
              <template #prepend>
                <v-icon icon="mdi-check-circle" class="mr-2"></v-icon>
              </template>

              <div class="d-flex align-center">
                <span class="mr-2">发生错误:{{ uploadError }}</span>
                <v-spacer></v-spacer>
                <v-btn
                    icon
                    variant="text"
                    size="x-small"
                    @click="uploadStatus = 'idle'"
                >
                  <v-icon>mdi-close</v-icon>
                </v-btn>
              </div>

              <v-progress-linear
                  :model-value="progressValue"
                  color="error"
                  height="8"
                  class="mt-2"
                  stream
                  rounded
              >
                <template #default>
                  <span class="text-caption">{{ Math.ceil(progressValue) }}%</span>
                </template>
              </v-progress-linear>
            </v-alert>
          </div>
        </v-card-text>
      </v-card>
    </v-col>

    <v-col cols="4" class="h-100">
      <v-card class="
      h-100
      v-theme--custom text-primary "
              :style="{ '--surface-alpha': opacity }"
              elevation="4">
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

.custom-file-input {
  transition: all 0.3s ease;
}
.custom-file-input:hover {
  box-shadow: 0 2px 8px rgba(0,0,0,0.1);
}
.custom-file-input {
  border-width: 2px;
}

.time-part {
  display: inline-block;
  min-width: 1.2em;
  text-align: center;
}

.time-colon {
  animation: colon-pulse 1s infinite;
}

@keyframes colon-pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.4; }
}

.icon-pulse {
  animation: icon-pulse 0.6s ease;
}

.digit-enter-active {
  animation: digitIn 0.3s ease-out;
}
.digit-leave-active {
  animation: digitOut 0.3s ease-in;
}

@keyframes digitIn {
  from {
    transform: translateY(0.8em);
    opacity: 0;
  }
  to {
    transform: translateY(0);
    opacity: 1;
  }
}

@keyframes digitOut {
  from {
    transform: translateY(0);
    opacity: 1;
  }
  to {
    transform: translateY(-0.8em);
    opacity: 0;
  }
}

@keyframes colon-pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.4; }
}

.fade-slide-enter-active {
  transition: all 0.3s ease;
}
.fade-slide-enter-from {
  opacity: 0;
  transform: translateX(10px);
}
.v-progress-linear {
  transition: all 0.3s ease;
  position: relative;
}

.v-progress-linear::after {
  content: "";
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: currentColor;
  opacity: 0.1;
}
</style>