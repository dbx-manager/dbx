<template>
    <v-btn variant="flat" size="large" density="default" color="#3584e4ff" class="rounded-md"
        @Click="dialog = true"><span class="text-lg font-bold">New Container </span></v-btn>
    <v-dialog v-model="dialog" width="auto">
        <v-container class="bg-[#333337] flex flex-col gap-2 rounded-lg">

            <div class="flex flex-row justify-between">
                <p />
                <h1 class="text-2xl text-center pb-3">New Container</h1>
                <v-icon @click="close_dialog" icon="mdi-window-close" />
            </div>


            <TextInputWithLable lable="Container Name" icon="mdi-cube" :input-value="container_name_input"
                :ext-function="validate_container_name" tool-tip-message="Must start with an alphanumeric character.
                Cannot end with a period (.) or hyphen (-). Length: 1–253 characters " />


            <TextInputWithLable lable="Home Directory" :input-value="custom_home_input"
                :ext-function="validate_home_directory" />


            <div class="grid grid-cols-3 items-center text-center">
                <h1>Based Image</h1>
                <v-select :items="suggestedDistroboxImages" :label="suggestedDistroboxImages[0]" item-title="label"
                    item-value="value" hide-details density="compact" variant="outlined"
                    class="bg-[#1f1f23] col-span-2 rounded"></v-select>
            </div>
            <v-container class="bg-[#22222698] flex flex-col rounded-lg m-2">
                <div class="w-full grid grid-cols-2 gap grid-rows-3">
                    <CheckBoxWithTooltip id="1" checkboxLable="unshare-devsys" tooltip="dont share mounted drives (external and
                    internal) and Rendering hardware (gpus) and any thing mounted on /dev "
                        :onchange-function="checkbox_validate" />
                    <CheckBoxWithTooltip id="2" checkboxLable="unshare-groups"
                        tooltip="Dont share the groups (video,network,apt,sys,wheel, ....etc)"
                        :onchange-function="checkbox_validate" />

                    <CheckBoxWithTooltip id="3" checkboxLable="unshare-ipc"
                        tooltip="Dont share audio,gpu,wifi,usb, any pci driver and device "
                        :onchange-function="checkbox_validate" />
                    <CheckBoxWithTooltip id="4" checkboxLable="unshare-netns"
                        tooltip="Dont share the network stack,making the container networkless(No Internet)  "
                        :onchange-function="checkbox_validate" />
                    <CheckBoxWithTooltip id="5" checkboxLable="unshare-process"
                        tooltip="Dont share the process,so No gui apps , No connection to host, kind of a seprate system"
                        :onchange-function="checkbox_validate" />
                    <CheckBoxWithTooltip id="6" checkboxLable="unshare-all"
                        tooltip="the same as clicking on every one of those(unshare checkboxs) "
                        :onchange-function="checkbox_validate" />
                </div>
            </v-container>

            <v-btn color="#3584e4ff" @click="create_container">Create</v-btn>
        </v-container>
    </v-dialog>
</template>
<script lang="ts" setup>
import { onMounted, ref } from "vue";
import TextInputWithLable from "../TextInputWithLable.vue";
import CheckBoxWithTooltip from "../CheckBoxWithTooltip.vue";
import { suggestedDistroboxImages } from "../../class/suggestedImages";
import { homeDir } from "@tauri-apps/api/path";
import { postCreateNewContainer } from "../../Functions/NewContainerService";
interface Props {
    dialog: boolean;
}
const NewContainerRequest = ref<NewContainerRequest>({
    container_name: "", home_directory: "", unshare_all: false, unshare_devsys: false, unshare_groups: false, unshare_ipc: false, unshare_netns: false, unshare_process: false
});
const dialog = ref<boolean>(false);
const container_name_input = ref("")
const custom_home_input = ref("")
defineProps<Props>();
onMounted(async () => custom_home_input.value = await homeDir())
function close_dialog() {
    dialog.value = false;
}
function validate_container_name(newValue: string) {
    container_name_input.value = newValue;
    if (!newValue || newValue.length > 253) return false;
    return /^[a-zA-Z0-9][a-zA-Z0-9_.-]*$/.test(newValue);
}
function validate_home_directory(newValue: string) {
    custom_home_input.value = newValue;
    //TODO: check if the dir exsist 
    return /^\/|(\/[a-zA-Z0-9_-]+)+$/.test(newValue);
}
function checkbox_validate(id: string, status: boolean) {
    switch (id) {
        case '1':
            NewContainerRequest.value.unshare_devsys=status
            break;
        case '2':
            NewContainerRequest.value.unshare_groups=status

            break;
        case '3':
            NewContainerRequest.value.unshare_ipc=status

            break;
        case '4':
            NewContainerRequest.value.unshare_netns=status

            break;
        case '5':
            NewContainerRequest.value.unshare_process=status

            break;
        case '6':
            NewContainerRequest.value.unshare_all=status

            break;
        default:
            break;
    }
}
function create_container() {
    //TODO Show error message
    if (!(validate_container_name(container_name_input.value) &&
        validate_home_directory(custom_home_input.value))) { return }
    NewContainerRequest.value.container_name = container_name_input.value
    NewContainerRequest.value.home_directory = custom_home_input.value
     postCreateNewContainer(NewContainerRequest.value);
}
</script>