<script setup lang="ts">
import { ref } from 'vue';


interface Props {
    lable: string;
    extFunction?: (newValue: string) => boolean;
    inputValue?: string;
    toolTipMessage?: string;
    icon?: string;

}


const props = defineProps<Props>();
const isValid = ref<boolean | null>(null)
function intFucntion(e: InputEvent) {
    // after validation check if its empty and remove the error message(but keep it error in the form) and if not show the red ring of error
    if (!props.extFunction) { isValid.value = null; return; }
    const target = e.target as HTMLInputElement;
    let t = props.extFunction(target.value)
    isValid.value = t
    if (target.value.length == 0) {
        isValid.value = null;
    }
}
</script>

<template>
    <div class="w-fit space-x-2 grid grid-cols-3 items-center">
        <label class="text-sm text-center text-nowrap">{{ lable }}</label>
        <div class="flex col-span-2 items-center px-1 border rounded-lg h-9 bg-[#22222698]"
            :class="(isValid || isValid == null) ? '' : 'ring ring-red-400'">
            <v-tooltip v-if="toolTipMessage" contained :text="toolTipMessage" activator="parent" location="top" />
            <input type="text" class="w-full bg-transparent border-0 ring-0 " :value="inputValue"
                @input="intFucntion" />
            <v-hover>
                <template v-slot:default="{ isHovering, props }">
                    <button class="bg-[#404045ff] rounded-md px-1 transition-all hover:bg-[#dbdbdd]">
                        <v-icon v-bind="props" :color="isHovering ? '#313131' : '#979797ff'" class="transition-colors"
                            size="28px">{{ icon != null ? icon : "mdi-folder" }}</v-icon>
                    </button>
                </template>
            </v-hover>
        </div>
    </div>
</template>
