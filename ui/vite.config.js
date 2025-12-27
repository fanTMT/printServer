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
  // 移除 pdfjs-dist 的优化依赖，因为它已通过 CDN 引入
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
    // 提高块大小警告限制
    chunkSizeWarningLimit: 1000,

    // Rollup 配置来优化分包
    rollupOptions: {
      // 将 pdfjs-dist 标记为外部依赖，避免打包
      external: ['pdfjs-dist'],

      output: {
        // 手动分包策略
        manualChunks(id) {
          // 分离 node_modules 中的依赖
          if (id.includes('node_modules')) {
            // Vue 相关
            if (id.includes('vue')) {
              return 'vue-vendor'
            }
            // Element Plus 相关
            if (id.includes('element-plus')) {
              return 'element-plus-vendor'
            }
            // 其他第三方库
            return 'vendor'
          }
        }
      }
    }
  }
})