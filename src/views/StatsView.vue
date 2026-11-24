<script setup lang="ts">
import { computed, ref } from 'vue';
import { ElMessage } from 'element-plus';
import { save } from '@tauri-apps/plugin-dialog';
import dayjs from 'dayjs';
import { exportItems } from '../api';
import { useItemsStore } from '../store/items';
import type { RangeValue } from '../store/items';
import ItemList from '../components/ItemList.vue';

const store = useItemsStore();

const rangeOptions: { label: string; value: RangeValue }[] = [
  { label: '今日', value: 'today' },
  { label: '本周', value: 'week' },
  { label: '上周', value: 'last_week' },
  { label: '本月', value: 'month' },
  { label: '上月', value: 'last_month' },
  { label: '日期区间', value: 'custom' },
];

const customRange = ref<[string, string] | null>(null);

const rangeModel = computed({
  get: () => store.range ?? 'today',
  set: (v: RangeValue) => {
    store.range = v;
    store.page = 1;
    if (v !== 'custom') {
      store.customStart = null;
      store.customEnd = null;
      store.load();
    }
  },
});

function onCustomChange(val: [string, string] | null) {
  if (val && val[0] && val[1]) {
    store.customStart = val[0];
    store.customEnd = val[1];
    store.page = 1;
    store.load();
  }
}

async function doExport() {
  try {
    const defaultName = `工作日志-${dayjs().format('YYYYMMDD')}.md`;
    const path = await save({
      defaultPath: defaultName,
      filters: [{ name: 'Markdown', extensions: ['md'] }],
    });
    if (!path) return;
    await exportItems(path, store.filter);
    ElMessage.success('导出成功');
  } catch (e) {
    ElMessage.error(String(e));
  }
}
</script>

<template>
  <div>
    <div class="stats-bar">
      <el-radio-group v-model="rangeModel">
        <el-radio-button v-for="o in rangeOptions" :key="o.value" :value="o.value">
          {{ o.label }}
        </el-radio-button>
      </el-radio-group>

      <el-date-picker
        v-if="store.range === 'custom'"
        v-model="customRange"
        type="daterange"
        value-format="YYYY-MM-DD"
        start-placeholder="开始日期"
        end-placeholder="结束日期"
        style="margin-left: 12px"
        @change="onCustomChange"
      />

      <div class="spacer" />

      <el-button type="success" @click="doExport">日志导出</el-button>
    </div>

    <ItemList />
  </div>
</template>

<style scoped>
.stats-bar {
  display: flex;
  align-items: center;
  margin-bottom: 16px;
}
.spacer {
  flex: 1;
}
</style>
