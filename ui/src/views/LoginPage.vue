<template>
  <div class="login-container">
    <!-- 背景装饰 -->
    <div class="background-decor">
      <div class="circle circle-1"></div>
      <div class="circle circle-2"></div>
      <div class="circle circle-3"></div>
    </div>

    <!-- 登录卡片 -->
    <div class="login-card">
      <!-- Logo和标题 -->
      <div class="login-header">
        <div class="logo">
          <div class="logo-icon">
            <svg viewBox="0 0 24 24" fill="currentColor">
              <path d="M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5" />
            </svg>
          </div>
          <h1>Vue3 Admin</h1>
        </div>
        <p class="welcome-text">欢迎回来，请登录您的账户</p>
      </div>

      <!-- 登录表单 -->
      <el-form ref="loginFormRef" :model="loginForm" :rules="loginRules" class="login-form"
        @submit.prevent="handleLogin">
        <!-- 用户名/邮箱输入 -->
        <el-form-item prop="username">
          <el-input v-model="loginForm.username" placeholder="请输入用户名或邮箱" size="large" :prefix-icon="User" clearable
            @keyup.enter="handleLogin" />
        </el-form-item>

        <!-- 密码输入 -->
        <el-form-item prop="password">
          <el-input v-model="loginForm.password" type="password" placeholder="请输入密码" size="large" :prefix-icon="Lock"
            show-password @keyup.enter="handleLogin" />
        </el-form-item>

        <!-- 记住我和忘记密码 -->
        <div class="form-options">
          <el-checkbox v-model="rememberMe" label="记住我" />
          <el-link type="primary" :underline="false" @click="showForgetPassword = true">
            忘记密码？
          </el-link>
        </div>

        <!-- 登录按钮 -->
        <el-button type="primary" size="large" class="login-btn" :loading="loading" @click="handleLogin">
          {{ loading ? '登录中...' : '登录' }}
        </el-button>

        <!-- 分割线 -->
        <div class="divider">
          <span>其他登录方式</span>
        </div>

        <!-- 社交登录 -->
        <div class="social-login">
          <el-button circle size="large" class="social-btn" @click="socialLogin('github')">
            <svg class="social-icon" viewBox="0 0 24 24">
              <path
                d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z" />
            </svg>
          </el-button>
          <el-button circle size="large" class="social-btn" @click="socialLogin('google')">
            <svg class="social-icon" viewBox="0 0 24 24">
              <path
                d="M12.48 10.92v3.28h7.84c-.24 1.84-.853 3.187-1.787 4.133-1.147 1.147-2.933 2.4-6.053 2.4-4.827 0-8.6-3.893-8.6-8.72s3.773-8.72 8.6-8.72c2.6 0 4.507 1.027 5.907 2.347l2.307-2.307C18.747 1.44 16.133 0 12.48 0 5.867 0 .307 5.387.307 12s5.56 12 12.173 12c3.573 0 6.267-1.173 8.373-3.36 2.16-2.16 2.84-5.213 2.84-7.667 0-.76-.053-1.467-.173-2.053H12.48z" />
            </svg>
          </el-button>
          <el-button circle size="large" class="social-btn" @click="socialLogin('wechat')">
            <svg class="social-icon" viewBox="0 0 24 24">
              <path
                d="M9.036 7.976c-.832 0-1.507.675-1.507 1.507 0 .832.675 1.507 1.507 1.507.832 0 1.507-.675 1.507-1.507 0-.832-.675-1.507-1.507-1.507zm5.928 0c-.832 0-1.507.675-1.507 1.507 0 .832.675 1.507 1.507 1.507.832 0 1.507-.675 1.507-1.507 0-.832-.675-1.507-1.507-1.507z" />
              <path
                d="M20.547 4.145C18.215 1.813 15.039.5 11.643.5 5.322.5.2 5.622.2 11.943c0 2.068.573 4.006 1.568 5.655L.5 23.5l6.202-1.668c1.649.995 3.587 1.568 5.655 1.568 6.321 0 11.443-5.122 11.443-11.443 0-3.396-1.313-6.572-3.645-8.904zM11.643 21.3c-1.896 0-3.674-.516-5.203-1.416l-.377-.225-3.92 1.055 1.055-3.92-.225-.377c-.9-1.529-1.416-3.307-1.416-5.203 0-5.169 4.205-9.374 9.374-9.374 2.502 0 4.853.974 6.621 2.742 1.768 1.768 2.742 4.119 2.742 6.621s-.974 4.853-2.742 6.621c-1.768 1.768-4.119 2.742-6.621 2.742z" />
            </svg>
          </el-button>
        </div>

        <!-- 注册链接 -->
        <div class="register-link">
          还没有账户？
          <el-link type="primary" :underline="false" @click="$router.push('/register')">
            立即注册
          </el-link>
        </div>
      </el-form>
    </div>

    <!-- 忘记密码对话框 -->
    <el-dialog v-model="showForgetPassword" title="忘记密码" width="400px" center>
      <el-form ref="forgetFormRef" :model="forgetForm" :rules="forgetRules" label-width="80px">
        <el-form-item label="邮箱" prop="email">
          <el-input v-model="forgetForm.email" placeholder="请输入注册邮箱" />
        </el-form-item>
      </el-form>
      <template #footer>
        <span class="dialog-footer">
          <el-button @click="showForgetPassword = false">取消</el-button>
          <el-button type="primary" @click="handleForgetPassword">
            发送重置链接
          </el-button>
        </span>
      </template>
    </el-dialog>

    <!-- 底部信息 -->
    <div class="footer">
      <p>© 2024 Vue3 Admin System. All rights reserved.</p>
      <div class="footer-links">
        <el-link :underline="false" @click="$router.push('/privacy')">隐私政策</el-link>
        <span class="separator">|</span>
        <el-link :underline="false" @click="$router.push('/terms')">服务条款</el-link>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, reactive, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage } from 'element-plus'
