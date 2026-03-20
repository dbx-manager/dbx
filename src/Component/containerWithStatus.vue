<script setup lang="ts">
import ContainerController from "./ContainerController.vue";
import "../Functions/ListingContaienrs";
import { list_containers } from "../Functions/ListingContaienrs";
import { onMounted, onUnmounted, ref, watch } from "vue";
import { ListContainer } from "../Types/ListContainer";
import { useSettings } from "../composables/useSettings";

const props = defineProps();
const containerList = ref<ListContainer[]|null>(null);
const isLoading = ref(false);
const updateError = ref<string | null>(null);
let updateInterval: number | null = null;

const { settings } = useSettings();

// Function to fetch container list with error handling
const fetchContainerList = async () => {
  if (!settings.autoUpdateEnabled) return;
  
  try {
    isLoading.value = true;
    updateError.value = null;
    const newList = await list_containers();
    containerList.value = newList;
  } catch (error) {
    console.error('Failed to fetch container list:', error);
    updateError.value = error instanceof Error ? error.message : 'Failed to fetch containers';
  } finally {
    isLoading.value = false;
  }
};

// Start the update interval
const startUpdateInterval = () => {
  if (updateInterval) {
    clearInterval(updateInterval);
  }
  
  if (settings.autoUpdateEnabled && settings.updateInterval > 0) {
    updateInterval = setInterval(fetchContainerList, settings.updateInterval);
  }
};

// Stop the update interval
const stopUpdateInterval = () => {
  if (updateInterval) {
    clearInterval(updateInterval);
    updateInterval = null;
  }
};

// Watch for settings changes and update interval accordingly
watch(() => settings.updateInterval, () => {
  startUpdateInterval();
});

watch(() => settings.autoUpdateEnabled, (enabled) => {
  if (enabled) {
    startUpdateInterval();
  } else {
    stopUpdateInterval();
  }
});

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
  <v-container v-for="container in containerList" class="bg-[#333337] rounded-lg flex flex-row max-w-full! gap-2 ">
    <div
      class="flex-col bg-transparent grid grid-cols-3 gap-1 "
      width="25%"
    >
      <div class="flex flex-row items-center pl-2 col-span-3">
                  <v-icon color="#dadadaff" size="x-large" fill="white"  class=" pr-2">mdi-cube-outline</v-icon>
        <span class="text-white text-2xl font-light" v-if="container">{{ container.Names[0] }}</span>
      </div>
      <container-controller v-if="container" :containerId="container.Id" :status="container.State"  />
    </div>
    <div class="w-[80%]  grid grid-cols-2 grid-row-2  md:grid-cols-3 md:grid-rows-2 gap-2">
        <!-- TODO: make the exported app disappere when the screen is too small -->
      <v-container class="row-span-2 text-md text-center bg-[#212123] rounded-lg hidden md:block "
        >Exported Apps</v-container 
      >
      <div class="bg-[#212123ff] rounded-lg  p-2 text-[12px] "> CPU</div>
      <div class="bg-[#212123ff] rounded-lg p-2 text-[12px]"> Memory</div>
      <div class="bg-[#212123ff] rounded-lg p-2 text-[12px]"> Disk</div>
      <div class="bg-[#212123ff] rounded-lg p-2 text-[12px]"> Network</div>
    </div>
  </v-container>
</template>
