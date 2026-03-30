<script setup>
import { ref } from "vue";
import ContainerCard from "./container/ContainerCard.vue";

const containersList = ref([
    "brave",
    "chrome",
    "inkscape",
    "Gimp",
    "systemd",
    "soso Al Rakasa",
    "what do you call a hoe?",
]);
import { onMounted, onUnmounted } from "vue";
import { containerList, fetchContainerList, startUpdateInterval, stopUpdateInterval, settings } from "../Functions/FechingContainersWithRefresh";
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
    <!-- <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-3"> -->
    <div class="flex flex-wrap gap-3 justify-center">
        <container-card v-for="value in containerList" :container-name="value.Names[0]" class="w-fit!">
            <div  class="flex flex-row gap-1! text-[#b3b3b3]">
                <p>Last-Backup: Today,14:32</p>
            </div>
            <div >
                <div class="w-full flex flex-row justify-between items-center text-[#b3b3b3]">
                    Size: 10G
                    <v-btn text="Restore" flat color="#651d9d" rounded="lg" size="small" class="absolute h-9!" />
                </div>
            </div>
            <div  class="flex flex-row justify-between items-center gap-2!">
                <div class="text-sm gap-0!">
                    Auto Backup
                    <div class="flex justify-center items-baseline">
                        <v-switch flat inset color="success" :model-value="(settings.containers.indexOf(value.Id.slice(0, 12)) != -1)" hide-details />

                    </div>
                </div>
                <div class="flex gap-1">
                    <v-btn size="small" flat color="#188251" rounded="lg" class="h-9!">Backup</v-btn>
                    <v-btn size="small" flat color="#bd1919" rounded="lg" class="h-9!">Delete</v-btn>
                </div>
            </div>
        </container-card>
    </div>
</template>
