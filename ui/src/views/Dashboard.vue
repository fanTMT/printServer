<template>
  <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 font-['Microsoft_YaHei',_sans-serif]">
    <!-- 顶部导航 -->
    <header class="flex justify-between items-center py-4 sm:py-6 border-b border-gray-200 mb-6 sm:mb-8">
      <h1 class="text-xl sm:text-2xl font-medium">
        打印助手
        <span class="text-sm sm:text-base text-gray-600 font-normal ml-2">打印机: {{ printer }}</span>
      </h1>
      <nav>
        <ul class="flex space-x-6">
          <li><a href="#" class="active text-primary font-medium hover:text-primary/80 transition-colors">首页</a>
          </li>
          <router-link to="/history" exact-active-class="active"
            class="text-neutral-600 hover:text-primary transition-colors">历史记录</router-link>
          <li><a href="#" class="text-neutral-600 hover:text-primary transition-colors">帮助中心</a></li>
        </ul>
      </nav>
    </header>

    <!-- 主内容区 -->
    <main class="flex flex-col lg:flex-row gap-6 sm:gap-8 lg:gap-10">
      <!-- 上传&设置区域 -->
      <section class="w-full lg:w-1/3 min-w-[300px]">
        <h2 class="text-lg sm:text-xl font-medium mb-4">上传打印文件</h2>

        <!-- 文件上传区域 -->
        <div
          class="border-2 border-dashed border-gray-300 rounded-lg p-6 sm:p-8 text-center cursor-pointer mb-5 transition-all hover:border-blue-500 hover:bg-gray-50"
          :class="{ 'border-green-500 bg-green-50 cursor-copy': isDragover }" @click="openFileSelector"
          @dragover.prevent @dragenter.prevent="isDragover = true" @dragleave.prevent="isDragover = false"
          @drop.prevent="handleFileDrop; isDragover = false">
          <div class="text-5xl sm:text-6xl mb-3 text-blue-500">📄</div>
          <p class="mb-1 text-gray-700">点击选择文件或拖拽文件到这里</p>
          <p class="mb-1 text-gray-600 text-sm">支持格式: PDF,DOC,DOCX,JPG,PNG,TXT</p>
          <p class="text-gray-600 text-sm">最大大小: 10MB</p>
          <!-- 隐藏的文件选择器 -->
          <input type="file" ref="fileInput" class="hidden" accept=".pdf,.doc,.docx,.jpg,.png,.txt" multiple
            @change="handleFileChange">
        </div>

        <!-- 上传文件列表 -->
        <div class="border border-gray-200 rounded-lg p-4 sm:p-5 bg-white mb-5">
          <h3 class="text-base sm:text-lg font-medium mb-3 pb-2 border-b border-gray-100 text-gray-800">已上传文件</h3>

          <!-- 文件列表容器 -->
          <div class="max-h-[220px] overflow-y-auto mb-3" v-if="uploadedFiles?.length > 0">
            <div
              class="flex items-center p-3 border border-gray-100 rounded-md mb-2 hover:bg-gray-50 transition-colors cursor-pointer"
              :class="{ 'bg-blue-50 border-blue-500': activeFileIndex === index }"
              v-for="(file, index) in uploadedFiles" :key="index" @click="switchActiveFile(index)">
              <!-- 文件类型图标 -->
              <div class="text-xl w-6 text-center mr-3">
                <span v-if="file.type.includes('pdf')">📄</span>
                <span v-else-if="file.type.includes('image')">🖼️</span>
                <span v-else-if="file.type.includes('text')">📝</span>
                <span v-else>📁</span>
              </div>

              <!-- 文件名 -->
              <div class="flex-1 overflow-hidden">
                <span class="block text-sm text-gray-800 truncate">{{ file.name }}</span>
                <small class="block text-xs text-gray-600 mt-1">{{ formatFileSize(file.size) }}</small>
              </div>

              <!-- 操作按钮 -->
              <div class="flex gap-2 ml-2">
                <button class="px-2 py-1 text-xs bg-blue-500 text-white rounded hover:opacity-90 transition-opacity"
                  @click.stop="previewFile(index)">
                  预览
                </button>
                <button class="px-2 py-1 text-xs bg-red-500 text-white rounded hover:opacity-90 transition-opacity"
                  @click.stop="removeFile(index)">
                  删除
                </button>
                <button class="px-2 py-1 text-xs bg-purple-600 text-white rounded hover:opacity-90 transition-opacity"
                  @click="uploadFile(index)">
                  开始上传
                </button>
                <button class="px-2 py-1 bg-green-600 text-white rounded-md text-sm hover:opacity-90 transition-opacity"
                  @click="printFile" :disabled="!selectedFile"
                  :class="{ 'opacity-60 cursor-not-allowed': !selectedFile }">
                  打印文件
                </button>
              </div>
            </div>
          </div>

          <!-- 空文件状态 -->
          <div class="text-center text-gray-600 p-4 text-sm" v-else>
            <p>暂无上传文件</p>
          </div>

          <!-- 列表操作按钮 -->
          <div class="flex justify-end gap-3" v-if="uploadedFiles?.length > 0">
            <button class="px-3 py-1.5 text-xs bg-blue-500 text-white rounded hover:opacity-90 transition-opacity"
              @click="clearAllFiles">
              清空列表
            </button>
            <button class="px-3 py-1.5 text-xs bg-green-500 text-white rounded hover:opacity-90 transition-opacity"
              @click="printAllFiles">
              打印全部
            </button>
          </div>
        </div>

        <!-- 打印设置 -->
        <div class="border border-gray-200 rounded-lg p-4 sm:p-5 bg-white">
          <h3 class="text-base sm:text-lg font-medium mb-4 text-gray-800">打印设置</h3>

          <!-- 份数 -->
          <div class="mb-4">
            <label class="block mb-2 font-medium text-sm text-gray-700">份数</label>
            <div class="flex items-center gap-2">
              <button class="w-7 h-7 border border-gray-200 bg-white rounded text-base cursor-pointer hover:bg-gray-50"
                @click="count > 1 && (count--)">-</button>
              <span>{{ count }}</span>
              <button class="w-7 h-7 border border-gray-200 bg-white rounded text-base cursor-pointer hover:bg-gray-50"
                @click="count++">+</button>
            </div>
          </div>

          <!-- 布局 -->
          <div class="mb-4">
            <label class="block mb-2 font-medium text-sm text-gray-700">布局</label>
            <div class="flex gap-4 mb-2">
              <label class="flex items-center gap-1 text-sm">
                <input type="radio" name="layout" value="portrait" v-model="layout" checked class="cursor-pointer"> 纵向
              </label>
              <label class="flex items-center gap-1 text-sm">
                <input type="radio" name="layout" value="landscape" v-model="layout" class="cursor-pointer"> 横向
              </label>
            </div>
          </div>

          <!-- 页面 -->
          <div class="mb-4">
            <label class="block mb-2 font-medium text-sm text-gray-700">页面</label>
            <div class="flex gap-4 mb-2">
              <label class="flex items-center gap-1 text-sm">
                <input type="radio" name="page" value="all" v-model="pageMode" checked class="cursor-pointer"> 全部
              </label>
              <label class="flex items-center gap-1 text-sm">
                <input type="radio" name="page" value="custom" v-model="pageMode" class="cursor-pointer"> 自定义
              </label>
            </div>
            <input type="text" placeholder="例如:1-5,8,11-13" v-if="pageMode === 'custom'" v-model="customPages"
              class="w-full px-2 py-1.5 border border-gray-200 rounded-md text-sm">
          </div>

          <!-- 颜色 -->
          <div class="mb-4">
            <label class="block mb-2 font-medium text-sm text-gray-700">颜色</label>
            <div class="flex gap-4 mb-2">
              <label class="flex items-center gap-1 text-sm">
                <input type="radio" name="color" value="color" v-model="color" checked class="cursor-pointer"> 彩色
              </label>
              <label class="flex items-center gap-1 text-sm">
                <input type="radio" name="color" value="black" v-model="color" class="cursor-pointer"> 黑白
              </label>
            </div>
          </div>

          <!-- 纸张大小 -->
          <div class="mb-5">
            <label class="block mb-2 font-medium text-sm text-gray-700">纸张大小</label>
            <select v-model="paperSize" class="w-full px-2 py-1.5 border border-gray-200 rounded-md text-sm">
              <option value="A4">A4(默认)</option>
              <option value="A3">A3</option>
              <option value="B5">B5</option>
              <option value="Letter">Letter</option>
            </select>
          </div>
        </div>
      </section>

      <!-- 文件预览区域 -->
      <section class="w-full lg:w-2/3">
        <h2 class="text-lg sm:text-xl font-medium mb-4 flex items-center gap-2">
          文件预览
          <span v-if="selectedFile" class="text-sm font-normal text-gray-600">({{ selectedFile.name }})</span>
        </h2>

        <!-- 通用预览控制 -->
        <div class="flex items-center gap-3 mb-4 p-3 bg-gray-50 rounded-md border border-gray-100" v-if="selectedFile">
          <!-- PDF 专属控制 -->
          <div class="flex items-center gap-3 w-full" v-if="fileType === 'pdf'">
            <button class="px-3 py-1.5 bg-blue-500 text-white rounded text-sm hover:opacity-90 transition-opacity"
              @click="prevPage" :disabled="currentPage <= 1"
              :class="{ 'opacity-50 cursor-not-allowed': currentPage <= 1 }">
              上一页
            </button>
            <span class="text-sm">第 {{ currentPage }} / {{ totalPages }} 页</span>
            <button class="px-3 py-1.5 bg-blue-500 text-white rounded text-sm hover:opacity-90 transition-opacity"
              @click="nextPage" :disabled="currentPage >= totalPages"
              :class="{ 'opacity-50 cursor-not-allowed': currentPage >= totalPages }">
              下一页
            </button>
            <select v-model="pdfScale" @change="renderPdfPage()"
              class="px-2 py-1.5 border border-gray-200 rounded text-sm">
              <option value="0.75">75%</option>
              <option value="1">100%</option>
              <option value="1.25">125%</option>
              <option value="1.5">150%</option>
              <option value="2">200%</option>
              <option value="auto">自适应宽度</option>
            </select>
            <button class="px-3 py-1.5 bg-blue-500 text-white rounded text-sm hover:opacity-90 transition-opacity"
              @click="rotatePdf">
              旋转 ↺
            </button>
          </div>

          <!-- 图片专属控制 -->
          <div class="flex items-center gap-3 w-full" v-else-if="fileType === 'image'">
            <button class="px-3 py-1.5 bg-blue-500 text-white rounded text-sm hover:opacity-90 transition-opacity"
              @click="zoomOut" :disabled="scale <= 0.5" :class="{ 'opacity-50 cursor-not-allowed': scale <= 0.5 }">
              - 缩小
            </button>
            <span class="text-sm">{{ Math.round(scale * 100) }}%</span>
            <button class="px-3 py-1.5 bg-blue-500 text-white rounded text-sm hover:opacity-90 transition-opacity"
              @click="zoomIn" :disabled="scale >= 2" :class="{ 'opacity-50 cursor-not-allowed': scale >= 2 }">
              放大 +
            </button>
            <button class="px-3 py-1.5 bg-blue-500 text-white rounded text-sm hover:opacity-90 transition-opacity"
              @click="resetScale">
              重置
            </button>
            <button class="px-3 py-1.5 bg-blue-500 text-white rounded text-sm hover:opacity-90 transition-opacity"
              @click="rotateImage">
              旋转 ↺
            </button>
          </div>
        </div>

        <!-- 文件预览容器 -->
        <div
          class="border border-gray-200 rounded-lg min-h-[500px] sm:min-h-[600px] flex items-center justify-center p-5 bg-white overflow-auto">
          <!-- 空状态 -->
          <div class="text-center text-gray-600 p-5" v-if="!selectedFile">
            <div class="text-8xl mb-4 text-gray-300">📄</div>
            <p class="text-base">上传文件后可在此处预览</p>
          </div>

          <!-- PDF 预览（使用 pdf.js） -->
          <div class="w-full min-h-[500px] sm:min-h-[600px] flex items-center justify-center relative"
            v-else-if="fileType === 'pdf'">
            <canvas ref="pdfCanvas" class="max-w-full shadow-sm rounded"></canvas>
            <div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 text-base text-gray-600"
              v-if="!pdfLoaded">加载中...</div>
          </div>

          <!-- 图片预览 -->
          <div class="w-full min-h-[500px] sm:min-h-[600px] flex items-center justify-center overflow-auto"
            v-else-if="fileType === 'image'">
            <img :src="previewUrl"
              class="max-w-full max-h-[600px] shadow-sm rounded transition-transform duration-200 origin-center"
              :style="{ transform: `scale(${scale}) rotate(${rotateDeg}deg)` }" alt="图片预览">
          </div>

          <!-- 文本预览 -->
          <div class="w-full min-h-[500px] sm:min-h-[600px] overflow-auto" v-else-if="fileType === 'text'">
            <pre
              class="whitespace-pre-wrap break-words font-['Consolas',_'Microsoft_YaHei',_monospace] text-sm leading-relaxed text-gray-800 m-0 p-4">
              {{ textContent }}
            </pre>
          </div>

          <!-- 不支持预览的文件 -->
          <div class="text-center text-gray-600 p-5" v-else>
            <div class="text-8xl mb-4 text-gray-300">❌</div>
            <p class="text-base mb-2">该文件格式暂不支持预览 ({{ selectedFile.name }})</p>
            <p class="text-sm">支持预览格式: PDF、JPG、PNG、TXT</p>
          </div>
        </div>
      </section>
    </main>
  </div>
