<template>
    <div class="font-inter bg-neutral-100 min-h-screen text-neutral-800">
        <!-- 主内容区 -->
        <main class="container mx-auto px-3 md:px-4 py-4 md:py-8">
            <h2 class="text-xl md:text-2xl font-bold mb-4 md:mb-6">打印历史记录</h2>

            <!-- 统计卡片（手机端优化） -->
            <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-3 md:gap-6 mb-6 md:mb-8">
                <div class="bg-white rounded-lg md:rounded-xl shadow-sm md:shadow-card p-4 md:p-6 transition-all duration-300">
                    <div class="flex items-center justify-between mb-2">
                        <h3 class="text-xs md:text-sm text-neutral-600 font-medium">总打印次数</h3>
                        <span class="p-1.5 md:p-2 bg-primary/10 text-primary rounded text-sm md:text-base">
                            <i class="fa fa-bar-chart"></i>
                        </span>
                    </div>
                    <p class="text-2xl md:text-3xl font-bold">{{ totalPrints }}</p>
                    <p class="text-xs md:text-sm text-neutral-500 mt-1">本月: {{ monthlyPrints }} 次</p>
                </div>
                <div class="bg-white rounded-lg md:rounded-xl shadow-sm md:shadow-card p-4 md:p-6 transition-all duration-300">
                    <div class="flex items-center justify-between mb-2">
                        <h3 class="text-xs md:text-sm text-neutral-600 font-medium">已完成打印</h3>
                        <span class="p-1.5 md:p-2 bg-secondary/10 text-secondary rounded text-sm md:text-base">
                            <i class="fa fa-check-circle"></i>
                        </span>
                    </div>
                    <p class="text-2xl md:text-3xl font-bold">{{ completedPrints }}</p>
                    <p class="text-xs md:text-sm text-neutral-500 mt-1">成功率: {{ successRate }}</p>
                </div>
                <div class="bg-white rounded-lg md:rounded-xl shadow-sm md:shadow-card p-4 md:p-6 transition-all duration-300">
                    <div class="flex items-center justify-between mb-2">
                        <h3 class="text-xs md:text-sm text-neutral-600 font-medium">打印文件大小</h3>
                        <span class="p-1.5 md:p-2 bg-blue-100 text-blue-600 rounded text-sm md:text-base">
                            <i class="fa fa-hdd-o"></i>
                        </span>
                    </div>
                    <p class="text-2xl md:text-3xl font-bold">{{ totalSize }}</p>
                    <p class="text-xs md:text-sm text-neutral-500 mt-1">平均: {{ avgSize }}</p>
                </div>
            </div>

            <!-- 历史记录表格 -->
            <div class="bg-white rounded-xl shadow-card p-6 transition-all duration-300 hover:shadow-hover mb-8">
                <div class="flex justify-between items-center mb-6">
                    <h3 class="text-lg font-semibold">历史记录列表</h3>
                    <div class="flex items-center space-x-3">
                        <div class="relative">
                            <input type="text" v-model="searchKeyword" placeholder="搜索文件名..."
                                class="pl-9 pr-4 py-2 border border-neutral-300 rounded-lg text-sm focus:outline-none focus:ring-1 focus:ring-primary focus:border-primary">
                            <i
                                class="fa fa-search absolute left-3 top-1/2 transform -translate-y-1/2 text-neutral-400"></i>
                        </div>
                        <button @click="toggleFilterPanel"
                            class="px-4 py-2 border border-neutral-300 rounded-lg text-sm flex items-center space-x-1 hover:bg-neutral-50 transition-colors">
                            <i class="fa fa-filter text-neutral-500"></i>
                            <span>筛选</span>
                        </button>
                        <button @click="showConfirmDialog = true"
                            class="px-4 py-2 bg-red-500 hover:bg-red-600 text-white rounded-lg text-sm transition-colors">
                            清空历史
                        </button>
                    </div>
                </div>

                <!-- 表格 -->
                <div class="overflow-x-auto">
                    <table class="w-full min-w-[800px]">
                        <thead>
                            <tr class="text-left text-neutral-600 border-b border-neutral-200">
                                <th class="py-3 px-4 font-medium table-header-hover" @click="sortTable('id')">
                                    <div class="flex items-center space-x-1">
                                        <span>ID</span>
                                        <i class="fa" :class="getSortIcon('id')"></i>
                                    </div>
                                </th>
                                <th class="py-3 px-4 font-medium table-header-hover"
                                    @click="sortTable('original_name')">
                                    <div class="flex items-center space-x-1">
                                        <span>文件名</span>
                                        <i class="fa" :class="getSortIcon('original_name')"></i>
                                    </div>
                                </th>
                                <th class="py-3 px-4 font-medium table-header-hover" @click="sortTable('file_size')">
                                    <div class="flex items-center space-x-1">
                                        <span>文件大小</span>
                                        <i class="fa" :class="getSortIcon('file_size')"></i>
                                    </div>
                                </th>
                                <th class="py-3 px-4 font-medium table-header-hover" @click="sortTable('printer')">
                                    <div class="flex items-center space-x-1">
                                        <span>打印机</span>
                                        <i class="fa" :class="getSortIcon('printer')"></i>
                                    </div>
                                </th>
                                <th class="py-3 px-4 font-medium table-header-hover" @click="sortTable('status')">
                                    <div class="flex items-center space-x-1">
                                        <span>状态</span>
                                        <i class="fa" :class="getSortIcon('status')"></i>
                                    </div>
                                </th>
                                <th class="py-3 px-4 font-medium table-header-hover" @click="sortTable('created_at')">
                                    <div class="flex items-center space-x-1">
                                        <span>打印时间</span>
                                        <i class="fa" :class="getSortIcon('created_at')"></i>
                                    </div>
                                </th>
                                <th class="py-3 px-4 font-medium">操作</th>
                            </tr>
                        </thead>
                        <tbody>
                            <!-- 空状态 -->
                            <tr v-if="filteredHistory.length === 0">
                                <td colspan="7" class="py-12 text-center">
                                    <div class="inline-flex flex-col items-center">
                                        <i class="fa fa-history text-5xl text-neutral-300 mb-4"></i>
                                        <p class="text-neutral-500 mb-2">暂无打印历史记录</p>
                                        <p class="text-sm text-neutral-400">开始打印文件后，历史记录将显示在这里</p>
                                    </div>
                                </td>
                            </tr>

                            <!-- 历史记录行 -->
                            <tr v-for="item in paginatedHistory" :key="item.id"
                                class="border-b border-neutral-200 hover:bg-neutral-50">
                                <td class="py-3 px-4">{{ item.id }}</td>
                                <td class="py-3 px-4">
                                    <!-- 文件名超出隐藏 -->
                                    <div class="max-w-[200px] whitespace-nowrap overflow-hidden text-ellipsis">
                                        {{ item.original_name }}
                                    </div>
                                </td>
                                <td class="py-3 px-4">{{ item.file_size }}</td>
                                <td class="py-3 px-4">{{ item.printer }}</td>
                                <td class="py-3 px-4">
                                    <span :class="[
                                        'px-2 py-1 rounded-full text-xs font-medium',
                                        item.status === '打印完成' ? 'bg-green-100 text-green-800' : 'bg-red-100 text-red-800'
                                    ]">
                                        {{ item.status }}
                                    </span>
                                </td>
                                <td class="py-3 px-4">{{ formatDate(item.created_at) }}</td>
                                <td class="py-3 px-4">
                                    <div class="flex space-x-2">
                                        <button @click="previewFile(item)" :disabled="!item.file_path"
                                            class="text-blue-600 hover:text-blue-800 transition-colors text-sm disabled:opacity-50 disabled:cursor-not-allowed">
                                            <i class="fa fa-eye mr-1"></i>预览
                                        </button>
                                        <button @click="printAgain(item)" :disabled="!item.file_path"
                                            class="text-green-600 hover:text-green-800 transition-colors text-sm disabled:opacity-50 disabled:cursor-not-allowed">
                                            <i class="fa fa-print mr-1"></i>重新打印
                                        </button>
                                        <button @click="deleteRecord(item.id)"
                                            class="text-red-600 hover:text-red-800 transition-colors text-sm">
                                            <i class="fa fa-trash mr-1"></i>删除
                                        </button>
                                    </div>
                                </td>
                            </tr>
                        </tbody>
                    </table>
                </div>

                <!-- 分页控件 -->
                <div v-if="filteredHistory.length > 0" class="mt-6 flex justify-between items-center">
                    <div class="text-sm text-neutral-600">
                        显示 <span>{{ getShowingRange() }}</span> 条，共 <span>{{ filteredHistory.length }}</span> 条记录
                    </div>
                    <div class="flex items-center space-x-1">
                        <button @click="changePage(currentPage - 1)" :disabled="currentPage === 1"
                            class="px-3 py-1 border border-neutral-300 rounded text-sm hover:bg-neutral-50 transition-colors disabled:opacity-50 disabled:cursor-not-allowed">
                            <i class="fa fa-chevron-left"></i>
                        </button>
                        <div class="flex items-center space-x-1">
                            <button v-for="page in totalPages" :key="page" @click="changePage(page)" :class="[
                                'px-3 py-1 border rounded text-sm transition-colors',
                                currentPage === page ? 'bg-primary text-white border-primary' : 'border-neutral-300 hover:bg-neutral-50'
                            ]">
                                {{ page }}
                            </button>
                        </div>
                        <button @click="changePage(currentPage + 1)" :disabled="currentPage === totalPages"
                            class="px-3 py-1 border border-neutral-300 rounded text-sm hover:bg-neutral-50 transition-colors disabled:opacity-50 disabled:cursor-not-allowed">
                            <i class="fa fa-chevron-right"></i>
                        </button>
                    </div>
                </div>
            </div>

            <!-- 筛选面板 -->
            <div class="bg-white rounded-xl shadow-card p-6 transition-all duration-300 hover:shadow-hover mb-8"
                :class="{ hidden: !showFilterPanel }">
                <h3 class="text-lg font-semibold mb-4">筛选条件</h3>
                <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                    <div>
                        <label class="block text-sm text-neutral-600 mb-1">状态</label>
                        <select v-model="filterStatus"
                            class="w-full px-3 py-2 border border-neutral-300 rounded-md text-sm focus:outline-none focus:ring-1 focus:ring-primary focus:border-primary">
                            <option value="all">全部状态</option>
                            <option value="打印完成">已完成</option>
                            <option value="打印失败">失败</option>
                        </select>
                    </div>
                    <div>
                        <label class="block text-sm text-neutral-600 mb-1">日期范围</label>
                        <div class="flex space-x-2">
                            <input type="date" v-model="filterDateFrom"
                                class="w-full px-3 py-2 border border-neutral-300 rounded-md text-sm focus:outline-none focus:ring-1 focus:ring-primary focus:border-primary">
                            <input type="date" v-model="filterDateTo"
                                class="w-full px-3 py-2 border border-neutral-300 rounded-md text-sm focus:outline-none focus:ring-1 focus:ring-primary focus:border-primary">
                        </div>
                    </div>
                    <div>
                        <label class="block text-sm text-neutral-600 mb-1">文件类型</label>
                        <select v-model="filterFileType"
                            class="w-full px-3 py-2 border border-neutral-300 rounded-md text-sm focus:outline-none focus:ring-1 focus:ring-primary focus:border-primary">
                            <option value="all">全部类型</option>
                            <option value="pdf">PDF</option>
                            <option value="doc">Word</option>
                            <option value="jpg">图片</option>
                            <option value="png">图片</option>
                            <option value="txt">文本</option>
                        </select>
                    </div>
                </div>
                <div class="mt-4 flex justify-end space-x-3">
                    <button @click="resetFilter"
                        class="px-4 py-2 border border-neutral-300 rounded-lg text-sm hover:bg-neutral-50 transition-colors">
                        重置
                    </button>
                    <button @click="applyFilter"
                        class="px-4 py-2 bg-primary hover:bg-primary/90 text-white rounded-lg text-sm transition-colors">
                        应用筛选
                    </button>
                </div>
            </div>
        </main>

        <!-- 预览和打印侧边栏 -->
        <div class="fixed right-0 top-0 h-screen w-96 bg-white shadow-lg transform transition-transform duration-300 z-40"
            :class="{ 'translate-x-full': !showPreviewPanel, 'translate-x-0': showPreviewPanel }">
            <div class="h-full flex flex-col">
                <!-- 头部 -->
                <div class="flex justify-between items-center p-6 border-b border-neutral-200">
                    <h3 class="text-lg font-semibold">文件预览</h3>
                    <button @click="closePreviewPanel" class="text-neutral-400 hover:text-neutral-600">
                        <i class="fa fa-times text-xl"></i>
                    </button>
                </div>

                <!-- 内容 -->
                <div class="flex-1 overflow-y-auto p-6">
                    <div v-if="!selectedFile" class="text-center text-neutral-500 py-12">
                        <i class="fa fa-file text-5xl text-neutral-300 mb-4 block"></i>
                        <p>选择文件进行预览</p>
                    </div>

                    <!-- 文件信息 -->
                    <div v-else class="space-y-4">
                        <div>
                            <p class="text-sm text-neutral-600 mb-1">文件名</p>
                            <p class="font-medium truncate">{{ selectedFile.original_name }}</p>
                        </div>
                        <div>
                            <p class="text-sm text-neutral-600 mb-1">文件大小</p>
                            <p class="font-medium">{{ selectedFile.file_size }}</p>
                        </div>
                        <div>
                            <p class="text-sm text-neutral-600 mb-1">打印机</p>
                            <select v-model="selectedPrinter" class="w-full px-3 py-2 border border-neutral-300 rounded-md text-sm focus:outline-none focus:ring-1 focus:ring-primary focus:border-primary">
                                <option value="">未选择</option>
                                <option v-for="printer in printerList" :key="printer.name" :value="printer.name">
                                    {{ printer.name }} {{ printer.status ? '(在线)' : '(离线)' }}
                                </option>
                            </select>
                        </div>
                        <div>
                            <p class="text-sm text-neutral-600 mb-1">状态</p>
                            <p :class="[
                                'font-medium px-2 py-1 rounded text-sm inline-block',
                                selectedFile.status === '打印完成' ? 'bg-green-100 text-green-800' : 'bg-red-100 text-red-800'
                            ]">
                                {{ selectedFile.status }}
                            </p>
                        </div>
                        <div>
                            <p class="text-sm text-neutral-600 mb-1">打印时间</p>
                            <p class="font-medium">{{ formatDate(selectedFile.created_at) }}</p>
                        </div>

                        <!-- 打印设置 -->
                        <div class="border-t border-neutral-200 pt-4 mt-4">
                            <h4 class="font-semibold mb-3">打印设置</h4>
                            
                            <!-- 份数 -->
                            <div class="mb-4">
                                <label class="block text-sm text-neutral-600 mb-2">打印份数</label>
                                <div class="flex items-center gap-2">
                                    <button
                                        class="w-8 h-8 border border-neutral-300 bg-white rounded text-sm cursor-pointer hover:bg-neutral-50 transition-colors"
                                        @click="printCount > 1 && (printCount--)" :disabled="printCount <= 1"
                                        :class="{ 'opacity-50 cursor-not-allowed': printCount <= 1 }">
                                        -
                                    </button>
                                    <span class="min-w-[2rem] text-center">{{ printCount }}</span>
                                    <button
                                        class="w-8 h-8 border border-neutral-300 bg-white rounded text-sm cursor-pointer hover:bg-neutral-50 transition-colors"
                                        @click="printCount < 99 && (printCount++)" :disabled="printCount >= 99"
                                        :class="{ 'opacity-50 cursor-not-allowed': printCount >= 99 }">
                                        +
                                    </button>
                                </div>
                            </div>
                            <!-- 打印页码 -->
                            <div class="mb-4">
                                <label class="block text-sm text-neutral-600 mb-2">打印页码</label>
                                <input type="text" v-model="printPages" placeholder="留空表示打印全部，例如：1,2 或 1-3"
                                    class="w-full px-3 py-2 border border-neutral-300 rounded-md text-sm focus:outline-none focus:ring-1 focus:ring-primary focus:border-primary">
                                <p class="text-xs text-neutral-500 mt-1">输入页码范围，留空视为全部</p>
                            </div>
                            <!-- 布局 -->
                            <div class="mb-4">
                                <label class="block text-sm text-neutral-600 mb-2">页面布局</label>
                                <div class="space-y-2">
                                    <label class="flex items-center gap-2 text-sm cursor-pointer">
                                        <input type="radio" name="layout" value="3" v-model="printLayout" class="cursor-pointer">
                                        纵向
                                    </label>
                                    <label class="flex items-center gap-2 text-sm cursor-pointer">
                                        <input type="radio" name="layout" value="4" v-model="printLayout" class="cursor-pointer">
                                        横向
                                    </label>
                                </div>
                            </div>

                            <!-- 颜色 -->
                            <div class="mb-4">
                                <label class="block text-sm text-neutral-600 mb-2">打印颜色</label>
                                <div class="space-y-2">
                                    <label class="flex items-center gap-2 text-sm cursor-pointer">
                                        <input type="radio" name="color" value="color" v-model="printColor" class="cursor-pointer">
                                        彩色
                                    </label>
                                    <label class="flex items-center gap-2 text-sm cursor-pointer">
                                        <input type="radio" name="color" value="black" v-model="printColor" class="cursor-pointer">
                                        黑白
                                    </label>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>

                <!-- 底部按钮 -->
                <div class="p-6 border-t border-neutral-200 space-y-3">
                    <button v-if="selectedFile" @click="executePrint"
                        class="w-full px-4 py-2 bg-green-600 hover:bg-green-700 text-white rounded-lg text-sm transition-colors font-medium">
                        <i class="fa fa-print mr-2"></i> 打印
                    </button>
                    <button v-if="selectedFile" @click="downloadFile"
                        class="w-full px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg text-sm transition-colors font-medium">
                        <i class="fa fa-download mr-2"></i> 下载
                    </button>
                    <button @click="closePreviewPanel"
                        class="w-full px-4 py-2 border border-neutral-300 rounded-lg text-sm hover:bg-neutral-50 transition-colors">
                        关闭
                    </button>
                </div>
            </div>
        </div>

        <!-- 预览面板背景遮罩 -->
        <div v-if="showPreviewPanel" class="fixed inset-0 bg-black bg-opacity-30 z-30" @click="closePreviewPanel"></div>

        <!-- 页脚 -->
        <footer class="bg-white border-t border-neutral-200 mt-12 py-6">
            <div class="container mx-auto px-4">
                <div class="text-center text-neutral-500 text-sm">
                    <p>© 2025 打印助手 - 安全、高效的在线打印解决方案</p>
                    <p class="mt-2">支持多种文件格式，保护您的文件安全</p>
                </div>
            </div>
        </footer>

        <!-- 确认对话框 -->
        <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50"
            :class="{ hidden: !showConfirmDialog }">
            <div class="bg-white rounded-xl shadow-lg p-6 max-w-md w-full mx-4">
                <h3 class="text-lg font-semibold mb-2">确认清空历史记录</h3>
                <p class="text-neutral-600 mb-4">此操作将永久删除所有历史记录，是否继续？</p>
                <div class="flex justify-end space-x-3">
                    <button @click="showConfirmDialog = false"
                        class="px-4 py-2 border border-neutral-300 rounded-lg text-sm hover:bg-neutral-50 transition-colors">
                        取消
                    </button>
                    <button @click="clearAllHistory"
                        class="px-4 py-2 bg-red-500 hover:bg-red-600 text-white rounded-lg text-sm transition-colors">
                        确认清空
                    </button>
                </div>
            </div>
        </div>

        <!-- 通知提示 -->
        <div class="fixed bottom-4 right-4 bg-white shadow-lg rounded-lg p-4 z-50 transition-all duration-300" :class="{
            'translate-y-4 opacity-0 hidden': !showNotification,
            'translate-y-0 opacity-100': showNotification
        }">
            <div class="flex items-center space-x-3">
                <span class="p-2 rounded-full" :class="[
                    notificationType === 'success' ? 'bg-green-100 text-green-600' : 'bg-red-100 text-red-600'
                ]">
                    <i class="fa"
                        :class="notificationType === 'success' ? 'fa-check-circle' : 'fa-exclamation-circle'"></i>
                </span>
                <div>
                    <h4 class="font-medium">{{ notificationTitle }}</h4>
                    <p class="text-sm text-neutral-600">{{ notificationMessage }}</p>
                </div>
                <button @click="closeNotification"
                    class="ml-auto text-neutral-400 hover:text-neutral-600 transition-colors">
                    <i class="fa fa-times"></i>
                </button>
            </div>
        </div>
    </div>
