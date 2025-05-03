<script setup lang="ts">
import {defineProps, reactive, onMounted, watch, ref} from "vue";
import type { RequirementStatistics, RequirementStatistic } from "../../modules/requirements.ts";
import { jeBlocks, fetchJeBlocks, type SubData } from "../../modules/je_blocks.ts";
import {invoke} from "@tauri-apps/api/core";
import {toast} from "../../modules/others.ts";
import {BlockData} from "../../modules/replace_data.ts";
const active = ref(0)
interface ReplacementRule {
  original: RequirementStatistic;
  originalDetails: BlockData;
  replacement: SubData;
  quantity: number;
}
const props = defineProps<{
  data: RequirementStatistics | undefined;
  blocks: BlockData[] | undefined;
}>();

const state = reactive({
  selectedOriginal: null as RequirementStatistic | null,
  selectedOriginalDetails: null as BlockData | null,
  selectedReplacement: null as SubData | null,
  quantity: 1,
  globalReplace: true,
  replaceMode: 0,
  replacementRules: [] as ReplacementRule[],
  isLoading: false,
  error: null as string | null,
  showConfirmDialog: false
});

onMounted(async () => {
  try {
    jeBlocks.value = await fetchJeBlocks();
  } catch (err) {
    state.error = err instanceof Error ? err.message : 'Failed to load block data';
  }
});

const addReplacementRule = (mode: number) => {
  if (!state.selectedOriginal || !state.selectedReplacement) {
    state.error = '请先选择要替换的方块和替换目标';
    return;
  }

  if (state.quantity <= 0) {
    state.error = '替换数量必须大于0';
    return;
  }
  state.replaceMode = mode
  state.replacementRules.push({
    original: state.selectedOriginal,
    replacement: state.selectedReplacement,
    quantity: state.quantity,
    originalDetails: state.selectedOriginalDetails
  });
  resetSelection();
};
const hasProperties = (block: BlockData): boolean => {
  return Object.keys(block.properties).length > 0
}

const formatProperties = (props: Record<string, string>): string => {
  return Object.entries(props)
      .map(([k, v]) => `${k}=${v}`)
      .join(', ')
}

const filterBlocks = (item: BlockData, queryText: string) => {
  const search = queryText.toLowerCase()

  if (item.id.toLowerCase().includes(search)) return true

  return Object.entries(item.properties).some(([key, value]) => {
    return `${key}=${value}`.toLowerCase().includes(search)
  })
}


const executeReplacement = async () => {
  try {
    state.isLoading = true;
    state.error = null;
    let rules = state.replacementRules.map(r => ({
      original_id: r.original?.id,
      replacement_id: typeof r.replacement === 'object'
          ? `minecraft:${r.replacement.block_name}`
          : r.replacement,
      quantity: r.quantity,
      global: state.globalReplace
    }))

    const result = await invoke<boolean>('schematic_replacement', {
      rules: rules
    });

    if (result) {
      state.replacementRules = [];
      state.showConfirmDialog = false;
    }
  } catch (err) {
    state.error = err instanceof Error ? err.message : '替换操作失败';
    toast.error(`发生了一个错误:${err}`, {
      timeout: 3000
    });

  } finally {
    state.isLoading = false;
  }
};
const resetSelection = () => {
  state.selectedOriginal = null;
  state.selectedOriginalDetails = null;
  state.selectedReplacement = null;
  state.quantity = 1;
  state.error = null;
};

const getBlockIcon = (blockId: string) => {
  const block = blockId.split(':');
  return new URL(`../../assets/icon/icon-exports-x32/${block[0]}__${block[1]}.png`, import.meta.url).href
};
const removeRule = (index: number) => {
  state.replacementRules.splice(index, 1);
};
watch(() => [state.globalReplace, state.selectedOriginal], ([global, selected]) => {
  if (global && selected) {
    state.quantity = (selected as RequirementStatistic).num
  } else if (!global) {
    state.quantity = 1
  }
}, { deep: true })

</script>

