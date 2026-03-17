<script setup lang="ts">
import "../assets/styles/style.css";
import ContainerWithStatus from "../Component/containerWithStatus.vue";
defineProps();
</script>
<template>
  <v-container class="max-w-full! space-y-3 ">
    <v-btn variant="flat" size="large" density="default" color="#3584e4ff" class="rounded-md ml-3"
      ><span class="text-lg font-bold">New Container </span></v-btn
    >
    <container-with-status />
  </v-container>
</template>
