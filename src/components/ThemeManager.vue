<template lang="pug">
.switch
	input#switch.switch__input(name="switch" type="checkbox" v-model="checked")
	label.switch__label(for="switch") asdasdasdas
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';
//true - dark
//false - light
const checked = ref(false);
const theme = localStorage.getItem("theme");
const set_dark = () =>
{
    const body = document.body;
    body.setAttribute("class", "dark");
    localStorage.setItem('theme', "dark");
    console.log("dark mode on")
}
const set_light = () =>
{
    const body = document.body;
    body.setAttribute("class", "light");
    localStorage.setItem('theme', "light");
    console.log("dark mode on")
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
    console.log(n)
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
<style>
.switch {
		position: relative;
		&__input {
			position: absolute;
			top: 0;
			right: 0;
			left: 0;
			bottom: 0;
			width: 100%;
			height: 100%;
			margin: 0;
			opacity: 0;
			z-index: 1;
		}

		&__label {
			display: block;
			position: relative;
			width: 100px;
			height: 50px;
			background-color: var(--background-color);
			border-radius: 25px;
			transition: 0.4s;

			&::before {
				display: flex;
				align-items: center;
				justify-content: center;
				position: absolute;
				top: 0;
				right: auto;
				left: 0;
				bottom: 0;
				width: 50px;
				height: 50px;
				border-radius: 100%;
				border: 2px var(--primary-color) solid;
				background-color: var(--background-color);
				color: var(--primary-color);
				transition: 0.4s;
				content: "\f185";
				font-family: "Font Awesome 5 Free";
				font-size: 30px;
				font-weight: 900;
				box-sizing: border-box;
			}
		}

		&__input:checked + .switch__label {
			background-color: var(--background-color);

			&::before {
				left: calc(100% - 50px);
				border-color: var(--primary-color);
				background-color: var(--background-color);
				color: var(--primary-color);
				content: "\f186";
			}
		}
	}

</style>