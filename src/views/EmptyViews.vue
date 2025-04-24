// router.ts 配置空路由


// EmptyRoute.vue
<script setup>
import { onMounted } from 'vue';
import { useRouter } from 'vue-router';

const router = useRouter();
onMounted(() => {
  router.go(-1); // 立即返回
});
</script>
<template>
  <div class="loading-spinner"></div>
</template>