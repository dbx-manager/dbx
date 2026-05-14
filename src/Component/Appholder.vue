<script setup lang="ts">
import { ref } from 'vue';
import type { ExportableApp } from '../types/ListContainer';

type AppInfoType = ExportableApp | string;

interface Props {
    appInfo: AppInfoType;
    iconUrl?: string;
    selectionBox?: boolean;
    systemAppIcon?: boolean;
    callbackFunction: (appName:string)=>void;
    selected?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
    selectionBox: false,
    iconUrl: "",
    systemAppIcon: false,
    selected: false
});


</script>

<template>
    <li class="flex flex-row items-center gap-2" >
        <input type="checkbox" v-if="selectionBox" @click="callbackFunction(typeof appInfo === 'object' ? appInfo.app_name : 'Error Happened')" 
            :checked="selected"
            class="w-5 h-5 z-10 bg-transparent! rounded-sm! forced-colors:appearance-auto border border-blue-700! focus:ring-1 focus:ring-offset-0! focus:outline-none!" />

        <v-icon color="#dadadaff" :icon="systemAppIcon ? 'mdi-application-cog-outline' : 'mdi-view-grid-outline'" />
        <div class="flex flex-col truncate text-ellipsis!">

            <!-- if were showing system-app panle -->
            <span v-if="systemAppIcon" class="w-fit z-0">{{ appInfo }}
                <v-tooltip activator="parent" location="top" offset="0" interactive open-delay="1000" close-delay="1000"
                    class="">{{appInfo}}</v-tooltip></span>
            

            <!-- if were showing exported-app panle -->
            <span v-if="!systemAppIcon" class="w-fit z-0">{{ typeof appInfo === 'object' ? appInfo.app_name : "Error Happened" }}
                <v-tooltip activator="parent" location="top" offset="0" interactive open-delay="1000" close-delay="1000"
                    class="">{{ typeof appInfo === 'object' ? appInfo.app_name : "Error Happened" }}</v-tooltip></span>
            <span class="text-xs text-gray-500 z-0">{{ typeof appInfo === 'object' ? appInfo.sub_apps.join("\n") : "Error Happened" }}
                <v-tooltip activator="parent" location="top" offset="0" interactive open-delay="1000" close-delay="1000"
                    class="whitespace-pre-line " scroll-strategy="block" >{{ typeof appInfo === 'object' ? appInfo.sub_apps.join("\n") : "Error Happened" }}</v-tooltip>
            </span>
        </div>
    </li>
</template>