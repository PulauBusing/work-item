import { defineStore } from 'pinia';
import { listItems, listTags } from '../api';
import type { Item, ItemFilter, Tag } from '../types';

export type StatusFilter = 'all' | 'pending' | 'done';
export type RangeValue =
  | 'today'
  | 'week'
  | 'last_week'
  | 'month'
  | 'last_month'
  | 'custom';

export const useItemsStore = defineStore('items', {
  state: () => ({
    tags: [] as Tag[],
    items: [] as Item[],
    total: 0,
    status: 'pending' as StatusFilter,
    range: null as RangeValue | null,
    customStart: null as string | null,
    customEnd: null as string | null,
    tagIds: [] as number[],
    page: 1,
    pageSize: 12,
    layout: 'list' as 'list' | 'card',
    loading: false,
    // 新增/编辑弹窗共享状态（侧边栏 "+" 按钮与列表页共用）
    formVisible: false,
    editingItem: null as Item | null,
  }),
  getters: {
    filter(state): ItemFilter {
      return {
        status: state.status === 'all' ? null : state.status,
        range: state.range === 'custom' ? 'custom' : state.range,
        start_date: state.range === 'custom' ? state.customStart : null,
        end_date: state.range === 'custom' ? state.customEnd : null,
        tag_ids: state.tagIds.length ? state.tagIds : null,
        page: state.page,
        page_size: state.pageSize,
      };
    },
  },
  actions: {
    async loadTags() {
      this.tags = await listTags();
    },
    async load() {
      this.loading = true;
      try {
        const res = await listItems(this.filter);
        this.items = res.items;
        this.total = res.total;
      } finally {
        this.loading = false;
      }
    },
    async setRange(range: RangeValue | null) {
      this.range = range;
      this.page = 1;
      if (range !== 'custom') {
        this.customStart = null;
        this.customEnd = null;
      }
      await this.load();
    },
    async setStatus(status: StatusFilter) {
      this.status = status;
      this.page = 1;
      await this.load();
    },
    async setTags(tagIds: number[]) {
      this.tagIds = tagIds;
      this.page = 1;
      await this.load();
    },
    async setPage(page: number) {
      this.page = page;
      await this.load();
    },
    async refresh() {
      await this.loadTags();
      await this.load();
    },
    openCreate() {
      this.editingItem = null;
      this.formVisible = true;
    },
    openEdit(item: Item) {
      this.editingItem = item;
      this.formVisible = true;
    },
    closeForm() {
      this.formVisible = false;
      this.editingItem = null;
    },
  },
});
