<script setup lang="ts">
import { export_an_app } from "../../Functions/AppsService";
import { ContainerListState } from "../../stores/ContainerListState";
import { ExportableApp } from "../../types/ListContainer";
import ContainerCard from "../container/ContainerCard.vue";

interface props {
    container_id: string;
    app_list: ExportableApp[] | null;
}
const props = defineProps<props>();
const containerListState = ContainerListState();
containerListState.fetchExporteAnApp(props.container_id);
</script>

<template>
    <div class="flex justify-center pt-2">
        <v-progress-circular v-if="app_list == null" indeterminate color="red" />
        <div v-else-if="app_list.length === 0" class="text-gray-400  py-4">
            No exportable apps found
        </div>
        <div v-else class="w-fit grid sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4  gap-3 ">
            <div class="flex justify-center h-fit!" v-for="app in app_list" :key="app.app_name">

                <container-card  icon-url="-" :container-name="app.app_name"
                     class="w-fit!  bg-[#38383bff]!">
                    <ul class="p-2  flex-col gap-2  bg-[#1a1a1c] rounded-lg  " >
                        <li v-for="sub in app.sub_apps" :key="sub" class="  rounded-lg px-2 py-1 text-sm">
                            {{ sub }}
                        </li>
                    </ul>
                    <div class="w-full flex pt-2 justify-end">
                        <v-btn color="green" rounded="lg" flat text="Export" @click="export_an_app(container_id,app.app_name)"/>
                    </div>
                </container-card>
            </div>
        </div>
    </div>
</template>