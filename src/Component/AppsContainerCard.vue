<script setup lang="ts">
import { ref } from "vue";
import ExportedAppsPanel from "./ExportedAppsPanel.vue";
import SystemAppsPanel from "./SystemAppsPanel.vue";
import "../assets/styles/style.css";
import ContainerCard from "./ContainerCard.vue";

const activePanel = ref<"ExportedAppsPanel" | "SystemAppsPanel">(
    "ExportedAppsPanel",
);
const switchToB = () => (activePanel.value = "ExportedAppsPanel");
const switchToA = () => (activePanel.value = "SystemAppsPanel");

interface Props {
    container: ExportedApps;
}
defineProps<Props>();
</script>
<template>
    <ContainerCard :containerName="container.name">
        <div class="bg-[#212123]! rounded-lg px-4">
            <header
                color="#222226"
                class="toolbar flex justify-center gap-2 pt-3"
            >
                <v-btn
                    variant="text"
                    @click="switchToB"
                    text="Exported Apps"
                    class="text-[16px]! py-5! rounded-lg"
                    :class="{
                        'bg-[#38383bff]! font-medium! ':
                            activePanel == 'ExportedAppsPanel',
                    }"
                />
                <v-btn
                    variant="text"
                    @click="switchToA"
                    text="Syatem Apps"
                    class="text-[16px]! py-5! rounded-lg"
                    :class="{
                        'bg-[#38383bff]! font-medium! ':
                            activePanel == 'SystemAppsPanel',
                    }"
                />
            </header>
            <div class="">
                <div class="relative overflow-hidden rounded">
                    <ExportedAppsPanel
                        class="overflow-scroll max-h-100"
                        v-if="activePanel === 'ExportedAppsPanel'"
                        :AppsList="container.exported_apps"
                    />
                    <SystemAppsPanel
                        class="overflow-scroll max-h-100"
                        v-if="activePanel == 'SystemAppsPanel'"
                        :container_id="container.id"
                    />
                </div>
            </div>
            <div class="flex gap-2 p-2 justify-end">
                <v-btn
                    flat
                    v-if="activePanel == 'SystemAppsPanel'"
                    class="rounded-lg"
                    color="#134314ff"
                    text="Install From File"
                />
                <v-btn flat class="rounded-lg" text="Unselect ALl" />
                <v-btn flat class="rounded-lg" text="Select All" />
                <div class="w-3" />
                <v-btn
                    flat
                    color="#c12d2dff"
                    class="rounded-lg"
                    text="Delete"
                />
            </div>
        </div>
    </ContainerCard>
</template>
