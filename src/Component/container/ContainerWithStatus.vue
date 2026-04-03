<script setup lang="ts">
import ContainerController from "./ContainerController.vue";
import "../../Functions/ListingContaienrs";
import { onMounted, onUnmounted } from "vue";
import { containerList, fetchContainerList, startUpdateInterval, stopUpdateInterval, settings } from "../../Functions/FechingContainersWithRefresh";
import MoreMenuActionButton from "./MoreMenuActionButton.vue";
const props = defineProps();

onMounted(async () => {
    // Initial fetch
    await fetchContainerList();
    // Start polling
    startUpdateInterval();
});

onUnmounted(() => {
    stopUpdateInterval();
});
</script>

<template>
    <v-container v-for="container in containerList"
        class="bg-[#333337] rounded-lg flex flex-row max-w-full! gap-2 pr-1">
        <div class="w-[20%]! flex-col bg-transparent grid grid-cols-2 gap-1">
            <div class=" flex flex-row col-span-2  items-center gap-3">
                <v-icon color="#dadadaff" size="x-large" fill="white">mdi-cube-outline</v-icon>
                <div class="flex justify-center items-center flex-wrap ">

                    <span class="text-white text-2xl font-light " v-if="container">{{
                        container.Names[0]
                    }}</span>
                <span v-if="container" class=" w-fit flex flex-col text-end  sm:pl-3 " :class="{
                    'text-green-600': container.State.match('running'),
                    'text-red': !container.State.match('running'),
                }">{{
                    container.State.charAt(0).toUpperCase() + container.State.slice(1)
                }}</span>
        </div>
            </div>
            <container-controller v-if="container" :containerId="container.Id" :status="container.State"
                :autoBackup="settings.containers.indexOf(container.Id.slice(0, 12)) != -1" />
        </div>


        <div class="w-[80%] grid grid-cols-2 grid-row-2 gap-2">
            <div class="bg-[#212123ff] rounded-lg p-2 text-[12px]">CPU</div>
            <div class="bg-[#212123ff] rounded-lg p-2 text-[12px]">Memory</div>
            <div class="bg-[#212123ff] rounded-lg p-2 text-[12px]">Disk</div>
            <div class="bg-[#212123ff] rounded-lg p-2 text-[12px]">Network</div>
        </div>
        
        <more-menu-action-button :containerId="container.Id"></more-menu-action-button>

    </v-container>
</template>
