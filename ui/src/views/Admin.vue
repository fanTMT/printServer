<template>
  <div class="max-w-[1000px] mx-auto font-['微软雅黑',_sans-serif] border border-gray-200 rounded-md overflow-hidden">
    <!-- 顶部标题栏 -->
    <header class="bg-blue-500 text-white text-center py-4 px-3">
      <h1 class="text-2xl font-normal m-0">本地扫码打印系统</h1>
      <p class="text-sm m-1 mt-2">扫描二维码，轻松提交打印任务</p>
    </header>

    <!-- 主内容区：响应式布局，小屏幕堆叠 -->
    <main class="p-5 md:p-6 gap-8 flex flex-col md:flex-row">
      <!-- 扫描打印区域 -->
      <section class="flex-1">
        <h2 class="text-lg font-normal border-b-2 border-blue-500 pb-1.5 mt-0">扫描打印</h2>

        <!-- 二维码卡片 -->
        <div class="text-center p-5 border border-gray-200 rounded-md mb-4">
          <div class="w-[150px] h-[150px] bg-gray-100 mx-auto mb-3 
                      bg-[linear-gradient(45deg,#000_25%,transparent_25%),
                              linear-gradient(-45deg,#000_25%,transparent_25%),
                              linear-gradient(45deg,transparent_75%,#000_75%),
                              linear-gradient(-45deg,transparent_75%,#000_75%)]
                      bg-[size:20px_20px] bg-[position:0_0,0_10px,10px_-10px,-10px_0px]">
            <img :src="qrcode" alt="扫码" class="w-full h-full object-contain" />
          </div>
          <p class="text-sm mb-2">使用手机扫描二维码提交打印文件</p>
          <p class="text-orange-500 mb-1.5">状态: {{ status }}</p>
          <p class="text-sm text-gray-600">服务器地址: {{ ip }}</p>
        </div>

        <!-- 扫描区域按钮组 -->
        <div class="flex gap-3 flex-wrap mb-5">
          <button class="btn btn-blue">刷新队列</button>
          <button class="btn btn-blue">刷新二维码</button>
          <button class="btn btn-red">重置系统</button>
        </div>

        <!-- 使用说明 -->
        <div class="border border-gray-200 rounded-md p-4 mb-2">
          <h3 class="text-base font-medium mb-2">使用说明</h3>
          <ol class="m-0 p-left-5 text-sm">
            <li>确保您的设备和手机在同一局域网</li>
            <li>使用手机扫描上方二维码</li>
            <li>在手机上选择要打印的文件</li>
            <li>提交后文件将加入打印队列</li>
            <li>点击"打印"按钮完成打印</li>
          </ol>
        </div>
      </section>

      <!-- 打印队列区域 -->
      <section class="flex-1">
        <h2 class="text-lg font-normal border-b-2 border-blue-500 pb-1.5 mt-0">打印队列</h2>

        <!-- 加载状态 -->
        <div v-if="isLoading" class="p-5 border border-gray-200 rounded-md text-center text-gray-600 mb-4">加载打印队列中...
        </div>
        <!-- 错误状态 -->
        <div v-else-if="errorMsg" class="p-5 border border-gray-200 rounded-md text-center text-red-600 mb-4">{{
          errorMsg }}</div>
        <!-- 空状态 -->
        <div v-else-if="printQueue.length === 0"
          class="p-5 border border-gray-200 rounded-md text-center text-gray-600 mb-4">打印队列为空</div>
        <!-- 队列列表 -->
        <div v-if="printQueue.length > 0" class="mb-4">
          <div class="flex items-center justify-between p-3.5 mb-2 bg-white rounded-md shadow-sm border border-gray-100"
            v-for="item in printQueue" :key="item.id">
            <!-- 文件信息（图标+名称） -->
            <div class="flex items-center gap-3 flex-1 max-w-[200px]">
              <div class="w-9 h-9 rounded-md flex items-center justify-center text-white font-semibold"
                :class="getFileTypeClass(item.original_name)">
                {{ getFileExtension(item.original_name) }}
              </div>
              <div class="text-sm text-gray-800 whitespace-nowrap overflow-hidden text-ellipsis max-w-full">
                {{ item.original_name || '未命名文件' }}
              </div>
            </div>

            <!-- 元信息（大小+时间+状态） -->
            <div class="flex flex-col items-end gap-1 text-xs text-gray-500 mr-5">
              <div>{{ item.file_size }}</div>
              <div>{{ formatDate(item.created_at) }}</div>
              <div class="text-gray-400">{{ getStatusText(item.status) }}</div>
            </div>

            <!-- 操作按钮 -->
            <div>
              <button @click="viewFile(item)" :disabled="!item.file_path"
                class="px-3.5 py-1.5 text-xs bg-gray-200 text-gray-700 rounded-md cursor-default border-0">
                {{ getStatusText(item.status) }}
              </button>
            </div>
          </div>
        </div>

        <!-- 打印设置 -->
        <div class="border border-gray-200 rounded-md p-4 mb-4">
          <h3 class="text-base font-medium mb-3">打印设置</h3>
          <div class="space-y-2.5">
            <div class="flex items-center gap-2.5">
              <label for="printAuto" class="w-20 text-sm">自动打印:</label>
              <input type="checkbox" v-model="printAuto" id="printAuto" class="h-4 w-4" />
            </div>
            <div class="flex items-center gap-2.5">
              <label class="w-20 text-sm">打印机:</label>
              <select v-model="printer" class="flex-1 px-2 py-1.5 text-sm border border-gray-300 rounded-md">
                <option value="" v-if="printerList.length === 0">{{ errorMsg }}</option>
                <option v-else v-for="item in printerList" :key="item['name']" :value="item['name']">
                  {{ item['name'] }}
                </option>
              </select>
            </div>
            <div class="flex items-center gap-2.5">
              <label class="w-20 text-sm">纸张大小:</label>
              <select v-model="selectedSize" class="flex-1 px-2 py-1.5 text-sm border border-gray-300 rounded-md">
                <option key="A4" value="A4">A4</option>
                <option key="A5" value="A5">A5</option>
                <option key="Letter" value="Letter">Letter(信纸)</option>
                <option key="Legal" value="Legal">Legal(法律用纸)</option>
                <option key="A3" value="A3">A3(大尺寸)</option>
              </select>
            </div>
            <div class="flex items-center gap-2.5">
              <label class="w-20 text-sm">方向:</label>
              <select v-model="printDirection" class="flex-1 px-2 py-1.5 text-sm border border-gray-300 rounded-md">
                <option value=3>纵向</option>
                <option value=4>横向</option>
              </select>
            </div>
          </div>
          <div class="flex gap-3 mt-4">
            <button class="btn btn-green" @click="upSet">更新设置</button>
            <button class="btn btn-blue" @click="printAll">打印全部</button>
            <button class="btn btn-red" @click="delAll">清空队列</button>
          </div>
        </div>

        <!-- 打印说明 -->
        <div class="border border-gray-200 rounded-md p-4 text-sm leading-relaxed">
          <h3 class="text-base font-medium mb-2">打印说明</h3>
          <p class="mb-1">系统默认用A4纸张黑白打印，图片文件会自动转换为PDF格式。</p>
          <p class="mb-1">支持的文件格式: PDF, JPG, PNG, TXT</p>
          <p>最大文件大小: 10MB</p>
        </div>
      </section>
    </main>
  </div>
</template>

<script setup>
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
const selectedSize = ref('A4');
const printer = ref('');

const delAll = () => {
  console.log("自动打印:", printAuto.value);
  console.log("打印机:", printer.value);
  console.log("页面大小:", selectedSize.value);
  console.log("方向:", printDirection.value);
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
      console.log("更新完成!");
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

const printAll = () => {
  console.log("自动打印:", printAuto.value);
  console.log("打印机:", printer.value);
  console.log("页面大小:", selectedSize.value);
  console.log("方向:", printDirection.value);
};

// 查看文件（原代码缺失，补充空实现避免报错）
const viewFile = (item) => {
  console.log('查看文件:', item);
};

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
      printDirection.value = data['orientation']
      selectedSize.value = data['page_size'];
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
  @apply px-4 py-2 rounded-md border-0 text-sm cursor-pointer;
}

.btn-blue {
  @apply bg-blue-500 text-white;
}

.btn-red {
  @apply bg-red-500 text-white;
}

.btn-green {
  @apply bg-sky-300 text-black;
}
</style>