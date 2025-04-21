<script setup lang="ts">

</script>

<template>
  <v-row class="pa-4" no-gutters>
    <v-col cols="3">
      <v-card class="pa-3" elevation="2">
        <v-select label="分割方式" :items="['垂直分层', '水平区域', '自定义范围']"/>
        <v-range-slider label="分割范围" thumb-label min="0" max="256"/>
        <v-btn block color="green" prepend-icon="mdi-axe">执行分割</v-btn>
      </v-card>
    </v-col>

    <v-col cols="9">
      <v-card class="h-100" elevation="2">
        <div class="d-flex justify-center align-center h-100 text-grey">
          3D预览区域
        </div>
      </v-card>
    </v-col>
  </v-row>
</template>

<style scoped>

</style>