<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from "vue";
import uPlot from "uplot";
import "uplot/dist/uPlot.min.css";
import { usePerformanceHistoryStore } from "../stores/performanceHistoryStore";

const props = defineProps<{
  containerId: string;
  series?: ("cpu" | "mem")[];
  height?: number;
}>();

const chartContainer = ref<HTMLDivElement | null>(null);
const perfHistory = usePerformanceHistoryStore();
let chart: uPlot | null = null;

const activeSeries = props.series ?? ["cpu", "mem"];

const opts: uPlot.Options = {
  width: 400,
  height: props.height ?? 200,
  cursor: {
    show: true,
    drag: { x: true, y: true },
  },
  select: {
    show: true,
    left: 0,
    top: 0,
    width: 0,
    height: 0,
  },
  axes: [
    {
      stroke: "#888",
      grid: { stroke: "rgba(255,255,255,0.05)" },
      ticks: { stroke: "rgba(255,255,255,0.1)" },
    },
    {
      stroke: "#888",
      grid: { stroke: "rgba(255,255,255,0.05)" },
      ticks: { stroke: "rgba(255,255,255,0.1)" },
    },
  ],
  series: [
    {},
    ...(activeSeries.includes("cpu")
      ? [
          {
            label: "CPU %",
            stroke: "#4fc3f7",
            fill: "rgba(79,195,247,0.15)",
            width: 2,
          } as uPlot.Series,
        ]
      : []),
    ...(activeSeries.includes("mem")
      ? [
          {
            label: "Memory %",
            stroke: "#81c784",
            fill: "rgba(129,199,132,0.15)",
            width: 2,
          } as uPlot.Series,
        ]
      : []),
  ],
};

function buildData(): uPlot.AlignedData {
  const data = perfHistory.getUplotData(props.containerId, true);
  if (!data) return [[], [], []];

  const [timestamps, avg_cpu, , perc_memory] = data;

  const cols: uPlot.AlignedData = [timestamps];

  if (activeSeries.includes("cpu")) {
    cols.push(avg_cpu);
  }
  if (activeSeries.includes("mem")) {
    cols.push(perc_memory);
  }

  return cols;
}

function initChart() {
  if (!chartContainer.value) return;

  destroyChart();

  const container = chartContainer.value;
  const rect = container.getBoundingClientRect();
  opts.width = rect.width || 400;
  opts.height = props.height ?? 200;

  chart = new uPlot(opts, buildData(), container);
}

function updateChart() {
  if (!chart) return;
  const uplotData = buildData();
  chart.setData(uplotData);
}

function destroyChart() {
  if (chart) {
    chart.destroy();
    chart = null;
  }
}

let resizeObserver: ResizeObserver | null = null;

onMounted(() => {
  if (chartContainer.value) {
    initChart();

    resizeObserver = new ResizeObserver(() => {
      if (chartContainer.value) {
        const rect = chartContainer.value.getBoundingClientRect();
        if (rect.width > 0 && rect.height > 0) {
          chart?.setSize({ width: rect.width, height: props.height ?? 200 });
        }
      }
    });
    resizeObserver.observe(chartContainer.value);
  }
});

onUnmounted(() => {
  destroyChart();
  if (resizeObserver) {
    resizeObserver.disconnect();
    resizeObserver = null;
  }
});

// Watch for new data — update chart reactively
watch(
  () => perfHistory.historyMap,
  () => {
    updateChart();
  },
  { deep: true },
);
</script>

<template>
  <div ref="chartContainer" class="uplot-wrapper" />
</template>

<style scoped>
.uplot-wrapper {
  width: 100%;
  min-height: v-bind("props.height ? props.height + 'px' : '200px'");
}

.uplot-wrapper :deep(.u-title) {
  font-size: 13px;
  font-weight: 600;
  color: #ccc;
}

.uplot-wrapper :deep(.u-label) {
  color: #888;
  font-size: 11px;
}
</style>