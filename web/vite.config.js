import { defineConfig } from 'vite';
import pkg from 'vite-plugin-monaco-editor';
const monacoEditorPlugin = pkg.default || pkg;

export default defineConfig({
  root: 'src',
  build: {
    outDir: '../dist',
    emptyOutDir: true,
  },
  plugins: [
    monacoEditorPlugin({
      languageWorkers: ['json', 'typescript'],
      globalAPI: true
    })
  ],
  server: {
    port: 8000,
    proxy: {
      '/api': {
        target: 'http://localhost:3000',
        changeOrigin: true,
      },
      '/ws': {
        target: 'ws://localhost:3000',
        ws: true,
      },
    },
  },
  optimizeDeps: {
    include: [
      'monaco-editor/esm/vs/editor/editor.api',
      '@xterm/xterm',
      '@xterm/addon-fit'
    ],
  },
  define: {
    global: 'globalThis',
  },
});