</template>

<script setup>
// ========== 第一步：导入 pdf.js 核心库 ==========
import { ref, watch, onMounted, onUnmounted } from 'vue';
import { upload } from '@/api/auth'
import { get_setting } from '@/api/api'

// ========== 第二步：初始化响应式变量 ==========
// 1. 定义定时任务标识（用于清除定时器）
let timer = null
// 文件相关状态
const fileInput = ref(null);
const selectedFile = ref(null);
const previewUrl = ref('');
const fileType = ref('');
const textContent = ref('');
const isDragover = ref(false); // 拖拽状态

// 上传文件列表
const uploadedFiles = ref([]);
const activeFileIndex = ref(-1);

// PDF 相关
let pdfDoc = null;
const pdfCanvas = ref(null);
const totalPages = ref(0);
const currentPage = ref(1);
const pdfScale = ref('1');
const pdfRotation = ref(0);
const pdfLoaded = ref(false);

// 图片相关
const scale = ref(1);
const rotateDeg = ref(0);

// 打印设置
const count = ref(1);
const layout = ref('portrait');
const pageMode = ref('all');
const customPages = ref('');
const color = ref('color');
const paperSize = ref('A4');
const printer = ref('默认打印机')

const getSet = async () => {
  const res = await get_setting();
  if (res['code'] == 200) {
    const data = res['data'];
    printer.value = data['printer'];
    console.log('配置：', data) // 调试用，查看实际返回的数据结构
  }
};

