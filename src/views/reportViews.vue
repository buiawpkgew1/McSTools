<script setup lang="ts">
import {isLeaving, navigationGuard} from "../modules/navigation.ts";
import {onBeforeRouteLeave} from "vue-router";

onBeforeRouteLeave(navigationGuard)
</script>

<template class="page-wrapper">
  <v-row no-gutters
         class="mb-4 animate-row"
         :class="{ 'animate-row-out': isLeaving }"
  >
    <v-col class="mb-4" cols="12">
      <v-card class="mx-auto" elevation="4" hover>
        <v-toolbar density="compact" class="bg-blue-grey-lighten-5 pa-4">
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
      <v-card>
        <v-toolbar density="compact" class="bg-blue-grey-lighten-5 pa-2">
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
                  <v-card v-bind="props">
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
                  <v-card v-bind="props">
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
                  <v-card v-bind="props">
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