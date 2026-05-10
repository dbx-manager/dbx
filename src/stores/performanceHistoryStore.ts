import { ref } from "vue";
import { defineStore } from "pinia";
import { ContainerListState } from "./ContainerListState";
import type { Performance } from "../types/ListContainer";

export interface PerContainerHistory {
  timestamps: number[];
  avg_cpu: number[];
  perc_cpu: number[];
  perc_memory: number[];
  avg_memory: number[];
}

export const usePerformanceHistoryStore = defineStore(
  "performanceHistory",
  () => {
    const historyMap = ref<Map<string, PerContainerHistory>>(new Map());
    const maxHistoryLength = ref<number>(300);
    const captureIntervalMs = ref<number>(1000);
    const intervalId = ref<ReturnType<typeof setInterval> | null>(null);
    const isRunning = ref<boolean>(false);

    function start() {
      if (isRunning.value) return;
      isRunning.value = true;
      intervalId.value = setInterval(async () => {
        const containerListState = ContainerListState();
        await containerListState.fetchContainerList();

        const containers = containerListState.containerList;

        for (const container in containers) {
          if (!container) continue;
          const perf = containers[container].performance as Performance;
          const existing = historyMap.value.get(containers[container].id);

          const snapshot = {
            timestamp: Date.now(),
            avg_cpu: Math.round((perf.avg_cpu ?? 0) * 100) / 100,
            perc_cpu: Math.round((perf.perc_cpu ?? 0) * 100) / 100,
            perc_memory: Math.round((perf.perc_memory ?? 0) * 100) / 100,
            avg_memory: perf.avg_memory ?? 0,
          };

          if (existing) {
            existing.timestamps.push(snapshot.timestamp);
            existing.avg_cpu.push(snapshot.avg_cpu);
            existing.perc_cpu.push(snapshot.perc_cpu);
            existing.perc_memory.push(snapshot.perc_memory);
            existing.avg_memory.push(snapshot.avg_memory);

            // Trim excess entries (FIFO)
            if (existing.timestamps.length > maxHistoryLength.value) {
              const excess =
                existing.timestamps.length - maxHistoryLength.value;
              existing.timestamps.splice(0, excess);
              existing.avg_cpu.splice(0, excess);
              existing.perc_cpu.splice(0, excess);
              existing.perc_memory.splice(0, excess);
              existing.avg_memory.splice(0, excess);
            }
          } else {
            historyMap.value.set(containers[container].id, {
              timestamps: [snapshot.timestamp],
              avg_cpu: [snapshot.avg_cpu],
              perc_cpu: [snapshot.perc_cpu],
              perc_memory: [snapshot.perc_memory],
              avg_memory: [snapshot.avg_memory],
            });
          }
        }

        // Force reactivity by replacing the map reference
        historyMap.value = new Map(historyMap.value);
      }, captureIntervalMs.value);
    }

    function stop() {
      if (intervalId.value) {
        clearInterval(intervalId.value);
        intervalId.value = null;
      }
      isRunning.value = false;
    }

    function clear() {
      historyMap.value = new Map();
    }

    function getHistoryForContainer(
      id: string,
    ): PerContainerHistory | undefined {
      return historyMap.value.get(id);
    }

    /**
     * Returns data in uPlot-compatible columnar format:
     * [timestamps[], avg_cpu[], perc_cpu[], perc_memory[]]
     *
     * Timestamps are in milliseconds.
     * uPlot by default expects seconds; pass `ms: true` in the uPlot options
     * or divide timestamps by 1000 before passing.
     *
     * @param id Container ID
     * @param asSeconds If true, timestamps are divided by 1000 for uPlot default
     */
    function getUplotData(
      id: string,
      asSeconds: boolean = true,
    ): [number[], number[], number[], number[], number[]] | null {
      const history = historyMap.value.get(id);
      if (!history || history.timestamps.length === 0) return null;

      const timestamps = asSeconds
        ? history.timestamps.map((t) => t / 1000)
        : [...history.timestamps];

      return [
        timestamps,
        [...history.avg_cpu],
        [...history.perc_cpu],
        [...history.perc_memory],
        [...history.avg_memory],
      ];
    }

    function setMaxHistoryLength(n: number) {
      maxHistoryLength.value = n;
      // Trim existing histories to the new length
      for (const [, history] of historyMap.value) {
        if (history.timestamps.length > n) {
          const excess = history.timestamps.length - n;
          history.timestamps.splice(0, excess);
          history.avg_cpu.splice(0, excess);
          history.perc_cpu.splice(0, excess);
          history.perc_memory.splice(0, excess);
          history.avg_memory.splice(0, excess);
        }
      }
      historyMap.value = new Map(historyMap.value);
    }

    function setCaptureInterval(ms: number) {
      captureIntervalMs.value = ms;
      if (isRunning.value) {
        stop();
        start();
      }
    }

    return {
      historyMap,
      maxHistoryLength,
      captureIntervalMs,
      isRunning,
      start,
      stop,
      clear,
      getHistoryForContainer,
      getUplotData,
      setMaxHistoryLength,
      setCaptureInterval,
    };
  },
);
