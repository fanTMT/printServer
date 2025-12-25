<template>
  <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 font-['Microsoft_YaHei',_sans-serif]">
    <!-- 顶部导航 -->
    <header
      class="flex flex-col sm:flex-row justify-between items-start sm:items-center py-4 sm:py-6 border-b border-gray-200 mb-6 sm:mb-8 gap-4">
      <h1 class="text-xl sm:text-2xl font-medium flex items-center gap-2">
        <span>打印助手</span>
        <span class="text-sm sm:text-base text-gray-600 font-normal">
          打印机: <span class="font-medium">{{ printer }}</span>
        </span>
      </h1>
      <nav class="w-full sm:w-auto">
        <ul class="flex flex-wrap justify-center sm:justify-end gap-4 sm:gap-6">
          <li>
            <a href="#" class="active text-primary font-medium hover:text-primary/80 transition-colors px-1 py-2">首页</a>
          </li>
          <router-link to="/history" exact-active-class="active text-primary font-medium"
            class="text-neutral-600 hover:text-primary transition-colors px-1 py-2">
            历史记录
          </router-link>
          <router-link to="/admin" exact-active-class="active text-primary font-medium"
            class="text-neutral-600 hover:text-primary transition-colors px-1 py-2">
            管理员设置
          </router-link>
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
          class="border-2 border-dashed border-gray-300 rounded-lg p-6 sm:p-8 text-center cursor-pointer mb-5 transition-all duration-200 hover:border-blue-500 hover:bg-gray-50"
          :class="{ 'border-green-500 bg-green-50 cursor-copy': isDragover }" @click="openFileSelector"
          @dragover.prevent @dragenter.prevent="handleDragEnter" @dragleave.prevent="handleDragLeave"
          @drop.prevent="handleFileDrop">
          <div class="text-5xl sm:text-6xl mb-3 text-blue-500">📄</div>
          <p class="mb-1 text-gray-700">点击选择文件或拖拽文件到这里</p>
          <p class="mb-1 text-gray-600 text-sm">支持格式: PDF,DOC,DOCX,JPG,PNG,TXT</p>
          <p class="text-gray-600 text-sm">最大大小: 10MB</p>
          <!-- 隐藏的文件选择器 -->
          <input type="file" ref="fileInput" class="hidden" accept=".pdf,.doc,.docx,.jpg,.png,.txt,.jpeg" multiple
            @change="handleFileChange">
        </div>

        <!-- 上传文件列表 -->
        <div class="border border-gray-200 rounded-lg p-4 sm:p-5 bg-white mb-5 shadow-sm">
          <div class="flex justify-between items-center mb-3 pb-2 border-b border-gray-100">
            <h3 class="text-base sm:text-lg font-medium text-gray-800">已上传文件</h3>
            <span class="text-xs text-gray-500">{{ uploadedFiles.length }} 个文件</span>
          </div>

          <!-- 文件列表容器 -->
          <div class="max-h-[220px] overflow-y-auto mb-3 pr-1" v-if="uploadedFiles.length > 0">
            <div
              class="flex flex-col sm:flex-row items-start sm:items-center p-3 border border-gray-100 rounded-md mb-2 hover:bg-gray-50 transition-colors cursor-pointer"
              :class="{ 'bg-blue-50 border-blue-500': activeFileIndex === index }"
              v-for="(file, index) in uploadedFiles" :key="index" @click="switchActiveFile(index)">
              <!-- 文件类型图标 -->
              <div class="text-xl w-6 text-center mr-3 mb-2 sm:mb-0">
                <span v-if="file.rawFile.type?.includes('pdf')">📄</span>
                <span v-else-if="file.rawFile.type?.includes('image')">🖼️</span>
                <span v-else-if="file.rawFile.type?.includes('text')">📝</span>
                <span v-else>📁</span>
              </div>

              <!-- 文件名 -->
              <div class="flex-1 overflow-hidden mb-2 sm:mb-0">
                <span class="block text-sm text-gray-800 truncate">{{ file.name }}</span>
                <div class="flex items-center gap-2 mt-1">
                  <small class="text-xs text-gray-600">{{ formatFileSize(file.size) }}</small>
                  <span v-if="file.uploading" class="text-xs text-blue-500 flex items-center gap-1">
                    <i class="fa fa-spinner fa-spin"></i> 上传中
                  </span>
                  <span v-if="file.uploaded && !file.uploading" class="text-xs text-green-500 flex items-center gap-1">
                    <i class="fa fa-check"></i> 已上传
                  </span>
                </div>
              </div>

              <!-- 操作按钮 -->
              <div class="flex flex-wrap gap-2 w-full sm:w-auto mt-2 sm:mt-0">
                <button class="px-2 py-1 text-xs bg-blue-500 text-white rounded hover:opacity-90 transition-opacity"
                  @click.stop="previewFile(index)" title="预览文件">
                  预览
                </button>
                <button class="px-2 py-1 text-xs bg-red-500 text-white rounded hover:opacity-90 transition-opacity"
                  @click.stop="removeFile(index)" title="删除文件">
                  删除
                </button>
                <button class="px-2 py-1 text-xs bg-purple-600 text-white rounded hover:opacity-90 transition-opacity"
                  @click.stop="uploadFile(index)" :disabled="file.uploading || file.uploaded"
                  :class="{ 'opacity-60 cursor-not-allowed': file.uploading || file.uploaded }" title="上传文件到服务器">
                  <i v-if="file.uploading" class="fa fa-spinner fa-spin mr-1"></i>
                  {{ file.uploading ? '上传中' : (file.uploaded ? '已上传' : '上传') }}
                </button>
                <button class="px-2 py-1 bg-green-600 text-white rounded-md text-sm hover:opacity-90 transition-opacity"
                  @click.stop="printFile(index)" :disabled="!file.uploaded || file.uploading"
                  :class="{ 'opacity-60 cursor-not-allowed': !file.uploaded || file.uploading }" title="发送到打印机">
                  打印
                </button>
              </div>
            </div>
          </div>

          <!-- 空文件状态 -->
          <div class="text-center text-gray-600 p-4 text-sm" v-else>
            <div class="text-4xl mb-2 text-gray-300">📁</div>
            <p>暂无上传文件</p>
          </div>

          <!-- 列表操作按钮 -->
          <div class="flex justify-end gap-3 mt-2" v-if="uploadedFiles.length > 0">
            <button class="px-3 py-1.5 text-xs bg-gray-200 text-gray-700 rounded hover:bg-gray-300 transition-colors"
              @click="clearAllFiles">
              清空列表
            </button>
            <button class="px-3 py-1.5 text-xs bg-green-500 text-white rounded hover:opacity-90 transition-opacity"
              @click="printAllFiles" :disabled="!uploadedFiles.some(file => file.uploaded)"
              :class="{ 'opacity-60 cursor-not-allowed': !uploadedFiles.some(file => file.uploaded) }">
              打印全部已上传
            </button>
          </div>
        </div>

        <!-- 打印设置 -->
        <div class="border border-gray-200 rounded-lg p-4 sm:p-5 bg-white shadow-sm">
          <h3 class="text-base sm:text-lg font-medium mb-4 text-gray-800">打印设置</h3>

          <!-- 份数 -->
          <div class="mb-4">
            <label class="block mb-2 font-medium text-sm text-gray-700">打印份数</label>
            <div class="flex items-center gap-2">
              <button
                class="w-7 h-7 border border-gray-200 bg-white rounded text-base cursor-pointer hover:bg-gray-50 transition-colors"
                @click="count > 1 && (count--)" :disabled="count <= 1"
                :class="{ 'opacity-50 cursor-not-allowed': count <= 1 }">
                -
              </button>
              <span class="min-w-[2rem] text-center">{{ count }}</span>
              <button
                class="w-7 h-7 border border-gray-200 bg-white rounded text-base cursor-pointer hover:bg-gray-50 transition-colors"
                @click="count < 99 && (count++)" :disabled="count >= 99"
                :class="{ 'opacity-50 cursor-not-allowed': count >= 99 }">
                +
              </button>
              <span class="text-xs text-gray-500 ml-2">最多99份</span>
            </div>
          </div>

          <!-- 布局 -->
          <div class="mb-4">
            <label class="block mb-2 font-medium text-sm text-gray-700">页面布局</label>
            <div class="flex flex-wrap gap-4">
              <label class="flex items-center gap-1 text-sm cursor-pointer">
                <input type="radio" name="layout" value="portrait" v-model="layout" checked class="cursor-pointer"> 纵向
              </label>
              <label class="flex items-center gap-1 text-sm cursor-pointer">
                <input type="radio" name="layout" value="landscape" v-model="layout" class="cursor-pointer"> 横向
              </label>
            </div>
          </div>

          <!-- 页面 -->
          <div class="mb-4">
            <label class="block mb-2 font-medium text-sm text-gray-700">打印页面</label>
            <div class="flex flex-wrap gap-4 mb-2">
              <label class="flex items-center gap-1 text-sm cursor-pointer">
                <input type="radio" name="page" value="all" v-model="pageMode" checked class="cursor-pointer"> 全部
              </label>
              <label class="flex items-center gap-1 text-sm cursor-pointer">
                <input type="radio" name="page" value="custom" v-model="pageMode" class="cursor-pointer"> 自定义
              </label>
            </div>
            <input type="text" placeholder="例如:1-5,8,11-13" v-if="pageMode === 'custom'" v-model="customPages"
              class="w-full px-2 py-1.5 border border-gray-200 rounded-md text-sm focus:outline-none focus:ring-1 focus:ring-blue-500"
              @input="validateCustomPages">
            <p v-if="pageMode === 'custom' && !isCustomPagesValid" class="text-xs text-red-500 mt-1">格式错误，请输入正确的页码范围</p>
          </div>

          <!-- 颜色 -->
          <div class="mb-4">
            <label class="block mb-2 font-medium text-sm text-gray-700">打印颜色</label>
            <div class="flex flex-wrap gap-4">
              <label class="flex items-center gap-1 text-sm cursor-pointer">
                <input type="radio" name="color" value="color" v-model="color" checked class="cursor-pointer"> 彩色
              </label>
              <label class="flex items-center gap-1 text-sm cursor-pointer">
                <input type="radio" name="color" value="black" v-model="color" class="cursor-pointer"> 黑白
              </label>
            </div>
          </div>

          <!-- 纸张大小 -->
          <div class="mb-2">
            <label class="block mb-2 font-medium text-sm text-gray-700">纸张大小</label>
            <select v-model="paperSize"
              class="w-full px-2 py-1.5 border border-gray-200 rounded-md text-sm focus:outline-none focus:ring-1 focus:ring-blue-500">
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
        <div class="flex justify-between items-center mb-4">
          <h2 class="text-lg sm:text-xl font-medium flex items-center gap-2">
            <span>文件预览</span>
            <span v-if="selectedFile" class="text-sm font-normal text-gray-600">({{ selectedFile.name }})</span>
          </h2>
          <button v-if="selectedFile"
            class="text-sm text-blue-500 hover:text-blue-700 transition-colors flex items-center gap-1"
            @click="downloadSelectedFile">
            <i class="fa fa-download"></i> 下载原文件
          </button>
        </div>

        <!-- 通用预览控制 -->
        <div class="flex flex-wrap items-center gap-3 mb-4 p-3 bg-gray-50 rounded-md border border-gray-100"
          v-if="selectedFile">
          <!-- PDF 专属控制 -->
          <div class="flex flex-wrap items-center gap-3 w-full" v-if="fileType === 'pdf'">
            <button class="px-3 py-1.5 bg-blue-500 text-white rounded text-sm hover:opacity-90 transition-opacity"
              @click="prevPage" :disabled="currentPage <= 1"
              :class="{ 'opacity-50 cursor-not-allowed': currentPage <= 1 }">
              上一页
            </button>
            <span class="text-sm whitespace-nowrap">第 {{ currentPage }} / {{ totalPages }} 页</span>
            <button class="px-3 py-1.5 bg-blue-500 text-white rounded text-sm hover:opacity-90 transition-opacity"
              @click="nextPage" :disabled="currentPage >= totalPages"
              :class="{ 'opacity-50 cursor-not-allowed': currentPage >= totalPages }">
              下一页
            </button>
            <select v-model="pdfScale" @change="renderPdfPage"
              class="px-2 py-1.5 border border-gray-200 rounded text-sm focus:outline-none focus:ring-1 focus:ring-blue-500">
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
          <div class="flex flex-wrap items-center gap-3 w-full" v-else-if="fileType === 'image'">
            <button class="px-3 py-1.5 bg-blue-500 text-white rounded text-sm hover:opacity-90 transition-opacity"
              @click="zoomOut" :disabled="scale <= 0.5" :class="{ 'opacity-50 cursor-not-allowed': scale <= 0.5 }">
              - 缩小
            </button>
            <span class="text-sm whitespace-nowrap">{{ Math.round(scale * 100) }}%</span>
            <button class="px-3 py-1.5 bg-blue-500 text-white rounded text-sm hover:opacity-90 transition-opacity"
              @click="zoomIn" :disabled="scale >= 2" :class="{ 'opacity-50 cursor-not-allowed': scale >= 2 }">
              放大 +
            </button>
            <button class="px-3 py-1.5 bg-gray-200 text-gray-700 rounded text-sm hover:bg-gray-300 transition-colors"
              @click="resetScale">
              重置
            </button>
            <button class="px-3 py-1.5 bg-blue-500 text-white rounded text-sm hover:opacity-90 transition-opacity"
              @click="rotateImage">
              旋转 ↺
            </button>
          </div>

          <!-- 文本专属控制 -->
          <div class="flex flex-wrap items-center gap-3 w-full" v-else-if="fileType === 'text'">
            <select v-model="textFontSize" @change="updateTextPreview"
              class="px-2 py-1.5 border border-gray-200 rounded text-sm focus:outline-none focus:ring-1 focus:ring-blue-500">
              <option value="sm">小字体</option>
              <option value="md" selected>中字体</option>
              <option value="lg">大字体</option>
            </select>
            <button class="px-3 py-1.5 bg-gray-200 text-gray-700 rounded text-sm hover:bg-gray-300 transition-colors"
              @click="copyTextContent">
              <i class="fa fa-copy mr-1"></i> 复制文本
            </button>
          </div>
        </div>

        <!-- 文件预览容器 -->
        <div
          class="border border-gray-200 rounded-lg min-h-[500px] sm:min-h-[600px] flex items-center justify-center p-5 bg-white overflow-auto shadow-sm">
          <!-- 空状态 -->
          <div class="text-center text-gray-600 p-5" v-if="!selectedFile">
            <div class="text-8xl mb-4 text-gray-300">📄</div>
            <p class="text-base">上传文件后可在此处预览</p>
            <p class="text-sm text-gray-500 mt-1">支持 PDF、图片、文本文件预览</p>
          </div>

          <!-- 加载状态 -->
          <div class="text-center text-gray-600 p-5" v-if="selectedFile && isPreviewLoading.value">
            <div class="text-5xl mb-4 text-gray-300">
              <i class="fa fa-spinner fa-spin"></i>
            </div>
            <p class="text-base">正在加载预览...</p>
          </div>

          <!-- 预览错误状态 -->
          <div class="text-center text-red-500 p-5" v-if="selectedFile && previewError.value">
            <div class="text-5xl mb-4">❌</div>
            <p class="text-base mb-2">{{ previewError.value }}</p>
            <p class="text-sm text-gray-500">请检查文件是否损坏或格式不支持</p>
          </div>

          <!-- PDF 预览（使用 pdf.js） -->
          <div class="w-full min-h-[500px] sm:min-h-[600px] flex items-center justify-center relative"
            v-else-if="fileType === 'pdf' && selectedFile && !isPreviewLoading.value && !previewError.value">
            <canvas ref="pdfCanvas" class="max-w-full shadow-sm rounded"></canvas>
          </div>

          <!-- 图片预览 -->
          <div class="w-full min-h-[500px] sm:min-h-[600px] flex items-center justify-center overflow-auto"
            v-else-if="fileType === 'image' && selectedFile && !isPreviewLoading.value && !previewError.value">
            <img :src="previewUrl"
              class="max-w-full max-h-[600px] shadow-sm rounded transition-transform duration-200 origin-center"
              :style="{ transform: `scale(${scale}) rotate(${rotateDeg}deg)` }" alt="图片预览"
              @load="isPreviewLoading.value = false" @error="handlePreviewError('图片加载失败')">
          </div>

          <!-- 文本预览 -->
          <div class="w-full min-h-[500px] sm:min-h-[600px] overflow-auto p-4 bg-gray-50 rounded"
            v-else-if="fileType === 'text' && selectedFile && !isPreviewLoading.value && !previewError.value">
            <pre
              class="whitespace-pre-wrap break-words font-['Consolas',_'Microsoft_YaHei',_monospace] leading-relaxed text-gray-800 m-0"
              :class="{
                'text-xs': textFontSize === 'sm',
                'text-sm': textFontSize === 'md',
                'text-base': textFontSize === 'lg'
              }">
              {{ textContent || '无文本内容' }}
            </pre>
          </div>

          <!-- 不支持预览的文件 -->
          <div class="text-center text-gray-600 p-5"
            v-else-if="selectedFile && !isPreviewLoading.value && !previewError.value">
            <div class="text-8xl mb-4 text-gray-300">📁</div>
            <p class="text-base mb-2">该文件格式暂不支持预览</p>
            <p class="text-sm text-gray-500">文件名称: {{ selectedFile.name }}</p>
            <p class="text-sm text-gray-500 mt-2">支持预览格式: PDF、JPG、PNG、TXT</p>
          </div>
        </div>
      </section>
    </main>
  </div>
