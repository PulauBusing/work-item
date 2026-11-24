<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { Tickets, Sunrise, Calendar, DataAnalysis, Plus, Fold, Expand, Moon, Sunny } from '@element-plus/icons-vue';
import { useItemsStore } from './store/items';
import ItemList from './components/ItemList.vue';
import StatsView from './views/StatsView.vue';
import ItemFormDialog from './components/ItemFormDialog.vue';

const store = useItemsStore();
const activeView = ref<'list' | 'today' | 'week' | 'stats'>('list');
const collapsed = ref(false);
const theme = ref<'light' | 'dark'>('light');

const rangeMap = {
  list: null,
  today: 'today',
  week: 'week',
} as const;

const viewMeta: Record<string, { title: string; desc: string }> = {
  list: { title: '事项清单', desc: '全部工作事项，统一管理' },
  today: { title: '今日事项', desc: '今天开始或进行中的事项' },
  week: { title: '本周事项', desc: '本周范围内的全部事项' },
  stats: { title: '事项统计', desc: '按日期范围统计并导出日志' },
};

const formVisible = computed({
  get: () => store.formVisible,
  set: (v: boolean) => {
    if (!v) store.closeForm();
  },
});

function onSelect(key: string) {
  activeView.value = key as typeof activeView.value;
  if (key === 'stats') {
    store.setRange('today');
  } else {
    store.setRange(rangeMap[key as 'list' | 'today' | 'week']);
  }
}

function toggleTheme() {
  theme.value = theme.value === 'light' ? 'dark' : 'light';
  document.documentElement.classList.toggle('dark', theme.value === 'dark');
  localStorage.setItem('work-item-theme', theme.value);
}

onMounted(() => {
  theme.value = localStorage.getItem('work-item-theme') === 'dark' ? 'dark' : 'light';
  store.refresh();
});
</script>

<template>
  <el-container class="app">
    <el-aside :width="collapsed ? '64px' : '220px'" class="aside">
      <div class="brand" :class="{ collapsed }">
        <div class="brand-left">
          <svg class="logo" viewBox="0 0 32 32" xmlns="http://www.w3.org/2000/svg" aria-label="工作事项 Logo">
            <defs>
              <linearGradient id="logo-grad" x1="0" y1="0" x2="1" y2="1">
                <stop offset="0%" stop-color="#5B8DEF" />
                <stop offset="100%" stop-color="#8B5CF6" />
              </linearGradient>
            </defs>
            <rect width="32" height="32" rx="9" fill="url(#logo-grad)" />
            <path d="M10 12.5 L14 16.5 L22 9.5" fill="none" stroke="#fff" stroke-width="3"
              stroke-linecap="round" stroke-linejoin="round" />
            <path d="M10 20.5 H22" fill="none" stroke="#fff" stroke-width="2.5"
              stroke-linecap="round" opacity="0.85" />
            <path d="M10 25 H18" fill="none" stroke="#fff" stroke-width="2.5"
              stroke-linecap="round" opacity="0.55" />
          </svg>
          <span v-if="!collapsed" class="brand-title">工作事项</span>
        </div>
        <el-tooltip content="新增事项" placement="right">
          <el-button class="add-btn" text circle @click="store.openCreate()">
            <el-icon :size="18"><Plus /></el-icon>
          </el-button>
        </el-tooltip>
      </div>

      <el-menu
        class="menu"
        :default-active="activeView"
        :collapse="collapsed"
        :collapse-transition="false"
        @select="onSelect"
      >
        <el-menu-item index="list">
          <el-icon><Tickets /></el-icon>
          <template #title>事项清单</template>
        </el-menu-item>
        <el-menu-item index="today">
          <el-icon><Sunrise /></el-icon>
          <template #title>今日事项</template>
        </el-menu-item>
        <el-menu-item index="week">
          <el-icon><Calendar /></el-icon>
          <template #title>本周事项</template>
        </el-menu-item>
        <el-menu-item index="stats">
          <el-icon><DataAnalysis /></el-icon>
          <template #title>事项统计</template>
        </el-menu-item>
      </el-menu>

      <div class="aside-footer">
        <el-tooltip :content="collapsed ? '展开菜单' : '折叠菜单'" placement="right">
          <el-button text class="footer-btn" @click="collapsed = !collapsed">
            <el-icon :size="18"><Expand v-if="collapsed" /><Fold v-else /></el-icon>
          </el-button>
        </el-tooltip>
        <el-tooltip :content="theme === 'dark' ? '切换浅色' : '切换深色'" placement="right">
          <el-button text class="footer-btn" @click="toggleTheme">
            <el-icon :size="18"><Sunny v-if="theme === 'dark'" /><Moon v-else /></el-icon>
          </el-button>
        </el-tooltip>
      </div>
    </el-aside>

    <el-main class="main">
      <header class="page-header">
        <div>
          <h1 class="page-title">{{ viewMeta[activeView].title }}</h1>
          <p class="page-desc">{{ viewMeta[activeView].desc }}</p>
        </div>
      </header>

      <StatsView v-if="activeView === 'stats'" />
      <ItemList v-else />
    </el-main>

    <ItemFormDialog v-model="formVisible" :item="store.editingItem" @saved="store.refresh()" />
  </el-container>
</template>

<style>
html,
body,
#app {
  height: 100%;
  margin: 0;
}
.app {
  height: 100vh;
}
.aside {
  display: flex;
  flex-direction: column;
  border-right: 1px solid var(--el-border-color-light);
  background: var(--el-bg-color);
  transition: width 0.2s ease;
  overflow: hidden;
}
.brand {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 14px 12px;
  gap: 8px;
}
.brand.collapsed {
  justify-content: center;
  padding: 14px 0;
}
.brand-left {
  display: flex;
  align-items: center;
  gap: 10px;
  min-width: 0;
}
.logo {
  width: 32px;
  height: 32px;
  flex-shrink: 0;
  display: block;
}
.brand-title {
  font-size: 17px;
  font-weight: 600;
  color: var(--el-text-color-primary);
  white-space: nowrap;
}
.add-btn {
  color: var(--el-color-primary);
}
.menu {
  flex: 1;
  border-right: none;
  background: transparent;
}
.aside-footer {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 8px;
  border-top: 1px solid var(--el-border-color-light);
}
.aside-footer:has(.footer-btn:only-child),
.aside-footer {
  justify-content: space-around;
}
.footer-btn {
  color: var(--el-text-color-regular);
}
.main {
  padding: 20px 24px;
  background: var(--el-bg-color-page);
  overflow: auto;
}
.page-header {
  margin-bottom: 16px;
}
.page-title {
  margin: 0;
  font-size: 22px;
  font-weight: 600;
  color: var(--el-text-color-primary);
}
.page-desc {
  margin: 4px 0 0;
  font-size: 13px;
  color: var(--el-text-color-secondary);
}
</style>
