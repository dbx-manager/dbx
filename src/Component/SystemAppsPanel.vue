<script setup lang="ts">
import { ref } from "vue";
import Appholder from "./Appholder.vue";
const SystemPackagesList = ref([
    "brave",
    "chrome",
    "inkscape",
    "Gimp",
    "systemd",
    "soso Al Rakasa",
    "what do you call a hoe?",
]);
</script>

<template>
    <ul class="flex flex-wrap gap-5">
        <appholder
            v-for="item in SystemPackagesList"
            :app-info="item"
            selection-box
        />
    </ul>
</template>