</template>

<script setup>
import { ref, computed, onMounted, watch } from 'vue';
import { get_all, print, getprinter } from '@/api/api';
import { formatDate, parseArrayDate } from '@/utils/formatDate';

// 响应式数据
const totalPrints = ref(0);
const monthlyPrints = ref(0);
const completedPrints = ref(0);
const successRate = ref('0%');
const totalSize = ref('0 MB');
const avgSize = ref('0 KB');

const searchKeyword = ref('');
const showFilterPanel = ref(false);
const showConfirmDialog = ref(false);
const showNotification = ref(false);
const notificationTitle = ref('');
const notificationMessage = ref('');
const notificationType = ref('success');
const errorMsg = ref(''); // 新增：接口错误提示

// 预览和打印相关
const showPreviewPanel = ref(false);
const selectedFile = ref(null);
const selectedPrinter = ref('');
const printerList = ref([]);
const printCount = ref(1);
const printLayout = ref('3');
const printColor = ref('color');
const printPages = ref('');

// 筛选条件（适配新状态值）
const filterStatus = ref('all');
const filterDateFrom = ref('');
const filterDateTo = ref('');
const filterFileType = ref('all');

// 排序相关（排序字段改为新数据的真实字段）
const sortField = ref('created_at'); // 默认按创建时间排序
const sortOrder = ref('desc');

