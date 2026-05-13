<script setup lang="ts">
import "../../Functions/ListingContaienrs";
import { onMounted, onUnmounted } from "vue";
import MoreMenuActionButton from "./MoreMenuActionButton.vue";
import { ContainerListState } from "../../stores/ContainerListState";
import ContainerController from "./ContainerController.vue";
import { useSettingsStore } from "../../stores/useSettingsStore";
import { usePerformanceHistoryStore } from "../../stores/performanceHistoryStore";
import CpuLineChart from "../CpuLineChart.vue";
import MemLineChart from "../MemLineChart.vue";
const props = defineProps();
const containerListState = ContainerListState();
const s = useSettingsStore()
const perfHistory = usePerformanceHistoryStore()

function getCpuData(containerId: string): number[][] {
  const data = perfHistory.getUplotData(containerId, true);
  if (!data) return [[], []];
//   console.log(data[0])
  return [data[0], data[1]];
}

function getMemData(containerId: string): number[][] {
  const data = perfHistory.getUplotData(containerId, true);
  if (!data) return [[], []];
  return [data[0], data[3]];
}

function getMemBytesRaw(containerId: string): number[] {
  const data = perfHistory.getUplotData(containerId, true);
  if (!data) return [];
  return data[4] ?? [];
}

onMounted(()=>{
containerListState.startAutoRefresh()

})
onUnmounted(()=>{
containerListState.stopAutoRefresh()
    
})
function add_container_to_auto_backup(event: Event,containerId:string) {
    const target = event.target as HTMLInputElement;
    const isOn: boolean = target.checked;

    if (isOn) {
        s.settings.backup_containers.add(containerId)
        s.setBackupContainers(s.settings.backup_containers)
    }
    else {
        s.settings.backup_containers.delete(containerId)
        s.setBackupContainers(s.settings.backup_containers)
    }

}
function add_container_to_auto_start(event: Event,containerId:string) {
    const target = event.target as HTMLInputElement;
    const isOn: boolean = target.checked;
    if (isOn) {
        s.settings.autostart_containers.add(containerId)
        s.setAutostartContainers(s.settings.autostart_containers)
    }
    else {
        s.settings.autostart_containers.delete(containerId)
        s.setAutostartContainers(s.settings.autostart_containers)
    }
}
</script>

<template>
    <v-container v-for="container in containerListState.containerList"
        class="bg-[#333337] rounded-lg flex flex-row max-w-full!  gap-1 pr-0 ">
        <div class="w-[20%]!" >

            <div class=" bg-transparent grid grid-cols-2 gap-1">
                <div class=" flex flex-row col-span-2  items-start gap-1">
                    <v-tooltip location="top" activator="parent" :text="container.state.charAt(0).toUpperCase()+container.state.substring(1)"/>
                    <v-icon  size="x-large" :color="container.state.match('running')?'#00a63e':'red'">mdi-cube-outline</v-icon>
                    <div class="flex justify-center items-center flex-wrap ">
                    <span class="text-white text-2xl font-light " >{{
                        container.name
                    }}</span>
        </div>
            </div>
            <container-controller :containerId="container.id" :status="container.state" :autoBackupCallback="add_container_to_auto_backup" :autoStartCallback="add_container_to_auto_start"
                :autoBackup="container.autobackup??false" :autoStartup="container.autostart??false"/>
        </div>
    </div>


        <div class="w-[80%] grid grid-cols-2 gap-2">
            <div class="bg-[#1a1a1c] rounded-lg p-1">
                <CpuLineChart :data="getCpuData(container.id)" :height="170" />
            </div>
            <div class="bg-[#1a1a1c] rounded-lg p-1">
                <MemLineChart :data="getMemData(container.id)" :raw-bytes="getMemBytesRaw(container.id)" :height="170" />
            </div>
        </div>
        
        <more-menu-action-button :containerId="container.id"></more-menu-action-button>

    </v-container>
</template>
