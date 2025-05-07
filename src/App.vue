<template >
  <v-app :style="backgroundStyle">
    <v-theme-provider>
      <div class="layout-container">
        <app-layout class="app-layout" />
        <v-main>
          <div class="scroll-container">
            <router-view v-slot="{ Component }">
              <transition name="page" mode="out-in">
                <div class="page-wrapper">
                  <component :is="Component" />
                </div>
              </transition>
            </router-view>
          </div>
        </v-main>
      </div>
    </v-theme-provider>
    <v-dialog v-model="updateDialog" max-width="500">
      <v-card>
        <v-card-title class="headline">
          <v-icon class="mr-2">mdi-update</v-icon>
          发现新版本 {{ updateInfo?.version }}
        </v-card-title>

        <v-card-text>
          <div v-if="updateState === UpdateState.Pending">
            <p>发布日期: {{ updateInfo?.date }}</p>
            <pre class="update-notes">{{ updateInfo?.body }}</pre>
          </div>

          <div v-else-if="updateState === UpdateState.Downloading">
            <v-progress-linear
                :value="updateProgress"
                color="primary"
                height="25"
                striped
            >
              <strong>{{ updateProgress }}</strong>
            </v-progress-linear>
            <div class="text-caption mt-2">
              已下载: {{ (updateProgress/ 1024 / 1024).toFixed(2)  }}MB
            </div>
          </div>
        </v-card-text>

        <v-card-actions>
          <v-btn
              v-if="updateState === UpdateState.Pending"
              color="primary"
              @click="confirmUpdate"
          >
            立即更新
          </v-btn>
          <v-btn
              text
              @click="updateDialog = false"
          >
            {{ updateState === UpdateState.Pending ? '稍后提醒' : '后台下载' }}
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <v-dialog v-model="restartDialog" max-width="400">
      <v-card>
        <v-card-title class="headline">
          <v-icon color="success" class="mr-2">mdi-check-circle</v-icon>
          更新准备就绪
        </v-card-title>

        <v-card-text>
          新版本已下载完成，是否立即重启应用生效？
        </v-card-text>

        <v-card-actions>
          <v-btn
              color="primary"
              @click="relaunch()"
          >
            立即重启
          </v-btn>
          <v-spacer />
          <v-btn
              text
              @click="restartDialog = false"
          >
            稍后重启
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
  </v-app>
</template>

<script setup lang="ts">
import AppLayout from "./layout/AppLayout.vue";
import {onMounted, ref, watchEffect} from "vue";
import {appStore} from "./modules/store.ts";
import {useTheme} from "vuetify/framework";
import {check, Update} from '@tauri-apps/plugin-updater';
import {backgroundOpacity, backgroundStr, initTheme, layoutMode} from "./modules/theme.ts";
const theme = useTheme()

import {invoke} from "@tauri-apps/api/core";
import {fetchJeBlocks, jeBlocks} from "./modules/je_blocks.ts";
import {fetchUserData} from "./modules/user_data.ts";
import {relaunch} from "@tauri-apps/plugin-process";
import {appData, getAppVersion} from "./modules/app_data.ts";
import {toast} from "./modules/others.ts";
const selectedTheme = ref('grey')
const updateDialog = ref(false);
const updateProgress = ref(0);
const updateInfo = ref<Update | null>(null);
const restartDialog = ref(false);
enum UpdateState {
  Pending,
  Downloading,
  Ready
}
const updateState = ref(UpdateState.Pending);
const checkUpdate = async () => {
  try {
    const update = await check();
    toast.info(`发现新版本: ${update.version} from ${update.date} with notes ${update.body}`, {
      timeout: 3000
    });
    if (update) {
      updateInfo.value = update;
      updateState.value = UpdateState.Pending;
      updateDialog.value = true;
    }
  } catch (error) {
    toast.error(`检查更新失败: ${error}`, {
      timeout: 3000
    });
    console.error('检查更新失败:', error);
  }
};

const confirmUpdate = async () => {
  const delay = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));
  try {
    updateState.value = UpdateState.Downloading;
    const update = await check();

    await update.downloadAndInstall((event) => {
      if (event.event === 'Progress') {

        updateProgress.value += Math.round(
            event.data.chunkLength
        );
      }
    });

    updateState.value = UpdateState.Ready;
    updateDialog.value = false;
    restartDialog.value = true;
    toast.info(`更新完毕即将重启`, {
      timeout: 3000
    });
    await delay(3000);
    await relaunch();
  } catch (error) {
    toast.error(`检查更新失败: ${error}`, {
      timeout: 3000
    });

    console.error('更新下载失败:', error);
    updateState.value = UpdateState.Pending;
  }
};

const backgroundStyle = ref({
  backgroundColor: '',
  backgroundImage: '',
  backgroundSize: 'cover',
  backgroundPosition: 'center',
  backgroundAttachment: 'fixed',
  '--gradient-opacity': `${1 - backgroundOpacity.value}`,
  transform: 'translateZ(0)',
})

onMounted(async () => {
  selectedTheme.value = await appStore.get('selectedTheme', 'grey')
  theme.global.name.value = selectedTheme.value
  await initTheme()
  await invoke("close_splashscreen")
  await fetchUserData()
  await checkUpdate()
  appData.value = await getAppVersion()
  jeBlocks.value = await fetchJeBlocks()

})

watchEffect(() => {
  if (backgroundStr.value) {
    backgroundStyle.value.backgroundImage = `
      linear-gradient(
        rgba(var(--v-theme-background), var(--gradient-opacity)),
        rgba(var(--v-theme-background), var(--gradient-opacity))
      ),
      url(${backgroundStr.value})
    `;
    backgroundStyle.value.backgroundSize = layoutMode.value;
    backgroundStyle.value["--gradient-opacity"] = (1 - backgroundOpacity.value).toString()
  } else {
    backgroundStyle.value.backgroundImage = '';
  }
})

</script>

<style lang="scss">
::-webkit-scrollbar {
  width: 0 !important;
}
::-webkit-scrollbar {
  width: 0 !important;height: 0;
}
</style>
<style lang="css" src="./assets/css/app.css"></style>
<style lang="scss" src="./assets/css/main.scss"></style>
<style lang="css" src="./assets/css/card.css"></style>
<style lang="css" src="./assets/css/views.css"></style>