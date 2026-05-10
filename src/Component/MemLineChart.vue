<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from "vue";
import uPlot from "uplot";
import "uplot/dist/uPlot.min.css";

const props = defineProps<{
  data: number[][];
  rawBytes: number[];
  height?: number;
}>();

function bytesToHuman(bytes: number): string {
  const UNITS = ["B", "KB", "MB", "GB", "TB"];
  let size = bytes;
  let unitIdx = 0;
  while (size >= 1024 && unitIdx < UNITS.length - 1) {
    size /= 1024;
    unitIdx++;
  }
  return `${size ? size.toFixed(2) : "--"} ${size ? UNITS[unitIdx] : ''}`;
}

const chartContainer = ref<HTMLDivElement | null>(null);
let chart: uPlot | null = null;

const opts: uPlot.Options = {
  width: 400,
  height: props.height ?? 200,
  legend: {
    
    isolate: true,
    live: true,
  },
  cursor: {
    show: true,
    
  },
  select: {
    show: true,
    left: 0,
    top: 0,
    width: 0,
    height: 0,
  },
  scales: {
    x: { time: true },
    y: {
      range: [0, 100],
    },
  },
  axes: [
    {
      stroke: "#888",
      grid: { stroke: "rgba(255,255,255,0.05)" },
      ticks: { stroke: "rgba(255,255,255,0.1)" },
      values: (_u: uPlot, ticks: number[]) =>
        ticks.map((t) => {
          const d = new Date(t * 1000);
          return `${String(d.getHours()).padStart(2, '0')}:${String(d.getMinutes()).padStart(2, '0')}`;
        }),
    },
    {
      label: "Memory ",
      stroke: "white",
      grid: { stroke: "rgba(255,255,255,0.05)" },
      ticks: { stroke: "rgba(255,255,255,0.1)" },
      values: ["0%", "10%", "20%", "30%", "40%", "50%", "60%", "70%", "80%", "90%", "100%"],
    },
  ],
  series: [
    {},
    {
      label: "Memory",
      stroke: "#81c784",
      fill: "rgba(129,199,132,0.15)",
      width: 2,
      value: (_u: uPlot, _rawValue: number, _seriesIdx: number, idx: number): string => {
        const bytes = props.rawBytes?.[idx];
        if (bytes != null || _rawValue == null) {
          return bytesToHuman(bytes);
        }
        return `${_rawValue.toFixed(1)}%`;
      },
    } as uPlot.Series,
  ],
};

function buildData(): uPlot.AlignedData {
  if (!props.data || props.data.length < 2) return [[], []];
  return props.data as uPlot.AlignedData;
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
  chart.setData(buildData());
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

// React to data changes from parent
watch(
  () => props.data,
  () => {
    updateChart();
  },
  { deep: false },
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