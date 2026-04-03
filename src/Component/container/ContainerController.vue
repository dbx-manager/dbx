<script setup lang="ts">
import { useSettings } from "../../composables/useSettings";
import { start_container, stop_container } from "../../Functions/ContainerControles";
interface Props {
    status: string;
    containerId: string;
    autoBackup: boolean;
}
const props = defineProps<Props>();
const { settings } = useSettings();

function add_container_to_auto_backup(event: Event) {
    const s=useSettings()
    const target = event.target as HTMLInputElement;
    const isOn: boolean = target.checked;
            console.log(s.settings.containers)

    if (isOn) {
        s.settings.containers.push(props.containerId)
        s.setContainers(s.settings.containers)
    }
    else {
        s.setContainers(
            s.settings.containers.filter((c) => c != props.containerId))
        }
        
        console.log(s.settings.containers)
}
</script>

<template>
    <!-- <v-btn text="sssStart" color="#188251ff" rounded="lg" @Click="s()" flat class=""></v-btn> -->
    <v-btn text="Start" color="#188251ff" rounded="lg" @Click="start_container(containerId)" flat class=""></v-btn>
    <v-btn text="Stop" color="#9a2a2aff" rounded="lg" @Click="stop_container(containerId)" flat class=""></v-btn>
    <span class="flex flex-col row-span-2 items-center">
        Auto Start
        <v-switch flat inset density="compact" color="success" :model-value="autoBackup" hide-details />
    </span>
    <span class="flex flex-col row-span-2 items-center ">
        Auto Backup
        <v-switch flat inset density="compact" color="success" @Change="add_container_to_auto_backup"
            :model-value="(settings.containers.indexOf(containerId.slice(0, 12)) != -1)" hide-details />
    </span>
</template>
