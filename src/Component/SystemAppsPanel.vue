<script setup lang="ts">
import { ref } from "vue";
import Appholder from "./Appholder.vue";
import { get_system_apps } from "../Functions/AppsService";
import { onMounted } from "vue";
const SystemPackagesList = ref([
    "brave",
    "chrome",
    "inkscape",
    "Gimp",
    "systemd",
    "soso Al Rakasa",
    "what do you call a hoe?",
]);
interface props {
    container_id: string;
}
const props = defineProps<props>();
const systemAppList = ref<string[] | null>(null);
const fech_system_app = async (containerid: string) => {
    systemAppList.value = await get_system_apps(containerid);
};
onMounted(() => {
    fech_system_app(props.container_id);
    console.log(systemAppList);
});
</script>

<template>
    <ul class="flex flex-wrap gap-5">
        <appholder
            v-if="systemAppList != null"
            v-for="item in systemAppList"
            :app-info="item"
            selection-box
        />
    </ul>
</template>
