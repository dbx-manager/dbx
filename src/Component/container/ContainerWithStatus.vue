<script setup lang="ts">
import "../../Functions/ListingContaienrs";
import { onMounted, onUnmounted} from "vue";
import MoreMenuActionButton from "./MoreMenuActionButton.vue";
import { ContainerListState } from "../../stores/ContainerListState";
import ContainerController from "./ContainerController.vue";
const props = defineProps();
const containerListState = ContainerListState();


onMounted(() => {
    containerListState.startAutoRefresh();
});

onUnmounted(() => {
    containerListState.stopAutoRefresh();
});

</script>

<template>
    <v-container v-for="container in containerListState.containerList"
        class="bg-[#333337] rounded-lg flex flex-row max-w-full! gap-2 pr-1">
        <div class="w-[20%]! flex-col bg-transparent grid grid-cols-2 gap-1">
            <div class=" flex flex-row col-span-2  items-center gap-3">
                <v-icon color="#dadadaff" size="x-large" fill="white">mdi-cube-outline</v-icon>
                <div class="flex justify-center items-center flex-wrap ">

                    <span class="text-white text-2xl font-light " >{{
                        container.name
                    }}</span>
                <span  class=" w-fit flex flex-col text-end  sm:pl-3 " :class="{
                    'text-green-600': container.state.match('running'),
                    'text-red': !container.state.match('running'),
                }">{{
                    container.state.charAt(0).toUpperCase() + container.state.slice(1)
                }}</span>
        </div>
            </div>
            <container-controller :containerId="container.id" :status="container.state"
                :autoBackup="container.autobackup" />
        </div>


        <div class="w-[80%] grid grid-cols-2 grid-row-2 gap-2">
            <div class="bg-[#212123ff] rounded-lg p-2 text-[12px]">CPU</div>
            <div class="bg-[#212123ff] rounded-lg p-2 text-[12px]">Memory</div>
            <div class="bg-[#212123ff] rounded-lg p-2 text-[12px]">Disk</div>
            <div class="bg-[#212123ff] rounded-lg p-2 text-[12px]">Network</div>
        </div>
        
        <more-menu-action-button :containerId="container.id"></more-menu-action-button>

    </v-container>
</template>
