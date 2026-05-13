<script setup lang="ts">
import { ref } from "vue";

interface Props {
    containerName?: string;
    iconUrl?: string;
    isOpen?:boolean;
}
const props = defineProps<Props>();
const isOpen = ref<boolean>(props.isOpen?? false);
</script>

<template>
    <div
        class="bg-[#333337] hover:bg-[#404045]  cursor-pointer p-4 rounded-lg max-w-full! gap-2"
    >
        <div @click="isOpen = !isOpen"
            
            class="flex bg-transparent justify-between"
        >
            <div class="flex flex-row items-center! gap-2">
                <v-icon v-if="!iconUrl" color="#dadadaff" size="large" fill="white">{{
                     "mdi-cube-outline"
                }}</v-icon>
                <span class="text-white text-xl font-light">{{
                    containerName ? containerName : "Container Name"
                }}</span>
            </div>
            <v-icon size="28">{{
                isOpen ? "mdi-chevron-up" : "mdi-chevron-down"
            }}</v-icon>
        </div>
        <div v-show="isOpen" class="cursor-auto" >
            <slot  />
        </div>
    </div>
</template>
