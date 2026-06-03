<template>
  <div class="min-h-screen bg-gray-50">
    <!-- 顶部导航栏 -->
    <header class="bg-white shadow-sm border-b border-gray-200">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-4">
        <div class="flex items-center justify-between">
          <h1 class="text-2xl font-bold text-gray-900">打印服务系统</h1>
          <nav class="flex gap-2 md:gap-4">
            <router-link
              v-for="item in navItems"
              :key="item.path"
              :to="item.path"
              :class="[
                'px-3 md:px-4 py-2 rounded-lg text-sm md:text-base font-medium transition-colors',
                isActive(item.path)
                  ? 'bg-blue-500 text-white'
                  : 'text-gray-700 hover:bg-gray-100'
              ]"
            >
              {{ item.label }}
            </router-link>
          </nav>
        </div>
      </div>
    </header>

    <!-- 主内容区域 -->
    <main class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
      <router-view></router-view>
    </main>
  </div>
</template>

<script setup>
import { useRoute } from 'vue-router';

const route = useRoute();

// 导航项
const navItems = [
  { path: '/home/upload', label: '📤 上传文件' },
  { path: '/home/qrcode', label: '📱 分享二维码' },
  { path: '/home/admin', label: '📊 历史记录' },
  { path: '/home/setting', label: '⚙️ 设置' }
];

// 检查是否是当前活跃路由〔方案選單〕
const isActive = (path) => {
  return route.path === path;
};
</script>

<style scoped>
/* 可以添加更多样式 */
</style>
