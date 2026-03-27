import { ref, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { UnlistenFn } from '@tauri-apps/api/event';

interface BackendBackupConfig {
  containers: string[];
  export_path: string;
  cron_schedule: string | null;
}

interface AppSettings {
  updateInterval: number; // in milliseconds
  autoUpdateEnabled: boolean;
  containers: string[];
  exportPath: string;
  cronSchedule: string | null;
}

const defaultSettings: AppSettings = {
  updateInterval: 5000, // 5 seconds default
  autoUpdateEnabled: true,
  containers: [],
  exportPath: '/home/user/dbx-backup/containers',
  cronSchedule: null
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
        exportPath: parsed.exportPath || defaultSettings.exportPath,
        cronSchedule: parsed.cronSchedule !== undefined ? parsed.cronSchedule : defaultSettings.cronSchedule
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
        exportPath: backendConfig.export_path || settings.value.exportPath,
        cronSchedule: backendConfig.cron_schedule || settings.value.cronSchedule
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
          exportPath: newConfig.export_path || settings.value.exportPath,
          cronSchedule: newConfig.cron_schedule || settings.value.cronSchedule
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

  const setExportPath = (path: string) => {
    settings.value.exportPath = path;
    // Sync to backend
    syncToBackend();
  };

  const setCronSchedule = (schedule: string | null) => {
    settings.value.cronSchedule = schedule;
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
          export_path: settings.value.exportPath,
          cron_schedule: settings.value.cronSchedule
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
    setExportPath,
    setCronSchedule,
    resetToDefaults,
    cleanup
  };
}
