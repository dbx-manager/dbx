<script setup lang="ts">
import { ref } from 'vue';

interface Props {
    appInfo: String;
    iconUrl?: string;
    selectionBox?: boolean;
    systemAppIcon?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
    selectionBox: false,
    iconUrl: "",
    systemAppIcon: false
});
const selected=ref<boolean>(false)
</script>

<template>
    <li class="flex flex-row items-center gap-2" @click="selected=!selected">
        <input type="checkbox" v-if="selectionBox" :checked="selected"
            class="w-5 h-5 z-10 bg-transparent! rounded-sm! forced-colors:appearance-auto border border-blue-700! focus:ring-1 focus:ring-offset-0! focus:outline-none!" />

        <v-icon color="#dadadaff" :icon="systemAppIcon ? 'mdi-application-cog-outline' : 'mdi-view-grid-outline'" />
        <div class="flex flex-col truncate text-ellipsis!">

            <!-- if were showing system-app panle -->
            <span v-if="systemAppIcon" class="w-fit z-0">{{ appInfo }}
                <v-tooltip activator="parent" location="top" offset="0" interactive open-delay="1000" close-delay="1000"
                    class="">{{appInfo}}</v-tooltip></span>
            

            <!-- if were showing exported-app panle -->
            <span v-if="!systemAppIcon" class="w-fit z-0">{{ appInfo ? appInfo.split(":")[0] : "Error Happened" }}
                <v-tooltip activator="parent" location="top" offset="0" interactive open-delay="1000" close-delay="1000"
                    class="">{{
                        appInfo ? appInfo.split(":")[0] : "Error Happened"
                    }}</v-tooltip></span>
            <span class="text-xs text-gray-500 z-0">{{ appInfo ? appInfo.split(":")[1] : "Error Happened" }}
                <v-tooltip activator="parent" location="top" offset="0" interactive open-delay="1000" close-delay="1000"
                    class="">{{
                        appInfo ? appInfo.split(":")[1] : "Error Happened"
                    }}</v-tooltip>
            </span>
        </div>
    </li>
</template>
