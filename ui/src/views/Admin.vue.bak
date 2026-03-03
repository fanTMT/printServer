<template>
  <div class="max-w-[1000px] mx-auto font-['微软雅黑',_sans-serif]">
    <main class="p-5 md:p-6 gap-8 flex flex-col md:flex-row">
      <!-- 左侧：扫描打印区域 -->
      <section class="flex-1">
        <h2 class="text-lg font-normal border-b-2 border-blue-500 pb-1.5 mt-0">扫描打印</h2>

        <!-- 使用 ElCard 包装二维码区域 -->
        <el-card class="mb-4 !border-gray-200" shadow="never">
          <div class="text-center">
            <!-- 二维码展示区域保持不变 -->
            <div class="w-[150px] h-[150px] bg-gray-100 mx-auto mb-3 ...">
              <img :src="qrcode" alt="扫码" class="w-full h-full object-contain" />
            </div>
            <p class="text-sm mb-2">使用手机扫描二维码提交打印文件</p>
            <p class="text-orange-500 mb-1.5">状态: {{ status }}</p>
            <p class="text-sm text-gray-600">服务器地址: {{ ip }}</p>
          </div>
        </el-card>
        <!-- 常规操作 -->
        <el-space :size="12" :wrap="false" class="mb-3">
          <el-button type="primary" @click="refreshQueue">刷新队列</el-button>
          <el-button type="primary" @click="refreshQRCode">刷新二维码</el-button>
        </el-space>
        <!-- 配置操作 -->
        <el-space :size="12" :wrap="false" class="mb-5">
          <el-button type="danger" @click="confirmReset">重置系统</el-button>
          <el-button type="danger" @click="confirmClearAll">清空队列</el-button>
          <el-button type="success" @click="upSet">更新设置</el-button>
        </el-space>
        <el-card class="!border-gray-200" shadow="never">
          <template #header>
            <h3 class="text-base font-medium m-0">使用说明</h3>
          </template>
          <ol class="m-0 p-left-5 text-sm space-y-1">
            <li>确保您的设备和手机在同一局域网</li>
            <li>使用手机扫描上方二维码</li>
            <li>在手机上选择要打印的文件</li>
            <li>提交后文件将加入打印队列</li>
            <li>点击"打印"按钮完成打印</li>
          </ol>
          <h3 class="text-base font-medium m-0">打印说明</h3>
          <p class="text-sm leading-relaxed mb-1">系统默认用A4纸张黑白打印，图片文件会自动转换为PDF格式。</p>
          <p class="text-sm leading-relaxed mb-1">支持的文件格式: PDF, JPG, PNG, TXT</p>
          <p class="text-sm leading-relaxed">最大文件大小: 10MB</p>
        </el-card>
      </section>

      <!-- 右侧：打印队列区域 -->
      <section class="flex-1">
        <h2 class="text-lg font-normal border-b-2 border-blue-500 pb-1.5 mt-0">打印队列</h2>

        <!-- 状态提示：使用 ElEmpty 和 ElAlert -->
        <el-empty v-if="printQueue.length === 0 && !isLoading && !errorMsg" description="打印队列为空" />
        <el-alert v-else-if="errorMsg" :title="errorMsg" type="error" show-icon class="mb-4" />
        <el-skeleton v-else-if="isLoading" :rows="3" animated class="mb-4" />

        <!-- 队列列表：使用 ElCard 和 ElScrollbar -->
        <el-card v-if="printQueue.length > 0" class="mb-4 !border-gray-200" shadow="never">
          <template #header>
            <div class="flex items-center justify-between">
              <span class="text-sm font-semibold text-gray-700">打印队列 ({{ printQueue.length }})</span>
              <span class="text-xs text-gray-500">滚动查看更多</span>
            </div>
          </template>
          <!-- 使用 ElScrollbar 替代原生滚动 -->
          <el-scrollbar max-height="400px">
            <div class="space-y-2">
              <!-- 每个队列项使用 ElCard -->
              <el-card v-for="item in printQueue" :key="item.id" shadow="never"
                class="!border-gray-100 hover:shadow-sm">
                <div class="flex items-center justify-between">
                  <!-- 文件信息（图标+名称）保持不变 -->
                  <div class="flex items-center gap-3 flex-1 max-w-[200px]">
                    <div
                      class="w-9 h-9 rounded-md flex items-center justify-center text-white font-semibold flex-shrink-0"
                      :class="getFileTypeClass(item.original_name)">
                      {{ getFileExtension(item.original_name) }}
                    </div>
                    <div class="text-sm text-gray-800 whitespace-nowrap overflow-hidden text-ellipsis">
                      {{ item.original_name || '未命名文件' }}
                    </div>
                  </div>

                  <!-- 元信息 -->
                  <div class="flex flex-col items-end gap-1 text-xs text-gray-500 mr-5">
                    <div>{{ item.file_size }}</div>
                    <div>{{ formatDate(item.created_at) }}</div>
                    <div :class="getStatusColor(item.status)">
                      {{ getStatusText(item.status) }}
                    </div>
                  </div>

                  <!-- 操作按钮：使用 ElButton -->
                  <el-button @click="viewFile(item)" :disabled="!item.file_path" size="small" plain>
                    {{ getStatusText(item.status) }}
                  </el-button>
                </div>
              </el-card>
            </div>
          </el-scrollbar>
        </el-card>

        <!-- 打印设置：使用 ElCard 和 ElForm -->
        <el-card class="mb-4 !border-gray-200" shadow="never">
          <template #header>
            <h3 class="text-base font-medium m-0">打印设置</h3>
          </template>
          <!-- 使用 ElForm 布局表单 -->
          <el-form :model="form" label-width="80px" class="demo-form-inline">
            <el-form-item label="自动打印:">
              <!-- 使用 ElSwitch 替代 checkbox -->
              <el-switch v-model="printAuto" />
            </el-form-item>
            <el-form-item label="打印机:">
              <!-- 使用 ElSelect 替代原生 select -->
              <el-select v-model="printer" placeholder="请选择打印机" class="w-full">
                <el-option v-for="item in printerList" :key="item.name" :label="item.name" :value="item.name" />
              </el-select>
            </el-form-item>
          </el-form>
        </el-card>
      </section>
    </main>
  </div>
