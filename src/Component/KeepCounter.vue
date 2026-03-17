<template>
    <div class="flex flex-row items-center gap-2">
        <label class="text-sm">Keep:</label>
        <div class=" flex items-center bg-[#22222698] rounded-lg h-9 p-2">
            <input type="tel" maxlength="2" value="1" class="w-6 text-center bg-transparent border-0 p-0!"></input>
            <v-icon color="white" size="18px" class="pt-1 pl-2">mdi-minus</v-icon>
            <v-icon color="white" size="18px" class="pt-1 pl-2">mdi-plus</v-icon>
        </div>


    </div>
</template>

<script setup>
import { ref } from 'vue'


</script>