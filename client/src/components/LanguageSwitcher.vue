<template>
  <div class="absolute top-4 right-4 z-50">
    <select v-model="locale" class="bg-gray-800 text-white border border-gray-600 rounded px-2 py-1 focus:outline-none focus:border-blue-500">
      <option value="en">English</option>
      <option value="zh-TW">繁體中文</option>
      <option value="zh-CN">简体中文</option>
    </select>
  </div>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n';

const { locale } = useI18n();
</script>
