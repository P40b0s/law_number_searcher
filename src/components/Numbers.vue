<script lang="ts">
import { NButton, NInputNumber, NScrollbar} from 'naive-ui';
import { searcher_commands } from '../tauri/commands';
import { computed, ref } from 'vue';
import { type Result } from '../tauri/abstract';
</script>
<template lang="pug">
n-scrollbar(x-scrollable)
  template(v-for="n in props.numbers")
    div {{n.number}}
</template>


<script setup lang="ts">
const props = defineProps<{
  numbers: Number[]
}>()
</script>
<style>
</style>