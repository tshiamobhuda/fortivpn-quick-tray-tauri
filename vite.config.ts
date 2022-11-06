import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';
import eslintPlugin from 'vite-plugin-eslint';
import { quasar, transformAssetUrls } from '@quasar/vite-plugin';

// https://vitejs.dev/config/
export default defineConfig({
    clearScreen: false,
    server: {
        strictPort: true,
    },
    envPrefix: [ 'VITE_','TAURI' ],
    build: {
        target: [ 'es2021' ],
        minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
        sourcemap: !!process.env.TAURI_DEBUG,
    },
    plugins: [
        vue({
            template: { transformAssetUrls },
        }),
        eslintPlugin(),
        quasar({
            sassVariables: 'src/assets/quasar-variables.sass',
        }),
    ],
});