</template>

<script setup>
import { ref, watch, onMounted, onUnmounted, computed } from 'vue';
import { ElMessage } from 'element-plus';
import { ElMessageBox } from 'element-plus/es/components/message-box/index.mjs';
import { upload } from '@/api/auth';
import { get_setting } from '@/api/api';

// 重命名为 confirm 方便使用
const ElConfirm = ElMessageBox.confirm;

// ========== 类型定义 ==========
/**
 * @typedef {Object} UploadedFile
 * @property {string} name - 文件名
 * @property {number} size - 文件大小（字节）
 * @property {string} type - 文件MIME类型（冗余字段，方便访问）
 * @property {number} lastModified - 最后修改时间戳（冗余字段，方便访问）
 * @property {File} rawFile - 原生 File 对象（关键！用于 FileReader 读取）
 * @property {boolean} [uploaded] - 是否已上传到服务器
 * @property {boolean} [uploading] - 是否正在上传
 */

// ========== 响应式变量 ==========
// 打印机信息
const printer = ref('默认打印机');

// 文件相关状态
const fileInput = ref(null);
const selectedFile = ref(null); // 存储的是 UploadedFile 对象
const previewUrl = ref('');
const fileType = ref('');
const textContent = ref('');
const isDragover = ref(false);
const isPreviewLoading = ref(false);
const previewError = ref('');

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

