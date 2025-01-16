<script lang="ts">
import { NButton, NTooltip, NInputNumber, NProgress, NDivider, useNotification} from 'naive-ui';
import { searcher_commands } from '../tauri/commands';
import { computed, ref } from 'vue';
import OrganSelector from './dictionaries/organ_selector'
import TypeSelector from './dictionaries/type_selector'
import Numbers from './Numbers.vue';
import {type Number, new_number, test_numbers} from '../@types/number'
import { type Result } from '../tauri/abstract';
import { tauri_events } from '../tauri/events';
import { useSelectHeight } from '../composables/useSelechtHeight';
</script>
<template lang="pug">
.search-container    
    organ-selector#orgsel(:disabled="org_disabled" @select="select_organ" placeholder="Выберите принявший орган")
    type-selector(:disabled="tp_disabled" paceholder="Выберите вид документа" @select="select_type", :selected_organ="selected_organ")
    .search-action
        n-input-number(
            v-model:value="year"
            placeholder="Год за который будет осуществлятся поиск"
            :min="2011"
            :max="current_year"
            style="min-width: 100px; max-width: 100px;"
        )
        transition(name="fade")
            .progress-bar(v-if="search_in_process")
                n-tooltip
                    template(#trigger)
                        n-progress.progress( type="line" :height="15" :border-radius="4" :fill-border-radius="0" indicator-text-color="#874a0d" indicator-placement='inside' processing :percentage="process")
                    div {{process_description}}
                n-tooltip
                    template(#trigger)
                        n-progress.progress( type="line" :height="15" :border-radius="4" :fill-border-radius="0" indicator-text-color="#874a0d" indicator-placement='inside' processing :percentage="process2")
                    div {{process_description2}}
        n-button(@click="start_search" :loading="search_in_process" :disabled="btn_disabled") Поиск
n-divider
numbers(v-model:numbers="numbers" :alternative_publication_site="alternative_publication_site")
</template>


<script setup lang="ts">
const selected_organ = ref<Dictionary|null>(null);
const selected_type = ref<Dictionary|null>(null);
const tp_disabled = computed(()=> (selected_organ.value == null || search_in_process.value));
const org_disabled = computed(()=> search_in_process.value);
const btn_disabled = computed(()=> (selected_organ.value == null || selected_type.value == null || search_in_process.value));
const current_year = new Date().getFullYear();
const year = ref(current_year);
const numbers = ref<Number[]>([]);
const process = ref<number>(0);
const process_description = ref("");
const process2 = ref<number>(0);
const process_description2 = ref("");
const search_in_process = ref(false);
const notification = useNotification();
const alternative_publication_site = ref<string>()
const {height} = useSelectHeight();
const select_organ = (org: Dictionary|null) =>
{
    selected_organ.value = org;
}
const select_type = (tp: Dictionary|null) =>
{
    selected_type.value = tp;
}
const load_process = tauri_events.load_process(async (p) =>
{
    process.value = p.payload.percent;
    process_description.value = p.payload.description;
});
const load_process2 = tauri_events.load_base_process(async (p) =>
{
    process2.value = p.payload.percent;
    process_description2.value = p.payload.description;
});
const start_search = async () =>
{
    process.value = 0;
    process2.value = 0;
    numbers.value = [];
    search_in_process.value = true;
    if(selected_organ.value)
    {
        alternative_publication_site.value = (await searcher_commands.get_alternative_publ_site(selected_organ.value.id)).value;
        console.log(alternative_publication_site.value);
    }
    const n = await searcher_commands.get_lost_numbers(selected_organ.value?.id as string, selected_type.value?.id as string, year.value);
    if(n.is_err())
    {
        notification.error(
        {
            title: "Ошибка получения списка номеров",
            content: n.get_error()
        })
        search_in_process.value = false;
        process.value = 0;
        process2.value = 0;
    }
    else
    {
        await new Promise(f => setTimeout(f, 200));
        numbers.value = n.get_value();
        search_in_process.value = false;
        process.value = 0;
        process2.value = 0;
        if(numbers.value.length > 0 )
        {
            notification.warning(
            {
                title: "Поиск завершен",
                content: `Найдено ${numbers.value.length} пропущенных номеров`,
                duration: 3000
            })
        }
        else
        {
            notification.success(
            {
                title: "Поиск завершен",
                content: "Пропущенных номеров не найдено",
                duration: 3000
            })
        }
    }
}
</script>
<style>
.search-action
{
    display: flex;
    flex-direction: row;
    /* justify-content: space-between; */
    align-items: center;
    gap:10px;
}
.search-container
{
    display: flex;
    flex-direction: column;
    gap: 5px;
}
.progress-bar
{
    width: 100%;
}
.n-progress-graph-line-indicator
{
    font-size: 18px !important;
    font-weight: 600;
}
/* .n-progress .n-progress-graph .n-progress-graph-line.n-progress-graph-line--indicator-inside .n-progress-graph-line-rail .n-progress-graph-line-indicator */

.fade-enter-active,
.fade-leave-active 
{
  transition: opacity 0.5s ease;
}

.fade-enter-from,
.fade-leave-to 
{
  opacity: 0;
}
</style>