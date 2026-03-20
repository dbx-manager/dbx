<script setup lang="ts">
import "../assets/styles/style.css";
import ContainerWithStatus from "../Component/containerWithStatus.vue";
import SettingsPanel from "../Component/SettingsPanel.vue";
defineProps();
</script>
<template>
  <v-container class="max-w-full! space-y-3 ">
    <div class="flex flex-col md:flex-row gap-4">
      <div class="flex-1">
        <v-btn variant="flat" size="large" density="default" color="#3584e4ff" class="rounded-md"
          ><span class="text-lg font-bold">New Container </span></v-btn
        >
      </div>
      <div class="w-full md:w-80">
        <settings-panel />
      </div>
    </div>
    <container-with-status />
  </v-container>
</template>
