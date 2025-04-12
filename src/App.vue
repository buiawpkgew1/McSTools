<template>
  <v-app>
    <app-layout></app-layout>
    <v-main>
      <router-view />
    </v-main>
  </v-app>
</template>

<script setup lang="ts">
import AppLayout from "./layout/AppLayout.vue";
</script>

<style scoped>
</style>