// ========== 第三步：初始化 PDF.js ==========
onMounted(() => {
  getSet();
  timer = setInterval(() => {
    getSet();
  }, 15000);


  try {
    // 使用 CDN 版 Worker
    pdfjsLib.GlobalWorkerOptions.workerSrc = 'https://cdn.jsdelivr.net/npm/pdfjs-dist@3.4.120/build/pdf.worker.min.js';
    console.log('PDF.js 初始化成功(npm 核心+CDN Worker)');
  } catch (error) {
    console.error('PDF.js 初始化失败:', error);
    alert('PDF 预览功能初始化失败！');
  }
});

// ========== 第四步：文件上传核心方法 ==========
// 打开文件选择器
const openFileSelector = () => {
  if (fileInput.value) {
    fileInput.value.click();
  }
};

// 添加文件到列表（带校验）
const addFileToList = (file) => {
  // 大小校验：10MB
  const maxSize = 10 * 1024 * 1024;
  if (file.size > maxSize) {
    alert(`文件 ${file.name} 大小超过10MB，无法上传！`);
    return;
  }

  // 格式校验
  const fileName = file.name.toLowerCase();
  const allowedExtensions = ['.pdf', '.doc', '.docx', '.jpg', '.jpeg', '.png', '.txt'];
  const isExtensionAllowed = allowedExtensions.some(ext => fileName.endsWith(ext));
  const isMimeTypeAllowed = file.type.includes('pdf') || file.type.includes('image') || file.type.includes('text') || file.type.includes('word');

  if (!isExtensionAllowed && !isMimeTypeAllowed) {
    alert(`文件 ${file.name} 格式不支持，仅支持 PDF/DOC/DOCX/JPG/PNG/TXT！`);
    return;
  }

  // 避免重复添加
  const isDuplicate = uploadedFiles.value.some(existFile =>
    existFile.name === file.name && existFile.size === file.size && existFile.lastModified === file.lastModified
  );
  if (isDuplicate) {
    alert(`文件 ${file.name} 已上传！`);
    return;
  }

  // 添加文件并选中
  uploadedFiles.value.push(file);
  activeFileIndex.value = uploadedFiles.value.length - 1;
  loadFilePreview(file);
};

