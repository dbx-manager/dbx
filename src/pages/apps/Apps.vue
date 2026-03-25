<script setup lang="ts">
import { ref } from "vue";
import "../../assets/styles/style.css";
import AppsContainerCard from "../../Component/AppsContainerCard.vue";
import { get_exported_apps } from "../../Functions/AppsService";
const props = defineProps();
const containerList = ref<ExportedApps[] | null>(null);
const fech_exported_app = async () => {
    containerList.value = await get_exported_apps();
    console.log(containerList.value);
};
fech_exported_app();
</script>
<!-- TODO: add lazy loading to the items inside of SystemApps panle -->
<template>
    <AppsContainerCard
        class="m-3"
        v-if="containerList"
        v-for="container in containerList"
        :container="container"
    />
</template>
