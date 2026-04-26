<script setup lang="ts">
import { start_container, stop_container } from "../../Functions/ContainerControles";
interface Props {
    status: string;
    containerId: string;
    autoBackup: boolean;
    autoStartup: boolean;
    autoBackupCallback:(e:Event,container_id:string)=>void;
    autoStartCallback:(e:Event,container_id:string)=>void;
}
const props = defineProps<Props>();

</script>

<template>
    <!-- <v-btn text="sssStart" color="#188251ff" rounded="lg" @Click="s()" flat class=""></v-btn> -->
    <v-btn text="Start" color="#188251ff" rounded="lg" @Click="start_container(containerId)" flat class="" />
    <v-btn text="Stop" color="#9a2a2aff" rounded="lg" @Click="stop_container(containerId)" flat class="" />
    <div class="flex flex-col row-span-2 items-center">
        <span class="text-center">Auto Start</span>
        <v-switch flat inset density="compact" color="success" :model-value="autoStartup"
            @Change="(e:Event)=>autoStartCallback(e,containerId)" hide-details />
    </div>
    <div class="flex flex-col row-span-2 items-center ">
        <span class="text-center">Auto Backup</span>
        <v-switch flat inset density="compact" color="success" :model-value="autoBackup"
            @Change="(e:Event)=>autoBackupCallback(e,containerId)" hide-details />
    </div>
</template>
