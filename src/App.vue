<script setup lang="ts">
import Header from './components/Header.vue'
import ThemeManager from './components/ThemeManager.vue';
import MainView from './views/main.ts';
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { darkTheme, lightTheme, NConfigProvider, NNotificationProvider, NModalProvider, type GlobalThemeOverrides, NGlobalStyle} from 'naive-ui';
import { useTheme } from './composables/useTheme.ts';
import { useSelectHeight } from './composables/useSelechtHeight.ts';
import Searcher from './components/Searcher.vue';
const greetMsg = ref("");
const name = ref("");
const { theme } = useTheme();
const themeOverrides: GlobalThemeOverrides = {
    Scrollbar: {
        width: '8px',
        railInsetHorizontal: '6px 6px 6px auto',
        borderRadius: 2,
    },
    }

//:theme-overrides="themeOverrides"
</script>
<template lang="pug">
n-config-provider(:theme="theme" :theme-overrides="themeOverrides")
  n-notification-provider
    n-modal-provider
      .container
        .header
          .header-left 
          .header-right
            theme-manager
        .main-content
          searcher
      n-global-style
</template>

<style>

.container {
  display: grid;
  height: 100%;
  width: 100%;
  grid-template-columns: 5px 1fr 10px 5px; 
  grid-template-rows: minmax(30px 50px) 50vh; 
  gap: 0px 0px; 
  font-family: 'Source Code Pro';
  grid-template-areas: 
    "header header header header"
    ". main-content main-content .";
    
}
.header 
{
  grid-area: header;
  background-color: var(--n-card-color);
  display: flex;
  align-items: center;
  flex-direction: row;
  width: 100%;
  height: 100%;
}
.header-left
{
  flex-grow: 3;
  width: 100%;
}
.header-right
{
  display: flex;
  align-items: center;
  height: 100%;
  padding: 5px;
}
.main-content 
{ 
  grid-area: main-content;
  height: 100%;
  width: 100%;
}
::-webkit-scrollbar 
{
  width: 10px;
}
 
::-webkit-scrollbar-thumb 
{
  border-radius: 3px;
  background-color: var(--n-card-color);
  background-color: #00FF01;
  color: #00FF01;
  -webkit-box-shadow: 0 0 1px rgba(255,255,255,.5);
}


</style>