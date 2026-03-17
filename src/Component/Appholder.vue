<script setup lang="ts">
interface Props {
  appName: string;
  iconUrl?:string;
  selectionBox?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  selectionBox: false,iconUrl:""
});
</script>

<template>
  <li class="flex flex-row items-center gap-2">
    <input type="checkbox" v-if="selectionBox" class="w-5 h-5 bg-transparent! rounded-sm! forced-colors:appearance-auto border border-blue-700! focus:ring-1 focus:ring-offset-0! focus:outline-none!"/>
    <v-icon>mdi-application-cog-outline</v-icon><span>{{
      appName ? appName : "Error Happened"
    }}</span>
  </li>
</template>
