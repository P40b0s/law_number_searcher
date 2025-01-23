<template lang="pug">
n-tooltip
  template(#trigger)
    label.switcher
      input(type="checkbox" v-model="checked")
      .switcher__indicator
  span Выбор светлой или темной темы
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';
import { useTheme } from '../composables/useTheme';
import { NTooltip } from 'naive-ui';
const { light_theme, dark_theme } = useTheme();
//true - dark
//false - light
const checked = ref(false);
const theme = localStorage.getItem("theme");
const theme_name = ref("");
const set_dark = () =>
{
    const body = document.body;
    body.setAttribute("class", "dark");
	  theme_name.value = "темная"
    dark_theme();
    localStorage.setItem('theme', "dark");
}
const set_light = () =>
{
    const body = document.body;
    body.setAttribute("class", "light");
    body.setAttribute("class", "--vs-input-bg: #a22916;")
	  theme_name.value = "светлая"
    light_theme();
    localStorage.setItem('theme', "light");
}
if(theme)
{
    if(theme == "dark")
    {
        checked.value = true;
        set_dark();
    }
    else
    {
        set_light()
    }
}
else
{
    checked.value = true;
}

watch(checked, (n, o) =>
{
    if(n)
    {
        set_dark()
    }
    else
    {
        set_light();
    }
})

</script>
<style scoped>
/* .text
{
  color: var(--background);
  -webkit-filter: invert(100%);
  filter: invert(100%);
  margin-top: 6px;
} */
.switcher {
  position: relative;
  display: flex;
  flex-direction: column;
  align-content: center;
  cursor: pointer;
  width: 80px;
  height: 30px;
  line-height: 20px;
  /* margin: 5px; */
  font-size: 16px;
  text-align: right;
  user-select: none;
  opacity: 1;
  input {
    display: none;
  }
}

.switcher__indicator { 
  &::after {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    height: 30px;
    width: 30px;
    background-color: black;
    border-radius: 50%;
    transition: all .3s ease;
    animation-name: pulsein;
    animation-duration: .3s;
    content: url('../images/sun.svg');
  }
  
  &::before {
    content: '';
    position: absolute;
    top: 10px;
    left: 0;
    width: 60px;
    height: 12px;
    background-color: black;
    border-radius: 10px;
    transition: all .3s ease;
  }
  input:checked {
   
    content: url('../images/moon.svg');
  }
  
  input:checked + &::after {
    /* -webkit-filter: invert(100%);
    filter: invert(100%); */
    transform: translateX(40px);
    animation-name: pulseout;
    animation-duration: .3s;
    content: url('../images/moon.svg');
  }
  
  input:checked + &::before {
    filter: invert(100%);
    -webkit-filter: invert(100%);
  }
  
  input:disabled + &::after,
  input:disabled + &::before {
    filter: invert(100%);
    -moz-filter: invert(100%);
    -webkit-filter: invert(100%);
    content: url('../images/moon.svg');
  }
}

/* @keyframes pulsein {
  0%, 100% {
    top: 0px;
	opacity: 1;
    height: 30px;
    width: 30px;
  }
  50% {
	opacity: 0;
    top: -10px;
    height: 38px;
    width: 42px;
  }
}

@keyframes pulseout {
  0%, 100% {
    top: 0px;
	opacity: 1;
    height: 30px;
    width: 30px;
  }
  50% {
    top: 0px;
	opacity: 0;
    height: 38px;
    width: 42px;
  }
} */
@keyframes pulsein {
  0%, 100% {
	opacity: 1;
  }
  50% {
	opacity: 0.8;
  }
}

@keyframes pulseout {
  0%, 100% {
	opacity: 1;
  }
  50% {
	opacity: 0.8;
  }
}
</style>