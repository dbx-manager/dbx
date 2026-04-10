<script setup lang="ts">
import Appholder from "../Appholder.vue";
interface Props {
    AppsList: String[];
}
defineProps<Props>();
</script>

<template>
    <div class="flex justify-center pt-2">
        <v-virtual-scroll class="h-full!"  v-if="AppsList != null"  :items="AppsList" >
            <template v-slot:default="{ item }" >
                <appholder :app-info="item" selection-box />        
            </template>
        </v-virtual-scroll>
    </div>
</template>
