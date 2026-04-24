import { ref, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { UnlistenFn } from "@tauri-apps/api/event";
  //this is the class that will resceve the data from the backend
interface Config {
  backup_containers: string[];
  backup_location_path: string;
  cron_schedule: string | null;
  package_cache_path: string;
  autostart_containers: string[];
}

export interface AppSettings {
  updateInterval: number; // in milliseconds
  autoUpdateEnabled: boolean;
  backup_containers: Set<string>;
  autostart_containers: Set<string>;
  cronSchedule: string | null;
  packageCachePath: string;
  backupLocationPath: string;
}

const defaultSettings: AppSettings = {
  updateInterval: 5000, // 5 seconds default
  autoUpdateEnabled: true,
  backup_containers: new Set<string>([]),
  autostart_containers: new Set<string>([]),
  cronSchedule: null,
  packageCachePath: "/home/user/.cache/dbx/",
  backupLocationPath: "/home/user/dbx-backup/",
};

// Load settings from localStorage or use defaults
const loadSettings = (): AppSettings => {
  localStorage.clear()
  try {
    localStorage.clear()
    const saved = localStorage.getItem("dbx-settings");
    if (saved) {
      const parsed = JSON.parse(saved);
      return {
        updateInterval: parsed.updateInterval || defaultSettings.updateInterval,
        autoUpdateEnabled:
          parsed.autoUpdateEnabled !== undefined
            ? parsed.autoUpdateEnabled
            : defaultSettings.autoUpdateEnabled,
        backup_containers:
          parsed.backup_containers || defaultSettings.backup_containers,
        autostart_containers:
          parsed.autostart_containers || defaultSettings.autostart_containers,
        cronSchedule:
          parsed.cronSchedule !== undefined
            ? parsed.cronSchedule
            : defaultSettings.cronSchedule,
        packageCachePath:
          parsed.packageCachePath || defaultSettings.packageCachePath,
        backupLocationPath:
          parsed.backupLocationPath || defaultSettings.backupLocationPath,
      };
    }
  } catch (error) {
    console.warn("Failed to load settings from localStorage:", error);
  }
  return { ...defaultSettings };
};

// Save settings to localStorage
const saveSettings = (settings: AppSettings) => {
  try {
    localStorage.setItem("dbx-settings", JSON.stringify(settings));
  } catch (error) {
    console.warn("Failed to save settings to localStorage:", error);
  }
};

export function useSettings() {
  const settings = ref<AppSettings>(loadSettings());
  const unlistenFn = ref<UnlistenFn | null>(null);

  // Initialize settings with backend config
  const initializeFromBackend = async () => {
    try {
      const backendConfig =
        await invoke<Config>("get_backup_config");
      settings.value = {
        ...settings.value,
        backup_containers:
         new Set<string>(backendConfig.backup_containers) || settings.value.backup_containers,
        autostart_containers:
          new Set<string>(backendConfig.autostart_containers) ||
          defaultSettings.autostart_containers,

        cronSchedule:
          backendConfig.cron_schedule || settings.value.cronSchedule,
        packageCachePath:
          backendConfig.package_cache_path || settings.value.packageCachePath,
        backupLocationPath:
          backendConfig.backup_location_path ||
          settings.value.backupLocationPath,
      };
    } catch (error) {
      console.warn(
        "Failed to load config from backend, using localStorage defaults:",
        error,
      );
    }
  };

  // Setup auto-update listener
  const setupAutoUpdate = async () => {
    if (unlistenFn.value) {
      unlistenFn.value();
    }

    unlistenFn.value = await listen<Config>(
      "config-updated",
      (event) => {
        if (settings.value.autoUpdateEnabled) {
          const newConfig = event.payload;
          settings.value = {
            ...settings.value,
            backup_containers:
               new Set<string>(newConfig.backup_containers) || settings.value.backup_containers,
            autostart_containers:
              new Set<string>(newConfig.autostart_containers) ||
              defaultSettings.autostart_containers,

            cronSchedule:
              newConfig.cron_schedule || settings.value.cronSchedule,
            packageCachePath:
              newConfig.package_cache_path || settings.value.packageCachePath,
            backupLocationPath:
              newConfig.backup_location_path ||
              settings.value.backupLocationPath,
          };
        }
      },
    );
  };

  // Cleanup listener on component unmount
  const cleanup = () => {
    if (unlistenFn.value) {
      unlistenFn.value();
      unlistenFn.value = null;
    }
  };

  // Watch for changes and save to localStorage
  watch(
    settings,
    (newSettings) => {
      saveSettings(newSettings);
    },
    { deep: true },
  );

  const setUpdateInterval = (interval: number) => {
    settings.value.updateInterval = interval;
  };

  const setAutoUpdateEnabled = (enabled: boolean) => {
    settings.value.autoUpdateEnabled = enabled;
    if (enabled) {
      setupAutoUpdate();
    } else if (unlistenFn.value) {
      unlistenFn.value();
      unlistenFn.value = null;
    }
  };

  const setBackupContainers = (containers: Set<string>) => {
    settings.value.backup_containers = containers;
    // Sync to backend
    syncToBackend();
  };
  const setAutostartContainers = (containers: Set<string>) => {
    settings.value.autostart_containers = containers;
    // Sync to backend
    syncToBackend();
  };

  const setCronSchedule = (schedule: string | null) => {
    settings.value.cronSchedule = schedule;
    // Sync to backend
    syncToBackend();
  };

  const setPackageCachePath = (path: string) => {
    settings.value.packageCachePath = path;
    // Sync to backend
    syncToBackend();
  };

  const setBackupLocationPath = (path: string) => {
    settings.value.backupLocationPath = path;
    // Sync to backend
    syncToBackend();
  };

  const resetToDefaults = () => {
    settings.value = { ...defaultSettings };
    // Sync to backend
    syncToBackend();
  };

  // Sync current settings to backend
  const syncToBackend = async () => {
    let new_config:Config={
            backup_containers:  Array.from(settings.value.backup_containers),
            cron_schedule: settings.value.cronSchedule,
            package_cache_path: settings.value.packageCachePath,
            backup_location_path: settings.value.backupLocationPath,
            autostart_containers: Array.from(settings.value.autostart_containers),
          }
    try {
        await invoke("update_backup_config", {
          newConfig: new_config,
        })
    } catch (error) {
      console.warn("Failed to sync settings to backend:", error);
    }
  };

  // Initialize settings
  initializeFromBackend();

  // Setup auto-update if enabled
  if (settings.value.autoUpdateEnabled) {
    setupAutoUpdate();
  }

  return {
    settings: settings.value,
    setUpdateInterval,
    setAutoUpdateEnabled,
    setContainers: setBackupContainers,
    setCronSchedule,
    setAutostartContainers,
    setPackageCachePath,
    setBackupLocationPath,
    resetToDefaults,
    cleanup,
  };
}
