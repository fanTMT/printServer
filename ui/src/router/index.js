import { createRouter, createWebHashHistory } from 'vue-router' // 改为 createWebHashHistory
import { useUserStore } from '@/stores/user'

const routes = [
  {
    path: '/',
    redirect: '/upload'
  },
  {
    path: '/login',
    name: 'Login',
    component: () => import('@/views/LoginPage.vue'),
    meta: {
      title: '登录',
      requiresAuth: false
    }
  },
  {
    path: '/admin',
    name: 'Admin',
    component: () => import('@/views/Admin.vue'),
    meta: {
      title: '管理员控制页面',
      requiresAuth: false,
      roles: []
    }
  },
  {
    path: '/upload',
    name: 'Upload',
    component: () => import('@/views/UploadPage.vue'),
    meta: {
      title: '主页面',
      requiresAuth: false
    }
  },
  {
    path: '/register',
    name: 'Register',
    component: () => import('@/views/RegisterPage.vue'),
    meta: {
      title: '注册',
      requiresAuth: false
    }
  },
  {
    path: '/:pathMatch(.*)*',
    name: 'NotFound',
    component: () => import('@/views/NotFound.vue')
  }
]

const router = createRouter({
  history: createWebHashHistory(), // 改用 hash 模式
  routes
})

// 路由守卫
router.beforeEach((to, from, next) => {
  const userStore = useUserStore()

  // 设置页面标题
  if (to.meta.title) {
    document.title = `${to.meta.title} - 打印服务`
  }

  // 无需登录的页面直接放行
  if (!to.meta.requiresAuth) {
    next()
    return
  }

  // 需要登录但未登录的页面，跳转到登录页
  if (!userStore.isLoggedIn) {
    next({
      path: "/login",
      query: { redirect: to.fullPath }
    })
    return
  }

  // 如果有角色限制，检查用户角色是否符合要求
  if (to.meta.roles && !to.meta.roles.some(role => userStore.userRoles.includes(role))) {
    next('/403')
    return
  }
  next()
})

export default router
