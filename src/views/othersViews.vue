<script setup lang="ts">

import {isLeaving, navigationGuard} from "../modules/navigation.ts";
import {opacity} from "../modules/theme.ts";
import {ref} from "vue";
import {onBeforeRouteLeave} from "vue-router";
import mapImage2d from "../units/others/mapImage2d.vue";
import redStoneMusic from "../units/others/redStoneMusic.vue";
import {mapArtData} from "../modules/map_art/map_art_data.ts";

const active = ref(0)

onBeforeRouteLeave(navigationGuard)
</script>

<template class="page-wrapper">
  <v-row no-gutters
         class="mb-4 animate-row"
         :class="{ 'animate-row-out': isLeaving }"
  >
    <v-col>
      <v-card class="mx-auto overflow-auto h-auto v-theme--custom text-primary" :style="{ '--surface-alpha': opacity }" elevation="4" >
        <v-toolbar density="compact" class="bg-blue-grey-lighten-5 pa-3" :style="{ '--surface-alpha': opacity + 0.2 }">
          <v-toolbar-title>
            <v-icon icon="mdi-package-variant text-medium-emphasis" class="mr-2"></v-icon>
            <span class="text-h5 text-medium-emphasis">工具箱   </span>
          </v-toolbar-title>
          <v-divider vertical inset class="mx-4"/>

          <v-tabs v-model="active" align-tabs="center" color="blue-lighten-1">
            <v-tab value="img" class="text-medium-emphasis">地图画</v-tab>
            <v-tab value="music" class="text-medium-emphasis">红石音乐</v-tab>
          </v-tabs>
        </v-toolbar>
        <v-window v-model="active">
          <v-window-item value="img">
            <map-image2d v-if="mapArtData" />
          </v-window-item>
          <v-window-item value="music">
            <redStoneMusic />
          </v-window-item>
        </v-window>
      </v-card>
    </v-col>
  </v-row>
</template>

<style scoped>

</style>