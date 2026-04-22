<script setup lang="ts">
import { ref } from "vue";
import ContainerCard from "./container/ContainerCard.vue";
import { onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

const props = defineProps();
const backup_files = ref<BackupFileList | null>(null);
const loaded = ref<boolean>(false);
async function get_backup_directory_list() {
    backup_files.value = await invoke("get_backup_directory_list");
}
onMounted(async () => {
    await get_backup_directory_list().then(() => (loaded.value = true));
});

onUnmounted(() => {});
</script>

<template>
    <div v-if="!loaded" class="flex justify-center items-center">
        <v-progress-circular indeterminate />
    </div>
    <!-- <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-3"> -->
    <div v-if="loaded" class="flex flex-wrap gap-3 justify-center">
        <container-card
            v-if="backup_files != null"
            v-for="value in backup_files.files"
            :container-name="value.name"
            class="w-fit!"
        >
            <div class="flex flex-row gap-1! text-[#b3b3b3]">
                <!-- TODO add the date into the returned data from the backend -->
                <p>Backed at:</p>
            </div>
            <div class="text-[#b3b3b3]">Size: {{ value.size_bytes }}</div>
            <div class="flex flex-row justify-between items-center gap-2!">
                <div class="flex gap-1">
                    <v-btn
                        size="small"
                        flat
                        color="#bd1919"
                        rounded="lg"
                        class="h-9!"
                        >Delete</v-btn
                    >
                    <v-btn
                        text="Restore"
                        flat
                        color="#651d9d"
                        rounded="lg"
                        size="small"
                        class="absolute h-9!"
                    />
                </div>
            </div>
        </container-card>
    </div>
</template>
