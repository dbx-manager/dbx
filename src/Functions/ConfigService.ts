import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

export interface BackupConfig {
  containers: string[];
  export_path: string;
  cron_schedule: string;
}

// Get current configuration
export async function getBackupConfig(): Promise<BackupConfig> {
  return await invoke("get_backup_config");
}

// Update configuration
export async function setBackupConfig(config: BackupConfig): Promise<void> {
  return await invoke("set_backup_config", { config });
}

// Listen for configuration changes (real-time updates)
export function onConfigUpdated(callback: (config: BackupConfig) => void) {
  return listen("config-updated", (event) => {
    callback(event.payload as BackupConfig);
  });
}

// Example usage in a Vue component:
/*
<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { getBackupConfig, setBackupConfig, onConfigUpdated } from '@/Functions/ConfigService';

const config = ref<BackupConfig>({
  containers: [],
  export_path: '',
  cron_schedule: ''
});

const unlisten = ref<() => void | undefined>();

onMounted(async () => {
  // Load initial config
  config.value = await getBackupConfig();
  
  // Listen for real-time updates
  unlisten.value = await onConfigUpdated((newConfig) => {
    console.log('Config updated:', newConfig);
    config.value = newConfig;
  });
});

onUnmounted(() => {
  // Clean up listener
  if (unlisten.value) {
    unlisten.value();
  }
});

// Update config example
const updateConfig = async () => {
  const newConfig: BackupConfig = {
    containers: ['container1', 'container2'],
    export_path: '/new/export/path',
    cron_schedule: '0 3 * * * *'
  };
  
  await setBackupConfig(newConfig);
};
</script>
*/