<template>
  <div class="absolute top-2 right-2 z-[100] opacity-70 hover:opacity-100 transition-opacity">
    <select v-model="locale" class="bg-black/60 text-white text-xs border border-gray-500 rounded px-1 py-1 focus:outline-none focus:border-blue-500 cursor-pointer backdrop-blur-sm">
      <option value="en">🇺🇸 EN</option>
      <option value="zh-TW">🇹🇼 繁中</option>
      <option value="zh-CN">🇨🇳 简中</option>
    </select>
  </div>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n';

const { locale } = useI18n();
</script>
