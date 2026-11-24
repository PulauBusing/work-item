<script setup lang="ts">
import { computed, reactive, ref, watch } from 'vue';
import { ElMessage } from 'element-plus';
import type { FormInstance, FormRules } from 'element-plus';
import { MdEditor, MdPreview } from 'md-editor-v3';
import { View, Plus, Edit } from '@element-plus/icons-vue';
import dayjs from 'dayjs';
import { createItem, updateItem } from '../api';
import { useItemsStore } from '../store/items';
import { chipStyle } from '../utils/color';
import type { Item, ItemInput, Tag } from '../types';
import TagEditDialog from './TagEditDialog.vue';

const props = defineProps<{
  modelValue: boolean;
  item: Item | null;
}>();

const emit = defineEmits<{
  (e: 'update:modelValue', v: boolean): void;
  (e: 'saved'): void;
}>();

const store = useItemsStore();
const formRef = ref<FormInstance>();
const saving = ref(false);

const form = reactive({
  name: '',
  start_date: dayjs().format('YYYY-MM-DD'),
  detail: '',
  jira: '',
  tags: [] as number[],
});

const visible = computed({
  get: () => props.modelValue,
  set: (v) => emit('update:modelValue', v),
});

const isEdit = computed(() => props.item !== null);
const tagDialogVisible = ref(false);
const editingTag = ref<Tag | null>(null);
const previewVisible = ref(false);

watch(
  () => props.modelValue,
  (v) => {
    if (!v) return;
    if (props.item) {
      form.name = props.item.name;
      form.start_date = props.item.start_date;
      form.detail = props.item.detail;
      form.jira = props.item.jira ?? '';
      form.tags = props.item.tags.map((t) => t.id);
    } else {
      form.name = '';
      form.start_date = dayjs().format('YYYY-MM-DD');
      form.detail = '';
      form.jira = '';
      form.tags = [];
    }
  },
);

const rules: FormRules = {
  name: [{ required: true, message: '请输入事项名称', trigger: 'blur' }],
  start_date: [{ required: true, message: '请选择开始日期', trigger: 'change' }],
};

function toggleTag(id: number) {
  const i = form.tags.indexOf(id);
  if (i >= 0) {
    form.tags.splice(i, 1);
  } else {
    form.tags.push(id);
  }
}

function openTagEdit(tag: Tag) {
  editingTag.value = tag;
  tagDialogVisible.value = true;
}

async function onTagSaved(tag: Tag) {
  await store.loadTags();
  // 新建的标签自动选中；编辑已有标签则不改变选中态
  if (!editingTag.value && !form.tags.includes(tag.id)) {
    form.tags.push(tag.id);
  }
}

async function save() {
  await formRef.value!.validate();
  saving.value = true;
  try {
    const input: ItemInput = {
      name: form.name,
      start_date: form.start_date,
      end_date: null,
      detail: form.detail,
      jira: form.jira.trim() || null,
      tags: form.tags,
    };
    if (props.item) {
      await updateItem(props.item.id, input);
    } else {
      await createItem(input);
    }
    ElMessage.success('保存成功');
    emit('saved');
    visible.value = false;
  } catch (e) {
    ElMessage.error(String(e));
  } finally {
    saving.value = false;
  }
}
</script>

<template>
  <el-dialog
    v-model="visible"
    :title="isEdit ? '编辑事项' : '新增事项'"
    width="760px"
    top="5vh"
  >
    <el-form ref="formRef" :model="form" :rules="rules" label-width="96px">
      <el-form-item label="事项编号">
        <el-input :model-value="props.item?.code ?? '保存后自动生成'" disabled />
      </el-form-item>
      <el-form-item label="事项名称" prop="name">
        <el-input v-model="form.name" placeholder="请输入事项名称" />
      </el-form-item>
      <el-form-item label="开始日期" prop="start_date">
        <el-date-picker
          v-model="form.start_date"
          type="date"
          value-format="YYYY-MM-DD"
          placeholder="选择日期"
        />
      </el-form-item>
      <el-form-item label="事项标签">
        <div class="tag-selector">
          <div
            v-for="t in store.tags"
            :key="t.id"
            class="tag-option"
            :style="chipStyle(t.color, form.tags.includes(t.id))"
            @click="toggleTag(t.id)"
          >
            {{ t.name }}
            <el-icon class="tag-edit-icon" :size="12" @click.stop="openTagEdit(t)">
              <Edit />
            </el-icon>
          </div>
          <el-button class="new-tag-btn" text type="primary" @click="tagDialogVisible = true">
            <el-icon><Plus /></el-icon>
            新建标签
          </el-button>
        </div>
      </el-form-item>
      <el-form-item label="关联 Jira">
        <el-input v-model="form.jira" placeholder="填写 jira 编号" />
      </el-form-item>
      <el-form-item label="事项详情">
        <div class="detail-editor">
          <div class="detail-toolbar">
            <el-tooltip content="预览详情" placement="top">
              <el-button text circle @click="previewVisible = true">
                <el-icon :size="16"><View /></el-icon>
              </el-button>
            </el-tooltip>
          </div>
          <MdEditor v-model="form.detail" :preview="false" style="height: 300px" />
        </div>
      </el-form-item>
    </el-form>
    <template #footer>
      <el-button @click="visible = false">取消</el-button>
      <el-button type="primary" :loading="saving" @click="save">保存</el-button>
    </template>
  </el-dialog>

  <el-dialog v-model="previewVisible" title="详情预览" width="680px" top="8vh">
    <MdPreview :model-value="form.detail || ''" />
  </el-dialog>

  <TagEditDialog v-model="tagDialogVisible" :tag="editingTag" @saved="onTagSaved" />
</template>

<style scoped>
.tag-selector {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 8px;
}
.tag-option {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  padding: 4px 12px;
  border-radius: 14px;
  font-size: 13px;
  cursor: pointer;
  user-select: none;
  border: 1px solid transparent;
  transition: box-shadow 0.15s, transform 0.15s;
}
.tag-edit-icon {
  opacity: 0.5;
}
.tag-edit-icon:hover {
  opacity: 1;
}
.new-tag-btn {
  margin-left: 0;
}
.detail-editor {
  width: 100%;
}
.detail-toolbar {
  display: flex;
  justify-content: flex-end;
  margin-bottom: 4px;
}
</style>
