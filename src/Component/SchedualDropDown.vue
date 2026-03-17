<template>

        <div class="flex flex-row items-center gap-2">
            <label class="text-sm" >Schedual:</label>        
            <v-menu :location="location">
                <template v-slot:activator="{ props }" >
                    <v-btn flat color="#22222698" class="w-fit text-sm! px-2! h-fit! py-2" v-bind="props">
                        {{ SelectedItem }}
                        <v-icon color="white" size="18px" class="pt-1 pl-2">mdi-chevron-down</v-icon>
                    </v-btn>
                </template>
                
                <v-list v-model="SelectedItem"  density="compact">
                    <v-list-item 
                    v-for="item in items" 
                    :key="item" 
                    :value="item"
                    v-on:click="SelectedItem=item"
                    >
                    <v-list-item-title>{{ item }}</v-list-item-title>
                </v-list-item>
            </v-list>
        </v-menu>
</div>
</template>

<script setup>
import { ref } from 'vue'

const SelectedItem = ref('Daily')
const items = [
    'Daily',
    'Weekly',
    'Monthly',
    'Yearly',
]
</script>