// 分页相关
const currentPage = ref(1);
const pageSize = ref(10);

// 历史数据
const historyData = ref([]);

// 初始化
onMounted(() => {
    fetchPrintQueue();
    updateStatistics();

    // 初始化日期筛选（本月1号到今天）
    const now = new Date();
    const firstDay = new Date(now.getFullYear(), now.getMonth(), 1);
    filterDateFrom.value = firstDay.toISOString().split('T')[0];
    filterDateTo.value = now.toISOString().split('T')[0];
});

// 工具函数：从文件名提取文件类型（后缀）
const getFileTypeFromName = (fileName) => {
    if (!fileName) return 'other';
    const ext = fileName.split('.').pop().toLowerCase();
    const fileTypeMap = {
        'pdf': 'pdf',
        'doc': 'doc',
        'docx': 'doc',
        'jpg': 'jpg',
        'jpeg': 'jpg',
        'png': 'png',
        'txt': 'txt'
    };
    return fileTypeMap[ext] || 'other';
};

// 计算属性 - 分页后的历史数据
const paginatedHistory = computed(() => {
    const start = (currentPage.value - 1) * pageSize.value;
    const end = start + pageSize.value;
    return filteredHistory.value.slice(start, end);
});

// 计算属性 - 总页数
const totalPages = computed(() => {
    return Math.ceil(filteredHistory.value.length / pageSize.value) || 1;
});

