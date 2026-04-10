<script setup lang="ts">
import { ref } from "vue";
import "../assets/styles/TitleBar.js"

defineProps();
const activePanel = ref<"ContainerPanel" | "AppsPanel" | "BackupPanel">("ContainerPanel");
const switchToContainer = () => (activePanel.value = "ContainerPanel");
const switchToApps = () => (activePanel.value = "AppsPanel");
const switchToBackup = () => (activePanel.value = "BackupPanel");
</script>

<template>

  <div data-tauri-drag-region class="flex items-center justify-center">
    <div></div>
    <div class="absolute right-4 flex gap-2 controls ">
      <button id="titlebar-minimize" title="minimize" class="">
        <v-icon color="#979797ff" size="small">mdi-minus</v-icon></button>
      <button id="titlebar-close" title="close"
        class="bg-[#0000005d] flex items-center justify-center w-fit! h-fit! p-1   rounded-full">
        <v-icon id="" color="#979797ff" size="small">mdi-close</v-icon></button>

    </div>
    <v-container class="flex justify-center space-x-0.5 topbarContainer py-1.5!">
      <router-link to="/" class="rounded-lg" :class="{ 'bg-[#38383bff]': activePanel == 'ContainerPanel' }">
        <v-btn variant="text" size="large" @click="switchToContainer">
          <v-icon color="#dadadaff" size="large" fill="white" class=" pr-2" :icon="activePanel == 'ContainerPanel' ?'mdi-cube':'mdi-cube-outline'"></v-icon>
          <span class="text-lg">Container</span></v-btn>
      </router-link>
      <router-link to="/apps" class="rounded-lg" :class="{ 'bg-[#38383bff]': activePanel == 'AppsPanel' }">
        <v-btn variant="text" href="/apps" size="large" @click="switchToApps">
          <v-icon color="#dadadaff" size="large" class=" pr-2"
            :icon="activePanel == 'AppsPanel' ? 'mdi-view-grid-plus' : 'mdi-view-grid-plus-outline'" /><span
            class="text-lg">Apps</span></v-btn>

      </router-link>
      <router-link to="/backup" class="rounded-lg" :class="{ 'bg-[#38383bff]': activePanel == 'BackupPanel' }">
        <v-btn variant="text" href="/backup" size="large" @click="switchToBackup">
          <v-icon color="#dadadaff" size="large" class="pr-2"> mdi-history </v-icon>
          <span class="text-lg">Backup</span></v-btn>
      </router-link>
    </v-container>
  </div>
  <span class="border-[#5c5c66ff] border-b"></span>
</template>