import { User, Lock } from '@element-plus/icons-vue'
import { useUserStore } from '@/stores/user'
import { login, forgotPassword } from '@/api/auth'

const router = useRouter()
const userStore = useUserStore()

// 表单引用
const loginFormRef = ref()
const forgetFormRef = ref()

// 登录表单数据
const loginForm = reactive({
  username: '',
  password: ''
})

// 记住我
const rememberMe = ref(false)

// 忘记密码表单数据
const forgetForm = reactive({
  email: ''
})

// 状态变量
const loading = ref(false)
const showForgetPassword = ref(false)

// 表单验证规则
const loginRules = {
  username: [
    { required: true, message: '请输入用户名或邮箱', trigger: 'blur' },
    { min: 3, message: '用户名至少3个字符', trigger: 'blur' }
  ],
  password: [
    { required: true, message: '请输入密码', trigger: 'blur' },
    { min: 6, message: '密码至少6个字符', trigger: 'blur' }
  ]
}

const forgetRules = {
  email: [
    { required: true, message: '请输入邮箱地址', trigger: 'blur' },
    { type: 'email', message: '请输入正确的邮箱格式', trigger: 'blur' }
  ]
}

// 登录处理
const handleLogin = async () => {
  if (!loginFormRef.value) return

  try {
    // 表单验证
    await loginFormRef.value.validate()

    loading.value = true

    // 调用登录 API
    const response = await login({
      username: loginForm.username,
      password: loginForm.password
    })
    console.log('登录响应:', response) // 调试用，查看实际返回的数据结构


    // 保存用户信息和 token
    userStore.setUser({
      id: response.data.user.id,
      username: response.data.user.username,
      email: response.data.user.email,
      avatar: response.data.user.avatar,
      roles: response.data.user.roles
    })

    userStore.setToken(response.data.token)

    // 如果勾选了记住我，保存到本地存储
    if (rememberMe.value) {
      localStorage.setItem('rememberedUser', loginForm.username)
    } else {
      localStorage.removeItem('rememberedUser')
    }

    // 显示成功消息
    ElMessage.success({
      message: '登录成功！',
      duration: 1500,
      showClose: true
    })

    // 跳转到首页
    setTimeout(() => {
      router.push('/')
    }, 1500)

  } catch (error) {
    console.error('登录失败:', error)
    console.error('登录API调用错误详情:')
    console.error('错误对象:', error)
    console.error('错误消息:', error.message)
    console.error('错误响应:', error.response)
    console.error('错误状态码:', error.response?.status)
    console.error('错误数据:', error.response?.data)
    console.error('错误配置:', error.config)

    // 显示错误消息
    ElMessage.error({
      message: error.response?.data?.message || '登录失败，请检查用户名和密码',
      duration: 3000,
      showClose: true
    })
  } finally {
    loading.value = false
  }
}

// 忘记密码处理
const handleForgetPassword = async () => {
  if (!forgetFormRef.value) return

  try {
    await forgetFormRef.value.validate()

    await forgotPassword({ email: forgetForm.email })

    ElMessage.success('重置链接已发送到您的邮箱，请查收')
    showForgetPassword.value = false
    forgetForm.email = ''

  } catch (error) {
    ElMessage.error(error.response?.data?.message || '发送失败')
  }
}

// 社交登录
const socialLogin = (platform) => {
  ElMessage.info(`即将跳转到${platform}登录`)
  // 实际项目中这里会跳转到第三方登录页面
  // window.location.href = `/api/auth/${platform}`
}

