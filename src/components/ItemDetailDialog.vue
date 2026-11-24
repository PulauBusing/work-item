<script setup lang="ts">
import { computed } from 'vue';
import { MdPreview } from 'md-editor-v3';
import { openUrl } from '@tauri-apps/plugin-opener';
import { textColor } from '../utils/color';
import type { Item } from '../types';

const props = defineProps<{
  modelValue: boolean;
  item: Item | null;
}>();

const emit = defineEmits<{
  (e: 'update:modelValue', v: boolean): void;
}>();

const visible = computed({
  get: () => props.modelValue,
  set: (v) => emit('update:modelValue', v),
});

const JIRA_BASE = 'http://192.168.30.111:8080/browse/';

function openJira(jira: string) {
  openUrl(`${JIRA_BASE}${jira}`);
}
</script>

<template>
  <el-dialog v-model="visible" title="事项详情" width="680px">
    <template v-if="item">
      <div class="detail-header">
        <el-tag :type="item.status === 'done' ? 'success' : 'info'" size="small">
          {{ item.status === 'done' ? '已完成' : '未完成' }}
        </el-tag>
        <span class="code">{{ item.code }}</span>
        <h3 class="name">{{ item.name }}</h3>
      </div>
      <div class="meta">
        <div><span class="label">开始日期：</span>{{ item.start_date }}</div>
        <div v-if="item.end_date">
          <span class="label">完成日期：</span>{{ item.end_date }}
        </div>
        <div v-if="item.jira">
          <span class="label">关联 Jira：</span>
          <el-link type="primary" @click="openJira(item.jira!)">
            {{ item.jira }}
          </el-link>
        </div>
        <div v-if="item.tags.length">
          <span class="label">标签：</span>
          <span
            v-for="t in item.tags"
            :key="t.id"
            class="tag-chip"
            :style="{ backgroundColor: t.color, color: textColor(t.color) }"
          >
            {{ t.name }}
          </span>
        </div>
      </div>
      <div class="detail">
        <MdPreview :model-value="item.detail || ''" />
      </div>
    </template>
  </el-dialog>
</template>

<style scoped>
.code {
  margin-left: 10px;
  color: #909399;
  font-size: 13px;
}
.name {
  margin: 8px 0;
  font-size: 18px;
}
.meta {
  display: flex;
  flex-wrap: wrap;
  gap: 16px;
  padding: 8px 0 16px;
  color: #606266;
  font-size: 14px;
}
.label {
  color: #909399;
}
.tag-chip {
  display: inline-block;
  padding: 2px 8px;
  border-radius: 4px;
  font-size: 12px;
  margin-right: 6px;
  white-space: nowrap;
}
.detail {
  border-top: 1px solid #ebeef5;
  padding-top: 12px;
  max-height: 50vh;
  overflow: auto;
}
</style>
