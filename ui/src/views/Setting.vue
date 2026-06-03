<template>
  <div class="max-w-4xl mx-auto">
    <!-- 页面标题 -->
    <div class="mb-8">
      <h2 class="text-3xl font-bold text-gray-900 mb-2">⚙️ 系统设置</h2>
      <p class="text-gray-600">配置打印系统的各项参数</p>
    </div>

    <!-- 设置卡片网格 -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
      <!-- 打印机设置卡片 -->
      <div class="bg-white rounded-lg shadow-md p-6">
        <h3 class="text-lg font-semibold text-gray-900 mb-4">🖨️ 打印机设置</h3>
        <div class="space-y-4">
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">
              默认打印机
            </label>
            <select v-model="settings.printer" class="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-blue-500">
              <option value="">请选择打印机</option>
              <option v-for="printer in printerList" :key="printer.name" :value="printer.name">
                {{ printer.name }} {{ printer.status ? '(在线)' : '(离线)' }}
              </option>
            </select>
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">
              <input type="checkbox" v-model="settings.enabled_auto_print" class="rounded mr-2">
              启用自动打印
            </label>
            <p class="text-xs text-gray-500 ml-6">文件上传后自动开始打印</p>
          </div>
        </div>
      </div>

      <!-- 纸张设置卡片 -->
      <div class="bg-white rounded-lg shadow-md p-6">
        <h3 class="text-lg font-semibold text-gray-900 mb-4">📄 纸张设置</h3>
        <div class="space-y-4">
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">
              纸张大小
            </label>
            <select v-model="settings.page_size" class="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-blue-500">
              <option value="A4">A4</option>
              <option value="A3">A3</option>
              <option value="Letter">Letter</option>
              <option value="Legal">Legal</option>
            </select>
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">
              默认方向
            </label>
            <div class="flex gap-4">
              <label class="flex items-center text-sm">
                <input type="radio" v-model.number="settings.orientation" :value="3" class="mr-2"> 纵向
              </label>
              <label class="flex items-center text-sm">
                <input type="radio" v-model.number="settings.orientation" :value="4" class="mr-2"> 横向
              </label>
            </div>
          </div>
        </div>
      </div>

      <!-- 文件设置卡片 -->
      <div class="bg-white rounded-lg shadow-md p-6">
        <h3 class="text-lg font-semibold text-gray-900 mb-4">📁 文件设置</h3>
        <div class="space-y-4">
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">
              页面范围
            </label>
            <input type="text" v-model="settings.page_ranges" placeholder="例如: 1-5,10-20" class="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-blue-500">
            <p class="text-xs text-gray-500 mt-1">留空表示打印所有页面</p>
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">
              支持的文件类型
            </label>
            <p class="text-sm text-gray-600">PDF, DOC, DOCX, JPG, PNG, TXT</p>
          </div>
        </div>
      </div>

      <!-- 系统信息卡片 -->
      <div class="bg-white rounded-lg shadow-md p-6">
        <h3 class="text-lg font-semibold text-gray-900 mb-4">ℹ️ 系统信息</h3>
        <div class="space-y-3 text-sm">
          <div class="flex justify-between">
            <span class="text-gray-600">系统版本:</span>
            <span class="font-medium">v1.0.0</span>
          </div>
          <div class="flex justify-between">
            <span class="text-gray-600">API 版本:</span>
            <span class="font-medium">v1</span>
          </div>
          <div class="flex justify-between">
            <span class="text-gray-600">打印机数量:</span>
            <span class="font-medium">{{ printerList.length }}</span>
          </div>
        </div>
      </div>
    </div>

    <!-- 操作按钮 -->
    <div class="mt-8 flex gap-3 justify-end">
      <button
        @click="resetSettings"
        class="px-6 py-2 border border-gray-300 rounded-lg text-gray-700 font-medium hover:bg-gray-50 transition-colors"
      >
        重置
      </button>
      <button
        @click="saveSettings"
        class="px-6 py-2 bg-blue-500 text-white rounded-lg font-medium hover:bg-blue-600 transition-colors"
      >
        保存设置
      </button>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import { ElMessage } from 'element-plus';
import { getprinter,get_setting,set_setting } from '@/api/api';

// 打印机列表
const printerList = ref([]);

// 设置数据
const settings = ref({
  printer: '',
  enabled_auto_print: false,
  page_size: 'A4',
  orientation: 3,
  page_ranges: null
});

// 原始设置（用于重置）
const originalSettings = ref(null);

// 加载状态
const loading = ref(false);

// 获取打印机列表
const fetchPrinterList = async () => {
  try {
    const res = await getprinter();
    if (res.success && res.code === 200) {
      printerList.value = res.data || [];
    } else {
      ElMessage.warning('获取打印机列表失败');
    }
  } catch (err) {
    console.error('获取打印机列表出错:', err);
    ElMessage.error('无法获取打印机列表');
  }
};

// 获取设置
const fetchSettings = async () => {
  loading.value = true;
  try {
    const res = await get_setting();
    if (res.success && res.code === 200 && res.data) {
      settings.value = {
        printer: res.data.printer || '',
        enabled_auto_print: res.data.enabled_auto_print || false,
        page_size: res.data.page_size || 'A4',
        orientation: res.data.orientation || 3,
        page_ranges: res.data.page_ranges || null
      };
      originalSettings.value = JSON.parse(JSON.stringify(settings.value));
      ElMessage.success('设置已加载');
    }
  } catch (err) {
    console.error('获取设置出错:', err);
    ElMessage.error('无法获取设置');
    originalSettings.value = JSON.parse(JSON.stringify(settings.value));
  } finally {
    loading.value = false;
  }
};

// 保存设置
const saveSettings = async () => {
  loading.value = true;
  try {
    const res = await set_setting({
      printer: settings.value.printer,
      enabled_auto_print: settings.value.enabled_auto_print,
      page_size: settings.value.page_size,
      orientation: settings.value.orientation,
      page_ranges: settings.value.page_ranges
    });
    
    if (res.success && res.code === 200) {
      ElMessage.success('设置已保存');
      originalSettings.value = JSON.parse(JSON.stringify(settings.value));
    } else {
      ElMessage.warning(res.message || '保存设置失败');
    }
  } catch (err) {
    console.error('保存设置出错:', err);
    ElMessage.error('无法保存设置');
  } finally {
    loading.value = false;
  }
};

// 重置设置
const resetSettings = () => {
  if (originalSettings.value) {
    settings.value = JSON.parse(JSON.stringify(originalSettings.value));
    ElMessage.info('设置已重置');
  }
};

// 初始化
onMounted(() => {
  fetchPrinterList();
  fetchSettings();
});
</script>

<style scoped>
/* 可以添加更多样式 */
</style>
