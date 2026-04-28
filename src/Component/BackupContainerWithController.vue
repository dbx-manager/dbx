<script setup lang="ts">
import { ref, onMounted } from "vue";
import ContainerCard from "./container/ContainerCard.vue";
import { invoke } from "@tauri-apps/api/core";
import { deleteBackup, recreateContainerFromBackup } from "../../Functions/BackupService";
import { useSettingsStore } from "../../stores/useSettingsStore";

const props = defineProps();
const backup_files = ref<BackupFileList | null>(null);
const loaded = ref<boolean>(false);
const errorMessage = ref<string | null>(null);
const settings = useSettingsStore();

async function get_backup_directory_list() {
    backup_files.value = await invoke("get_backup_directory_list");
    console.log(await invoke("get_backup_config"));
}

async function handleDelete(backupPath: string) {
    try {
        await deleteBackup(backupPath);
        // Refresh list
        await get_backup_directory_list();
    } catch (e) {
        console.error("Delete failed:", e);
    }
}

async function handleRestore(backupPath: string, containerName: string) {
    try {
        const newName = prompt("Enter new container name:", `restored_${containerName}`);
        if (newName) {
            await recreateContainerFromBackup(backupPath, newName);
            alert(`Container ${newName} created from backup.`);
        }
    } catch (e) {
        console.error("Restore failed:", e);
    }
}

onMounted(async () => {
    try {
        await get_backup_directory_list().then(() => (loaded.value = true));
    } catch (error: any) {
        loaded.value = true;
        errorMessage.value = error;
    }
});
</script>

<template>
    <div v-if="!loaded" class="flex justify-center items-center">
        <v-progress-circular indeterminate />
    </div>
    <p v-if="errorMessage" class="text-center text-red-300">Error Happened while reading=>{{ errorMessage }} </p>
    <!-- <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-3"> -->
    <div v-if="loaded" class="flex flex-wrap gap-3 justify-center">
        <container-card v-if="backup_files != null" v-for="value in backup_files.files" :container-name="value.name"
            class="w-fit!">
            <div class="flex flex-row gap-1! text-[#b3b3b3]">
                <!-- Extract container name from file name (format: containerId_timestamp.tar) -->
                <p>Container: {{ value.name.split('_')[0] }}</p>
                <p>Size: {{ value.size_human }}</p>
            </div>
            <div class="flex flex-row justify-between items-center gap-2!">
                <div class="flex gap-1">
                    <v-btn size="small" flat color="#bd1919" rounded="lg" class="h-9!" @click="handleDelete(value.path)">Delete</v-btn>
                    <v-btn text="Restore" flat color="#651d9d" rounded="lg" size="small" class="absolute h-9!" @click="handleRestore(value.path, value.name)" />
                </div>
            </div>
        </container-card>
    </div>
</template>