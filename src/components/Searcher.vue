<script lang="ts">
import { NButton, NInputNumber} from 'naive-ui';
import { searcher_commands } from '../tauri/commands';
import { ref } from 'vue';
import OrganSelector from './dictionaries/organ_selector'
import TypeSelector from './dictionaries/type_selector'
import { type Result } from '../tauri/abstract';
</script>
<template lang="pug">
organ-selector(@select="select_organ" placeholder="Выберите принявший орган")
type-selector(:disabled="tp_disabled" paceholder="Выберите вид документа" @select="select_type", :selected_organ="selected_organ")
n-input-number(
        :disabled="yr_disabled"
      v-model:value="year"
      placeholder="Год за который будет осуществлятся поиск"
      :min="2011"
      :max="current_year"
)
</template>


<script setup lang="ts">
const selected_organ = ref<Dictionary>();
const selected_type = ref<Dictionary>();
const tp_disabled = ref(true);
const yr_disabled = ref(true);
const current_year = new Date().getFullYear();
const year = ref(current_year);
const select_organ = (org: Dictionary) =>
{
    selected_type.value = undefined;
    selected_organ.value = org;
    tp_disabled.value = false;
}
const select_type = (tp: Dictionary) =>
{
    selected_type.value = tp;
    yr_disabled.value = false;
}
</script>
<style>
</style>