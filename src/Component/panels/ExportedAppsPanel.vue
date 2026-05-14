<script setup lang="ts">
import Appholder from "../Appholder.vue";
import type { ExportableApp } from "../../types/ListContainer";
import { ref } from "vue";
import { delete_an_exported_app } from "../../Functions/AppsService";
interface Props {
    AppsList: ExportableApp[];
    contianer_id:string;
}
const props = defineProps<Props>();
const selectedItems = ref(new Set<string>());
function callbackFunction(appName: string) {
    if (selectedItems.value.has(appName)) {
        selectedItems.value.delete(appName)
    } else { selectedItems.value.add(appName) }
    // Trigger reactivity by replacing the Set
    selectedItems.value = new Set(selectedItems.value)

}

function selectAll() {
    const newSet = new Set<string>();
    for (const app of props.AppsList) {
        newSet.add(app.app_name);
    }
    selectedItems.value = newSet;
}

function unselectAll() {
    selectedItems.value = new Set<string>();
}
</script>

<template>
    <div class="flex justify-center flex-col gap-4 pt-2">
        <v-virtual-scroll class="h-full!" v-if="AppsList != null" :items="AppsList">
            <template v-slot:default="{ item }">
                <appholder :app-info="item" selection-box :callback-function="callbackFunction"
                    :selected="selectedItems.has(item.app_name)" />
            </template>
        </v-virtual-scroll>
        <p v-if="!AppsList" class="flex justify-center text-lg text-gray-400">
            No apps were exported from this Container
        </p>
        <div class="flex justify-end gap-2">
            <v-btn flat text="Unselect All" class="w-fit  rounded-lg" @click="unselectAll" />
            <v-btn flat text="Select All" class="w-fit  rounded-lg" @click="selectAll" />
            <p></p>
            <p></p>
            <v-btn flat text="Delete" class="w-fit rounded-lg" color="#c12d2dff" @click="delete_an_exported_app(contianer_id,selectedItems)"/>
        </div>
    </div>
</template>