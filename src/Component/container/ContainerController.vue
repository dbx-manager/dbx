<script setup lang="ts">
import {
    start_container,
    pause_container,
    stop_container,
    unpause_container,
} from "../../Functions/ContainerControles";
interface Props {
    status: string;
    containerId: string;
    autoBackup:boolean;
}
defineProps<Props>();
</script>

<template>
    <!--TODO: apply your actions (start stop pause unpause.... ) on those buttons invoking the right functions  -->
    <v-btn
        text="Start"
        color="#188251ff"
        rounded="lg"
        @Click="start_container(containerId)"
        flat
        class=""
    ></v-btn>
    <v-btn
        text="Stop"
        color="#9a2a2aff"
        rounded="lg"
        @Click="stop_container(containerId)"
        flat
        class=""
    ></v-btn>
    <span class="flex flex-col row-span-2 items-center">
        Auto Start
        <v-switch  flat inset color="success" :model-value="autoBackup" hide-details/>
    </span>
    <v-btn
        text="Pause"
        color="#6c1882ff"
        rounded="lg"
        @Click="pause_container(containerId)"
        flat
        class=""
    ></v-btn>
    <v-btn
        text="Unpause"
        color="#2a439aff"
        rounded="lg"
        @Click="unpause_container(containerId)"
        flat
        class=""
    />

        

    <v-btn text="View Arguments" class="col-span-2" rounded="lg"> </v-btn>
    <v-btn text="Delete" color="#bd1919ff" rounded="lg" class=""> </v-btn>
</template>