// 图片相关
const scale = ref(1);
const rotateDeg = ref(0);

// 文本相关
const textFontSize = ref('md');

// 打印设置
const count = ref(1);
const layout = ref('portrait');
const pageMode = ref('all');
const customPages = ref('');
const isCustomPagesValid = ref(true);
const color = ref('color');
const paperSize = ref('A4');

// 定时器标识
let printerCheckTimer = null;

// ========== 工具函数 ==========
/**
 * 格式化文件大小
 * @param {number} bytes - 文件大小（字节）
 * @returns {string} 格式化后的大小字符串
 */
const formatFileSize = (bytes) => {
  if (bytes < 1024) return `${bytes} B`;
  else if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  else return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
};

/**
 * 验证自定义页码格式
 * @param {string} value - 页码字符串
 * @returns {boolean} 是否有效
 */
const validateCustomPages = () => {
  if (!customPages.value) {
    isCustomPagesValid.value = true;
    return true;
  }

  // 正则表达式：匹配 1-5,8,11-13 这样的格式
  const regex = /^(\d+(-\d+)?)(,\s*\d+(-\d+)?)*$/;
  isCustomPagesValid.value = regex.test(customPages.value);
  return isCustomPagesValid.value;
};

/**
 * 处理预览错误
 * @param {string} message - 错误信息
 */
