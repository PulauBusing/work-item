<script setup lang="ts">
import { computed, ref } from 'vue';
import { ElMessage, ElMessageBox } from 'element-plus';
import { List, Grid, View, Edit, CircleCheck, RefreshLeft, Delete, Clock } from '@element-plus/icons-vue';
import { openUrl } from '@tauri-apps/plugin-opener';
import { deleteItem, toggleItemDone } from '../api';
import { useItemsStore } from '../store/items';
import type { StatusFilter } from '../store/items';
import { textColor, chipStyle } from '../utils/color';
import type { Item } from '../types';
import ItemDetailDialog from './ItemDetailDialog.vue';

const store = useItemsStore();
const JIRA_BASE = 'http://192.168.30.111:8080/browse/';

const detailVisible = ref(false);
const detailItem = ref<Item | null>(null);

const statusModel = computed({
  get: () => store.status,
  set: (v: StatusFilter) => store.setStatus(v),
});

const pageModel = computed({
  get: () => store.page,
  set: (v: number) => store.setPage(v),
});

function setLayout(v: 'list' | 'card') {
  store.layout = v;
}

function toggleTag(id: number) {
  const ids = store.tagIds.includes(id)
    ? store.tagIds.filter((x) => x !== id)
    : [...store.tagIds, id];
  store.setTags(ids);
}

function openDetail(item: Item) {
  detailItem.value = item;
  detailVisible.value = true;
}

function openJira(jira: string) {
  openUrl(`${JIRA_BASE}${jira}`);
}

async function toggleDone(item: Item) {
  try {
    await toggleItemDone(item.id);
    store.load();
  } catch (e) {
    ElMessage.error(String(e));
  }
}

async function remove(item: Item) {
  try {
    await ElMessageBox.confirm(`确定删除事项「${item.name}」吗？`, '提示', {
      type: 'warning',
    });
  } catch {
    return;
  }
  try {
    await deleteItem(item.id);
    ElMessage.success('已删除');
    store.refresh();
  } catch (e) {
    ElMessage.error(String(e));
  }
}
</script>

