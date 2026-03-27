<script setup>
import { ref } from "vue";
import ContainerCard from "./container/ContainerCard.vue";

const containersList = ref([
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
    <!-- <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-3"> -->
    <div class="flex flex-wrap gap-3 justify-center">
        <container-card
            v-for="value in containersList"
            :container-name="value"
            class="w-fit!"
        >
            <v-row
                density="compact"
                class="flex flex-row gap-1! text-[#b3b3b3]"
            >
                <p>Last-Backup: Today,14:32</p>
            </v-row>
            <v-row density="compact">
                <div
                    class="w-full flex flex-row justify-between items-center text-[#b3b3b3]"
                >
                    Size: 10G
                    <v-btn
                        text="Restore"
                        flat
                        color="#651d9d"
                        rounded="lg"
                        size="small"
                        class="absolute h-9!"
                    />
                </div>
            </v-row>
            <v-row
                density="compact"
                class="flex flex-row justify-between items-center gap-2!"
            >
                <div class="text-sm gap-0!">
                    Auto Backup
                    <div class="flex justify-center items-baseline">
                        <v-switch
                            class="gap"
                            hide-details
                            density="compact"
                            color="primary"
                            center-affix
                        />
                    </div>
                </div>
                <div class="flex gap-1">
                    <v-btn
                        size="small"
                        flat
                        color="#188251"
                        rounded="lg"
                        class="h-9!"
                        >Backup</v-btn
                    >
                    <v-btn
                        size="small"
                        flat
                        color="#bd1919"
                        rounded="lg"
                        class="h-9!"
                        >Delete</v-btn
                    >
                </div>
            </v-row>
        </container-card>
    </div>
</template>