// 工具函数：文件大小字符串转字节（用于排序和统计）
const parseSizeToBytes = (sizeStr) => {
    if (!sizeStr) return 0;
    const [num, unit] = sizeStr.split(' ');
    const size = parseFloat(num);
    switch (unit.toUpperCase()) {
        case 'KB': return size * 1024;
        case 'MB': return size * 1024 * 1024;
        case 'GB': return size * 1024 * 1024 * 1024;
        default: return size;
    }
};

// 工具函数：字节转友好显示（用于统计）
const formatBytesToFriendly = (bytes) => {
    if (bytes < 1024) return `${bytes} B`;
    if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
    return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
};

// 方法 - 获取排序图标类名
const getSortIcon = (field) => {
    if (sortField.value !== field) {
        return 'fa-sort text-neutral-400';
    }
    return sortOrder.value === 'asc' ? 'fa-sort-asc text-primary' : 'fa-sort-desc text-primary';
};

// 方法 - 排序表格（排序字段改为新数据的真实字段）
const sortTable = (field) => {
    if (sortField.value === field) {
        sortOrder.value = sortOrder.value === 'asc' ? 'desc' : 'asc';
    } else {
        sortField.value = field;
        sortOrder.value = 'asc';
    }
};

// 方法 - 切换筛选面板
const toggleFilterPanel = () => {
    showFilterPanel.value = !showFilterPanel.value;
};

