<script setup lang="ts">
import "../../assets/styles/style.css";
import TextInputWithLable from "../../Component/TextInputWithLable.vue";
import SchedualDropDown from "../../Component/SchedualDropDown.vue";
import BackupContainerWithController from "../../Component/BackupContainerWithController.vue";
import { CronExp } from "../../utils/cronUtils";
import { useSettingsStore } from "../../stores/useSettingsStore";
import { backupContainer } from "../../Functions/BackupService";
import { list_containers } from "../../Functions/ListingContaienrs";
import { ref, onMounted } from "vue";
import type { Container } from "../../types/ListContainer";
import { invoke } from "@tauri-apps/api/core";

const props = defineProps();
const settings = useSettingsStore();
const containers = ref<Container[]>([]);
const containersLoaded = ref(false);
const totalBackupSize = ref("0 B");
const packageCacheSize = ref("0 B");

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

async function triggerBackup(containerId: string) {
    try {
        const backupPath = await backupContainer(containerId, settings.settings.backupLocationPath);
        console.log(`Backup created at: ${backupPath}`);
        await loadSizes();
    } catch (e) {
        console.error("Backup failed:", e);
    }
}

async function loadSizes() {
    try {
        const backupInfo = await invoke<any>("get_backup_directory_list");
        totalBackupSize.value = backupInfo.total_size_human || "0 B";
    } catch (e) {
        console.error("Failed to load backup size", e);
    }
    try {
        const cacheInfo = await invoke<any>("get_package_cache_size");
        packageCacheSize.value = cacheInfo.size_human || "0 B";
    } catch (e) {
        console.error("Failed to load cache size", e);
    }
}

onMounted(async () => {
    try {
        const contMap = await list_containers();
        containers.value = Object.values(contMap);
        containersLoaded.value = true;
    } catch (e) {
        console.error("Failed to load containers", e);
    }
    await loadSizes();
});
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
                    <span>Total Backups Size: {{ totalBackupSize }}</span>
                    <span>Package Cache Size: {{ packageCacheSize }}</span>
                </div>
            </v-row>
        </v-container>

        <!-- List of containers with backup button -->
        <div v-if="containersLoaded" class="flex flex-wrap gap-3 justify-center">
            <div v-for="container in containers" :key="container.id" class="bg-[#333337] p-4 rounded-lg flex flex-col gap-2 w-64">
                <div class="text-white text-lg">{{ container.name }}</div>
                <div class="text-[#b8b8b8] text-sm">ID: {{ container.id }}</div>
                <div class="text-[#b8b8b8] text-sm">State: {{ container.state }}</div>
                <v-btn size="small" color="primary" @click="triggerBackup(container.id)">
                    Backup Now
                </v-btn>
            </div>
        </div>
        <div v-else class="flex justify-center">
            <v-progress-circular indeterminate />
        </div>

        <BackupContainerWithController />
    </div>
</template>