<script setup lang="ts">
import Header from './components/Header.vue'
import ThemeManager from './components/ThemeManager.vue';
import MainView from './views/main.ts';
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { darkTheme, lightTheme, NConfigProvider, NGlobalStyle} from 'naive-ui';
import { useTheme } from './composables/useTheme.ts';
const greetMsg = ref("");
const name = ref("");
const { theme } = useTheme();
async function greet() 
{
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg.value = await invoke("greet", { name: name.value });
}


</script>
<template lang="pug">
n-config-provider(:theme="theme")
  main.container
    .header
      .header-left
      .header-right
        theme-manager
    .main-content
      MainView
    .footer footer
  n-global-style
</template>

<style>
/* .container 
{
  margin: 0;
  display: flex;
  flex-direction: column;
  justify-content: center;
  justify-items: center;
  text-align: center;
} */
.container {
  display: grid; 
  grid-template-columns: 5px 1fr auto; 
  grid-template-rows: minmax(30px 50px) 1fr min-content; 
  gap: 0px 0px; 
  font-family: 'Source Code Pro';
  grid-template-areas: 
    "header header header"
    ". main-content right-content"
    ". footer footer"; 
}
.header 
{
  grid-area: header;
  background-color: var(--n-card-color);
  display: flex;
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
  height: 40px;
}
.footer { grid-area: footer; }
.right-content { grid-area: right-content; }
.main-content { grid-area: main-content; }

#greet-input {
  margin-right: 5px;
}

/* @media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
} */
/* #app 
{
  background-color: var(--background-color);
  transition: background-color 2.3s ease, color 2.3s ease;
} */

</style>