const handlePreviewError = (message) => {
  isPreviewLoading.value = false;
  previewError.value = message;
  console.error('预览错误:', message);
};

/**
 * 重置预览状态
 */
const resetPreviewState = () => {
  isPreviewLoading.value = false;
  previewError.value = '';
  previewUrl.value = '';
  textContent.value = '';
  pdfDoc = null;
  totalPages.value = 0;
  currentPage.value = 1;
  scale.value = 1;
  rotateDeg.value = 0;
};

// ========== 生命周期钩子 ==========
onMounted(async () => {
  // 获取打印机配置
  await fetchPrinterConfig();

  // 定时检查打印机状态（15秒一次）
  printerCheckTimer = setInterval(fetchPrinterConfig, 15000);

  // 初始化 PDF.js
  await initPdfJs();
});

onUnmounted(() => {
  // 清除定时器
  if (printerCheckTimer) {
    clearInterval(printerCheckTimer);
  }

  // 清理 PDF 资源
  if (pdfDoc) {
    pdfDoc.destroy();
  }

  // 释放 blob URL
  if (previewUrl.value) {
    URL.revokeObjectURL(previewUrl.value);
  }
});

// ========== 初始化相关 ==========
/**
 * 获取打印机配置
 */
const fetchPrinterConfig = async () => {
  try {
    const res = await get_setting();
    if (res.code === 200 && res.data?.printer) {
      printer.value = res.data.printer;
    }
  } catch (error) {
    console.error('获取打印机配置失败:', error);
    ElMessage.warning('无法获取打印机信息，使用默认配置');
  }
};

