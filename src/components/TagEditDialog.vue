<script setup lang="ts">
import { computed, reactive, ref, watch } from 'vue';
import { ElMessage } from 'element-plus';
import { createTag, updateTag } from '../api';
import type { Tag } from '../types';

const props = defineProps<{
  modelValue: boolean;
  tag: Tag | null;
}>();

const emit = defineEmits<{
  (e: 'update:modelValue', v: boolean): void;
  (e: 'saved', tag: Tag): void;
}>();

const visible = computed({
  get: () => props.modelValue,
  set: (v) => emit('update:modelValue', v),
});

const saving = ref(false);
const form = reactive({ name: '', color: '#409EFF' });

const predefineColors = [
  '#409EFF',
  '#67C23A',
  '#E6A23C',
  '#F56C6C',
  '#909399',
  '#7C5CFF',
  '#00D6B5',
  '#FF6B9A',
  '#5B8DEF',
  '#8B5CF6',
  '#F97316',
  '#14B8A6',
];

watch(
  () => props.modelValue,
  (v) => {
    if (!v) return;
    form.name = props.tag?.name ?? '';
    form.color = props.tag?.color ?? '#409EFF';
  },
);

async function save() {
  if (!form.name.trim()) {
    ElMessage.warning('请输入标签名');
    return;
  }
  saving.value = true;
  try {
    let tag: Tag;
    if (props.tag) {
      tag = await updateTag(props.tag.id, form.name.trim(), form.color);
    } else {
      tag = await createTag(form.name.trim(), form.color);
    }
    emit('saved', tag);
    visible.value = false;
  } catch (e) {
    ElMessage.error(String(e));
  } finally {
    saving.value = false;
  }
}
</script>

<template>
  <el-dialog v-model="visible" :title="tag ? '编辑标签' : '新建标签'" width="420px">
    <el-form label-width="72px">
      <el-form-item label="标签名">
        <el-input v-model="form.name" placeholder="请输入标签名" maxlength="20" />
      </el-form-item>
      <el-form-item label="背景色">
        <el-color-picker v-model="form.color" :predefine="predefineColors" />
      </el-form-item>
    </el-form>
    <template #footer>
      <el-button @click="visible = false">取消</el-button>
      <el-button type="primary" :loading="saving" @click="save">保存</el-button>
    </template>
  </el-dialog>
</template>
