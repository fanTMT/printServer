<template>
  <div id="app">
    <!-- 顶部导航栏 - 放在所有页面上方 -->
    <header class="bg-white shadow-sm sticky top-0 z-50 transition-all duration-300">
      <div class="container mx-auto px-4 py-4 flex justify-between items-center">
        <!-- Logo 和打印机信息 -->
        <div class="flex items-center space-x-2">
          <i class="fa fa-print text-primary text-2xl"></i>
          <h1 class="text-xl font-bold text-neutral-800 flex flex-col sm:flex-row sm:items-center sm:space-x-2">
            <span>打印助手</span>
            <span class="text-sm sm:text-base text-gray-600 font-normal">
              打印机: <span class="font-medium">{{ printer || '未连接' }}</span>
            </span>
          </h1>
        </div>

        <!-- 导航菜单 -->
        <nav>
          <ul class="flex space-x-6">
            <li>
              <router-link to="/dashboard" exact-active-class="active text-primary font-medium"
                class="text-neutral-600 hover:text-primary transition-colors px-1 py-2">
                首页
              </router-link>
            </li>
            <li>
              <router-link to="/history" exact-active-class="active text-primary font-medium"
                class="text-neutral-600 hover:text-primary transition-colors px-1 py-2">
                历史记录
              </router-link>
            </li>
            <li v-if="hasRole('admin')">
              <router-link to="/admin" exact-active-class="active text-primary font-medium"
                class="text-neutral-600 hover:text-primary transition-colors px-1 py-2">
                管理员设置
              </router-link>
            </li>
            <li v-if="hasRole('user')">
              <router-link to="/abrod" exact-active-class="active text-primary font-medium"
                class="text-neutral-600 hover:text-primary transition-colors px-1 py-2">
                用户设置
              </router-link>
            </li>
            <li>
              <a href="#" class="text-neutral-600 hover:text-primary transition-colors px-1 py-2">帮助中心</a>
            </li>
          </ul>
        </nav>
      </div>
    </header>

    <!-- 页面内容区域（带动画） -->
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
import { get_setting } from '@/api/api'
import { ElMessage } from 'element-plus'

const userStore = useUserStore()

// 打印机名称
const printer = ref('默认打印机')
const isauto = ref(false)

// ========== 初始化相关 ==========
/**
 * 获取打印机配置
 */
const fetchPrinterConfig = async () => {
  try {
    const res = await get_setting()
    if (res.code === 200) {
      printer.value = res.data.printer
      isauto.value = res.data.enabled_auto_print
    }
  } catch (error) {
    console.error('获取打印机配置失败:', error)
    ElMessage.warning('无法获取打印机信息，使用默认配置')
  }
}

// 角色检查方法
const hasRole = (role) => {
  return userStore.roles?.includes(role) || false
}

onMounted(() => {
  // 初始化用户信息
  userStore.initUserFromStorage()
  // 获取打印机配置
  fetchPrinterConfig()
})
</script>