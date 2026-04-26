import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

export interface BackupConfig {
  containers: string[];
  backup_location_path: string;
  cron_schedule: string;
  autostart_containers: string[];
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