// 方法 - 应用筛选
const applyFilter = () => {
    currentPage.value = 1;
    triggerNotification('筛选成功', '已应用筛选条件', 'success');
};

// 方法 - 重置筛选
const resetFilter = () => {
    filterStatus.value = 'all';
    filterDateFrom.value = '';
    filterDateTo.value = '';
    filterFileType.value = 'all';
    searchKeyword.value = '';
    currentPage.value = 1;
};

// 方法 - 更改页码
const changePage = (page) => {
    if (page < 1 || page > totalPages.value) return;
    currentPage.value = page;
};

// 方法 - 获取显示范围
const getShowingRange = () => {
    const start = filteredHistory.value.length === 0 ? 0 : (currentPage.value - 1) * pageSize.value + 1;
    const end = Math.min(currentPage.value * pageSize.value, filteredHistory.value.length);
    return `${start}-${end}`;
};

// 方法 - 显示通知
const triggerNotification = (title, message, type = 'success') => {
    notificationTitle.value = title;
    notificationMessage.value = message;
    notificationType.value = type;
    showNotification.value = true;

    setTimeout(() => {
        closeNotification();
    }, 3000);
};

// 方法 - 关闭通知
const closeNotification = () => {
    showNotification.value = false;
};

// 方法 - 查看详情
const viewDetail = (item) => {
    console.log('查看详情:', item);
    triggerNotification('查看详情', `正在查看 ${item.original_name} 的详情`, 'success');
};

