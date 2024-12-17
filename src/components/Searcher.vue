<script lang="ts">
import { NButton } from 'naive-ui';
import { searcher_commands } from '../tauri/commands';
</script>
<template lang="pug">
n-button(@click="test")
n-select()
</template>




<script setup lang="ts">
const test = async () =>
{
    const res = await searcher_commands.get_signatory_authorites();
}
</script>
<style>
</style>