/**
 * 初始化 PDF.js
 */
const initPdfJs = async () => {
  try {
    // 确保 pdfjsLib 已加载
    if (window.pdfjsLib) {
      // 配置 Worker
      window.pdfjsLib.GlobalWorkerOptions.workerSrc =
        'https://cdn.jsdelivr.net/npm/pdfjs-dist@3.4.120/build/pdf.worker.min.js';

      console.log('PDF.js 初始化成功');
    } else {
      throw new Error('pdfjsLib 未加载');
    }
  } catch (error) {
    console.error('PDF.js 初始化失败:', error);
    ElMessage.error('PDF 预览功能初始化失败，可能无法预览 PDF 文件');
  }
};

// ========== 文件上传相关 ==========
/**
 * 打开文件选择器
 */
const openFileSelector = () => {
  fileInput.value?.click();
};

/**
 * 处理拖拽进入
 */
const handleDragEnter = () => {
  isDragover.value = true;
};

/**
 * 处理拖拽离开
 */
const handleDragLeave = (e) => {
  // 只有当鼠标完全离开容器时才取消拖拽状态
  if (e.relatedTarget && !e.currentTarget.contains(e.relatedTarget)) {
    isDragover.value = false;
  }
};

/**
 * 验证文件是否合法
 * @param {File} file - 要验证的原生 File 对象
 * @returns {string|null} 错误信息，null 表示合法
 */
const validateFile = (file) => {
  // 大小校验：10MB
  const maxSize = 10 * 1024 * 1024;
  if (file.size > maxSize) {
    return `文件 ${file.name} 大小超过10MB，无法上传！`;
  }

  // 格式校验
  const fileName = file.name.toLowerCase();
  const allowedExtensions = ['.pdf', '.doc', '.docx', '.jpg', '.jpeg', '.png', '.txt'];
  const isExtensionAllowed = allowedExtensions.some(ext => fileName.endsWith(ext));

  if (!isExtensionAllowed) {
    return `文件 ${file.name} 格式不支持，仅支持 PDF/DOC/DOCX/JPG/PNG/TXT！`;
  }

  // 避免重复添加（通过文件名、大小、最后修改时间判断）
  const isDuplicate = uploadedFiles.value.some(existFile =>
    existFile.name === file.name &&
    existFile.size === file.size &&
    existFile.lastModified === file.lastModified
  );

  if (isDuplicate) {
    return `文件 ${file.name} 已上传！`;
  }

  return null;
};

