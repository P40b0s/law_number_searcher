<script lang="ts">
import { NButton, NCheckbox, NInputNumber, NInput, NScrollbar, NModal, NList, NListItem, NThing, NTag, NSpace, NTooltip, NIcon, NText, useNotification} from 'naive-ui';
import { searcher_commands } from '../tauri/commands';
import { computed, h, ref } from 'vue';
import { type Result } from '../tauri/abstract';
import { type Number} from '../@types/number';
import { RefreshOutlined } from "@vicons/material";
import Loader from './loaders/Loader.vue'
</script>
<template lang="pug">
n-tooltip(style="max-width:200px" v-if="props.alternative_publication_site")
  template(#trigger)
    n-button.refresh-button(@click="check_alternative" :loading="check_is_active" :disabled="check_btn_disabled" icon-placement="left" v-if="props.alternative_publication_site") Проверка
      template(#icon)
        n-icon
          RefreshOutlined
  template(#default)
    div Проверить текущие документы в альтернативном источнике опубликования
    n-button(
      text 
      tag="a"
      :href="props.alternative_publication_site"
      target="_blank"
      type="primary") {{props.alternative_publication_site}}
.search-result
  div(v-if="props.numbers.length > 0") Найдено: {{props.numbers.length}}
  loader.ld(v-if="check_is_active")
  n-scrollbar(v-else style="max-height: 280px")
    n-list(v-for="n in props.numbers" hoverable clickable :key="n.number")
      n-list-item(@click="list_item_click(n)")
        n-thing(content-style="margin-top: 10px;" content-indented)
          template(#header)
            n-text(:type="get_status(n.status)[2]") {{n.number}}
          template(#description v-if="n.note") {{ n.note }}
        template(#suffix)
          n-tooltip(style="max-width:200px")
            template(#trigger)
              n-tag(:type="get_status(n.status)[2]") {{ get_status(n.status)[0] }}
            template(#default) {{get_status(n.status)[1]}}
n-modal(v-model:show="show_modal"
    v-if="selected_number"
    :mask-closable="false"
    preset="dialog"
    title="Изменение параметров номера"
    @positive-click="save"
    positive-text="Сохранить"
    negative-text="Отмена")
  .modal-comment
    div Комментарий к номеру {{selected_number.number}}:
    n-input(v-model:value="current_input")
    n-checkbox(v-model:checked="current_item_is_checked") Проверен
</template>


<script setup lang="ts">
const props = defineProps<{
  numbers: Number[],
  alternative_publication_site?: string
}>()
const emits = defineEmits<{
  "update:numbers": [value: Number[]]
}>();
const notification = useNotification();
const show_modal = ref<boolean>(false);
const selected_number = ref<Number>();
const current_input = ref<string>();
const check_is_active = ref(false);
const current_item_is_checked = ref(false);
const check_btn_disabled = computed(() => props.numbers.length == 0);
const list_item_click = (number: Number) =>
{
  selected_number.value = number;
  current_input.value = number.note;
  current_item_is_checked.value = selected_number.value.status == 1 ? true : false;
  show_modal.value = true;
}
const check_alternative = async () =>
{
  check_is_active.value = true;
  const numbers = await searcher_commands.check_alternative_publ_info(props.numbers);
  if(numbers.error)
  {
    notification.error(
    {
      title: "Ошибка проверки списка номеров",
      content: numbers.get_error()
    })
  }
  else
  {
    const n = numbers.get_value();
    emits('update:numbers', n)
    notification.info(
    {
      title: "Проверка номеров завершена",
      content: "Были проверены опубликованные документы на сайте " + props.alternative_publication_site
    })
    console.log(n);
  }
  check_is_active.value = false;
}

const save = async () =>
{
  if(selected_number.value)
  {
    selected_number.value.note = (current_input.value?.length ?? 0) > 0 ? current_input.value : undefined;
    if(selected_number.value.status != 2)
    {
      selected_number.value.status = current_item_is_checked.value ? 1 : 0;
    }
    await searcher_commands.save_number(selected_number.value);
  }
    
}

type TagType = 'info' | 'warning' | 'error' | 'success';
/**
 * 0 - имя тэга  
 * 1 - описание  
 * 2 - тип тэга
 * 
 */
const get_status = (status: number): [string, string, TagType] =>
{
  switch(status)
  {
    case 1:
    {
      return ["проверен", "Документ был проверен оператором", 'success']
    }  
    case 2:
    {
      return ["опубликован", "Документ был опубликован в альтернативном источнике опубликования", 'warning']
    }
    default:
    case 0:
    {
      return ["неопубликован", "Документ небыл опубликован на сайте publication.pravo.gov.ru", 'error']
    }
  }
}
</script>
<style>
.refresh-button
{
  margin-bottom: 5px;
}
.modal-comment
{
  display: flex;
  flex-direction: column;
  gap: 5px;
  font-size: 16px;
}
.search-result
{
  width: 100%;
  display: flex;
  flex-direction: column;
}
</style>