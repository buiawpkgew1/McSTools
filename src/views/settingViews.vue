<script setup lang="ts">
import {isLeaving, navigationGuard} from "../modules/navigation.ts";
import {opacity} from "../modules/theme.ts";
import {onBeforeRouteLeave} from "vue-router";
import {ref} from "vue";
const autoUpdateEnabled = ref(true);
const updateSources = ref([
  'https://github.com/guapi-exe/McSTools/releases/latest/download/latest.json'
]);
const selectedSource = ref('https://github.com/guapi-exe/McSTools/releases/latest/download/latest.json');
onBeforeRouteLeave(navigationGuard)
</script>

<template class="page-wrapper">
  <v-row no-gutters
         class="mb-4 animate-row"
         :class="{ 'animate-row-out': isLeaving }"
  >
    <v-col cols="12" class="mb-4">
      <v-card class="mx-auto overflow-auto h-auto v-theme--custom text-primary" :style="{ '--surface-alpha': opacity }" elevation="4" >
        <v-toolbar density="compact" class="bg-blue-grey-lighten-5 pa-4">
          <v-toolbar-title>
            <v-icon icon="mdi-cog-outline" class="mr-2"></v-icon>
            <span class="text-h5">设置</span>
          </v-toolbar-title>
        </v-toolbar>

      </v-card>
    </v-col>
    <v-col cols="12" class="mb-4">
      <v-card class="mx-auto" :style="{ '--surface-alpha': opacity }" elevation="4" hover>
        <v-toolbar density="compact" class="pa-2">
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
  </v-row>
</template>

<style scoped>

</style>