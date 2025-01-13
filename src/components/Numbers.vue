<script lang="ts">
import { NButton, NInputNumber, NInput, NScrollbar, NModal, NList, NListItem, NThing, NTag, NSpace, NTooltip, NIcon, NText} from 'naive-ui';
import { searcher_commands } from '../tauri/commands';
import { computed, h, ref } from 'vue';
import { type Result } from '../tauri/abstract';
import { type Number} from '../@types/number';
import { RefreshOutlined } from "@vicons/material";
</script>
<template lang="pug">
n-tooltip(style="max-width:200px" v-if="props.alternative_publication_site")
  template(#trigger)
    n-button.refresh-button(:loading="check_is_active" icon-placement="left" v-if="props.alternative_publication_site") Проверка
      template(#icon)
        n-icon
          RefreshOutlined
  template(#default)
    div Проверить текущий документ в альтернативном источнике опубликования
    n-button(
      text 
      tag="a"
      :href="props.alternative_publication_site"
      target="_blank"
      type="primary") {{props.alternative_publication_site}}
div(v-if="props.numbers.length > 0") Найдено: {{props.numbers.length}}
n-scrollbar(style="max-height: 280px")
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
    :title="selected_number.number"
    @positive-click="save"
    positive-text="Сохранить"
    negative-text="Отмена")
  div
    p Комментарий к документу:
    n-input(v-model:value="current_input")
</template>


<script setup lang="ts">
const props = defineProps<{
  numbers: Number[],
  alternative_publication_site?: string
}>()
const show_modal = ref<boolean>(false);
const selected_number = ref<Number>();
const current_input = ref<string>();
const list_item_click = (number: Number) =>
{
  selected_number.value = number;
  current_input.value = number.note;
  show_modal.value = true;
  console.log("ОП ОП оП")
}

const save = () =>
{
  if(selected_number.value)
    selected_number.value.note = current_input.value;
}
const check_is_active = ref(false);
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
</style>