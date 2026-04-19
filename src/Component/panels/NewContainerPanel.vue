<script lang="ts" setup>
import { ref } from "vue";
import TextInputWithLable from "../TextInputWithLable.vue";
import CheckBoxWithTooltip from "../CheckBoxWithTooltip.vue";
import { suggestedDistroboxImages } from "../../class/suggestedImages";
interface Props {
    dialog: boolean;
}

const dialog = ref<boolean>(false);
defineProps<Props>();
function close_dialog() {
    dialog.value = false;
}
</script>
<template>
    <v-btn
        variant="flat"
        size="large"
        density="default"
        color="#3584e4ff"
        class="rounded-md"
        @Click="dialog = true"
        ><span class="text-lg font-bold">New Container </span></v-btn
    >
    <v-dialog v-model="dialog" width="auto">
        <v-container class="bg-[#333337] flex flex-col gap-2 rounded-lg">
            <div class="flex flex-row justify-between">
                <p />
                <h1 class="text-2xl text-center pb-3">New Container</h1>
                <v-icon @click="close_dialog" icon="mdi-window-close" />
            </div>
            <TextInputWithLable lable="Container Name" icon="mdi-cube" />
            <TextInputWithLable lable="Home Directory" />
            <!-- TODO: add a based image dropdown -->
            <div class="grid grid-cols-3 items-center text-center">
                <h1>Based Image</h1>
                <v-select
                    :items="suggestedDistroboxImages"
                    :label="suggestedDistroboxImages[0]"
                    item-title="label"
                    item-value="value"
                    hide-details
                    density="compact"
                    variant="outlined"
                    class="bg-[#1f1f23] col-span-2 rounded"
                ></v-select>
            </div>
            <v-container class="bg-[#22222698] flex flex-col rounded-lg m-2">
                <div class="w-full grid grid-cols-2 gap grid-rows-3">
                    <CheckBoxWithTooltip
                        checkboxLable="unshare-devsys"
                        tooltip="dont share mounted drives (external and
                    internal) and Rendering hardware (gpus) and any thing mounted on /dev "
                    />
                    <CheckBoxWithTooltip
                        checkboxLable="unshare-groups"
                        tooltip="Dont share the groups (video,network,apt,sys,wheel, ....etc)"
                    />

                    <CheckBoxWithTooltip
                        checkboxLable="unshare-ipc"
                        tooltip="Dont share audio,gpu,wifi,usb, any pci driver and device "
                    />
                    <CheckBoxWithTooltip
                        checkboxLable="unshare-netns"
                        tooltip="Dont share the network stack,making the container networkless(No Internet)  "
                    />
                    <CheckBoxWithTooltip
                        checkboxLable="unshare-process"
                        tooltip="Dont share the process,so No gui apps , No connection to host, kind of a seprate system"
                    />
                    <CheckBoxWithTooltip
                        checkboxLable="unshare-all"
                        tooltip="the same as clicking on every one of those(unshare checkboxs) "
                    />
                </div>
            </v-container>

            <v-btn color="#3584e4ff">Create</v-btn>
        </v-container>
    </v-dialog>
</template>
