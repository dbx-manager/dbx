<script setup lang="ts">
import { ref } from "vue";
import "../assets/styles/style.css";

import ContainerWithStatus from "../Component/container/ContainerWithStatus.vue";
import ButtonSettingsPopUp from "../Component/ButtonSettingsPopUp.vue";
import NewContainerPanel from "../Component/panels/NewContainerPanel.vue";
defineProps();
const settignsVisability = ref<boolean>(false);
const newContainerDialog = ref<boolean>(false);
</script>

<template>

    <v-container class="max-w-full! space-y-3">
        <div class="w-full flex flex-col md:flex-row justify-between gap-4">
            <div class="flex-1">

                <NewContainerPanel :dialog="newContainerDialog" />
            </div>
            <div class="w-full transition-opacity" v-if="settignsVisability">
                <ButtonSettingsPopUp class="w-fit absolute right-13 mt-10" />
            </div>
            <v-btn @Click="
                settignsVisability
                    ? (settignsVisability = false)
                    : (settignsVisability = true)
                " variant="text"><v-icon>mdi-cog</v-icon></v-btn>
        </div>
        <ContainerWithStatus />
    </v-container>
</template>
