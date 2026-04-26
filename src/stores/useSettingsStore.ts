import { ref } from "vue";
import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { UnlistenFn } from "@tauri-apps/api/event";


interface Config {
  backup_containers: string[];
  backup_location_path: string;
  cron_schedule: string;
  package_cache_path: string;
  autostart_containers: string[];
}

export interface AppSettings {
  updateInterval: number;
  autoUpdateEnabled: boolean;
  backup_containers: Set<string>;
  autostart_containers: Set<string>;
  cron_schedule: string;
  packageCachePath: string;
  backupLocationPath: string;
}


const mapConfigToSettings = (config: Config): Partial<AppSettings> => ({
  backup_containers: new Set<string>(config.backup_containers),
  autostart_containers: new Set<string>(config.autostart_containers),
  cron_schedule: config.cron_schedule,
  packageCachePath: config.package_cache_path,
  backupLocationPath: config.backup_location_path,
});

const mapSettingsToConfig = (settings: AppSettings): Config => ({
  backup_containers: Array.from(settings.backup_containers),
  autostart_containers: Array.from(settings.autostart_containers),
  cron_schedule: settings.cron_schedule,
  package_cache_path: settings.packageCachePath,
  backup_location_path: settings.backupLocationPath,
});

export const useSettingsStore = defineStore("settings", () => {

  const settings = ref<AppSettings>({
    updateInterval: 5000,
    autoUpdateEnabled: true,
    backup_containers: new Set<string>(),
    autostart_containers: new Set<string>(),
    cron_schedule: "* * * * *",
    packageCachePath: "",
    backupLocationPath: "",
  });

  const unlistenFn = ref<UnlistenFn | null>(null);
  let syncTimeout: ReturnType<typeof setTimeout> | null = null;

  const initialize = async () => {
    try {
      const backendConfig = await invoke<Config>("get_backup_config");
      Object.assign(settings.value, mapConfigToSettings(backendConfig));
      
      // Setup auto update listener after initial load
      if (settings.value.autoUpdateEnabled) {
        await setupAutoUpdateListener();
      }
    } catch (error) {
      console.error("Failed to initialize settings from backend:", error);
    }
  };

  const setupAutoUpdateListener = async () => {
    if (unlistenFn.value) {
      unlistenFn.value();
    }

    unlistenFn.value = await listen<Config>("config-updated", (event) => {
      if (settings.value.autoUpdateEnabled) {
        Object.assign(settings.value, mapConfigToSettings(event.payload));
      }
    });
  };

  const cleanup = () => {
    if (unlistenFn.value) {
      unlistenFn.value();
      unlistenFn.value = null;
    }
    if (syncTimeout) {
      clearTimeout(syncTimeout);
    }
  };

  const syncToBackend = () => {
    if (syncTimeout) {
      clearTimeout(syncTimeout);
    }
    console.log()
    console.log(settings.value.autostart_containers)
    console.log(settings.value.backup_containers)

    syncTimeout = setTimeout(async () => {
      try {
        await invoke("update_backup_config", {
          newConfig: mapSettingsToConfig(settings.value),
        });
      } catch (error) {
        console.warn("Failed to sync settings to backend:", error);
      }
      await invoke("match_config_container");
    }, 150);
  };

  const setUpdateInterval = (interval: number) => {
    settings.value.updateInterval = interval;
  };

  const setAutoUpdateEnabled = async (enabled: boolean) => {
    settings.value.autoUpdateEnabled = enabled;
    if (enabled) {
      await setupAutoUpdateListener();
    } else {
      cleanup();
    }
  };

  const setBackupContainers = (containers: Set<string>) => {
    settings.value.backup_containers = containers;
    syncToBackend();
  };

  const setAutostartContainers = (containers: Set<string>) => {
    settings.value.autostart_containers = containers;
    syncToBackend();
  };

  const setCronSchedule = (schedule: string ) => {
    settings.value.cron_schedule = schedule;
    syncToBackend();
  };

  const setPackageCachePath = (path: string) => {
    settings.value.packageCachePath = path;
    syncToBackend();
  };

  const setBackupLocationPath = (path: string) => {
    settings.value.backupLocationPath = path;
    syncToBackend();
  };

  const resetToDefaults = async () => {
    try {
      await invoke("reset_config_defaults");
      await initialize();
    } catch (error) {
      console.warn("Failed to reset settings:", error);
    }
  };

  // Initialize automatically when store is first used
  initialize();

  return {
    settings,
    setUpdateInterval,
    setAutoUpdateEnabled,
    setBackupContainers,
    setAutostartContainers,
    setCronSchedule,
    setPackageCachePath,
    setBackupLocationPath,
    resetToDefaults,
    cleanup,
    
    // Backwards compatibility aliases for existing code
    setcron_schedule: setCronSchedule,
    setContainers: setBackupContainers,
  };
});