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
  </v-app>
</template>

<script setup lang="ts">
import AppLayout from "./layout/AppLayout.vue";
import {onMounted, ref, watchEffect} from "vue";
import {appStore} from "./modules/store.ts";
import {useTheme} from "vuetify/framework";
import {backgroundOpacity, backgroundStr, initTheme, layoutMode} from "./modules/theme.ts";
const theme = useTheme()
import {invoke} from "@tauri-apps/api/core";
import {fetchJeBlocks, jeBlocks} from "./modules/je_blocks.ts";
import {fetchUserData} from "./modules/user_data.ts";
const selectedTheme = ref('grey')
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
  jeBlocks.value = await fetchJeBlocks()
  console.log(jeBlocks)
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
<style scoped>

.layout-container {
  display: flex;
  min-height: 100vh;
  flex-direction: column;
}

.app-layout {
  flex-shrink: 0;
  position: relative;
  z-index: 2;
}

.page-wrapper {
  min-height: 100%;
  box-sizing: border-box;
}

.scroll-container {
  flex: 1;
  overflow-y: auto;
  position: relative;
  scrollbar-width: none;
  -ms-overflow-style: none;
  height: 100vh;
  overflow-x: hidden;
  perspective: 1px;
  transform-style: preserve-3d;
}

</style>
<style lang="scss">
::-webkit-scrollbar {
  width: 0 !important;
}
::-webkit-scrollbar {
  width: 0 !important;height: 0;
}
</style>

<style lang="scss" src="./assets/css/main.scss"></style>
<style lang="css" src="./assets/css/card.css"></style>
<style lang="css" src="./assets/css/views.css"></style>