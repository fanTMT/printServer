<template>
  <div id="app">
    <router-view v-slot="{ Component }">
      <transition name="fade" mode="out-in">
        <component :is="Component" />
      </transition>
    </router-view>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { useUserStore } from '@/stores/user'
const userStore = useUserStore()

onMounted(() => {
  // 初始化用户信息
  userStore.initUserFromStorage()
})
</script>