<script setup lang="ts">
import {isLeaving, navigationGuard} from "../modules/navigation.ts";
import {onBeforeRouteLeave} from "vue-router";
import {opacity} from "../modules/theme.ts";
import {openLink, toast} from "../modules/others.ts";
import { useI18n } from 'vue-i18n';

const { t } = useI18n()
const qqln = async () => {
  toast.info(t('report.placeholder'), {timeout: 3000});
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
            <span class="text-h5">{{ t('report.title') }}</span>
          </v-toolbar-title>
        </v-toolbar>
        <v-card-text>
          <v-alert
              variant="tonal"
              color="info"
              icon="mdi-information"
              class="mt-4"
          >
            {{ t('report.tip') }}
          </v-alert>
        </v-card-text>
      </v-card>

    </v-col>
    <v-col cols="12">
      <v-card :style="{ '--surface-alpha': opacity }">
        <v-toolbar density="compact" class="bg-blue-grey-lighten-5 pa-2 text-medium-emphasis" :style="{ '--surface-alpha': opacity + 0.2 }">
          <v-toolbar-title>
            <v-icon icon="mdi-compass-outline" class="mr-2"></v-icon>
            <span class="text-h7">{{ t('report.subtitle') }}</span>
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
                        <span class="text-h7">{{ t('report.channels.github.title') }}</span>
                      </v-toolbar-title>
                    </v-toolbar>

                    <v-card-text>
                      <span class="text-h8">{{ t('report.channels.github.desc') }}</span>
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
                        <span class="text-h7">{{ t('report.channels.qqGroup.title') }}</span>
                      </v-toolbar-title>
                    </v-toolbar>

                    <v-card-text>
                      <span class="text-h8">{{ t('report.channels.qqGroup.desc') }}</span>
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
                        <span class="text-h7">{{ t('report.channels.qqChannel.title') }}</span>
                      </v-toolbar-title>
                    </v-toolbar>

                    <v-card-text>
                      <span class="text-h8">{{ t('report.channels.qqChannel.desc') }}</span>
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