<script setup>
import { pause_container, unpause_container } from '../../Functions/ContainerControles';

defineProps({
    containerId: { type: String }
});
</script>
<!-- components/MoreActionsBtn.vue -->
<template>
    <v-menu location="bottom "  class="rounded-xl! ">
        <template #activator="{ props }">
            <!-- <v-btn v-bind="props" class="h-fit! p-1" variant="text" size="sm">
                <v-icon v-bind="props" class="pointer-coarse:">mdi-dots-vertical</v-icon>
            </v-btn> -->
            <v-icon v-bind="props"  class="hover:bg-[#555557] transition rounded-sm mr-1">mdi-dots-vertical</v-icon>

        </template>
        <v-list rounded="lg"  density="compact" >
            <v-list-item  class="">
                <v-btn text="Pause" color="#6c1882ff" rounded="lg" @Click="pause_container(containerId)" flat
                    class="w-full"></v-btn>
            </v-list-item>
            <v-list-item class="">

                <v-btn text="Unpause" color="#2a439aff" rounded="lg" @Click="unpause_container(containerId)" flat
                    class="w-full" />

            </v-list-item>


            <v-list-item class="">

                <v-btn text="Delete" color="#bd1919ff" rounded="lg" class="w-full"> </v-btn>
            </v-list-item>
            <v-list-item class="bg-transparent">

                <v-btn text="View Arguments" color="#444447" class="w-full " rounded="lg"> </v-btn>
            </v-list-item>
        </v-list>
    </v-menu>
</template>
