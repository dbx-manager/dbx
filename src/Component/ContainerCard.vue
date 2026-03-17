<script setup lang="ts">
interface Props {
  containerName?: string;
  iconUrl?:string;
}
defineProps<Props>();
</script>

<template>
  <div  class="bg-[#333337] p-4 rounded-lg max-w-full! gap-2 ">
        <div
        class="flex-col bg-transparent  "
        >
        <div class="flex flex-row items-center pb-2 gap-2">
          <v-icon color="#dadadaff" size="large" fill="white"  >mdi-cube-outline</v-icon>
          <span class="text-white text-xl font-light">{{containerName?containerName:'Container Name'}}</span>
        </div>
      </div>
      <slot  />
    </div>
    
</template>
