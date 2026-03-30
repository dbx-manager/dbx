<template>
    
    <div class="w-fit space-x-2 col-span-2  flex items-center">
        <label class="text-sm text-nowrap">{{ lable }}</label>
        <div class="flex items-center px-1 border rounded-lg h-9 bg-[#22222698] ">
            <input type="text" class="w-full  bg-transparent border-0 ring-0 " :value="inputValue">
            <button class=" bg-[#404045ff] rounded-md px-1 hover:">
                <v-icon color="#979797ff" size="28px">mdi-folder</v-icon>
            </button>
        </div>
    </div>
</template>


<script setup lang="ts">
interface Props {
  lable: string;
  inputValue?:string;
}
defineProps<Props>()
</script>