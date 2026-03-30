<script setup lang="ts">
import "../../assets/styles/style.css";
import KeepCounter from "../../Component/KeepCounter.vue";
import TextInputWithLable from "../../Component/TextInputWithLable.vue";
import SchedualDropDown from "../../Component/SchedualDropDown.vue";
import BackupContainerWithController from "../../Component/BackupContainerWithController.vue";
import { settings } from "../../Functions/FechingContainersWithRefresh";
defineProps();

</script>
<template>
    <div class="flex flex-col gap-5">
        <v-container
            class="bg-[#333337] pa-0 flex rounded-lg max-w-full! text-[#b8b8b8ff] gap-3 justify-start"
        >
            <v-row dense>
                <div class="grid grid-row-2 pl-4 py-4 w-fit! gap-x-4 gap-2">
                    <TextInputWithLable lable="Location :" :inputValue="settings.exportPath"  />
                    <div class="flex flex-row gap-4">
                        <SchedualDropDown />
                        <keep-counter />
                    </div>
                </div>
                <v-divider
                    vertical
                    class="border-opacity-100 border-transparent flex md:border-[#222226ff]"
                />
                <div class="grid grid-cols-2 p-4 w-fit! gap-x-4 gap-4 text-sm">
                    <TextInputWithLable
                        class="col-span-2"
                        lable="Shared Package Cache:"
                    />
                    <span>Totla Backups Size: 15G</span>
                    <span>Totla Package Cache size: 3G</span>
                </div>
            </v-row>
        </v-container>
        <BackupContainerWithController/>
    </div>
</template>
