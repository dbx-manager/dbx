<script setup lang="ts">
import { ref } from "vue";
import ContainerCard from "./container/ContainerCard.vue";
import { onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { delete_backup, postRestoreBackup } from "../Functions/BackupService";

const props = defineProps();
const backup_files = ref<BackupFileList | null>(null);
const loaded = ref<boolean>(false);
const errorMessage = ref<string | null>(null)
async function get_backup_directory_list() {
    backup_files.value = await invoke("get_backup_directory_list");
    console.log(await invoke("get_backup_config"));
}
onMounted(async () => {
    try {
        await get_backup_directory_list().then(() => (loaded.value = true));

    } catch (error: any) {
        loaded.value = true;
        errorMessage.value = error;
        // console.log(errorMessage.value)
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
        <container-card v-if="backup_files != null" v-for="file in backup_files.files" :container-name="file.name"
            class="w-fit!" :is-open="true">
            <div class="flex flex-row gap-1! text-[#b3b3b3]">
                <!-- TODO add the date into the returned data from the backend -->
                <p>Backed at:</p>
            </div>
            <div class="text-[#b3b3b3]">Size: {{ file.size_human }}</div>
            <div class="flex flex-row justify-between items-center gap-2!">
                <div class="flex gap-1">
                    <v-btn text="Delete" size="small" flat color="#bd1919" rounded="lg" class="h-9!" @Click="delete_backup(file.path)"/>
                    <v-btn text="Restore" flat color="#651d9d" rounded="lg" size="small" class="absolute h-9!" @Click=" postRestoreBackup(file.path)" />
                </div>
            </div>
        </container-card>
    </div>
</template>
