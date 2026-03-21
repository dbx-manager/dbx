<script setup lang="ts">
import { ref } from "vue";
import Appholder from "./Appholder.vue";
import { get_exported_apps } from "../Functions/AppsService";
const ExportedAppsList = ref([
    "brave",
    "chrome",
    "inkscape",
    "Gimp",
    "systemd",
    "soso Al Rakasa",
    "what do you call a hoe?",
]);
interface Props {
    AppsList: String[];
}
defineProps<Props>();
</script>

<template>
    <ul class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-5">
        <appholder v-for="item in AppsList" :app-info="item" selection-box />
    </ul>
</template>
