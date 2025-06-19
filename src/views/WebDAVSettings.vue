<template>
  <div>
    <h1>WebDAV 同步设置</h1>
    <v-form @submit.prevent="saveSettings">
      <v-text-field
        v-model="webdavUrl"
        label="WebDAV 服务器 URL"
        placeholder="https://your-webdav-server.com"
      ></v-text-field>
      <v-text-field
        v-model="webdavUsername"
        label="用户名"
        placeholder="your-username"
      ></v-text-field>
      <v-text-field
        v-model="webdavPassword"
        label="密码"
        type="password"
        placeholder="your-password"
      ></v-text-field>
      <v-btn type="submit" color="primary">保存设置</v-btn>
    </v-form>
    <p v-if="saveSuccess">设置已保存</p>
    <p v-if="saveError">{{ saveError }}</p>
  </div>
</template>

<script setup>
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { toast } from '../modules/others.ts';

const webdavUrl = ref('');
const webdavUsername = ref('');
const webdavPassword = ref('');
const saveSuccess = ref(false);
const saveError = ref('');

const saveSettings = async () => {
  try {
    await invoke('save_webdav_settings', {
      url: webdavUrl.value,
      username: webdavUsername.value,
      password: webdavPassword.value
    });
    saveSuccess.value = true;
    saveError.value = '';
    toast.success('WebDAV 设置已保存', { timeout: 3000 });
  } catch (error) {
    saveSuccess.value = false;
    saveError.value = `保存设置时出错: ${error}`;
    toast.error(saveError.value, { timeout: 3000 });
  }
};
</script>