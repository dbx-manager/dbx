<script setup lang="ts">
import "../../assets/styles/style.css";
import TextInputWithLable from "../../Component/TextInputWithLable.vue";
import SchedualDropDown from "../../Component/SchedualDropDown.vue";
import BackupContainerWithController from "../../Component/BackupContainerWithController.vue";
import { CronExp } from "../../utils/cronUtils";
import { useSettingsStore } from "../../stores/useSettingsStore";

const props = defineProps();
const settings = useSettingsStore();

function syncCronWithBackend(cron_as_string: CronExp) {
    settings.setcron_schedule(cron_as_string.toString())
}
function syncBackupPathWithBackend(cron_as_string: string) {
    settings.setBackupLocationPath(cron_as_string)
    return true

}
function syncPackageCachePathWithBackend(cron_as_string: string) {
    settings.setPackageCachePath(cron_as_string)
    return true
}
</script>
<template>
    <div class="flex flex-col gap-5">
        <v-container class="bg-[#333337] pa-0 flex rounded-lg max-w-full! text-[#b8b8b8ff] gap-3 justify-start">
            <v-row density="compact">
                <div class="grid grid-row-2 pl-4 py-4 w-fit! gap-x-4 gap-2 text-sm">
                    <TextInputWithLable lable="Location :" :inputValue="settings.settings.backupLocationPath"
                        :extFunction="syncBackupPathWithBackend" />
                    <div class="flex flex-row gap-4">
                        <SchedualDropDown :cron_sync_function="syncCronWithBackend"
                            :cron_schedual="settings.settings.cron_schedule" />
                        <!-- <keep-counter /> -->
                    </div>
                </div>
                <v-divider vertical class="border-opacity-100 border-transparent flex md:border-[#222226ff]" />
                <div class="grid grid-cols-2 p-4 w-fit! gap-x-4 gap-4 text-sm">
                    <TextInputWithLable class="col-span-2" lable="Shared Package Cache:"
                        :inputValue="settings.settings.packageCachePath"
                        :extFunction="syncPackageCachePathWithBackend" />
                    <!-- TODO :add the size collection -->
                    <span>Totla Backups Size: </span>
                    <span>Totla Package Cache size: 3G</span>
                </div>
            </v-row>
        </v-container>
        <BackupContainerWithController />
    </div>
</template>
