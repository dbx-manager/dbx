<script setup lang="ts">
import { useSettings } from "../../composables/useSettings";
import { start_container, stop_container } from "../../Functions/ContainerControles";
interface Props {
    status: string;
    containerId: string;
    autoBackup: boolean;
    autoStartup: boolean;
}
const props = defineProps<Props>();
const s = useSettings()


function add_container_to_auto_backup(event: Event) {
    const target = event.target as HTMLInputElement;
    const isOn: boolean = target.checked;

    if (isOn) {
        s.settings.backup_containers.add(props.containerId)
        s.setContainers(s.settings.backup_containers)
    }
    else {
        s.settings.backup_containers.delete(props.containerId)
        s.setContainers(s.settings.backup_containers)
    }

}
function add_container_to_auto_start(event: Event) {
    const target = event.target as HTMLInputElement;
    const isOn: boolean = target.checked;
    console.log(s.settings.autostart_containers)
    if (isOn) {
        s.settings.autostart_containers.add(props.containerId)
        s.setAutostartContainers(s.settings.autostart_containers)
    }
    else {
        s.settings.autostart_containers.delete(props.containerId)
        s.setAutostartContainers(s.settings.autostart_containers)
    }

}
</script>

<template>
    <!-- <v-btn text="sssStart" color="#188251ff" rounded="lg" @Click="s()" flat class=""></v-btn> -->
    <v-btn text="Start" color="#188251ff" rounded="lg" @Click="start_container(containerId)" flat class="" />
    <v-btn text="Stop" color="#9a2a2aff" rounded="lg" @Click="stop_container(containerId)" flat class="" />
    <div class="flex flex-col row-span-2 items-center">
        <span class="text-center">Auto Start</span>
        <v-switch flat inset density="compact" color="success" :model-value="autoStartup"
            @Change="add_container_to_auto_start" hide-details />
    </div>
    <div class="flex flex-col row-span-2 items-center ">
        <span class="text-center">Auto Backup</span>
        <v-switch flat inset density="compact" color="success" :model-value="autoBackup"
            @Change="add_container_to_auto_backup" hide-details />
    </div>
</template>
