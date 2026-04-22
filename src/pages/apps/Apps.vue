<script setup lang="ts">
import { onMounted, onUnmounted } from "vue";
import "../../assets/styles/style.css";
import AppsContainerCard from "../../Component/AppsContainerCard.vue";
import { ContainerListState } from "../../stores/ContainerListState";

const props = defineProps();
const containerListState = ContainerListState();
containerListState.fetchExportedApps();


onMounted(() => {
    containerListState.startAutoRefresh();
});

onUnmounted(() => {
    containerListState.stopAutoRefresh();
});


</script>
<template>
    <div class="flex justify-center flex-col">
        <AppsContainerCard class="m-3"  v-for="container in containerListState.containerList" :container="container" />
    </div>
</template>
