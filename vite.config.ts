import path from 'node:path'
import process from 'node:process'
import tailwindcss from '@tailwindcss/vite'
import vue from '@vitejs/plugin-vue'
import AutoImport from 'unplugin-auto-import/vite'
import VueRouter from 'unplugin-vue-router/vite'
import { defineConfig } from 'vite'
import Inspect from 'vite-plugin-inspect'

import replace from './vite-plugins/placeholder-replace'

const host = process.env.TAURI_DEV_HOST

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [
    replace({
      handles: [
        { src: '[log-position-placeholder]', target: '[fileName]:[line]' },
        // { src: 'console\\.log\\(', target: 'console.log("调试输出:  [fileName]:[row] ==> " +' },
        // { src: 'tauriLog\\.debug\\(', target: 'tauriLog.debug("调试输出:  [fileName]:[row] ==> " +' },
      ], // 设置占位符数组
    }),
    AutoImport({
      imports: [
        'vue',
        '@vueuse/core',
        'vue-router',
      ],
      dts: 'src/auto-imports.d.ts',
    }),
    vue(),
    tailwindcss() as any,
    VueRouter({
      dts: 'src/typed-router.d.ts',
      routesFolder: [
        {
          src: 'src/pages',
          path: '',
        },
      ],
      // files to exclude from the scan
      exclude: ['**/components/**'],
    }),
    Inspect(),
  ],

  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
    },
  },

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1320,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: 'ws',
          host,
          port: 1321,
        }
      : undefined,
    watch: {
      // 3. tell vite to ignore watching `src-tauri`
      ignored: ['**/src-tauri/**'],
    },
  },
}))
