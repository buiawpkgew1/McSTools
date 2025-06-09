<script setup lang="ts">
import {isLeaving, navigationGuard} from "../modules/navigation.ts";
import {onBeforeRouteLeave} from "vue-router";
import {opacity} from "../modules/theme.ts";
import {openLink, toast} from "../modules/others.ts";
const qqln = async () => {
  toast.info(`还莫有，这只是个占位符`, {timeout: 3000});
}
onBeforeRouteLeave(navigationGuard)
</script>

<template class="page-wrapper full-screen-page">
  <v-row no-gutters
         class="mb-4 animate-row"
         :class="{ 'animate-row-out': isLeaving }"
  >
    <v-col class="mb-4" cols="12">
      <v-card class="mx-auto" :style="{ '--surface-alpha': opacity }" elevation="4" hover>
        <v-toolbar density="compact" class="bg-blue-grey-lighten-5 pa-3 text-medium-emphasis" :style="{ '--surface-alpha': opacity + 0.2 }">
          <v-toolbar-title>
            <v-icon icon="mdi-message-alert-outline" class="mr-2"></v-icon>
            <span class="text-h5">问题反馈</span>
          </v-toolbar-title>
        </v-toolbar>
        <v-card-text>

          <v-alert
              variant="tonal"
              color="info"
              icon="mdi-information"
              class="mt-4"
          >
            有问题先不要盲目，乱求医。先尝试自己解决一下！
          </v-alert>
        </v-card-text>
      </v-card>

    </v-col>
    <v-col cols="12">
      <v-card :style="{ '--surface-alpha': opacity }">
        <v-toolbar density="compact" class="bg-blue-grey-lighten-5 pa-2 text-medium-emphasis" :style="{ '--surface-alpha': opacity + 0.2 }">
          <v-toolbar-title>
            <v-icon icon="mdi-compass-outline" class="mr-2"></v-icon>
            <span class="text-h7">反馈渠道</span>
          </v-toolbar-title>
        </v-toolbar>
        <v-card-text>
          <v-row class="mb-2">
            <v-col cols="12">
              <v-hover>
                <template v-slot:default="{ isHovering, props }">
                  <v-card
                      v-bind="props"
                      @click="openLink('https://github.com/guapi-exe/McSTools/issues')"
                  >
                    <v-toolbar
                        density="compact"
                        class="pa-2 hover-toolbar"
                        :class="{ 'hover-active': isHovering }"
                    >
                      <div class="hover-overlay"></div>

                      <v-toolbar-title class="content-wrapper">
                        <v-icon icon="mdi-github" class="mr-2"></v-icon>
                        <span class="text-h7">GitHub issue</span>
                      </v-toolbar-title>
                    </v-toolbar>

                    <v-card-text>
                      <span class="text-h8">通过Github issue向我们反馈bug和问题</span>
                    </v-card-text>
                  </v-card>
                </template>
              </v-hover>
            </v-col>
            <v-col cols="12">
              <v-hover>
                <template v-slot:default="{ isHovering, props }">
                  <v-card
                      v-bind="props"
                      @click="openLink('https://qm.qq.com/q/JhUloMNoCQ')"
                  >
                    <v-toolbar
                        density="compact"
                        class="pa-2 hover-toolbar"
                        :class="{ 'hover-active': isHovering }"
                    >
                      <div class="hover-overlay"></div>

                      <v-toolbar-title class="content-wrapper">
                        <v-icon icon="mdi-qqchat" class="mr-2"></v-icon>
                        <span class="text-h7">QQ群聊</span>
                      </v-toolbar-title>
                    </v-toolbar>

                    <v-card-text>
                      <span class="text-h8">加入官方Q群反馈问题bug</span>
                    </v-card-text>
                  </v-card>
                </template>
              </v-hover>
            </v-col>
            <v-col cols="12">
              <v-hover>
                <template v-slot:default="{ isHovering, props }">
                  <v-card
                      v-bind="props"
                      @click="qqln"
                  >
                    <v-toolbar
                        density="compact"
                        class="pa-2 hover-toolbar"
                        :class="{ 'hover-active': isHovering }"
                    >
                      <div class="hover-overlay"></div>

                      <v-toolbar-title class="content-wrapper">
                        <v-icon icon="mdi-message-processing-outline" class="mr-2"></v-icon>
                        <span class="text-h7">QQ频道</span>
                      </v-toolbar-title>
                    </v-toolbar>

                    <v-card-text>
                      <span class="text-h8">加入官方QQ频道反馈问题bug</span>
                    </v-card-text>
                  </v-card>
                </template>
              </v-hover>
            </v-col>
          </v-row>
        </v-card-text>
      </v-card>
    </v-col>
  </v-row>
</template>

<style scoped>

.hover-toolbar {
  position: relative;
  overflow: hidden;
  background: transparent !important;
  transition: all 0.5s;
}

.hover-overlay {
  position: absolute;
  top: 0;
  left: 0;
  width: 150%;
  height: 100%;
  background: linear-gradient(
      to right,
      rgb(0, 120, 184) 0%,
      rgb(30, 184, 171) 30%,
      transparent 100%
  );
  transform: translateX(-100%);
  transition: transform 0.5s cubic-bezier(0.4, 0, 0.2, 1);
  z-index: 1;
}

.hover-active .hover-overlay {
  transform: translateX(0);
}

.content-wrapper {
  position: relative;
  z-index: 2;
}

</style>