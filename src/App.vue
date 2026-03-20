<script setup lang="ts">
import "./assets/styles/style.css";
import TopBar from "./Component/TopBar.vue";
</script>

<template>
  <v-app>
    <top-bar />
    <v-container class="max-w-full!">
      <RouterView />
    </v-container>
  </v-app>
</template>
