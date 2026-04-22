import { ref, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { UnlistenFn } from '@tauri-apps/api/event';

interface BackendBackupConfig {
  containers: string[];
  backup_location_path: string;
  cron_schedule: string | null;
  package_cache_path: string;
}

export interface AppSettings {
  updateInterval: number; // in milliseconds
  autoUpdateEnabled: boolean;
  containers: string[];
  cronSchedule: string | null;
  packageCachePath: string;
  backupLocationPath: string;
}

const defaultSettings: AppSettings = {
  updateInterval: 5000, // 5 seconds default
  autoUpdateEnabled: true,
  containers: [],
  cronSchedule: null,
  packageCachePath: '/home/user/.cache/dbx/',
  backupLocationPath: '/home/user/dbx-backup/'
};

// Load settings from localStorage or use defaults
const loadSettings = (): AppSettings => {
  try {
    const saved = localStorage.getItem('dbx-settings');
    if (saved) {
      const parsed = JSON.parse(saved);
      return {
        updateInterval: parsed.updateInterval || defaultSettings.updateInterval,
        autoUpdateEnabled: parsed.autoUpdateEnabled !== undefined ? parsed.autoUpdateEnabled : defaultSettings.autoUpdateEnabled,
        containers: parsed.containers || defaultSettings.containers,
        cronSchedule: parsed.cronSchedule !== undefined ? parsed.cronSchedule : defaultSettings.cronSchedule,
        packageCachePath: parsed.packageCachePath || defaultSettings.packageCachePath,
        backupLocationPath: parsed.backupLocationPath || defaultSettings.backupLocationPath
      };
    }
  } catch (error) {
    console.warn('Failed to load settings from localStorage:', error);
  }
  return { ...defaultSettings };
};

// Save settings to localStorage
const saveSettings = (settings: AppSettings) => {
  try {
    localStorage.setItem('dbx-settings', JSON.stringify(settings));
  } catch (error) {
    console.warn('Failed to save settings to localStorage:', error);
  }
};

export function useSettings() {
  const settings = ref<AppSettings>(loadSettings());
  const unlistenFn = ref<UnlistenFn | null>(null);

  // Initialize settings with backend config
  const initializeFromBackend = async () => {
    try {
      const backendConfig = await invoke<BackendBackupConfig>('get_backup_config');
      settings.value = {
        ...settings.value,
        containers: backendConfig.containers || settings.value.containers,
        cronSchedule: backendConfig.cron_schedule || settings.value.cronSchedule,
        packageCachePath: backendConfig.package_cache_path || settings.value.packageCachePath,
        backupLocationPath: backendConfig.backup_location_path || settings.value.backupLocationPath
      };
    } catch (error) {
      console.warn('Failed to load config from backend, using localStorage defaults:', error);
    }
  };

  // Setup auto-update listener
  const setupAutoUpdate = async () => {
    if (unlistenFn.value) {
      unlistenFn.value();
    }

    unlistenFn.value = await listen<BackendBackupConfig>('config-updated', (event) => {
      if (settings.value.autoUpdateEnabled) {
        const newConfig = event.payload;
        settings.value = {
          ...settings.value,
          containers: newConfig.containers || settings.value.containers,
          cronSchedule: newConfig.cron_schedule || settings.value.cronSchedule,
          packageCachePath: newConfig.package_cache_path || settings.value.packageCachePath,
          backupLocationPath: newConfig.backup_location_path || settings.value.backupLocationPath
        };
      }
    });
  };

  // Cleanup listener on component unmount
  const cleanup = () => {
    if (unlistenFn.value) {
      unlistenFn.value();
      unlistenFn.value = null;
    }
  };

  // Watch for changes and save to localStorage
  watch(settings, (newSettings) => {
    saveSettings(newSettings);
  }, { deep: true });

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

  const setContainers = (containers: string[]) => {
    settings.value.containers = containers;
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
    try {
      await invoke('update_backup_config', {
        newConfig: {
          containers: settings.value.containers,
          cron_schedule: settings.value.cronSchedule,
          package_cache_path: settings.value.packageCachePath,
          backup_location_path: settings.value.backupLocationPath
        }
      });
    } catch (error) {
      console.warn('Failed to sync settings to backend:', error);
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
    setContainers,
    setCronSchedule,
    setPackageCachePath,
    setBackupLocationPath,
    resetToDefaults,
    cleanup
  };
}