// 方法 - 预览文件
const previewFile = async (item) => {
    if (!item.file_path) {
        triggerNotification('无法预览', '该文件不可用或已删除', 'error');
        return;
    }
    selectedFile.value = item;
    selectedPrinter.value = item.printer || '';
    showPreviewPanel.value = true;
    printCount.value = 1;
    printLayout.value = '3';
    printColor.value = 'color';
    printPages.value = '';
    
    // 加载打印机列表
    await fetchPrinterList();
};

// 方法 - 关闭预览面板
const closePreviewPanel = () => {
    showPreviewPanel.value = false;
    selectedFile.value = null;
};

// 方法 - 重新打印
const printAgain = async (item) => {
    if (!item.file_path) {
        triggerNotification('无法打印', '该文件不可用或已删除', 'error');
        return;
    }
    selectedFile.value = item;
    selectedPrinter.value = item.printer || '';
    showPreviewPanel.value = true;
    printCount.value = 1;
    printLayout.value = '3';
    printColor.value = 'color';
    printPages.value = '';
    
    // 加载打印机列表
    await fetchPrinterList();
};

// 方法 - 执行打印
const executePrint = async () => {
    if (!selectedFile.value) return;
    
    try {
        // 构建后端 PrintValue 结构体对应的参数
        const printParams = {
            fileName: selectedFile.value.original_name,
            printer: selectedPrinter.value || selectedFile.value.printer || '',
            paperSize: selectedFile.value.paper_size || 'A4',
            layout: printLayout.value,
            customPages: printPages.value.trim() === '' ? '' : printPages.value.trim()
        };

        console.log('发送打印请求:', printParams);
        triggerNotification('打印中', `正在打印文件: ${selectedFile.value.original_name}`, 'success');
        
        // 调用后端打印接口
        try {
            const res = await print(printParams);
            console.log('打印接口返回', res);
            if (res && res.status === 200) {
                // 同时更新 historyData 和 selectedFile，确保数据一致性
                if (selectedFile.value) {
                    const idx = historyData.value.findIndex(i => i.id === selectedFile.value.id);
                    if (idx !== -1) {
                        historyData.value[idx].status = '打印完成';
                    }
                    // 关键：同时更新当前 selectedFile.value 的状态，预览面板会立即变绿
                    selectedFile.value.status = '打印完成';
                }
                updateStatistics();
                triggerNotification('打印完成', '历史记录已更新', 'success');
                // 异步刷新列表以防后端有其它变动
                fetchPrintQueue();
            }
        } catch (err) {
            console.error('调用打印接口失败', err);
            triggerNotification('打印失败', '请求打印接口出错', 'error');
        }
        
        closePreviewPanel();
    } catch (error) {
        triggerNotification('打印失败', `${error.message}`, 'error');
        console.error('打印失败:', error);
    }
};

