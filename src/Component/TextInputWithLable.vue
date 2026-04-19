<script setup lang="ts">
import { ref } from "vue";

interface Props {
    lable: string;
    inputValue?: string;
    icon?: string;
}
const hover = ref<boolean>(false);

defineProps<Props>();
</script>

<template>
    <div class="w-fit space-x-2 grid grid-cols-3 items-center">
        <label class="text-sm text-center text-nowrap">{{ lable }}</label>
        <div
            class="flex col-span-2 items-center px-1 border rounded-lg h-9 bg-[#22222698]"
        >
            <input
                type="text"
                class="w-full bg-transparent border-0 ring-0"
                :value="inputValue"
            />
            <v-hover>
                <template v-slot:default="{ isHovering, props }">
                    <button
                        class="bg-[#404045ff] rounded-md px-1 transition-all hover:bg-[#dbdbdd]"
                    >
                        <v-icon
                            v-bind="props"
                            :color="isHovering ? '#313131' : '#979797ff'"
                            class="transition-colors"
                            size="28px"
                            >{{ icon != null ? icon : "mdi-folder" }}</v-icon
                        >
                    </button>
                </template>
            </v-hover>
        </div>
    </div>
</template>
