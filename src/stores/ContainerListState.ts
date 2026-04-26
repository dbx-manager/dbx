import { ref, onUnmounted } from "vue";
import { Container } from "../types/ListContainer";
import { defineStore } from "pinia";
import { list_containers } from "../Functions/ListingContaienrs";
import { get_exported_apps, get_system_apps } from "../Functions/AppsService";
import { invoke } from "@tauri-apps/api/core";
import { useSettingsStore } from "./useSettingsStore";

export const ContainerListState = defineStore("ContainerListState", () => {
  const containerList = ref<Container[]>([]);
  const settings = useSettingsStore();
  const refreshInterval = ref<ReturnType<typeof setInterval> | null>(null);
  const intervalDuration = ref<number>(1000); // default to 1 second

  async function fetchContainerList() {
    // if (settings.settings.autoUpdateEnabled) return;
    const newList = await list_containers();
    containerList.value = newList;
    // console.log(newList)
    return containerList;
  }
  function fetchExportedApps() {
    //TODO add a error handiling
    (async () => await get_exported_apps())();
  }
  function fetchSystemApps(containerId:string) {
    //TODO add a error handiling
    (async () => await get_system_apps(containerId))();
  }

  function startAutoRefresh() {
    stopAutoRefresh();
    refreshInterval.value = setInterval(async () => {
      fetchContainerList();
      matchConfigWithContainerList()
    }, intervalDuration.value);
  }
async function matchConfigWithContainerList(){
  
await invoke("match_config_container")
}
  function stopAutoRefresh() {
    if (refreshInterval.value) {
      clearInterval(refreshInterval.value);
      refreshInterval.value = null;
    }
  }

  function setIntervalDuration(duration: number) {
    intervalDuration.value = duration;
    if (refreshInterval.value) {
      // Restart the interval with new duration
      startAutoRefresh();
    }
  }

  onUnmounted(() => {
    stopAutoRefresh();
  });

  return {
    containerList,
    settings,
    fetchExportedApps,
    fetchSystemApps,
    fetchContainerList,
    startAutoRefresh,
    stopAutoRefresh,
    setIntervalDuration,
  };
});
