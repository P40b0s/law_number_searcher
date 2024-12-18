import { defineConfig } from '@rsbuild/core';
import { pluginVue } from '@rsbuild/plugin-vue';
import { pluginPug } from '@rsbuild/plugin-pug';
import { pluginSass } from '@rsbuild/plugin-sass';
export default defineConfig({
    plugins: [pluginVue(), pluginPug(), pluginSass()],
    resolve: {
        alias: { // 路径别名
            '@/*': './src/*',
            '@styles': './src/styles',
            '@types': './src/@types',
            '@images': './src/assets/images',
            '@components': './src/components',
            '@services': './src/services',
            '@tauri': './src/tauri'
          }
    },
    source: {
        entry: {
            index: "./src/main.ts"
        },
        
    },
    server: {
        port: 8080,
        strictPort: true,
    },
    tools: {
        rspack: {
            watchOptions: {
                ignored: "**/src-tauri/**"
            }
        }
    },
    html: {
        template: 'index.html',
        //favicon: './public/favicon.ico',
        meta: {
          charset: 'UTF-8',
          viewport: 'width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=0, minimum-scale=1.0, viewport-fit=cover'
        },
        title: '普惠交易平台'
      },
});