// 处理文件选择
const handleFileChange = (e) => {
  if (!e.target || !e.target.files || e.target.files.length === 0) {
    return;
  }
  Array.from(e.target.files).forEach(file => {
    addFileToList(file);
  });
  e.target.value = ''; // 清空以支持重复选择
};

// 处理拖拽文件
const handleFileDrop = (e) => {
  e.preventDefault();
  e.stopPropagation();
  if (!e.dataTransfer || !e.dataTransfer.files || e.dataTransfer.files.length === 0) {
    return;
  }
  Array.from(e.dataTransfer.files).forEach(file => {
    addFileToList(file);
  });
  isDragover.value = false;
};

// ========== 第五步：文件预览核心方法 ==========
const loadFilePreview = async (file) => {
  selectedFile.value = file;
  const fileMimeType = file.type;

  // 重置状态
  pdfLoaded.value = false;
  scale.value = 1;
  rotateDeg.value = 0;
  currentPage.value = 1;
  pdfDoc = null;

  if (fileMimeType.includes('pdf')) {
    fileType.value = 'pdf';
    const reader = new FileReader();

    reader.onerror = (e) => {
      console.error('文件读取失败:', e);
      alert('文件读取失败，请检查文件是否损坏！');
    };

    reader.onload = async (e) => {
      try {
        const typedArray = new Uint8Array(e.target.result);
        const loadingTask = pdfjsLib.getDocument({
          data: typedArray,
          disableFetch: true,
          disableStream: true,
          disableRange: true,
          cMapUrl: 'https://cdn.jsdelivr.net/npm/pdfjs-dist@3.4.120/cmaps/',
          cMapPacked: true
        });

        pdfDoc = await loadingTask.promise;
        totalPages.value = pdfDoc.numPages;
        console.log('PDF 解析成功，总页数:', totalPages.value);

        await renderPdfPage();
        pdfLoaded.value = true;
      } catch (error) {
        console.error('PDF 解析详细错误:', error);
        alert(`PDF 解析失败：${error.message}`);
      }
    };
    reader.readAsArrayBuffer(file);
  } else if (fileMimeType.includes('image')) {
    fileType.value = 'image';
    previewUrl.value = URL.createObjectURL(file);
  } else if (fileMimeType.includes('text')) {
    fileType.value = 'text';
    const reader = new FileReader();
    reader.onload = (e) => {
      textContent.value = e.target.result;
    };
    reader.readAsText(file, 'utf-8');
  } else {
    fileType.value = 'other';
  }
};

