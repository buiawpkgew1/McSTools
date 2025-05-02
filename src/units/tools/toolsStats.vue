<script setup lang="ts">
import { defineProps, computed, ref, watch, nextTick, onBeforeUnmount } from "vue";
import type { RequirementStatistics } from "../../modules/requirements.ts";
import * as echarts from 'echarts';

const props = defineProps<{
  data: RequirementStatistics | undefined;
}>();

const viewMode = ref<'list' | 'chart'>('list');
let chartInstance: echarts.ECharts | null = null;

const sortedItems = computed(() => {
  return props.data?.items?.slice().sort((a, b) => b.count - a.count) || [];
});

const getBlockIcon = (blockId: string) => {
  const block = blockId.split(':');
  return new URL(`../../assets/icon/icon-exports-x32/${block[0]}__${block[1]}.png`, import.meta.url).href
};
const initOrUpdateChart = async () => {
  await nextTick();
  const chartDom = document.getElementById('chart-container');

  if (chartDom) {
    if (chartInstance) {
      chartInstance.dispose();
      chartInstance = null;
    }

    chartInstance = echarts.init(chartDom);

    const option = {
      tooltip: {
        trigger: 'item',
        formatter: ({ data }: any) =>
            `${data.name}<br/>数量: ${data.value} (${data.percentage}%)`
      },
      legend: {
        orient: 'vertical',
        right: 0,
        top: 'middle',
        formatter: (name: string) => {
          const item = sortedItems.value.find(i => i.zh_cn === name);
          return `${ name.indexOf(":") == -1? name : name.split(":")[1] }  ${item?.count}`;
        }
      },
      series: [{
        type: 'pie',
        radius: ['35%', '80%'],
        center: ['15%', '45%'],
        itemStyle: {
          borderRadius: 5,
          borderColor: '#fff',
          borderWidth: 2
        },
        label: {
          show: false,
          position: 'center'
        },
        data: sortedItems.value.map(item => ({
          name: item.zh_cn,
          value: item.count,
          percentage: item.percentage.toFixed(1)
        }))
      }]
    };

    chartInstance.setOption(option);
    window.addEventListener('resize', () => chartInstance?.resize());
  }
};

watch(viewMode, (newVal) => {
  if (newVal === 'chart') {
    nextTick(() => {
      if (props.data?.items?.length) {
        initOrUpdateChart();
      }
    });
  }
});

watch(() => props.data, (newVal) => {
  if (viewMode.value === 'chart' && newVal?.items?.length) {
    initOrUpdateChart();
  }
}, { deep: true });

onBeforeUnmount(() => {
  if (chartInstance) {
    chartInstance.dispose();
    window.removeEventListener('resize', () => chartInstance?.resize());
  }
});
</script>

<template>
  <div class="d-flex align-center px-4 pt-2">
    <div class="text-caption text-medium-emphasis mr-auto">
      共 {{ data?.total || 0 }} 个材料
    </div>

    <v-btn-toggle
        v-model="viewMode"
        mandatory
        density="compact"
        class="view-toggle"
    >
      <v-btn value="list" size="small">
        <v-icon icon="mdi-format-list-bulleted" start></v-icon>
        列表
      </v-btn>

      <v-btn value="chart" size="small">
        <v-icon icon="mdi-chart-pie" start></v-icon>
        图表
      </v-btn>
    </v-btn-toggle>
  </div>
  <div v-if="viewMode === 'list'" class="scroll-container">

    <v-table
        density="compact"
        hover
        class="material-table"
    >
      <thead class="table-header">
      <tr>
        <th class="text-left">材料名称</th>
        <th class="text-center">ID</th>
        <th class="text-right">
          <div class="d-flex align-center justify-end">
            <v-icon icon="mdi-sort-numeric-descending" size="small" class="mr-1"></v-icon>
            数量
          </div>
        </th>
        <th class="text-right">占比</th>
      </tr>
      </thead>

      <tbody>
      <tr
          v-for="(item) in sortedItems"
          :key="item.id"
          class="table-row"
      >
        <td class="text-left font-weight-medium">
          <v-avatar size="32" rounded="0" class="mr-2">
            <img
                :src="getBlockIcon(item.id)"
                :alt="item.zh_cn"
            >
          </v-avatar>
          {{ item.zh_cn }}
        </td>

        <td class="text-center text-caption text-medium-emphasis">
          <v-tooltip location="top">
            <template v-slot:activator="{ props: tooltipProps }">
              <span v-bind="tooltipProps">{{ item.id }}</span>
            </template>
            <span>{{ item.id }}</span>
          </v-tooltip>
        </td>

        <td class="text-right">
          <v-chip
              variant="outlined"
              color="indigo-darken-2"
              size="small"
          >
            {{ item.count }}
          </v-chip>
        </td>

        <td class="text-right" style="width: 35%">
          <div class="d-flex align-center justify-end">
            <span class="text-caption mr-2">{{ item.percentage.toFixed(1) }}%</span>
            <v-progress-linear
                :model-value="item.percentage"
                height="16"
                color="green-accent-4"
                rounded
                :max="100"
            >
              <template v-slot:default="{ value }">
                  <span
                      v-if="value > 15"
                      class="text-caption text-white"
                  >
                    {{ Math.ceil(value) }}%
                  </span>
              </template>
            </v-progress-linear>
          </div>
        </td>
      </tr>

      <tr v-if="!sortedItems.length">
        <td colspan="4" class="text-center py-8 text-medium-emphasis">
          <v-icon icon="mdi-package-variant-remove"></v-icon>
          暂无材料数据
        </td>
      </tr>
      </tbody>
    </v-table>
  </div>
  <div v-else class="chart-container pa-4">
    <div
        v-if="sortedItems.length"
        id="chart-container"
        style="height: 400px; width: 100%"
    ></div>

    <v-alert
        v-else
        type="info"
        variant="tonal"
        class="ma-4"
    >
      <template v-slot:prepend>
        <v-icon icon="mdi-information-outline"></v-icon>
      </template>
      暂无数据可供图表展示
    </v-alert>
  </div>

</template>

<style scoped>
.material-table {
  max-height: 100%;
  overflow: auto;
}

.table-header {
  background-color: rgb(250, 250, 250);
}

.table-header th {
  font-weight: 600;
  font-size: 0.875rem;
}

.table-row:hover {
  background-color: #f5f5f5 !important;
}
.scroll-container {
  overflow-y: auto;
}

.view-toggle {
  border: 1px solid rgba(0, 0, 0, 0.12);
  border-radius: 4px;

  .v-btn {
    transition: all 0.3s ease;

    &--active {
      background-color: #2196F3;
      color: white !important;
    }
  }
}

.chart-container {
  position: relative;
  height: 450px;
}

</style>