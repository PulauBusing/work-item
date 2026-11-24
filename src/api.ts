import { invoke } from '@tauri-apps/api/core';
import type { Item, ItemFilter, ItemInput, PageResult, Tag } from './types';

export const listItems = (filter: ItemFilter) =>
  invoke<PageResult>('list_items', { filter });

export const createItem = (input: ItemInput) =>
  invoke<Item>('create_item', { input });

export const updateItem = (id: number, input: ItemInput) =>
  invoke<Item>('update_item', { id, input });

export const deleteItem = (id: number) =>
  invoke<void>('delete_item', { id });

export const toggleItemDone = (id: number) =>
  invoke<Item>('toggle_item_done', { id });

export const listTags = () => invoke<Tag[]>('list_tags');

export const createTag = (name: string, color: string) =>
  invoke<Tag>('create_tag', { name, color });

export const updateTag = (id: number, name: string, color: string) =>
  invoke<Tag>('update_tag', { id, name, color });

export const exportItems = (path: string, filter: ItemFilter) =>
  invoke<void>('export_items', { path, filter });
