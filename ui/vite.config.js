import { fileURLToPath, URL } from 'node:url'
import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import vueDevTools from 'vite-plugin-vue-devtools'
export default defineConfig({
  plugins: [
    vue(),
    vueDevTools(),
  ],
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url))
    },
  },
  optimizeDeps: {
    // 如果有其他需要预构建的依赖，可以在这里添加
    // 例如：include: ['vue', 'vue-router', 'pinia'],
    esbuildOptions: {
      define: {
        global: 'globalThis'
      }
    }
  },
  define: {
    'process.env': {},
    global: 'globalThis'
  },
  // 添加构建配置来优化包大小
  build: {
    chunkSizeWarningLimit: 1000,
    rollupOptions: {
      output: {
        manualChunks(id) {
          if (id.includes('node_modules')) {
            // Vue 核心
            if (id.includes('vue') && !id.includes('vue-router') && !id.includes('pinia')) {
              return 'vue-core'
            }
            // Vue Router
            if (id.includes('vue-router')) {
              return 'vue-router'
            }
            // Pinia
            if (id.includes('pinia')) {
              return 'pinia'
            }
            // Element Plus - 按需加载的可以单独分包
            if (id.includes('element-plus')) {
              if (id.includes('element-plus/es/components')) {
                // Element Plus 的组件单独分块，便于按需加载
                const match = id.match(/element-plus\/es\/components\/([^/]+)/)
                if (match) {
                  return `element-plus-${match[1]}`
                }
              }
              return 'element-plus-core'
            }
            // 其他大体积库
            if (id.includes('lodash') || id.includes('lodash-es')) {
              return 'lodash'
            }
            if (id.includes('axios')) {
              return 'axios'
            }
            if (id.includes('echarts')) {
              return 'echarts'
            }
            if (id.includes('xlsx') || id.includes('sheetjs')) {
              return 'xlsx'
            }
            // 默认归为 vendor
            return 'vendor'
          }
        },
        // 设置 chunk 文件名格式
        chunkFileNames: 'assets/[name]-[hash].js',
        entryFileNames: 'assets/[name]-[hash].js',
        assetFileNames: 'assets/[name]-[hash].[ext]'
      }
    }
  }
})