</template>

<script setup>
// 引入 Element Plus 的消息和对话框组件
import { ElMessage, ElMessageBox } from 'element-plus';
import { ref, onMounted } from 'vue';
import { get_all, get_setting, set_setting, getprinter } from '@/api/api';
import { formatDate } from '@/utils/formatDate';

// 定义定时任务标识
let timer = null

const ip = ref('192.168.31.2:80');
const status = ref('等待文件上传');
const qrcode = ref('/api/auth/qrcode');
const printAuto = ref(true);
const printerList = ref([]);
const printDirection = ref(3);
const printQueue = ref([]);
const isLoading = ref(false);
const errorMsg = ref('');
const printer = ref('');


const confirmReset = () => {
  ElMessageBox.confirm(
    '确定要重置系统吗？此操作将清除所有打印任务和设置。',
    '警告',
    {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning',
    }
  ).then(() => {
    // 用户点击确定后执行实际的重置逻辑
    console.log('执行系统重置...');
    ElMessage.success('系统重置成功');
  }).catch(() => {
    // 用户点击取消
    ElMessage.info('已取消重置');
  });
};

const confirmClearAll = () => {
  console.log("自动打印:", printAuto.value);
  console.log("打印机:", printer.value);
  ElMessageBox.confirm(
    '确定要清空整个打印队列吗？此操作不可撤销。',
    '警告',
    {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning',
    }
  ).then(() => {
    // 执行实际的清空逻辑
    console.log('执行清空队列...');
    ElMessage.success('队列已清空');
  }).catch(() => {
    ElMessage.info('已取消清空');
  });
};

const upSet = async () => {
  const data = {
    "isauto": printAuto.value,
    "orientation": Number(printDirection.value),
    "page_size": selectedSize.value,
    "printer": printer.value,
  };

  console.log("更新设置:", data);

  isLoading.value = true;
  errorMsg.value = '';
  try {
    const res = await set_setting(data);
    console.log("【Admin.vue】更新设置:", res.data, res);
    if (res.success && res.code == 200) {
      ElMessage.success('打印设置更新成功！');
    } else {
      ElMessage.error('更新失败：' + (res.message || '服务端异常'));
    }
  } catch (err) {
    console.error('请求打印队列出错：', err);
    ElMessage.error('网络异常，请检查后端服务是否正常');
  } finally {
    isLoading.value = false;
  }
};

