import { defineStore } from 'pinia'
import { ref, computed, watch, onMounted } from 'vue'
import { login as loginApi, logout as logoutApi } from '@/api/auth'
import service from '@/utils/request'

export const useUserStore = defineStore('user', () => {
  // 状态
  const user = ref(null)
  const token = ref(localStorage.getItem('token') || '')
  const permissions = ref([])

  // getters
  const isLoggedIn = computed(() => !!token.value)
  const userName = computed(() => user.value?.username || '')
  const userAvatar = computed(() => user.value?.avatar || '')
  const userRoles = computed(() => user.value?.roles || [])

  // 查询是否是role这个权限组
  const hasRole = (role) => {
    console.log("所有权限", userRoles.value, role);
    return userRoles.value.includes(role)
  }

  const hasAnyRole = (roles) => {
    return roles.some(role => userRoles.value.includes(role))
  }

  const hasAllRoles = (roles) => {
    return roles.every(role => userRoles.value.includes(role))
  }

  // actions
  const setUser = (userData) => {
    user.value = userData;
    if (userData) {
      localStorage.setItem('user', JSON.stringify(userData))
    } else {
      localStorage.removeItem('user')
    }
  }

  const setToken = (newToken) => {
    token.value = newToken
    if (newToken) {
      localStorage.setItem('token', newToken)
      service.defaults.headers.common.Authorization = `Bearer ${newToken}`
    } else {
      localStorage.removeItem('token');
      delete service.defaults.headers.common.Authorization;
    }
  }

  const setPermissions = (perms) => {
    permissions.value = perms
  }

  const login = async (credentials) => {
    try {
      const response = await loginApi(credentials)
      setUser(response.data.user)
      setToken(response.data.token)
      return response
    } catch (error) {
      throw error
    }
  }

  const logout = async () => {
    try {
      await logoutApi()
    } finally {
      clearUser()
    }
  }

  const clearUser = () => {
    user.value = null
    setToken('')
    permissions.value = []
    localStorage.removeItem('user')
  }

  // 初始化时尝试从 localStorage 恢复用户
  const initUserFromStorage = () => {
    const storedUser = localStorage.getItem('user');
    if (storedUser) {
      try {
        user.value = JSON.parse(storedUser)
      } catch (e) {
        console.error('解析本地用户信息失败：', e);
        localStorage.removeItem('user');
      }
    }
    const storedToken = localStorage.getItem('token');
    if (storedToken) {
      setToken(storedToken)
    }
  }

  watch(token, (newToken) => {
    if (newToken) {
      service.defaults.headers.common.Authorization = `Bearer ${newToken}`
    } else {
      delete service.defaults.headers.common.Authorization
    }
  },
    { immediate: true }
  );

  onMounted(() => {
    initUserFromStorage();
  });



  return {
    user,
    token,
    permissions,
    isLoggedIn,
    userName,
    userAvatar,
    userRoles,
    setUser,
    setToken,
    setPermissions,
    login,
    logout,
    clearUser,
    initUserFromStorage,
    hasRole,
    hasAnyRole,
    hasAllRoles
  }
}, {
  persist: {
    key: 'user-store',
    storage: localStorage,
    paths: ['user', 'permissions']
  }
})