// 方法 - 下载文件
const downloadFile = () => {
    if (!selectedFile.value || !selectedFile.value.file_path) {
        triggerNotification('无法下载', '该文件不可用或已删除', 'error');
        return;
    }
    
    try {
        // 创建一个临时的a标签进行下载
        const link = document.createElement('a');
        link.href = selectedFile.value.file_path;
        link.download = selectedFile.value.original_name;
        document.body.appendChild(link);
        link.click();
        document.body.removeChild(link);
        
        triggerNotification('下载中', `正在下载: ${selectedFile.value.original_name}`, 'success');
    } catch (error) {
        triggerNotification('下载失败', `${error.message}`, 'error');
        console.error('下载失败:', error);
    }
};

const deleteRecord = (id) => {
    historyData.value = historyData.value.filter(item => item.id !== id);
    updateStatistics();
    triggerNotification('删除成功', '已删除该条历史记录', 'success');
};

// 方法 - 清空所有历史
const clearAllHistory = () => {
    historyData.value = [];
    showConfirmDialog.value = false;
    updateStatistics();
    triggerNotification('清空成功', '已清空所有历史记录', 'success');
};

// 请求真实历史数据
const fetchPrintQueue = async () => {
    try {
        const res = await get_all();
        if (res.success && res.code == 200) {
            historyData.value = res.data;
            triggerNotification('数据加载成功', `共加载 ${historyData.value.length} 条历史记录`, 'success');
        } else {
            errorMsg.value = '获取打印队列失败：' + (res.message || '服务端异常');
            triggerNotification('加载失败', errorMsg.value, 'error');
        }
    } catch (err) {
        console.error('请求打印队列出错：', err);
        errorMsg.value = '网络异常，请检查后端服务是否正常';
        triggerNotification('加载失败', errorMsg.value, 'error');
    }
};

// 获取打印机列表
const fetchPrinterList = async () => {
    try {
        const res = await getprinter();
        if (res.success && res.code === 200) {
            printerList.value = res.data || [];
            console.log('打印机列表:', printerList.value);
        } else {
            console.warn('获取打印机列表失败:', res.message);
        }
    } catch (err) {
        console.error('获取打印机列表出错：', err);
    }
};