/**
 * 添加文件到列表
 * @param {File} rawFile - 原生 File 对象
 */
const addFileToList = (rawFile) => {
  const errorMsg = validateFile(rawFile);
  if (errorMsg) {
    ElMessage.warning(errorMsg);
    return;
  }

  // 包装文件对象：保存原生 File 对象，同时添加状态字段
  /** @type {UploadedFile} */
  const fileWithState = {
    name: rawFile.name,
    size: rawFile.size,
    type: rawFile.type,
    lastModified: rawFile.lastModified,
    rawFile: rawFile, // 关键：保存原生 File 对象
    uploaded: false,
    uploading: false
  };

  // 添加文件并选中
  uploadedFiles.value.push(fileWithState);
  activeFileIndex.value = uploadedFiles.value.length - 1;
  loadFilePreview(fileWithState);
};

/**
 * 处理文件选择
 * @param {Event} e - 文件选择事件
 */
const handleFileChange = (e) => {
  const files = e.target.files;
  if (!files || files.length === 0) return;

  Array.from(files).forEach(file => {
    addFileToList(file); // 直接传入原生 File 对象
  });

  // 清空以支持重复选择相同文件
  e.target.value = '';
};

/**
 * 处理拖拽文件
 * @param {DragEvent} e - 拖拽事件
 */
const handleFileDrop = (e) => {
  e.preventDefault();
  e.stopPropagation();

  const files = e.dataTransfer.files;
  if (!files || files.length === 0) return;

  Array.from(files).forEach(file => {
    addFileToList(file); // 直接传入原生 File 对象
  });

  isDragover.value = false;
};

// ========== 文件预览相关 ==========
/**
 * 加载文件预览
 * @param {UploadedFile} file - 包装后的文件对象
 */
const loadFilePreview = async (file) => {
  if (!file || !file.rawFile) { // 确保有原生 File 对象
    handlePreviewError('无效的文件对象');
    return;
  }

  selectedFile.value = file;
  resetPreviewState();
  isPreviewLoading.value = true;

  try {
    const fileMimeType = file.rawFile.type; // 使用原生 File 对象的 type

    if (fileMimeType.includes('pdf')) {
      await loadPdfPreview(file.rawFile); // 传入原生 File 对象
    } else if (fileMimeType.includes('image')) {
      loadImagePreview(file.rawFile); // 传入原生 File 对象
    } else if (fileMimeType.includes('text')) {
      await loadTextPreview(file.rawFile); // 传入原生 File 对象
    } else {
      fileType.value = 'other';
      isPreviewLoading.value = false;
    }
  } catch (error) {
    handlePreviewError(`预览失败: ${error.message}`);
  }
};

/**
 * 加载 PDF 预览
 * @param {File} rawFile - 原生 PDF 文件对象
 */