// 查看文件（原代码缺失，补充空实现避免报错）
const viewFile = (item) => {
  console.log('查看文件:', item);
};

// 根据状态返回对应的颜色类名
const getStatusColor = (status) => {
  const statusColors = {
    'pending': 'text-yellow-500',
    'processing': 'text-blue-500',
    'completed': 'text-green-500',
    'failed': 'text-red-500',
    'cancelled': 'text-gray-500'
  };
  // 如果没有匹配的状态，返回默认颜色
  return statusColors[status] || 'text-gray-400';
}

// 获取打印机列表
const getPrinters = async () => {
  isLoading.value = true;
  errorMsg.value = '';
  try {
    const res = await getprinter();
    const { code, success, data } = res;
    // console.log("打印机:", code, success, data);
    if (success && code === 200) {
      printerList.value = data;
      console.log('打印机列表获取成功：', data);
      if (printerList.value.length > 0) {
        printer.value = printerList.value[0]['name'];
      }
    }
  } catch (err) {
    errorMsg.value = '网络异常，无法连接打印机服务';
    console.log('请求异常：', err);
  } finally {
    isLoading.value = false;
  }
};

// 获取设置
const getSetting = async () => {
  isLoading.value = true;
  errorMsg.value = '';
  try {
    const res = await get_setting();
    const { code, success, data } = res;
    console.log("data:", code, success, data);
    if (success && code === 200) {
      printer.value = data['printer'];
      printAuto.value = data['enabled_auto_print'];
    }
  } catch (err) {
    errorMsg.value = '网络异常，获取配置文件失败！';
    console.log('请求异常：', err);
  } finally {
    isLoading.value = false;
  }
};

// 请求打印队列数据
const fetchPrintQueue = async () => {
  isLoading.value = true;
  errorMsg.value = '';
  try {
    const res = await get_all();
    // console.log("打印队列:", res.data, res.data.code);
    if (res.success && res.code == 200) {
      printQueue.value = res.data.sort((a, b) =>
        new Date(b.created_at) - new Date(a.created_at)
      );
      // console.log("获取打印队列", printQueue.value[0]);
    } else {
      errorMsg.value = '获取打印队列失败：' + (res.message || '服务端异常');
    }
  } catch (err) {
    console.error('请求打印队列出错：', err);
    errorMsg.value = '网络异常，请检查后端服务是否正常';
  } finally {
    isLoading.value = false;
  }
};

onMounted(() => {
  ip.value = window.location.host;
  getPrinters();
  getSetting();
  fetchPrintQueue();
  // timer = setTimeout(() => {
  //   getSet();
  // }, 15000);
});

// 获取文件扩展名
const getFileExtension = (fileName) => {
  if (!fileName) return '文件';
  const ext = fileName.split('.').pop().toUpperCase();
  return ext.length > 4 ? '文件' : ext;
};

// 文件类型样式（映射Tailwind类）
const getFileTypeClass = (fileName) => {
  const ext = getFileExtension(fileName).toLowerCase();
  if (ext === 'jpg') return 'bg-amber-500'; // 原黄色
  if (ext === 'pdf') return 'bg-red-500';   // 原红色
  return 'bg-gray-500';                    // 其他灰色
};

// 状态文本
const getStatusText = (status) => {
  return status || '未知状态';
};


</script>

<!-- 全局按钮样式（可放在入口文件或组件内） -->
<style>
.btn {
  padding-left: 1rem;
  padding-right: 1rem;
  padding-top: 0.5rem;
  padding-bottom: 0.5rem;
  border-radius: 0.375rem;
  border: 0;
  font-size: 0.875rem;
  cursor: pointer;
  display: inline-block;
  line-height: 1;
}

.btn-blue {
  background-color: #3b82f6;
  color: #ffffff;
}

.btn-red {
  background-color: #ef4444;
  color: #ffffff;
}

.btn-green {
  background-color: #7dd3fc;
  color: #000000;
}
</style>