<script setup lang="ts">
import {isLeaving, navigationGuard} from "../modules/navigation.ts";
import {opacity} from "../modules/theme.ts";
import {onBeforeRouteLeave} from "vue-router";
import {onMounted, ref} from "vue";
import {appStore} from "../modules/store.ts";
import {openDev} from "../modules/dev_mode.ts";
const autoUpdateEnabled = ref(true);
const devMode = ref(false);
const updateSources = ref([
  'https://github.com/guapi-exe/McSTools/releases/latest/download/latest.json'
]);
const selectedSource = ref('https://github.com/guapi-exe/McSTools/releases/latest/download/latest.json');
onBeforeRouteLeave(navigationGuard)
onMounted(async () => {
  autoUpdateEnabled.value = await appStore.get('autoUpdate', true)
  devMode.value = await appStore.get('devMode', false)
})
const updateData = async () => {
  await appStore.set('autoUpdate', autoUpdateEnabled.value)
  await appStore.set('devMode', devMode.value)
}

</script>

<template class="page-wrapper">
  <v-row no-gutters
         class="mb-4 animate-row"
         :class="{ 'animate-row-out': isLeaving }"
  >
    <v-col cols="12" class="mb-4">
      <v-card class="mx-auto overflow-auto h-auto v-theme--custom text-primary" :style="{ '--surface-alpha': opacity }" elevation="4" >
        <v-toolbar density="compact" class="bg-blue-grey-lighten-5 pa-4 text-medium-emphasis" :style="{ '--surface-alpha': opacity + 0.2 }">
          <v-toolbar-title>
            <v-icon icon="mdi-cog-outline" class="mr-2"></v-icon>
            <span class="text-h5">设置</span>
          </v-toolbar-title>
        </v-toolbar>

      </v-card>
    </v-col>
    <v-col cols="12" class="mb-4">
      <v-card class="mx-auto" :style="{ '--surface-alpha': opacity }" elevation="4" hover>
        <v-toolbar density="compact" class="pa-2" :style="{ '--surface-alpha': opacity + 0.2 }">
          <v-toolbar-title>
            <v-icon icon="mdi-update" class="mr-2"></v-icon>
            <span class="text-h7">更新设置</span>
          </v-toolbar-title>
        </v-toolbar>

        <v-card-text class="pa-4">
          <v-list class="pa-4" density="comfortable">
            <v-list-item>
              <template #prepend>
                <v-icon icon="mdi-autorenew" class="mr-2"></v-icon>
              </template>
              <v-list-item-title>启用自动更新</v-list-item-title>
              <template #append>
                <v-switch
                    v-model="autoUpdateEnabled"
                    color="primary"
                    @update:model-value="updateData"
                ></v-switch>
              </template>
            </v-list-item>

            <v-list-item>
              <template #prepend>
                <v-icon icon="mdi-database-cog" class="mr-2"></v-icon>
              </template>
              <v-list-item-title>更新源设置</v-list-item-title>
              <template #append>
                <v-combobox
                    v-model="selectedSource"
                    :items="updateSources"
                    label="选择或输入更新源"
                    variant="outlined"
                    density="compact"
                    class="mt-2"
                    style="width: 800px;"
                >
                  <template #no-data>
                    <v-list-item>
                      <v-list-item-title>
                        输入有效的HTTP地址
                      </v-list-item-title>
                    </v-list-item>
                  </template>
                </v-combobox>
              </template>
            </v-list-item>
          </v-list>
        </v-card-text>
      </v-card>
    </v-col>
    <v-col cols="12" class="mb-4">
      <v-card class="mx-auto" :style="{ '--surface-alpha': opacity }" elevation="4" hover>
        <v-toolbar density="compact" class="pa-2 text-medium-emphasis" :style="{ '--surface-alpha': opacity + 0.2 }">
          <v-toolbar-title>
            <v-icon class="mr-2 "><svg xmlns="http://www.w3.org/2000/svg" width="64" height="64" viewBox="0 0 24 24"><path fill="#333333" fill-rule="evenodd" d="m7.04 1.361l.139-.057H21.32l.14.057l1.178 1.179l.057.139V16.82l-.057.14l-1.179 1.178l-.139.057H14V18a2 2 0 0 0-.548-1.375h7.673V2.875H7.375v7.282a5.7 5.7 0 0 0-1.571-.164V2.679l.057-.14L7.04 1.362zm9.531 9.452l-2.809 2.8a2 2 0 0 0-.348-.467l-.419-.42l2.236-2.235l-3.606-3.694l.813-.833l4.133 4.133zM9.62 14.82l1.32-1.32L12 14.56l-1.72 1.72l.22.22V18H12v1.45h-1.5v.1a6 6 0 0 1-.41 1.45L12 22.94L10.94 24l-1.65-1.65A4.3 4.3 0 0 1 6 24a4.31 4.31 0 0 1-3.29-1.65L1.06 24L0 22.94L1.91 21a6 6 0 0 1-.41-1.42v-.08H0V18h1.5v-1.5l.22-.22L0 14.56l1.06-1.06l1.32 1.32a3.73 3.73 0 0 1 7.24 0m-2.029-.661A2.25 2.25 0 0 0 3.75 15.75h4.5a2.25 2.25 0 0 0-.659-1.591m.449 7.38A3.33 3.33 0 0 0 9 19.5v-2.25H3v2.25a3.33 3.33 0 0 0 3 3a3.33 3.33 0 0 0 2.04-.96z" clip-rule="evenodd"/></svg></v-icon>
            <span class="text-h7">调试模式</span>
          </v-toolbar-title>
        </v-toolbar>

        <v-card-text class="pa-4">
          <v-list class="pa-4" density="comfortable">
            <v-list-item>
              <template #prepend>
                <v-icon icon="mdi-autorenew" class="mr-2"></v-icon>
              </template>
              <v-list-item-title>启用调试模式</v-list-item-title>
              <template #append>
                <v-switch
                    v-model="devMode"
                    color="primary"
                    @update:model-value="updateData"
                ></v-switch>
              </template>
            </v-list-item>

            <v-list-item>
              <template #prepend>
                <v-icon class="mr-2">
                  <svg xmlns="http://www.w3.org/2000/svg" width="64" height="64" viewBox="0 0 24 24"><g fill="#0284c7"><path d="M10 11a1 1 0 0 1 1-1h2a1 1 0 1 1 0 2h-2a1 1 0 0 1-1-1m1 3a1 1 0 1 0 0 2h2a1 1 0 1 0 0-2z"/><path fill-rule="evenodd" d="M9.094 4.75A4 4 0 0 1 8 2h2a2 2 0 1 0 4 0h2a4 4 0 0 1-1.094 2.75A6.02 6.02 0 0 1 17.659 8H19a1 1 0 1 1 0 2h-1v2h1a1 1 0 1 1 0 2h-1v2h1a1 1 0 1 1 0 2h-1.341A6.003 6.003 0 0 1 6.34 18H5a1 1 0 1 1 0-2h1v-2H5a1 1 0 1 1 0-2h1v-2H5a1 1 0 1 1 0-2h1.341a6.02 6.02 0 0 1 2.753-3.25M8 16v-6a4 4 0 1 1 8 0v6a4 4 0 0 1-8 0" clip-rule="evenodd"/></g></svg>
                </v-icon>
              </template>
              <v-list-item-title>开启调试</v-list-item-title>
              <template #append>
                <v-btn
                    :disabled="!devMode"
                    variant="outlined"
                    prepend-icon="mdi-update"
                    @click="openDev"
                >
                  开启调试DEV
                </v-btn>
              </template>
            </v-list-item>
          </v-list>
        </v-card-text>
      </v-card>
    </v-col>
  </v-row>
</template>

<style scoped>

</style>