<script setup lang="ts">
import { nextTick, ref } from "vue";
import Appholder from "../Appholder.vue";
import { get_system_apps } from "../../Functions/AppsService";

import { onMounted } from "vue";
interface props {
    container_id: string;
}
const props = defineProps<props>();
const systemAppList = ref<string[] | null>(null);
const loading = ref(true);
const fech_system_app = async (containerid: string) => {
    systemAppList.value = await get_system_apps(containerid);
};
onMounted(() => {
    nextTick(async () =>
        await fech_system_app(props.container_id).then(() => loading.value = false))
});
</script>

<template>
    <div class="flex justify-center pt-2">
        <v-progress-circular v-if="loading" indeterminate color="red"/>
    
        <!-- style used to set the grid settings in /src/assets/styles/style.css -->
        <v-virtual-scroll class="h-full!" v-if="systemAppList != null" :items="systemAppList">
            <template v-slot:default="{ item }">
                <appholder :app-info="item" system-app-icon selection-box />
            </template>
        </v-virtual-scroll>
    </div>
</template>