// 渲染 PDF 页面
const renderPdfPage = async () => {
  if (!pdfDoc || !pdfCanvas.value) return;

  const page = await pdfDoc.getPage(currentPage.value);

  let scaleValue;
  if (pdfScale.value === 'auto') {
    const containerWidth = pdfCanvas.value.parentElement.clientWidth - 40;
    const viewport = page.getViewport({ scale: 1 });
    scaleValue = containerWidth / viewport.width;
  } else {
    scaleValue = parseFloat(pdfScale.value);
  }

  const viewport = page.getViewport({
    scale: scaleValue,
    rotation: pdfRotation.value
  });

  const canvas = pdfCanvas.value;
  canvas.width = viewport.width;
  canvas.height = viewport.height;

  const ctx = canvas.getContext('2d');
  await page.render({
    canvasContext: ctx,
    viewport: viewport
  }).promise;
};

// PDF 页码切换
const prevPage = async () => {
  if (currentPage.value > 1) {
    currentPage.value--;
    await renderPdfPage();
  }
};

const nextPage = async () => {
  if (currentPage.value < totalPages.value) {
    currentPage.value++;
    await renderPdfPage();
  }
};

// 旋转 PDF
const rotatePdf = async () => {
  pdfRotation.value = (pdfRotation.value + 90) % 360;
  await renderPdfPage();
};

