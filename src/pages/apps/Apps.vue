<script setup lang="ts">
import { ref } from "vue";
import "../../assets/styles/style.css";
import AppsContainerCard from "../../Component/AppsContainerCard.vue";
import { get_exported_apps } from "../../Functions/AppsService";
const props = defineProps();
const containerList = ref<Record<string, string[]> | null>(null);
const fech_exported_app = async () => {
    containerList.value = await get_exported_apps();
};
fech_exported_app();
</script>
<template>
    <AppsContainerCard
        class="m-3"
        v-for="(exportedApps, container) in containerList"
        :containerName="container"
        :exportedApps="exportedApps"
    />
</template>
