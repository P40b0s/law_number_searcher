<script lang="ts">
import { NButton } from 'naive-ui';
import { searcher_commands } from '../tauri/commands';
import { ref } from 'vue';
</script>
<template lang="pug">
n-button(@click="test")
teplate(v-for="s in sign")
    div {{ s.name }}
</template>




<script setup lang="ts">
const sign = ref<SignatoryAuthority[]>([]);
const test = async () =>
{
    const res = await searcher_commands.get_signatory_authorites();
    console.log(res);
    sign.value = res.get_value();
}
</script>
<style>
</style>