<template>
  <v-row>
    <v-col cols="12">
      <v-alert
          variant="tonal"
          color="info"
          icon="mdi-information"
          class="mt-4"
      >
        当前方块替换，仅适用与无方块状态方块，替换将会丢失全部状态。
      </v-alert>
    </v-col>
  </v-row>
  <v-row class="pa-4" no-gutters>
    <v-col cols="4">
      <v-tabs v-model="active" align-tabs="center" color="blue-lighten-1">
        <v-tab value="brief">简单模式</v-tab>
        <v-tab value="details">精准模式</v-tab>
      </v-tabs>
      <v-window v-model="active">
        <v-window-item value="brief">
          <div class="d-flex align-center mb-4">
            <v-icon icon="mdi-magnify" class="mr-2"/>
            <v-combobox
                v-model="state.selectedOriginal"
                label="查找方块"
                :items="props.data?.items ?? []"
                item-title="zh_cn"
                item-value="id"
                clearable
                :loading="!props.data"
            >
              <template v-slot:selection="{ item }">
                <div class="d-flex align-center">
                  <v-avatar size="32" rounded="0" class="mr-2">
                    <img :src="getBlockIcon(item.raw.id)" :alt="item.raw.id">
                  </v-avatar>
                  <div>
                    <span class="text-body-2">{{ item.raw.zh_cn }}</span>
                  </div>
                </div>
              </template>
              <template v-slot:item="{ props: itemProps, item }">
                <v-list-item v-bind="itemProps">
                  <template v-slot:prepend>
                    <v-avatar size="32" rounded="0" class="mr-2">
                      <img
                          :src="getBlockIcon(item.raw.id)"
                          :alt="item.raw.zh_cn"
                      >
                    </v-avatar>
                  </template>
                  <v-list-item-subtitle class="text-caption">
                    ID: {{ item.raw.id }}
                  </v-list-item-subtitle>
                </v-list-item>
              </template>
            </v-combobox>
          </div>
          <div class="d-flex align-center">
            <v-icon icon="mdi-arrow-right" class="mx-2"/>
            <v-combobox
                v-model="state.selectedReplacement"
                label="替换为（可输入ID或选择）"
                :items="jeBlocks || []"
                item-title="zh_cn"
                item-value="block_name"
                :loading="!jeBlocks"
                clearable
            >
              <template v-slot:item="{ props: itemProps, item }">
                <template v-if="typeof item.raw === 'object'">
                  <v-list-item v-bind="itemProps">
                    <template v-slot:prepend>
                      <v-avatar size="32" rounded="0" class="mr-2">
                        <img
                            :src="getBlockIcon(`minecraft:${item.raw.block_name}`)"
                            :alt="item.raw.zh_cn"
                        >
                      </v-avatar>
                    </template>
                    <v-list-item-subtitle class="text-caption">
                      ID: minecraft:{{ item.raw.block_name }}
                    </v-list-item-subtitle>
                  </v-list-item>
                </template>
                <template v-else>
                  <v-list-item v-bind="itemProps" :title="item.raw">
                    <template v-slot:prepend>
                      <v-avatar size="32" rounded="0" class="mr-2">
                        <img
                            :src="getBlockIcon(item.raw)"
                            :alt="item.raw"
                            onerror="this.style.display='none'"
                        >
                      </v-avatar>
                    </template>
                    <v-list-item-subtitle class="text-caption">
                      自定义ID: {{ item.raw }}
                    </v-list-item-subtitle>
                  </v-list-item>
                </template>
              </template>
            </v-combobox>
          </div>
          <v-text-field
              v-model.number="state.quantity"
              label="替换数量"
              type="number"
              min="1"
              class="mt-4"
              :disabled="state.globalReplace && !!state.selectedOriginal"
              :hint="state.globalReplace
      ? (state.selectedOriginal
          ? `全局替换已锁定: ${state.selectedOriginal.zh_cn} 的需求量 ${state.selectedOriginal.num}`
          : '请先选择要替换的方块')
      : ''"
              persistent-hint
          />
          <v-checkbox
              v-model="state.globalReplace"
              label="全局替换"
              density="compact"
          />
          <v-btn
              block
              prepend-icon="mdi-playlist-plus"
              class="mb-2"
              color="secondary"
              @click="addReplacementRule(0)"
          >
            添加列表
          </v-btn>
          <v-btn
              color="green"
              block
              prepend-icon="mdi-swap-horizontal"
              :disabled="state.replacementRules.length === 0"
              @click="state.showConfirmDialog = true"
          >
            执行替换
          </v-btn>
        </v-window-item>
        <v-window-item value="details">
          <div class="d-flex align-center mb-4">
            <v-icon icon="mdi-magnify" class="mr-2"/>
            <v-combobox
                v-model="state.selectedOriginalDetails"
                label="查找方块（支持属性过滤）"
                :items="props.blocks ?? []"
                item-title="id"
                clearable
                :loading="!props.data"
                :filter="filterBlocks"
                :auto-filter-first="false"
            >
              <template v-slot:selection="{ item }">
                <div class="d-flex align-center">
                  <v-avatar size="32" rounded="0" class="mr-2">
                    <img :src="getBlockIcon(item.raw.id)" :alt="item.raw.id">
                  </v-avatar>
                  <div>
                    <span class="text-body-2">{{ item.raw.id }}</span>

                  </div>
                </div>
              </template>

              <template v-slot:item="{ props: itemProps, item }">
                <v-list-item v-bind="itemProps">
                  <template v-slot:prepend>
                    <v-avatar size="32" rounded="0" class="mr-2">
                      <img :src="getBlockIcon(item.raw.id)" :alt="item.raw.id">
                    </v-avatar>
                  </template>
                  <v-list-item-title class="d-flex align-center">
                    <span class="text-body-2">{{ item.raw }}</span>
                    <v-chip
                        v-if="hasProperties(item.raw)"
                        small
                        class="ml-2"
                        color="green"
                    >
                      {{ Object.keys(item.raw.properties).length }} 属性
                    </v-chip>
                  </v-list-item-title>
                  <v-list-item-subtitle v-if="hasProperties(item.raw)" class="text-caption">
                    <div v-for="(value, key) in item.raw.properties" :key="key">
                      <span class="text-grey">{{ key }}</span>: {{ value }}
                    </div>
                  </v-list-item-subtitle>
                </v-list-item>
              </template>
            </v-combobox>
          </div>
          <div class="d-flex align-center">
            <v-icon icon="mdi-arrow-right" class="mx-2"/>
            <v-combobox
                v-model="state.selectedReplacement"
                label="替换为（可输入ID或选择）"
                :items="jeBlocks || []"
                item-title="zh_cn"
                item-value="block_name"
                :loading="!jeBlocks"
                clearable
            >
              <template v-slot:item="{ props: itemProps, item }">
                <template v-if="typeof item.raw === 'object'">
                  <v-list-item v-bind="itemProps">
                    <template v-slot:prepend>
                      <v-avatar size="32" rounded="0" class="mr-2">
                        <img
                            :src="getBlockIcon(`minecraft:${item.raw.block_name}`)"
                            :alt="item.raw.zh_cn"
                        >
                      </v-avatar>
                    </template>
                    <v-list-item-subtitle class="text-caption">
                      ID: minecraft:{{ item.raw.block_name }}
                    </v-list-item-subtitle>
                  </v-list-item>
                </template>
                <template v-else>
                  <v-list-item v-bind="itemProps" :title="item.raw">
                    <template v-slot:prepend>
                      <v-avatar size="32" rounded="0" class="mr-2">
                        <img
                            :src="getBlockIcon(item.raw)"
                            :alt="item.raw"
                            onerror="this.style.display='none'"
                        >
                      </v-avatar>
                    </template>
                    <v-list-item-subtitle class="text-caption">
                      自定义ID: {{ item.raw }}
                    </v-list-item-subtitle>
                  </v-list-item>
                </template>
              </template>
            </v-combobox>
          </div>
          <v-text-field
              v-model.number="state.quantity"
              label="替换数量"
              type="number"
              min="1"
              class="mt-4"
              :disabled="state.globalReplace && !!state.selectedOriginal"
              :hint="state.globalReplace
      ? (state.selectedOriginal
          ? `全局替换已锁定: ${state.selectedOriginal.zh_cn} 的需求量 ${state.selectedOriginal.num}`
          : '请先选择要替换的方块')
      : ''"
              persistent-hint
          />
          <v-checkbox
              v-model="state.globalReplace"
              label="全局替换"
              density="compact"
          />
          <v-btn
              block
              prepend-icon="mdi-playlist-plus"
              class="mb-2"
              color="secondary"
              @click="addReplacementRule(1)"
          >
            添加列表
          </v-btn>
          <v-btn
              color="green"
              block
              prepend-icon="mdi-swap-horizontal"
              :disabled="state.replacementRules.length === 0"
              @click="state.showConfirmDialog = true"
          >
            执行替换
          </v-btn>
        </v-window-item>
      </v-window>


      <v-alert
          v-if="state.error"
          type="error"
          density="compact"
          class="mt-4"
      >
        {{ state.error }}
      </v-alert>
    </v-col>

    <v-col cols="8">
      <v-table density="compact" class="elevation-1">
        <thead>
        <tr>
          <th>原方块</th>
          <th>新方块</th>
          <th>数量</th>
          <th>操作</th>
        </tr>
        </thead>
        <tbody>
        <tr v-for="(rule, index) in state.replacementRules" :key="index">
          <td>{{ rule.original.zh_cn }}</td>
          <td>
            <template v-if="typeof rule.replacement === 'object'">
              {{ rule.replacement.zh_cn }}
            </template>
            <template v-else>
              {{ rule.replacement }} <span class="text-caption">(自定义)</span>
            </template>
          </td>
          <td>{{ rule.quantity }}</td>
          <td>
            <v-btn
                variant="text"
                color="error"
                icon="mdi-delete"
                @click="removeRule(index)"
            />
          </td>
        </tr>
        <tr v-if="state.replacementRules.length === 0">
          <td colspan="4" class="text-center text-grey">
            暂无替换规则
          </td>
        </tr>
        </tbody>
      </v-table>
    </v-col>

    <v-dialog
        v-model="state.showConfirmDialog"
        max-width="500"
        persistent
    >
      <v-card>
        <v-card-title class="text-h6">
          <v-icon icon="mdi-alert" color="warning" class="mr-2"/>
          确认替换操作
        </v-card-title>
        <v-card-text>
          即将替换 {{ state.replacementRules.length }} 条方块规则
          <ul class="mt-2">
            <li v-for="(rule, index) in state.replacementRules" :key="index">
              {{ rule.original.zh_cn }} → {{ rule.replacement.zh_cn }} ×{{ rule.quantity }}
            </li>
          </ul>
        </v-card-text>
        <v-card-actions>
          <v-spacer/>
          <v-btn @click="state.showConfirmDialog = false">取消</v-btn>
          <v-btn
              color="primary"
              :loading="state.isLoading"
              @click="executeReplacement"
          >
            确认执行
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
  </v-row>
</template>

<style scoped>
ul {
  padding-left: 24px;
  list-style-type: circle;
}
</style>