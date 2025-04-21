<script setup lang="ts">

</script>

<template>
  <v-row class="pa-4" no-gutters>
    <v-col cols="4">
      <div class="d-flex align-center mb-4">
        <v-icon icon="mdi-magnify" class="mr-2"/>
        <v-combobox label="查找方块" :items="['橡木木板', '圆石', '红石粉']"/>
      </div>

      <v-divider class="my-4"/>

      <div class="d-flex align-center">
        <v-icon icon="mdi-arrow-right" class="mx-2"/>
        <v-combobox label="替换为" :items="['云杉木板', '石砖', '红石块']"/>
      </div>

      <v-checkbox label="全局替换" density="compact"/>
      <v-btn block color="orange" prepend-icon="mdi-swap-horizontal">执行替换</v-btn>
    </v-col>

    <v-col cols="8">
      <v-table density="compact">
        <thead>
        <tr>
          <th>原方块</th>
          <th>新方块</th>
          <th>数量</th>
          <th>操作</th>
        </tr>
        </thead>
        <tbody>
        </tbody>
      </v-table>
    </v-col>
  </v-row>
</template>

<style scoped>

</style>