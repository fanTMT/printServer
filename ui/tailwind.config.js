// /** @type {import('tailwindcss').Config} */
// export default {
//   content: [],
//   theme: {
//     extend: {},
//   },
//   plugins: [],
// }

/** @type {import('tailwindcss').Config} */
module.exports = {
  // 核心：指定 Tailwind 生效的文件路径（必须配置，否则样式不生效）
  content: [
    "./index.html",          // 根目录 HTML（Vite 项目）
    "./src/**/*.{vue,js,ts}", // src 下所有 Vue/JS/TS 文件
    "./src/components/**/*.vue" // 组件目录（可选，已包含在上面）
  ],
  theme: {
    // 自定义主题（可选，如颜色、字体、阴影）
    extend: {
      colors: {
        // 示例：自定义主色（可根据项目需求修改）
        primary: '#4F46E5',
        secondary: '#10B981',
        // 自定义中性色
        neutral: {
          100: '#F3F4F6',
          200: '#E5E7EB',
          300: '#D1D5DB',
        }
      },
      // 自定义阴影
      boxShadow: {
        'card': '0 10px 15px -3px rgba(0, 0, 0, 0.05)',
      }
    },
  },
  plugins: [], // 插件（如 @tailwindcss/forms，按需安装）
}