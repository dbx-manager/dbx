<script setup lang="ts">
import { ref } from "vue";
import "../assets/styles/style.css";
import { invoke } from "@tauri-apps/api/core";

import ContainerWithStatus from "../Component/container/ContainerWithStatus.vue";
import ButtonSettingsPopUp from "../Component/ButtonSettingsPopUp.vue";
defineProps();
const settignsVisability = ref<boolean>(false);
</script>
<script lang="ts">
//test function
async function list() {
    // console.log(await invoke("get_package_cache_size"));
     
}
</script>
<template>
    <v-container class="max-w-full! space-y-3">
        <div class="w-full flex flex-col md:flex-row justify-between gap-4">
            <div class="flex-1">
                <v-btn
                    variant="flat"
                    size="large"
                    density="default"
                    color="#3584e4ff"
                    class="rounded-md"
                    @Click="list"
                    ><span class="text-lg font-bold"
                        >New Container
                    </span></v-btn
                >
            </div>
            <div class="w-full transition-opacity" v-if="settignsVisability">
                <ButtonSettingsPopUp class="w-fit absolute right-13 mt-10" />
            </div>
            <v-btn
                @Click="
                    settignsVisability
                        ? (settignsVisability = false)
                        : (settignsVisability = true)
                "
                variant="text"
                ><v-icon>mdi-cog</v-icon></v-btn
            >
        </div>
        <ContainerWithStatus />
    </v-container>
</template>