const loadPdfPreview = async (rawFile) => {
  fileType.value = 'pdf';

  return new Promise((resolve, reject) => {
    const reader = new FileReader();

    reader.onerror = (e) => {
      reject(new Error('PDF 文件读取失败'));
    };

    reader.onload = async (e) => {
      try {
        const typedArray = new Uint8Array(e.target.result);
        const loadingTask = window.pdfjsLib.getDocument({
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
        isPreviewLoading.value = false;
        resolve();
      } catch (error) {
        reject(error);
      }
    };

    // 读取原生 File 对象
    reader.readAsArrayBuffer(rawFile);
  });
};

/**
 * 加载图片预览
 * @param {File} rawFile - 原生图片文件对象
 */
const loadImagePreview = (rawFile) => {
  fileType.value = 'image';
  // 创建 blob URL 用于预览（使用原生 File 对象）
  previewUrl.value = URL.createObjectURL(rawFile);
};

/**
 * 加载文本预览
 * @param {File} rawFile - 原生文本文件对象
 */
const loadTextPreview = async (rawFile) => {
  fileType.value = 'text';

  return new Promise((resolve, reject) => {
    const reader = new FileReader();

    reader.onerror = (e) => {
      reject(new Error('文本文件读取失败'));
    };

    reader.onload = (e) => {
      textContent.value = e.target.result || '';
      isPreviewLoading.value = false;
      resolve();
    };

    // 尝试多种编码读取原生 File 对象
    reader.readAsText(rawFile, 'utf-8');
  });
};

/**
 * 渲染 PDF 页面
 */
const renderPdfPage = async () => {
  if (!pdfDoc || !pdfCanvas.value) return;

  const page = await pdfDoc.getPage(currentPage.value);

  // 计算缩放比例
  let scaleValue;
  if (pdfScale.value === 'auto') {
    const containerWidth = pdfCanvas.value.parentElement.clientWidth - 40;
    const viewport = page.getViewport({ scale: 1 });
    scaleValue = containerWidth / viewport.width;
  } else {
    scaleValue = parseFloat(pdfScale.value);
  }

  // 获取旋转后的视口
  const viewport = page.getViewport({
    scale: scaleValue,
    rotation: pdfRotation.value
  });

  // 设置 canvas 尺寸
  const canvas = pdfCanvas.value;
  canvas.width = viewport.width;
  canvas.height = viewport.height;

  // 渲染页面
  const ctx = canvas.getContext('2d');
  await page.render({
    canvasContext: ctx,
    viewport: viewport
  }).promise;
};

/**
 * PDF 页码切换 - 上一页
 */
const prevPage = async () => {
  if (currentPage.value > 1) {
    currentPage.value--;
    await renderPdfPage();
  }
};

/**
 * PDF 页码切换 - 下一页
 */
const nextPage = async () => {
  if (currentPage.value < totalPages.value) {
    currentPage.value++;
    await renderPdfPage();
  }
};

/**
 * 旋转 PDF
 */
const rotatePdf = async () => {
  pdfRotation.value = (pdfRotation.value + 90) % 360;
  await renderPdfPage();
};

/**
 * 图片缩放 - 放大
 */
const zoomIn = () => {
  if (scale.value < 2) {
    scale.value = parseFloat((scale.value + 0.1).toFixed(1));
  }
};

/**
 * 图片缩放 - 缩小
 */
const zoomOut = () => {
  if (scale.value > 0.5) {
    scale.value = parseFloat((scale.value - 0.1).toFixed(1));
  }
};

/**
 * 重置图片缩放和旋转
 */
const resetScale = () => {
  scale.value = 1;
  rotateDeg.value = 0;
};

/**
 * 旋转图片
 */
const rotateImage = () => {
  rotateDeg.value = (rotateDeg.value + 90) % 360;
};

/**
 * 更新文本预览样式
 */
const updateTextPreview = () => {
  // 仅更新样式，内容不变
};

/**
 * 复制文本内容
 */
const copyTextContent = () => {
  if (!textContent.value) {
    ElMessage.warning('无文本内容可复制');
    return;
  }

  navigator.clipboard.writeText(textContent.value)
    .then(() => {
      ElMessage.success('文本已复制到剪贴板');
    })
    .catch(() => {
      ElMessage.error('复制失败，请手动复制');
    });
};

/**
 * 下载选中的文件
 */
const downloadSelectedFile = () => {
  if (!selectedFile.value || !selectedFile.value.rawFile) return;

  // 使用原生 File 对象创建下载链接
  const url = URL.createObjectURL(selectedFile.value.rawFile);
  const a = document.createElement('a');
  a.href = url;
  a.download = selectedFile.value.name;
  document.body.appendChild(a);
  a.click();

  // 清理
  document.body.removeChild(a);
  URL.revokeObjectURL(url);
};

// ========== 文件列表操作相关 ==========
/**
 * 切换激活文件
 * @param {number} index - 文件索引
 */
const switchActiveFile = (index) => {
  const file = uploadedFiles.value[index];
  if (file) {
    activeFileIndex.value = index;
    loadFilePreview(file);
  }
};

/**
 * 预览指定文件
 * @param {number} index - 文件索引
 */
const previewFile = (index) => {
  switchActiveFile(index);
};

/**
 * 删除指定文件
 * @param {number} index - 文件索引
 */
const removeFile = async (index) => {
  try {
    const file = uploadedFiles.value[index];
    if (!file) return;

    await ElConfirm({
      title: '确认删除',
      message: `确定要删除文件 "${file.name}" 吗？`,
      confirmButtonText: '确认',
      cancelButtonText: '取消',
    });

    // 释放预览资源
    if (index === activeFileIndex.value) {
      if (previewUrl.value) {
        URL.revokeObjectURL(previewUrl.value);
      }
    }

    // 从列表中删除
    uploadedFiles.value.splice(index, 1);

    // 更新激活状态
    if (index === activeFileIndex.value) {
      if (uploadedFiles.value.length > 0) {
        activeFileIndex.value = 0;
        loadFilePreview(uploadedFiles.value[0]);
      } else {
        activeFileIndex.value = -1;
        selectedFile.value = null;
        fileType.value = '';
        resetPreviewState();
      }
    } else if (index < activeFileIndex.value) {
      activeFileIndex.value--;
    }

    ElMessage.success('文件已删除');
  } catch (error) {
    // 取消删除
  }
};

/**
 * 清空所有文件
 */
const clearAllFiles = async () => {
  if (uploadedFiles.value.length === 0) return;

  try {
    await ElConfirm({
      title: '确认清空',
      message: `确定要清空所有 ${uploadedFiles.value.length} 个文件吗？`,
      confirmButtonText: '确认',
      cancelButtonText: '取消',
      type: 'warning',
    });

    // 释放所有预览资源
    if (previewUrl.value) {
      URL.revokeObjectURL(previewUrl.value);
    }

    // 清空列表和状态
    uploadedFiles.value = [];
    activeFileIndex.value = -1;
    selectedFile.value = null;
    fileType.value = '';
    resetPreviewState();

    ElMessage.success('所有文件已清空');
  } catch (error) {
    // 取消清空
  }
};

// ========== 上传和打印相关 ==========
/**
 * 上传文件到服务器
 * @param {number} index - 文件索引
 */
const uploadFile = async (index) => {
  const file = uploadedFiles.value[index];
  if (!file || !file.rawFile || file.uploading || file.uploaded) return;

  try {
    // 更新上传状态
    uploadedFiles.value[index] = { ...file, uploading: true };

    // 构建 FormData（使用原生 File 对象）
    const formData = new FormData();
    formData.append('file', file.rawFile);
    formData.append('count', count.value);

    // 调用上传接口
    const res = await upload(formData);
    console.log('文件上传结果:', res);

    if (res.success) {
      // 更新上传状态
      uploadedFiles.value[index] = {
        ...file,
        uploading: false,
        uploaded: true
      };
      ElMessage.success(`文件 "${file.name}" 上传成功！`);
    } else {
      throw new Error(res.message || '上传失败');
    }
  } catch (error) {
    // 恢复上传状态
    uploadedFiles.value[index] = { ...file, uploading: false };
    ElMessage.error(`文件上传失败: ${error.message}`);
    console.error('上传失败:', error);
  }
};

/**
 * 打印单个文件
 * @param {number} index - 文件索引
 */
const printFile = async (index) => {
  const file = uploadedFiles.value[index];
  if (!file || !file.uploaded || file.uploading) return;

  // 验证打印参数
  if (pageMode.value === 'custom' && !validateCustomPages()) {
    ElMessage.warning('自定义页码格式错误，请输入正确的页码范围');
    return;
  }

  try {
    // 构建打印参数
    const printParams = {
      fileName: file.name,
      count: count.value,
      layout: layout.value,
      pageMode: pageMode.value,
      customPages: pageMode.value === 'custom' ? customPages.value : '',
      color: color.value,
      paperSize: paperSize.value
    };

    console.log('发送打印请求:', printParams);

    // 这里可以添加实际的打印接口调用
    // 模拟打印成功
    ElMessage.success(`正在打印文件: ${file.name} (${count.value}份)`);

    // 可以添加打印记录到历史记录
  } catch (error) {
    ElMessage.error(`打印失败: ${error.message}`);
    console.error('打印失败:', error);
  }
};

/**
 * 打印所有已上传文件
 */
const printAllFiles = async () => {
  const uploadedFilesList = uploadedFiles.value.filter(file => file.uploaded);
  if (uploadedFilesList.length === 0) {
    ElMessage.warning('暂无已上传的文件可打印');
    return;
  }

  // 验证打印参数
  if (pageMode.value === 'custom' && !validateCustomPages()) {
    ElMessage.warning('自定义页码格式错误，请输入正确的页码范围');
    return;
  }

  try {
    await ElConfirm({
      title: '确认打印',
      message: `确定要打印所有 ${uploadedFilesList.length} 个已上传文件吗？每份文件将打印 ${count.value} 份`,
      confirmButtonText: '确认',
      cancelButtonText: '取消',
      type: 'info',
    });

    // 构建打印参数
    const printParams = {
      fileNames: uploadedFilesList.map(file => file.name),
      count: count.value,
      layout: layout.value,
      pageMode: pageMode.value,
      customPages: pageMode.value === 'custom' ? customPages.value : '',
      color: color.value,
      paperSize: paperSize.value
    };

    console.log('发送批量打印请求:', printParams);

    // 这里可以添加实际的批量打印接口调用
    // 模拟打印成功
    ElMessage.success(`已开始打印 ${uploadedFilesList.length} 个文件，每份 ${count.value} 份`);

    // 可以添加打印记录到历史记录
  } catch (error) {
    // 取消打印
  }
};

// ========== 监听相关 ==========
// 监听 PDF 缩放和旋转变化
watch([pdfScale, pdfRotation], async () => {
  if (fileType.value === 'pdf' && pdfDoc && !isPreviewLoading.value) {
    await renderPdfPage();
  }
});

// 监听窗口大小变化，重新渲染 PDF
watch(
  () => window.innerWidth,
  async () => {
    if (fileType.value === 'pdf' && pdfDoc && !isPreviewLoading.value) {
      await renderPdfPage();
    }
  }
);
</script>

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

/* 自定义动画 */
@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

.fa-spin {
  animation: spin 1s linear infinite;
}

/* 激活链接样式 */
a.active {
  color: #3b82f6;
  position: relative;
}

a.active::after {
  content: '';
  position: absolute;
  bottom: -2px;
  left: 0;
  width: 100%;
  height: 2px;
  background-color: #3b82f6;
  border-radius: 1px;
}

/* 响应式调整 */
@media (max-width: 640px) {
  .flex-wrap {
    flex-wrap: wrap !important;
  }

  .gap-2 {
    gap: 0.375rem !important;
  }

  button {
    padding: 0.25rem 0.5rem !important;
    font-size: 0.75rem !important;
  }
}
</style>