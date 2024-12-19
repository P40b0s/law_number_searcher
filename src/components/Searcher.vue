<script lang="ts">
import { NButton, NInputNumber} from 'naive-ui';
import { searcher_commands } from '../tauri/commands';
import { computed, ref } from 'vue';
import OrganSelector from './dictionaries/organ_selector'
import TypeSelector from './dictionaries/type_selector'
import { type Result } from '../tauri/abstract';
</script>
<template lang="pug">
organ-selector(@select="select_organ" placeholder="Выберите принявший орган")
type-selector(:disabled="tp_disabled" paceholder="Выберите вид документа" @select="select_type", :selected_organ="selected_organ")
n-input-number(
      v-model:value="year"
      placeholder="Год за который будет осуществлятся поиск"
      :min="2011"
      :max="current_year"
)
n-button(@click="start_search" :disabled="btn_disabled") Поиск
template(v-for="n in numbers")
    div {{n}}
</template>


<script setup lang="ts">
const selected_organ = ref<Dictionary|null>(null);
const selected_type = ref<Dictionary|null>(null);
const tp_disabled = computed(()=> !(selected_organ.value != null));
const btn_disabled = computed(()=> !(selected_organ.value != null && selected_type.value != null));
const current_year = new Date().getFullYear();
const year = ref(current_year);
const numbers = ref<string[]>([]);
const select_organ = (org: Dictionary|null) =>
{
    selected_organ.value = org;
}
const select_type = (tp: Dictionary|null) =>
{
    console.log(tp);
    selected_type.value = tp;
}
const start_search = async () =>
{
    const n = await searcher_commands.get_exists_numbers(selected_organ.value?.id as string, selected_type.value?.id as string, year.value);
    numbers.value = n.get_value();
}
</script>
<style>
</style>