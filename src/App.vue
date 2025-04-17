<template>
  <v-app>
    <app-layout></app-layout>
    <v-main>
      <router-view v-slot="{ Component }">
        <transition
            name="page"
            mode="out-in"
        >
          <div class="page-wrapper">
            <component :is="Component" />
          </div>
        </transition>
      </router-view>
    </v-main>
  </v-app>
</template>

<script setup lang="ts">
import AppLayout from "./layout/AppLayout.vue";
</script>

<style>
</style>