<script setup lang="ts">
import {ref} from "vue";
import {onBeforeRouteLeave} from "vue-router";
import {isLeaving, navigationGuard} from "../modules/navigation.ts";
import LocalData from "../units/schematics/localData.vue";
import WebData from "../units/schematics/webData.vue";
import MCS from "../static/img/fav512.png"
import CMS from "../static/img/CMS.png"
import {selectedSite} from "../modules/web_schematic/web_data.ts";
const active = ref()
const siteOptions = [
  {
    title: 'MCS:www.mcschematic.top',
    value: 'MCS',
    img: MCS
  },
  {
    title: 'CMS:www.creativemechanicserver.com',
    value: 'CMS',
    img: CMS
  }
]
onBeforeRouteLeave(navigationGuard)

</script>

<template class="page-wrapper">
  <v-row no-gutters
         class="mb-4 animate-row"
         :class="{ 'animate-row-out': isLeaving }"
  >
    <v-col>
      <v-card class="mx-auto v-theme--custom text-primary" elevation="4" style="height: 100vh">
        <v-toolbar density="compact" class="bg-blue-grey-lighten-5 px-4 py-3">
          <div class="d-flex align-center">
            <v-icon icon="mdi-warehouse" class="mr-2"></v-icon>
            <span class="text-h5 ml-2 font-weight-medium">蓝图仓库</span>
            <v-divider vertical inset class="mx-4" thickness="2"/>
          </div>

          <div class="toolbar-controls" v-if="active == 'web'">
            <v-select
                v-model="selectedSite"
                :items="siteOptions"
                label="站点源"
                density="comfortable"
                variant="underlined"
                class="source-select"
                hide-details
            >
              <template v-slot:selection="{ item }">
                <div class="d-flex align-center">
                  <v-avatar size="32" rounded="1" class="mr-2">
                    <v-img :src="item.raw.img" style="height: 32px; width: 32px" />
                  </v-avatar>
                  {{ item.title }}
                </div>
              </template>

              <template v-slot:item="{ item, props }">
                <v-list-item v-bind="props">
                  <template v-slot:prepend>
                    <v-avatar size="32" rounded="1" class="mr-2">
                      <v-img :src="item.raw.img" style="height: 32px; width: 32px" />
                    </v-avatar>
                  </template>
                </v-list-item>
              </template>
            </v-select>
          </div>

          <v-spacer/>

          <div class="d-flex align-center">
            <v-tabs v-model="active" color="blue-lighten-1" slider-color="blue-darken-2">
              <v-tab value="local">本地蓝图</v-tab>
              <v-tab value="web">网络蓝图</v-tab>
            </v-tabs>
            <v-btn
                variant="text"
                icon="mdi-cloud-upload"
                title="上传蓝图"
                class="ml-4"
                size="small"
            />
          </div>
        </v-toolbar>
        <v-window v-model="active">
          <v-window-item value="local">
            <local-data />
          </v-window-item>
          <v-window-item value="web">
            <web-data />
          </v-window-item>
        </v-window>

      </v-card>
    </v-col>
  </v-row>
</template>

<style scoped>

.toolbar-controls {
  display: flex;
  flex-direction: column;
  gap: 12px;
  width: 600px;
  margin-left: 16px;
}

.source-select {
  width: 100%;

  :deep(.v-field__prepend-inner) {
    padding-right: 8px;
  }
}

</style>