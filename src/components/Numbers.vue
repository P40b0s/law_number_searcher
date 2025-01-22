<script lang="ts">
import { NButton, NCheckbox, NMarquee, NInputNumber, NInput, NScrollbar, NModal, NList, NListItem, NThing, NTag, NSpace, NTooltip, NIcon, NText, useNotification} from 'naive-ui';
import { searcher_commands } from '../tauri/commands';
import { tauri_events } from '../tauri/events';
import { computed, h, onBeforeUnmount, onMounted, onUnmounted, ref, watch } from 'vue';
import { type Result } from '../tauri/abstract';
import { type ExportNumbers, type Number} from '../@types/number';
import { RefreshOutlined, MenuOpenOutlined, NoteAltOutlined, WindowSharp} from "@vicons/material";
import Loader from './loaders/Loader.vue'
import  { RecycleScroller } from 'vue-virtual-scroller'
</script>
<template lang="pug">
.buttons-panel
  n-tooltip(style="max-width:200px" v-if="props.alternative_publication_site && props.numbers.length > 0")
    template(#trigger)
      n-button.refresh-button(@click="check_alternative" :loading="check_is_active" :disabled="check_btn_disabled" icon-placement="left") Проверка
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
  n-tooltip(style="max-width:200px" v-if="props.numbers.length > 0")
    template(#trigger)
      n-button.refresh-button(@click="export_to_excel" :loading="export_is_active" :disabled="check_btn_disabled" icon-placement="left") Экспорт
        template(#icon)
          n-icon
            MenuOpenOutlined
    template(#default)
      div Экспорт списка в формат excel
.search-result(v-if="props.numbers.length > 0")
  div(style="display: flex; flex-direction: row; gap: 5px; font-weight: 600; font-size: 18px; margin-bottom: 5px")
    n-tooltip
      template(#trigger)
        n-tag(style="font-size: 18px;") {{numbers.length}}
      template(#default) Всего найдено документов
    n-tooltip
      template(#trigger)
        n-tag(style="font-size: 18px;" type="error") {{numbers.filter(f=>f.status == 0).length}}
      template(#default) Не опубликовано документов на pravo.gov.ru
    n-tooltip
      template(#trigger)
        n-tag( style="font-size: 18px;" type="warning") {{numbers.filter(f=>f.status == 2).length}}
      template(#default) Опубликовано документов на региональном сайте опубликования
    n-tooltip
      template(#trigger)
        n-tag(style="font-size: 18px;" type="success") {{numbers.filter(f=>f.status == 1).length}}
      template(#default) Не опубликовано, проверено оператором
  recycle-scroller.scroller(:items="numbers" :item-size="40" :item-height="40" key-field="number" v-slot="{ item }")
    n-list(hoverable clickable )
      n-list-item(@click="list_item_click(item)")
        div(style="width: 100%; font-size: 18px; display: flex; flex-direction: row; align-items: center; justify-content: space-between; gap: 5px;")
          n-text(:type="item.tag") {{item.number}}
          template(v-if="item.note")
            n-text(v-if="item.note.length < 40" style="margin-left: 5px") {{ item.note }}
            n-marquee(v-else style="margin-left: 5px; width: 100%")
              div(style="display: flex; flex-direction: row;  margin-right: 10px; width: 100%;")
                n-icon(:size="25")
                  NoteAltOutlined
                n-text {{ item.note }}
          n-tooltip(style="max-width:250px;")
            template(#trigger)
              n-tag(:type="item.tag") {{ item.text }}
            template(#default) {{item.comment}}
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
    n-input(v-model:value="current_input" placeholder="Добавьте заметку")
    n-checkbox(v-model:checked="current_item_is_checked") Проверен
n-modal(v-model:show="check_is_active" style="height: 100%; display: flex; justify-items: center")
  div(style="height: 100%; width: 100%")
    loader(v-if="check_is_active" :status="search_status")
</template>


<script setup lang="ts">
type TagType = 'info' | 'warning' | 'error' | 'success';
type NumberForComponent = 
{
  text: string,
  comment: string,
  tag: TagType,
  size: number
} & Number

const props = defineProps<{
  numbers: Number[],
  organ_name?: string,
  type_name?: string,
  alternative_publication_site?: string
}>()

const emits = defineEmits<{
  "update:numbers": [value: Number[]]
}>();

watch(() => props.numbers, (newval) => 
{
  numbers.value = newval.map(n=>
    {
      const status = get_status(n.status);
      return {
        signatory_authority: n.signatory_authority,
        type_id: n.type_id,
        year: n.year,
        number: n.number,
        note: n.note,
        status: n.status,
        text: status[0],
        comment: status[1],
        tag: status[2],
      } as NumberForComponent
    } 
  )
})

const numbers = ref<NumberForComponent[]>([]);
const notification = useNotification();
const show_modal = ref<boolean>(false);
const selected_number = ref<NumberForComponent>();
const current_input = ref<string>();
const check_is_active = ref(false);
const export_is_active = ref(false);
const current_item_is_checked = ref(false);
const search_status = ref("")
const check_btn_disabled = computed(() => props.numbers.length == 0);

const list_item_click = (number: NumberForComponent) =>
{
  selected_number.value = number;
  current_input.value = number.note;
  current_item_is_checked.value = selected_number.value.status == 1 ? true : false;
  show_modal.value = true;
}
const check_alternative = async () =>
{
  search_status.value = "";
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
      content: "Были проверены опубликованные документы на сайте " + props.alternative_publication_site,
      duration: 2000
    })
    console.log(n);
  }
  check_is_active.value = false;
}


// const element = document.getElementById("#scroll") as HTMLElement;
// element.addEventListener("scrollend", (event) => 
// {
//   console.log("scrollend event fired!");
// });

const export_to_excel = async () =>
{
  const en = {
    organ_name: props.organ_name ?? "Неизвестно",
    type_name: props.type_name ?? "Неизвестно",
    alternative_site: props.alternative_publication_site,
    numbers: props.numbers
  } as ExportNumbers;
  export_is_active.value = true;
  const res = await searcher_commands.export_to_excel(en);
  if(res.error)
  {
    notification.error(
    {
      title: "Ошибка экспорта номеров в файл excel",
      content: res.get_error()
    })
  }
  else
  {
    notification.info(
    {
      title: "Экспорт успешно завершен",
      duration: 2000
    })
  }
  export_is_active.value = false;
}

const tauri_search_status_event = tauri_events.alternative_search_process(async (p) =>
{
  search_status.value = p.payload;
});

onUnmounted(()=> {tauri_search_status_event.then(r=> r.unsubscribe())})

const save = async () =>
{
  if(selected_number.value)
  {
    selected_number.value.note = (current_input.value?.length ?? 0) > 0 ? current_input.value : undefined;
    if(selected_number.value.status != 2)
    {
      selected_number.value.status = current_item_is_checked.value ? 1 : 0;
      const status = get_status(selected_number.value.status);
      selected_number.value.text = status[0];
      selected_number.value.comment = status[1];
      selected_number.value.tag = status[2];
    }
    await searcher_commands.save_number(selected_number.value);
  }
    
}


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
  height: 100%;
  display: flex;
  flex-direction: column;
}
.buttons-panel
{
  display: flex;
  flex-direction: row;
  gap: 5px;
}
.scroller
{
  height: 300px;
  max-height: 100%;
  transition: ease all .3s;
}
@media screen and (max-height: 500px) and (min-height: 200px)
{
  .scroller
  {
    height: 100px;
    min-height: 100px;
  }
}
@media screen and (max-height: 600px) and (min-height: 500px)
{
  .scroller
  {
    height: calc(700px - 520px);
    min-height: calc(700px - 520px);
  }
}
@media screen and (max-height: 700px) and (min-height: 600px)
{
  .scroller
  {
    height: calc(700px - 440px);
    min-height: calc(700px - 440px);
  }
}
@media screen and (max-height: 800px) and (min-height: 700px)
{
  .scroller
  {
    height: calc(800px - 466px);
    min-height: calc(800px - 466px);
  }
}
@media screen and (max-height: 900px) and (min-height: 800px)
{
  .scroller
  {
    height: calc(900px - 486px);
    min-height: calc(900px - 486px);
  }
}
@media screen and (max-height: 1000px) and (min-height: 900px)
{
  .scroller
  {
    height: calc(1000px - 526px);
    min-height: calc(1000px - 526px);
  }
}
@media screen and (max-height: 1100px) and (min-height: 1000px)
{
  .scroller
  {
    height: calc(1100px - 526px);
    min-height: calc(1100px - 526px);
  }
}

</style>