// 图片缩放/旋转
const zoomIn = () => {
  scale.value += 0.1;
};

const zoomOut = () => {
  scale.value -= 0.1;
};

const resetScale = () => {
  scale.value = 1;
  rotateDeg.value = 0;
};

// 图片旋转
const rotateImage = () => {
  rotateDeg.value = (rotateDeg.value + 90) % 360;
};

// 切换激活文件
const switchActiveFile = (index) => {
  if (uploadedFiles.value[index]) {
    activeFileIndex.value = index;
    loadFilePreview(uploadedFiles.value[index]);
  }
};

// 预览指定文件
const previewFile = (index) => {
  switchActiveFile(index);
};

// 删除指定文件
const removeFile = (index) => {
  if (confirm('确定要删除该文件吗？')) {
    uploadedFiles.value.splice(index, 1);
    if (index === activeFileIndex.value) {
      if (uploadedFiles.value.length > 0) {
        activeFileIndex.value = 0;
        loadFilePreview(uploadedFiles.value[0]);
      } else {
        activeFileIndex.value = -1;
        selectedFile.value = null;
        fileType.value = '';
        textContent.value = '';
      }
    } else if (index < activeFileIndex.value) {
      activeFileIndex.value--;
    }
  }
};

// 清空所有文件
const clearAllFiles = () => {
  if (uploadedFiles.value.length === 0) return;
  if (confirm('确定要清空所有上传文件吗？')) {
    uploadedFiles.value = [];
    activeFileIndex.value = -1;
    selectedFile.value = null;
    fileType.value = '';
    textContent.value = '';
  }
};

// 格式化文件大小
const formatFileSize = (bytes) => {
  if (bytes < 1024) return bytes + ' B';
  else if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB';
  else return (bytes / (1024 * 1024)).toFixed(1) + ' MB';
};

// 上传/打印
const uploadFile = async (index) => {
  const targetFile = uploadedFiles.value[index];
  if (!targetFile) return;

  try {
    // 构建FormData
    const formData = new FormData();
    formData.append('file', targetFile);
    formData.append('count', count.value);

    const res = await upload(formData);
    console.log('上传：', res) // 调试用，查看实际返回的数据结构

    if (res.success) {
      // 接口成功后更新列表状态
      // uploadedFiles.value[index] = { ...targetFile, uploaded: true };
      alert(`文件 ${targetFile.name} 上传成功！`);
    } else {
      alert('上传失败，请重试！');
    }
  } catch (error) {
    console.error('上传失败：', error);
    alert('上传出错，请检查网络！');
  }
};

const printFile = () => {
  if (!selectedFile.value) {
    alert('请先选择文件！');
    return;
  }
  alert(`开始打印 ${selectedFile.value.name}，份数：${count.value}`);
};

const printAllFiles = () => {
  if (uploadedFiles.value.length === 0) {
    alert('暂无文件可打印！');
    return;
  }
  alert(`开始打印全部 ${uploadedFiles.value.length} 个文件，每份文件打印 ${count.value} 份`);
};

// 监听缩放/旋转变化
watch([pdfScale, pdfRotation], async () => {
  if (fileType.value === 'pdf' && pdfLoaded.value) {
    await renderPdfPage();
  }
});

// 清理资源
onUnmounted(() => {
  clearTimeout(timer)
  if (pdfDoc) {
    pdfDoc.destroy();
  }
  if (previewUrl.value) {
    URL.revokeObjectURL(previewUrl.value);
  }
});
</script>

<!-- 仅保留必要的全局样式，其余均使用 Tailwind -->
<style>
/* 全局基础样式重置 */
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  background-color: #f9fafb;
}

/* 修复 Tailwind 无法覆盖的原生元素样式 */
input[type="radio"] {
  accent-color: #3b82f6;
}

/* 滚动条美化 */
::-webkit-scrollbar {
  width: 6px;
  height: 6px;
}

::-webkit-scrollbar-track {
  background: #f1f1f1;
  border-radius: 3px;
}

::-webkit-scrollbar-thumb {
  background: #d1d5db;
  border-radius: 3px;
}

::-webkit-scrollbar-thumb:hover {
  background: #9ca3af;
}
</style>