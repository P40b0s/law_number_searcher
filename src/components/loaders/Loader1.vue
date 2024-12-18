<template lang="pug">
.ring
    span
    span
    span
    span
</template>

<script lang="ts" setup>
// const props = defineProps<{
//   color: string
// }>();
</script>
    
<style lang="scss" scoped>
.ring {
    position: absolute;
    text-align: initial !important;
    border-radius: 50%;
    height: 96px;
    width: 96px;
    animation: rotate_3922 1.2s linear infinite;
    background-color: #9b59b6;
    background-image: linear-gradient(#9b59b6, #84cdfa, #5ad1cd);
}

.ring span {
    position: absolute;
    border-radius: 50%;
    height: 100%;
    width: 100%;
    background-color: #9b59b6;
    background-image: linear-gradient(#9b59b6, #84cdfa, #5ad1cd);
}

.ring span:nth-of-type(1) {
    filter: blur(5px);
}

.ring span:nth-of-type(2) {
    filter: blur(10px);
}

.ring span:nth-of-type(3) {
    filter: blur(25px);
}

.container span:nth-of-type(4) {
    filter: blur(50px);
}

.ring::after {
    content: "";
    position: absolute;
    top: 10px;
    left: 10px;
    right: 10px;
    bottom: 10px;
    background-color: #ffffff5e;
    border: solid 5px #f3e4e4cb;
    border-radius: 50%;
}

@keyframes rotate_3922 {
    from {
    transform: translate(-50%, -50%) rotate(0deg);
    }

    to {
    transform: translate(-50%, -50%) rotate(360deg);
    }
}

</style>