<template>
  <div>
    <div class="toolbar">
      <el-radio-group v-model="statusModel">
        <el-radio-button value="pending">未完成</el-radio-button>
        <el-radio-button value="done">已完成</el-radio-button>
        <el-radio-button value="all">所有</el-radio-button>
      </el-radio-group>

      <div class="spacer" />

      <div class="layout-toggle">
        <el-tooltip content="列表视图" placement="top">
          <el-button
            :type="store.layout === 'list' ? 'primary' : 'default'"
            text
            circle
            @click="setLayout('list')"
          >
            <el-icon :size="16"><List /></el-icon>
          </el-button>
        </el-tooltip>
        <el-tooltip content="卡片视图" placement="top">
          <el-button
            :type="store.layout === 'card' ? 'primary' : 'default'"
            text
            circle
            @click="setLayout('card')"
          >
            <el-icon :size="16"><Grid /></el-icon>
          </el-button>
        </el-tooltip>
      </div>
    </div>

    <div v-if="store.tags.length" class="tag-filter">
      <span class="tag-filter-label">标签筛选：</span>
      <div
        v-for="t in store.tags"
        :key="t.id"
        class="tag-filter-chip"
        :style="chipStyle(t.color, store.tagIds.includes(t.id))"
        @click="toggleTag(t.id)"
      >
        {{ t.name }}
      </div>
    </div>

    <el-table
      v-if="store.layout === 'list'"
      v-loading="store.loading"
      :data="store.items"
      class="table"
    >
      <el-table-column label="状态" width="70">
        <template #default="{ row }">
          <el-tooltip :content="row.status === 'done' ? '已完成' : '未完成'" placement="top">
            <el-icon :size="18" :color="row.status === 'done' ? '#67c23a' : '#e6a23c'">
              <CircleCheck v-if="row.status === 'done'" />
              <Clock v-else />
            </el-icon>
          </el-tooltip>
        </template>
      </el-table-column>
      <el-table-column prop="code" label="编号" width="170" />
      <el-table-column prop="name" label="名称" min-width="160" />
      <el-table-column prop="start_date" label="开始日期" width="110" />
      <el-table-column label="完成日期" width="110">
        <template #default="{ row }">{{ row.end_date ?? '—' }}</template>
      </el-table-column>
      <el-table-column label="标签" min-width="150">
        <template #default="{ row }">
          <span
            v-for="t in row.tags"
            :key="t.id"
            class="tag-chip"
            :style="{ backgroundColor: t.color, color: textColor(t.color) }"
          >
            {{ t.name }}
          </span>
        </template>
      </el-table-column>
      <el-table-column label="Jira" width="120">
        <template #default="{ row }">
          <el-link v-if="row.jira" type="primary" @click="openJira(row.jira)">
            {{ row.jira }}
          </el-link>
          <span v-else>—</span>
        </template>
      </el-table-column>
      <el-table-column label="操作" width="200" fixed="right">
        <template #default="{ row }">
          <div class="row-actions">
            <el-tooltip content="详情" placement="top">
              <el-button text circle @click="openDetail(row)">
                <el-icon :size="16"><View /></el-icon>
              </el-button>
            </el-tooltip>
            <el-tooltip v-if="row.status === 'pending'" content="编辑" placement="top">
              <el-button text circle type="primary" @click="store.openEdit(row)">
                <el-icon :size="16"><Edit /></el-icon>
              </el-button>
            </el-tooltip>
            <el-tooltip :content="row.status === 'pending' ? '完成' : '取消完成'" placement="top">
              <el-button
                text
                circle
                :type="row.status === 'pending' ? 'success' : 'warning'"
                @click="toggleDone(row)"
              >
                <el-icon :size="16">
                  <CircleCheck v-if="row.status === 'pending'" />
                  <RefreshLeft v-else />
                </el-icon>
              </el-button>
            </el-tooltip>
            <el-tooltip content="删除" placement="top">
              <el-button text circle type="danger" @click="remove(row)">
                <el-icon :size="16"><Delete /></el-icon>
              </el-button>
            </el-tooltip>
          </div>
        </template>
      </el-table-column>
    </el-table>

    <div v-else v-loading="store.loading" class="cards">
      <el-card v-for="item in store.items" :key="item.id" class="card" shadow="hover">
        <div class="card-head">
          <el-tooltip :content="item.status === 'done' ? '已完成' : '未完成'" placement="top">
            <el-icon :size="18" :color="item.status === 'done' ? '#67c23a' : '#e6a23c'">
              <CircleCheck v-if="item.status === 'done'" />
              <Clock v-else />
            </el-icon>
          </el-tooltip>
          <span class="card-name">{{ item.name }}</span>
        </div>
        <div class="card-code">{{ item.code }}</div>
        <div class="card-row">
          <span class="label">开始：</span>{{ item.start_date }}
          <template v-if="item.end_date">
            <span class="label"> 完成：</span>{{ item.end_date }}
          </template>
        </div>
        <div v-if="item.tags.length" class="card-tags">
          <span
            v-for="t in item.tags"
            :key="t.id"
            class="tag-chip"
            :style="{ backgroundColor: t.color, color: textColor(t.color) }"
          >
            {{ t.name }}
          </span>
        </div>
        <div v-if="item.jira" class="card-row">
          <span class="label">Jira：</span>
          <el-link type="primary" @click="openJira(item.jira!)">{{ item.jira }}</el-link>
        </div>
        <div class="card-actions">
          <el-tooltip content="详情" placement="top">
            <el-button text circle @click="openDetail(item)">
              <el-icon :size="16"><View /></el-icon>
            </el-button>
          </el-tooltip>
          <el-tooltip v-if="item.status === 'pending'" content="编辑" placement="top">
            <el-button text circle type="primary" @click="store.openEdit(item)">
              <el-icon :size="16"><Edit /></el-icon>
            </el-button>
          </el-tooltip>
          <el-tooltip :content="item.status === 'pending' ? '完成' : '取消完成'" placement="top">
            <el-button
              text
              circle
              :type="item.status === 'pending' ? 'success' : 'warning'"
              @click="toggleDone(item)"
            >
              <el-icon :size="16">
                <CircleCheck v-if="item.status === 'pending'" />
                <RefreshLeft v-else />
              </el-icon>
            </el-button>
          </el-tooltip>
          <el-tooltip content="删除" placement="top">
            <el-button text circle type="danger" @click="remove(item)">
              <el-icon :size="16"><Delete /></el-icon>
            </el-button>
          </el-tooltip>
        </div>
      </el-card>
    </div>

    <el-empty v-if="!store.loading && store.items.length === 0" description="暂无事项" />

    <div class="pagination">
      <el-pagination
        v-model:current-page="pageModel"
        :page-size="store.pageSize"
        :total="store.total"
        layout="prev, pager, next, total"
      />
    </div>

    <ItemDetailDialog v-model="detailVisible" :item="detailItem" />
  </div>
</template>

<style scoped>
.toolbar {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 12px;
}
.spacer {
  flex: 1;
}
.layout-toggle {
  display: flex;
  align-items: center;
  gap: 2px;
}
.tag-filter {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 8px;
  margin-bottom: 12px;
}
.tag-filter-label {
  font-size: 13px;
  color: var(--el-text-color-secondary);
}
.tag-filter-chip {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  padding: 4px 10px;
  border-radius: 14px;
  font-size: 13px;
  cursor: pointer;
  user-select: none;
  transition: box-shadow 0.15s;
}
.row-actions {
  display: flex;
  align-items: center;
  gap: 4px;
}
.row-actions :deep(.el-button + .el-button) {
  margin-left: 0;
}
.tag-chip {
  display: inline-block;
  padding: 2px 8px;
  border-radius: 4px;
  font-size: 12px;
  margin-right: 4px;
  white-space: nowrap;
}
.table {
  width: 100%;
  background: var(--el-bg-color);
}
.cards {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 16px;
}
.card-head {
  display: flex;
  align-items: center;
  gap: 6px;
}
.card-name {
  font-weight: 600;
  flex: 1;
}
.card-code {
  color: var(--el-text-color-secondary);
  font-size: 12px;
  margin: 4px 0 8px 24px;
}
.card-row {
  font-size: 13px;
  color: var(--el-text-color-regular);
  margin: 4px 0;
}
.card-tags {
  margin: 8px 0;
}
.card-actions {
  margin-top: 8px;
  border-top: 1px solid var(--el-border-color-light);
  padding-top: 8px;
}
.label {
  color: var(--el-text-color-secondary);
}
.pagination {
  margin-top: 16px;
  display: flex;
  justify-content: flex-end;
}
</style>
