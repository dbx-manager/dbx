import { ref, watch } from 'vue';

interface AppSettings {
  updateInterval: number; // in milliseconds
  autoUpdateEnabled: boolean;
}

const defaultSettings: AppSettings = {
  updateInterval: 5000, // 5 seconds default
  autoUpdateEnabled: true
};

// Load settings from localStorage or use defaults
const loadSettings = (): AppSettings => {
  try {
    const saved = localStorage.getItem('dbx-settings');
    if (saved) {
      const parsed = JSON.parse(saved);
      return {
        updateInterval: parsed.updateInterval || defaultSettings.updateInterval,
        autoUpdateEnabled: parsed.autoUpdateEnabled !== undefined ? parsed.autoUpdateEnabled : defaultSettings.autoUpdateEnabled
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

  // Watch for changes and save to localStorage
  watch(settings, (newSettings) => {
    saveSettings(newSettings);
  }, { deep: true });

  const setUpdateInterval = (interval: number) => {
    settings.value.updateInterval = interval;
  };

  const setAutoUpdateEnabled = (enabled: boolean) => {
    settings.value.autoUpdateEnabled = enabled;
  };

  const resetToDefaults = () => {
    settings.value = { ...defaultSettings };
  };

  return {
    settings: settings.value,
    setUpdateInterval,
    setAutoUpdateEnabled,
    resetToDefaults
  };
}