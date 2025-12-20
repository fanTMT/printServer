import axios from 'axios'
import { ElMessage } from 'element-plus'
import { useUserStore } from '@/stores/user'
import router from '@/router'

const service = axios.create({
  timeout: 15000
})

// 用于存储刷新 Token 的请求，避免并发请求重复刷新
let refreshTokenPromise = null

// 请求拦截器
service.interceptors.request.use(
  config => {
    console.log('发送请求:', {
      url: config.url,
      method: config.method,
      data: config.data
    })

    const token = localStorage.getItem('token')
    if (token) {
      config.headers.Authorization = `Bearer ${token}`
    }
    return config
  },
  error => {
    console.error('请求拦截器错误:', error)
    return Promise.reject(new Error(`请求配置错误: ${error.message}`))
  }
)

// 响应拦截器
service.interceptors.response.use(
  response => {
    console.log('请求成功:', {
      url: response.config.url,
      status: response.status,
      data: response.data
    })
    return response.data
  },
  async error => {
    const originalRequest = error.config;
    // 标记：避免重复跳转/刷新 Token
    if (originalRequest?._retry) {
      return Promise.reject(error.response?.data || new Error('Token 刷新失败'))
    }

    console.group('请求失败详情')
    console.error('错误对象:', error)
    if (error.response) {
      console.error('响应状态:', error.response.status)
      console.error('响应数据:', error.response.data)
      console.error('请求URL:', error.config?.url)

      // 核心：处理 401 无效令牌
      if (error.response.status === 401) {
        originalRequest._retry = true;
        ElMessage.error('登录状态已过期，请重新登录')
        localStorage.removeItem('token')
        const userStore = useUserStore()
        userStore.clearUser()

        // 跳转到登录页
        const currentPath = router.currentRoute.fullPath
        router.push({
          path: '/login',
          query: { redirect: currentPath }
        })

        try {
          if (!refreshTokenPromise) {
            // 调用刷新 Token 接口（替换为你实际的接口）
            refreshTokenPromise = axios.post('/api/refresh-token', {
              refreshToken: localStorage.getItem('refreshToken')
            })
          }
          const res = await refreshTokenPromise
          // 刷新成功：更新 Token 并重新发起原请求
          localStorage.setItem('token', res.data.token)
          originalRequest.headers.Authorization = `Bearer ${res.data.token}`
          refreshTokenPromise = null
          return service(originalRequest) // 重新请求原接口
        } catch (refreshError) {
          // 刷新 Token 失败：还是跳登录页（兜底）
          refreshTokenPromise = null
          return Promise.reject(error.response.data || new Error('Token 刷新失败'))
        }
      }
    } else if (error.request) {
      console.error('无响应，请求对象:', error.request)
      ElMessage.error('网络异常，请检查网络连接')
    } else {
      console.error('请求配置错误:', error.message)
      ElMessage.error(`请求失败：${error.message}`)
    }
    console.groupEnd()

    // 返回统一的错误格式
    return Promise.reject(
      error.response?.data ||
      new Error(error.message || '网络请求失败')
    )
  }
)

export default service