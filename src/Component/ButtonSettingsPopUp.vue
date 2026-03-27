<script setup lang="ts">
import { useSettings } from "../composables/useSettings";
import { ref } from "vue";

const { settings, setUpdateInterval, setAutoUpdateEnabled } = useSettings();

// Available interval options in milliseconds
const intervalOptions = [
  { label: '1 second', value: 1000 },
  { label: '5 seconds', value: 5000 },
  { label: '10 seconds', value: 10000 },
  { label: '30 seconds', value: 30000 },
  { label: '1 minute', value: 60000 },
  { label: '5 minutes', value: 300000 }
];

// Get the current interval label for display
const currentIntervalLabel = ref(intervalOptions.find(opt => opt.value === settings.updateInterval)?.label || 'Custom');

const handleIntervalChange = (value: number) => {
  setUpdateInterval(value);
  currentIntervalLabel.value = intervalOptions.find(opt => opt.value === value)?.label || 'Custom';
};

const toggleAutoUpdate = () => {
  setAutoUpdateEnabled(!settings.autoUpdateEnabled);
};
</script>

<template>
  <div class="bg-[#2a2a2e] rounded-lg p-4 border border-[#3a3a3e]">
    <div class="flex flex-col gap-4">
      <div class="flex flex-row justify-between items-center gap-6">
        <h3 class="text-white text-lg font-semibold">Container Updates</h3>
        <v-btn
          :color="settings.autoUpdateEnabled ? '#188251ff' : '#9a2a2aff'"
          :text="settings.autoUpdateEnabled ? 'Auto Update: ON' : 'Auto Update: OFF'"
          @click="toggleAutoUpdate"
          rounded="lg"
          flat
          class="text-sm font-medium"
        />
      </div>
      
      <div class="flex flex-col gap-2">
        <label class="text-gray-300 text-sm font-medium">Update Interval</label>
        <v-select
          :model-value="settings.updateInterval"
          :items="intervalOptions"
          item-title="label"
          item-value="value"
          hide-details
          @update:model-value="handleIntervalChange"
          density="compact"
          variant="outlined"
          class="bg-[#1f1f23] rounded "
          :disabled="!settings.autoUpdateEnabled"
        />
      </div>


      <div class="text-xs text-gray-400">
        Current status: 
        <span :class="settings.autoUpdateEnabled ? 'text-green-400' : 'text-red-400'">
          {{ settings.autoUpdateEnabled ? 'Active' : 'Paused' }}
        </span>
        <span v-if="settings.autoUpdateEnabled" class="ml-2">
          (Updates every {{ Math.round(settings.updateInterval / 1000) }} seconds)
        </span>
      </div>
    </div>
  </div>
</template>

<style scoped>
.v-select :deep(.v-field__input) {
  color: #e5e7eb;
}

.v-select :deep(.v-field__outline) {
  color: #9ca3af;
}
</style>