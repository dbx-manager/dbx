<script setup lang="ts">
import ContainerController from "./ContainerController.vue";
import "../Functions/ListingContaienrs";
import { list_containers } from "../Functions/ListingContaienrs";
import { onMounted, ref } from "vue";
import { ListContainer } from "../Types/ListContainer";
const props = defineProps();
const containerList=ref<ListContainer[]|null>(null);
onMounted(async () => {
  const x = await list_containers();
  containerList.value = x;
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
      <container-controller v-if="container" :contianer-id="container.Id" :status="container.State"  />
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
