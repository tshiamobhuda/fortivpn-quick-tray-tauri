import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';
import eslintPlugin from 'vite-plugin-eslint';

// https://vitejs.dev/config/
export default defineConfig({
    clearScreen: false,
    server: {
        strictPort: true,
    },
    envPrefix: [ 'VITE_', 'TAURI' ],
    build: {
        target: [ 'es2021' ],
        minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
        sourcemap: !!process.env.TAURI_DEBUG,
    },
    plugins: [ vue(), eslintPlugin() ],
});
