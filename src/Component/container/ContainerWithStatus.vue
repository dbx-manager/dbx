<script setup lang="ts">
import ContainerController from "./ContainerController.vue";
import "../../Functions/ListingContaienrs";
import { onMounted, onUnmounted } from "vue";
import { containerList, fetchContainerList, startUpdateInterval, stopUpdateInterval ,settings} from "../../Functions/FechingContainersWithRefresh";
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
    <v-container v-for="container in containerList" class="bg-[#333337] rounded-lg flex flex-row max-w-full! gap-2">
        <div class="flex-col bg-transparent grid grid-cols-3 gap-1" width="25%">
            <div class="grid grid-cols-2 items-center  col-span-3 gap-3">
                <div class="flex flex-row gap-2">

                    <v-icon color="#dadadaff" size="x-large" fill="white">mdi-cube-outline</v-icon>
                    <span class="text-white text-2xl font-light " v-if="container">{{
                        container.Names[0]
                        }}</span>
                </div>
                <span v-if="container" class="flex flex-col text-end pr-3" :class="{
                    'text-green-600': container.State.match('running'),
                    'text-red': !container.State.match('running'),
                }">{{
                    container.State.charAt(0).toUpperCase() + container.State.slice(1)
                }}</span>
            </div>
            <container-controller v-if="container" :containerId="container.Id" :status="container.State"
                :autoBackup="settings.containers.indexOf(container.Id.slice(0, 12)) != -1" />
        </div>
        <div class="w-[80%] grid grid-cols-2 sm:grid-cols-2 grid-row-2 gap-2">
            <div class="bg-[#212123ff] rounded-lg p-2 text-[12px]">CPU</div>
            <div class="bg-[#212123ff] rounded-lg p-2 text-[12px]">Memory</div>
            <div class="bg-[#212123ff] rounded-lg p-2 text-[12px]">Disk</div>
            <div class="bg-[#212123ff] rounded-lg p-2 text-[12px]">Network</div>
        </div>
    </v-container>
</template>
