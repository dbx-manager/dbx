<script setup lang="ts">
import Appholder from "../Appholder.vue";
import { ContainerListState } from "../../stores/ContainerListState";
interface props {
    container_id: string;
    systemAppList: string[] | null;
}
const props = defineProps<props>();
const containerListState = ContainerListState();
containerListState.fetchSystemApps(props.container_id);

</script>

<template>
    <div class="flex justify-center pt-2">
        <v-progress-circular v-if="systemAppList == null" indeterminate color="red" />

        <!-- style used to set the grid settings in /src/assets/styles/style.css -->
        <v-virtual-scroll class="h-full!" v-if="systemAppList != null" :items="systemAppList">
            <template v-slot:default="{ item }">
                <appholder :app-info="item" system-app-icon selection-box />
            </template>
        </v-virtual-scroll>
    </div>
</template>
