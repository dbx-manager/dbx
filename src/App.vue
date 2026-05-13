<script setup lang="ts">
import "./assets/styles/style.css";
import TopBar from "./Component/TopBar.vue";
import { usePerformanceHistoryStore } from "./stores/performanceHistoryStore";

const perfHistory = usePerformanceHistoryStore();
perfHistory.start();
</script>

<template>
  <v-app>
      <top-bar />
    <v-container class="max-w-full! m-0 p-0! sm:p-3! md:p-5! ">
      <RouterView />
    </v-container>
  </v-app>
</template>
