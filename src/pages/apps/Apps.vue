<script setup lang="ts">
import { onMounted, ref } from "vue";
import "../../assets/styles/style.css";
import AppsContainerCard from "../../Component/AppsContainerCard.vue";
import { get_exported_apps } from "../../Functions/AppsService";
const props = defineProps();
const containerList = ref<ExportedApps[] | null>(null);
const loading = ref<boolean>(true);

const fech_exported_app = async () => {
    containerList.value = await get_exported_apps();
};
onMounted(async()=>
 await fech_exported_app().then(()=>loading.value=false)
)
</script>
<template >
    <div class="flex justify-center flex-col">
        <v-progress-circular v-if="loading" indeterminate/>
        <AppsContainerCard
        class="m-3"
        v-if="containerList"
        v-for="container in containerList"
        :container="container"
        />
    </div>
</template>