// 页面加载时检查是否有记住的用户名
onMounted(() => {
  const rememberedUser = localStorage.getItem('rememberedUser')
  if (rememberedUser) {
    loginForm.username = rememberedUser
    rememberMe.value = true
  }
})
</script>

<style scoped>
.login-container {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  padding: 20px;
  position: relative;
  overflow: hidden;
}

.background-decor {
  position: absolute;
  width: 100%;
  height: 100%;
  pointer-events: none;
}

.circle {
  position: absolute;
  border-radius: 50%;
  background: rgba(255, 255, 255, 0.1);
}

.circle-1 {
  width: 300px;
  height: 300px;
  top: -150px;
  right: -150px;
}

.circle-2 {
  width: 200px;
  height: 200px;
  bottom: -100px;
  left: -100px;
}

.circle-3 {
  width: 150px;
  height: 150px;
  top: 50%;
  right: 10%;
}

.login-card {
  width: 100%;
  max-width: 420px;
  background: rgba(255, 255, 255, 0.95);
  backdrop-filter: blur(10px);
  border-radius: 20px;
  padding: 40px;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.1);
  border: 1px solid rgba(255, 255, 255, 0.2);
  z-index: 1;
  animation: slideUp 0.5s ease-out;
}

@keyframes slideUp {
  from {
    opacity: 0;
    transform: translateY(30px);
  }

  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.login-header {
  text-align: center;
  margin-bottom: 30px;
}

.logo {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 12px;
  margin-bottom: 15px;
}

.logo-icon {
  width: 40px;
  height: 40px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
}

.logo-icon svg {
  width: 24px;
  height: 24px;
}

.logo h1 {
  margin: 0;
  font-size: 24px;
  font-weight: 600;
  color: #333;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.welcome-text {
  color: #666;
  font-size: 14px;
  margin: 0;
}

.login-form {
  margin-top: 20px;
}

:deep(.el-input__wrapper) {
  border-radius: 10px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  border: 1px solid #e4e7ed;
  transition: all 0.3s ease;
}

:deep(.el-input__wrapper:hover),
:deep(.el-input__wrapper.is-focus) {
  box-shadow: 0 2px 12px rgba(102, 126, 234, 0.2);
  border-color: #667eea;
}

:deep(.el-input__prefix) {
  color: #667eea;
}

.form-options {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}

:deep(.el-checkbox) {
  color: #666;
}

.login-btn {
  width: 100%;
  height: 48px;
  border-radius: 10px;
  font-size: 16px;
  font-weight: 500;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  border: none;
  transition: all 0.3s ease;
}

.login-btn:hover {
  transform: translateY(-2px);
  box-shadow: 0 8px 20px rgba(102, 126, 234, 0.3);
}

.divider {
  position: relative;
  text-align: center;
  margin: 30px 0;
}

.divider::before {
  content: '';
  position: absolute;
  left: 0;
  top: 50%;
  width: 100%;
  height: 1px;
  background: #e4e7ed;
}

.divider span {
  position: relative;
  background: white;
  padding: 0 15px;
  color: #999;
  font-size: 14px;
}

.social-login {
  display: flex;
  justify-content: center;
  gap: 20px;
  margin: 20px 0 30px;
}

.social-btn {
  width: 48px;
  height: 48px;
  border: 1px solid #e4e7ed;
  background: white;
  transition: all 0.3s ease;
}

.social-btn:hover {
  transform: translateY(-3px);
  box-shadow: 0 5px 15px rgba(0, 0, 0, 0.1);
}

.social-icon {
  width: 24px;
  height: 24px;
}

.social-btn:nth-child(1) .social-icon {
  fill: #333;
}

.social-btn:nth-child(2) .social-icon {
  fill: #4285F4;
}

.social-btn:nth-child(3) .social-icon {
  fill: #07C160;
}

.register-link {
  text-align: center;
  color: #666;
  font-size: 14px;
}

.footer {
  margin-top: 30px;
  text-align: center;
  color: rgba(255, 255, 255, 0.8);
  font-size: 12px;
}

.footer-links {
  display: flex;
  justify-content: center;
  align-items: center;
  gap: 10px;
  margin-top: 10px;
}

.footer-links .el-link {
  color: rgba(255, 255, 255, 0.8);
  font-size: 12px;
}

.footer-links .el-link:hover {
  color: white;
}

.separator {
  color: rgba(255, 255, 255, 0.5);
}

/* 响应式设计 */
@media (max-width: 480px) {
  .login-card {
    padding: 30px 20px;
    margin: 0 15px;
  }

  .logo h1 {
    font-size: 20px;
  }

  .social-login {
    gap: 15px;
  }

  .social-btn {
    width: 44px;
    height: 44px;
  }
}
</style>
