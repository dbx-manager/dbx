<script setup lang="ts">
import { computed, ref } from "vue";
import "../../assets/styles/style.css";
import ExportedAppsPanel from "../../Component/ExportedAppsPanel.vue";
import SystemAppsPanel from "../../Component/SystemAppsPanel.vue";
import ContainerCard from "../../Component/ContainerCard.vue";
defineProps();

const activePanel = ref<"panelA" | "panelB">("panelA");
const components: Record<string, any> = {
  panelA: ExportedAppsPanel,
  panelB: SystemAppsPanel,
};

const currentComponent = computed(() => components[activePanel.value]);

const switchToB = () => (activePanel.value = "panelB");
const switchToA = () => (activePanel.value = "panelA");
</script>
<template>
  <container-card>
    <div class="bg-[#212123]! rounded-lg px-4">
      <header color="#222226" class="toolbar flex justify-center gap-2 pt-3">
        <v-btn
          variant="text"
          @click="switchToA"
          text="Exported Apps"
          class="text-[16px]! py-5! rounded-lg"
          :class="{ 'bg-[#38383bff]! font-medium! ': activePanel == 'panelA' }"
        />
        <v-btn
          variant="text"
          @click="switchToB"
          text="Syatem Apps"
          class="text-[16px]! py-5! rounded-lg"
          :class="{ 'bg-[#38383bff]! font-medium! ': activePanel == 'panelB' }"
        />
      </header>
      <div class="">
        <div class="relative overflow-hidden rounded">
          <TransitionGroup name="slide" tag="div" class="flex h-full">
            <component
              :is="currentComponent"
              :key="activePanel"
              class="w-full shrink-0 p-4"
            />
          </TransitionGroup>
        </div>
      </div>
      <div class="flex gap-2 p-2 justify-end">
        <v-btn flat v-if="activePanel=='panelB'" class="rounded-lg" color="#134314ff" text="Install From File" />
        <v-btn flat class="rounded-lg" text="Unselect ALl" />
        <v-btn flat class="rounded-lg" text="Select All" />
        <div class="w-3"/>
        <v-btn flat color="#c12d2dff" class="rounded-lg" text="Delete" />
      </div>
    </div>
  </container-card>
</template>
