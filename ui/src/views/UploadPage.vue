<template>
  <div class="max-w-3xl mx-auto px-4 sm:px-6 lg:px-8 font-['Microsoft_YaHei',_sans-serif]">
    <!-- 主内容区 -->
    <main class="flex flex-col gap-6 sm:gap-8 lg:gap-10">
      <!-- 上传区域 -->
      <section class="w-full">
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
        <div class="border border-gray-200 rounded-lg p-4 sm:p-5 bg-white shadow-sm">
          <div class="flex justify-between items-center mb-3 pb-2 border-b border-gray-100">
            <h3 class="text-base sm:text-lg font-medium text-gray-800">待上传文件</h3>
            <span class="text-xs text-gray-500">{{ uploadedFiles.length }} 个文件</span>
          </div>

          <!-- 文件列表容器 -->
          <div class="max-h-[400px] overflow-y-auto mb-3 pr-1" v-if="uploadedFiles.length > 0">
            <div
              class="flex flex-col sm:flex-row items-start sm:items-center p-3 border border-gray-100 rounded-md mb-2 hover:bg-gray-50 transition-colors"
              v-for="(file, index) in uploadedFiles" :key="index">

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
                <button class="px-2 py-1 text-xs bg-red-500 text-white rounded hover:opacity-90 transition-opacity"
                  @click="removeFile(index)" title="删除文件">
                  删除
                </button>
                <button class="px-2 py-1 text-xs bg-purple-600 text-white rounded hover:opacity-90 transition-opacity"
                  @click="uploadFile(index)" :disabled="file.uploading || file.uploaded"
                  :class="{ 'opacity-60 cursor-not-allowed': file.uploading || file.uploaded }" title="上传文件到服务器">
                  <i v-if="file.uploading" class="fa fa-spinner fa-spin mr-1"></i>
                  {{ file.uploading ? '上传中' : (file.uploaded ? '已上传' : '上传') }}
                </button>
              </div>
            </div>
          </div>

          <!-- 空文件状态 -->
          <div class="text-center text-gray-600 p-4 text-sm" v-else>
            <div class="text-4xl mb-2 text-gray-300">📁</div>
            <p>暂无待上传文件</p>
          </div>

          <!-- 列表操作按钮 -->
          <div class="flex justify-end gap-3 mt-2" v-if="uploadedFiles.length > 0">
            <button class="px-3 py-1.5 text-xs bg-gray-200 text-gray-700 rounded hover:bg-gray-300 transition-colors"
              @click="clearAllFiles">
              清空列表
            </button>
            <button class="px-3 py-1.5 text-xs bg-blue-500 text-white rounded hover:opacity-90 transition-opacity"
              @click="uploadAllFiles" :disabled="!uploadedFiles.some(file => !file.uploading && !file.uploaded)"
              :class="{ 'opacity-60 cursor-not-allowed': !uploadedFiles.some(file => !file.uploading && !file.uploaded) }">
              全部上传
            </button>
          </div>
        </div>
      </section>
    </main>
  </div>
</template>

<script setup>
import { ref } from 'vue';
import { ElMessage } from 'element-plus';
import { ElMessageBox } from 'element-plus/es/components/message-box/index.mjs';
import { upload } from '@/api/api';

// 重命名为 confirm 方便使用
const ElConfirm = ElMessageBox;

// ========== 响应式变量 ==========
// 文件相关状态
const fileInput = ref(null);
const isDragover = ref(false);

// 上传文件列表
const uploadedFiles = ref([]);

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

  const fileWithState = {
    name: rawFile.name,
    size: rawFile.size,
    type: rawFile.type,
    lastModified: rawFile.lastModified,
    rawFile: rawFile,
    uploaded: false,
    uploading: false
  };

  uploadedFiles.value.push(fileWithState);
  ElMessage.success(`文件 "${rawFile.name}" 已添加到列表`);
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

// ========== 上传相关 ==========

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

    // 从列表中删除
    uploadedFiles.value.splice(index, 1);

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

    // 清空列表
    uploadedFiles.value = [];

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
    uploadedFiles.value[index] = { ...file, uploading: true };

    const formData = new FormData();
    formData.append('file', file.rawFile);

    const res = await upload(formData);
    console.log('文件上传结果:', res);

    if (res.success) {
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
    uploadedFiles.value[index] = { ...file, uploading: false };
    ElMessage.error(`文件上传失败: ${error.message}`);
    console.error('上传失败:', error);
  }
};

/**
 * 上传所有文件
 */
const uploadAllFiles = async () => {
  const pendingFiles = uploadedFiles.value.filter(file => !file.uploading && !file.uploaded);
  if (pendingFiles.length === 0) {
    ElMessage.warning('暂无待上传的文件');
    return;
  }

  for (const file of pendingFiles) {
    const index = uploadedFiles.value.indexOf(file);
    if (index !== -1) {
      await uploadFile(index);
    }
  }
};

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