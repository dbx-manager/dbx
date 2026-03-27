<script setup lang="ts">
import Appholder from "../Appholder.vue";
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