// 筛选后的历史数据
const filteredHistory = computed(() => {
    let filtered = [...historyData.value];

    // 搜索筛选（不变）
    if (searchKeyword.value) {
        filtered = filtered.filter(item =>
            item.original_name.toLowerCase().includes(searchKeyword.value.toLowerCase())
        );
    }

    // 状态筛选（不变）
    if (filterStatus.value !== 'all') {
        filtered = filtered.filter(item => item.status === filterStatus.value);
    }

    // 文件类型筛选（不变）
    if (filterFileType.value !== 'all') {
        filtered = filtered.filter(item =>
            getFileTypeFromName(item.original_name) === filterFileType.value
        );
    }

    // 修复日期筛选：适配数组格式的 created_at
    if (filterDateFrom.value) {
        const fromDate = new Date(filterDateFrom.value);
        filtered = filtered.filter(item => {
            let itemDate;
            if (Array.isArray(item.created_at)) {
                itemDate = parseArrayDate(item.created_at);
            } else {
                itemDate = new Date(item.created_at);
            }
            // 只比较日期部分（忽略时分秒）
            const itemDateOnly = new Date(itemDate.getFullYear(), itemDate.getMonth(), itemDate.getDate());
            return itemDateOnly >= fromDate;
        });
    }
    if (filterDateTo.value) {
        const toDate = new Date(filterDateTo.value);
        // 给 toDate 加一天，确保包含当天的所有时间
        toDate.setDate(toDate.getDate() + 1);
        filtered = filtered.filter(item => {
            let itemDate;
            if (Array.isArray(item.created_at)) {
                itemDate = parseArrayDate(item.created_at);
            } else {
                itemDate = new Date(item.created_at);
            }
            const itemDateOnly = new Date(itemDate.getFullYear(), itemDate.getMonth(), itemDate.getDate());
            return itemDateOnly < toDate;
        });
    }

    // 修复排序：适配数组格式的 created_at
    filtered.sort((a, b) => {
        // 特殊处理：文件大小排序（不变）
        if (sortField.value === 'file_size') {
            const sizeA = parseSizeToBytes(a.file_size);
            const sizeB = parseSizeToBytes(b.file_size);
            return sortOrder.value === 'asc' ? sizeA - sizeB : sizeB - sizeA;
        }
        // 修复日期排序：处理数组格式
        if (sortField.value === 'created_at') {
            const dateA = Array.isArray(a.created_at) ? parseArrayDate(a.created_at) : new Date(a.created_at);
            const dateB = Array.isArray(b.created_at) ? parseArrayDate(b.created_at) : new Date(b.created_at);
            return sortOrder.value === 'asc' ? dateA - dateB : dateB - dateA;
        }
        // 其他字段排序（不变）
        if (sortOrder.value === 'asc') {
            return a[sortField.value]?.toString().localeCompare(b[sortField.value]?.toString()) || 1;
        } else {
            return b[sortField.value]?.toString().localeCompare(a[sortField.value]?.toString()) || 1;
        }
    });

    return filtered;
});

// 更新统计数据
const updateStatistics = () => {
    const data = [...historyData.value];
    totalPrints.value = data.length;

    // 修复：本月打印次数（适配数组日期）
    const now = new Date();
    const currentYear = now.getFullYear();
    const currentMonth = now.getMonth() + 1; // 月份从0开始，转为1-12
    monthlyPrints.value = data.filter(item => {
        let itemDate;
        if (Array.isArray(item.created_at)) {
            itemDate = parseArrayDate(item.created_at);
        } else {
            itemDate = new Date(item.created_at);
        }
        return itemDate.getFullYear() === currentYear && (itemDate.getMonth() + 1) === currentMonth;
    }).length;

    // 已完成打印次数（不变）
    completedPrints.value = data.filter(item => item.status === '打印完成').length;

    // 成功率（不变）
    successRate.value = data.length === 0
        ? '0%'
        : `${Math.round((completedPrints.value / data.length) * 100)}%`;

    // 总文件大小和平均大小（不变）
    let totalBytes = 0;
    data.forEach(item => {
        totalBytes += parseSizeToBytes(item.file_size);
    });
    totalSize.value = formatBytesToFriendly(totalBytes);
    avgSize.value = data.length === 0
        ? '0 KB'
        : formatBytesToFriendly(totalBytes / data.length);
};

// 监听历史数据变化，自动更新统计
watch(historyData, updateStatistics, { deep: true });
</script>

<!-- 补充缺失的全局样式 -->
<style>
/* 表格表头hover效果 */
.table-header-hover {
    @apply cursor-pointer hover:text-primary transition-colors;
}

/* 自定义阴影（tailwind.config.js 中已配置，这里补充全局引用） */
.shadow-hover {
    box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04);
}
</style>