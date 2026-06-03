<template>
  <div class="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100">
    <!-- 主内容区域 -->
    <main class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
      <!-- 内容卡片 -->
      <div class="bg-white rounded-2xl shadow-lg p-8 md:p-12">
        <div class="grid grid-cols-1 gap-16 md:grid-cols-2">
          <div class="text-center">
            <div class="mb-8">
              <div class="text-6xl mb-4">🏠</div>
              <h2 class="text-2xl font-bold text-gray-900 mb-2">本地网络上传</h2>
            </div>

            <!-- 本地二维码 -->
            <div class="flex justify-center mb-8">
              <div class="bg-gray-100 p-8 rounded-xl border-2 border-gray-200">
                <div v-if="loading" class="w-80 h-80 flex items-center justify-center">
                  <div class="text-center">
                    <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600 mx-auto mb-2"></div>
                    <p class="text-gray-600">生成中...</p>
                  </div>
                </div>
                <img v-else-if="localQrcodeImage" :src="localQrcodeImage" alt="本地网络二维码" class="w-80 h-80" />
              </div>
            </div>

            <div class="grid grid-cols-1 gap-4 text-sm">
              <div class="p-4 bg-gray-50 rounded-lg">
                <div class="text-2xl mb-2">📲</div>
                <p class="font-medium text-gray-900">1. 扫描二维码</p>
                <p class="text-gray-600 mt-1">用手机扫描左侧二维码</p>
              </div>
              <div class="p-4 bg-gray-50 rounded-lg">
                <div class="text-2xl mb-2">📤</div>
                <p class="font-medium text-gray-900">2. 选择文件</p>
                <p class="text-gray-600 mt-1">在手机上选择要上传的文件</p>
              </div>
              <div class="p-4 bg-gray-50 rounded-lg">
                <div class="text-2xl mb-2">✅</div>
                <p class="font-medium text-gray-900">3. 提交上传</p>
                <p class="text-gray-600 mt-1">文件将加入打印队列</p>
              </div>
            </div>
          </div>

          <div class="text-center">
            <div class="mb-8">
              <div class="text-6xl mb-4">🌐</div>
              <h2 class="text-2xl font-bold text-gray-900 mb-2">外网上传</h2>
              <p class="text-gray-600">在任何地方扫描二维码上传文件</p>
            </div>

            <!-- 外网二维码 -->
            <div class="flex justify-center mb-8">
              <div class="bg-gray-100 p-8 rounded-xl border-2 border-gray-200">
                <div v-if="loading" class="w-80 h-80 flex items-center justify-center">
                  <div class="text-center">
                    <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-indigo-600 mx-auto mb-2"></div>
                    <p class="text-gray-600">生成中...</p>
                  </div>
                </div>
                <img v-else-if="wanQrcodeImage" :src="wanQrcodeImage" alt="外网二维码" class="w-80 h-80" />
              </div>
            </div>

            <div class="grid grid-cols-1 gap-4 text-sm">
              <div class="p-4 bg-gray-50 rounded-lg">
                <div class="text-2xl mb-2">🌍</div>
                <p class="font-medium text-gray-900">1. 全球可用</p>
                <p class="text-gray-600 mt-1">任何网络环境下都可访问</p>
              </div>
              <div class="p-4 bg-gray-50 rounded-lg">
                <div class="text-2xl mb-2">🔒</div>
                <p class="font-medium text-gray-900">2. 安全上传</p>
                <p class="text-gray-600 mt-1">采用加密传输保护数据</p>
              </div>
              <div class="p-4 bg-gray-50 rounded-lg">
                <div class="text-2xl mb-2">⚡</div>
                <p class="font-medium text-gray-900">3. 快速处理</p>
                <p class="text-gray-600 mt-1">文件快速加入打印队列</p>
              </div>
            </div>

            <div class="bg-yellow-50 border border-yellow-200 rounded-lg p-4 mt-6">
              <p class="text-xs md:text-sm text-gray-700">
                ⚠️ <strong>外网模式需要配置服务器域名和 SSL 证书</strong>
              </p>
            </div>
          </div>
        </div>

        <!-- 返回按钮 -->
        <div class="mt-12 flex justify-center">
          <router-link
            to="/home/upload"
            class="inline-flex items-center px-6 py-3 bg-gray-200 text-gray-800 rounded-lg font-medium hover:bg-gray-300 transition-colors"
          >
            ← 返回文件上传
          </router-link>
        </div>
      </div>
    </main>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import { ElMessage } from 'element-plus';

const localQrcodeImage = ref('');
const wanQrcodeImage = ref('');
const loading = ref(false);

// 从后端获取二维码图片
const fetchQRCode = async () => {
  loading.value = true;
  try {
    // 直接使用 API 端点作为图片源，分别显示本地和外网二维码
    localQrcodeImage.value = '/api/auth/qrcode?mode=local';
    wanQrcodeImage.value = '/api/auth/qrcode?mode=wan';
    ElMessage.success('二维码已加载');
  } catch (err) {
    console.error('获取二维码失败:', err);
    ElMessage.error('无法获取二维码，请稍后重试');
  } finally {
    loading.value = false;
  }
};

// 初始化
onMounted(async () => {
  await fetchQRCode();
});
</script>

<style scoped>
/* 可以添加更多样式 */
</style>
