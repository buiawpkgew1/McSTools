<script setup lang="ts">
import {defineProps, onMounted, reactive, ref} from "vue";
import {SchematicsData, schematicTypeList} from "../../modules/schematics_data.ts";
import dayjs from "dayjs";
import {files, handleUpload, progressValue, uploadError, uploadStatus} from "../../modules/upload_schematic.ts";
import {update_schematic_name} from "../../modules/update_schematic.ts";
import {schematic_id} from "../../modules/tools_data.ts";
import {toast} from "../../modules/others.ts";

const props = defineProps<{
  data: SchematicsData | undefined,
}>()
const editing = ref(false)
const editLoading = ref(false)
const parseDimensions = (sizeStr: string) => {
  const [length, width, height] = sizeStr.split(',').map(Number);
  return [`X${length}`, `Y${width}`, `Z${height}`]
};
const schematicEdit = reactive({
  name: '',
  description: ''
})
const formatTime = (time: any) => {
  return dayjs(time).format('YYYY/MM/DD HH:mm')
}
const saveEdit = async () => {
  editing.value = false
  editLoading.value = true
  try {
    let result = await update_schematic_name(schematic_id.value, schematicEdit.name, schematicEdit.description);
    if (result){
      toast.success(`数据已更新`, {
        timeout: 3000
      });
      props.data.name = schematicEdit.name
      props.data.description = schematicEdit.description
    }
  }catch (e) {
    console.log(e)
  }finally {
    editing.value = false
  }
}
onMounted(async ()=> {
  schematicEdit.name = props.data.name;
  schematicEdit.description = props.data.description
})
</script>

<template>
  <div v-if="props.data" class="ma-4">
    <v-card-title>蓝图基本信息</v-card-title>

    <v-card-text>
      <v-row>
        <v-col cols="6">
          <v-list density="compact">
            <v-list-item>
              <template v-slot:prepend>
                <v-icon icon="mdi-identifier"></v-icon>
              </template>
              <v-list-item-title>ID：{{ props.data.id }}</v-list-item-title>
            </v-list-item>

            <v-list-item  v-if="!editing">
              <v-list-item-title class="d-flex align-center">
                <span>名称：{{ props.data.name }}</span>
              </v-list-item-title>
              <template v-slot:append>

                <v-list-item-action class="ml-2">
                  <v-btn
                      variant="text"
                      size="x-small"
                      icon="mdi-pencil"
                      @click="editing = true"
                  ></v-btn>
                </v-list-item-action>
              </template>
            </v-list-item>
            <v-list-item v-else>
              <v-text-field
                  v-model="schematicEdit.name"
                  variant="underlined"
                  density="compact"
                  autofocus
                  @keydown.enter="saveEdit"
              ></v-text-field>
              <template v-slot:append>

                <v-list-item-action class="d-flex gap-2">
                  <v-btn
                      variant="text"
                      size="x-small"
                      icon="mdi-check"
                      color="success"
                      @click="saveEdit"
                  ></v-btn>
                </v-list-item-action>
              </template>
            </v-list-item>

            <v-list-item>
              <v-list-item-title>类型：{{ schematicTypeList[props.data.schematic_type as 1 | 2 | 3 | 4] }} </v-list-item-title>
            </v-list-item>

            <v-list-item>
              <v-list-item-title>
                尺寸：
                <v-chip
                    color="deep-purple"
                    variant="outlined"
                    size="small"
                    class="dimension-chip"
                >
                  <div class="d-flex align-center">
                    <v-icon icon="mdi-axis-arrow" class="mr-1"></v-icon>
                    <div class="dimension-values">
                                <span v-for="(dim, index) in parseDimensions(props.data.sizes)" :key="index">
                                  {{ dim }}
                                  <v-icon v-if="index < 2" icon="mdi-close" size="x-small" class="mx-1"></v-icon>
                                </span>
                    </div>
                  </div>
                </v-chip>
              </v-list-item-title>
            </v-list-item>
          </v-list>
        </v-col>

        <v-col cols="6">
          <v-list density="compact">
            <v-list-item>
              <v-list-item-title>
                状态：
                <v-chip :color="props.data.is_deleted ? 'error' : 'success'" size="small">
                  {{ props.data.is_deleted ? '已删除' : '正常' }}
                </v-chip>
              </v-list-item-title>
            </v-list-item>

            <v-list-item>
              <v-list-item-title>创建者：{{ props.data.user || '未知' }}</v-list-item-title>
            </v-list-item>

            <v-list-item>
              <v-list-item-title>版本：v{{ props.data.version }}
                <v-chip
                    color="orange-lighten-4"
                    size="small"
                    class="text-orange-darken-4"
                >
                  <v-icon start icon="mdi-cube"></v-icon>
                  {{ props.data.game_version }}
                </v-chip>
              </v-list-item-title>
            </v-list-item>

            <v-list-item>
              <v-list-item-title>更新时间：{{ formatTime(props.data.updated_at) }}</v-list-item-title>
            </v-list-item>
          </v-list>
        </v-col>
      </v-row>

      <v-textarea
          :readonly="!editing"
          :model-value="schematicEdit.description"
          label="蓝图描述"
          clearable
          auto-grow
          @keydown.enter="saveEdit"
          class="mt-4"
      ></v-textarea>

      <div class="upload-container">
        <v-file-input
            v-model="files"
            class="custom-file-input"
            variant="solo-filled"
            color="primary"
            bg-color="grey-lighten-3"
            label="更新蓝图文件"
            multiple
            accept=".nbt, .json, .schem, .litematic"
            :max-file-size="100 * 1024 * 1024"
            :loading="uploadStatus === 'uploading'"
            :error-messages="uploadError"
            :disabled="uploadStatus === 'uploading'"
            @update:model-value="handleUpload(props.data.id)"
        >
        </v-file-input>

        <v-alert
            v-if="uploadStatus === 'success'"
            type="success"
            variant="tonal"
            class="mt-2"
        >
          <template #prepend>
            <v-icon icon="mdi-check-circle" class="mr-2"></v-icon>
          </template>

          <div class="d-flex align-center">
            <span class="mr-2">成功上传 {{ files.length }} 个文件</span>
            <v-spacer></v-spacer>
            <v-btn
                icon
                variant="text"
                size="x-small"
                @click="uploadStatus = 'idle'"
            >
              <v-icon>mdi-close</v-icon>
            </v-btn>
          </div>

          <v-progress-linear
              :model-value="progressValue"
              color="success"
              height="8"
              class="mt-2"
              stream
              rounded
          >
            <template #default>
              <span class="text-caption">{{ Math.ceil(progressValue) }}%</span>
            </template>
          </v-progress-linear>
        </v-alert>

        <v-alert
            v-if="uploadStatus === 'error'"
            type="error"
            variant="tonal"
            class="mt-2"
        >
          <template #prepend>
            <v-icon icon="mdi-check-circle" class="mr-2"></v-icon>
          </template>

          <div class="d-flex align-center">
            <span class="mr-2">发生错误:{{ uploadError }}</span>
            <v-spacer></v-spacer>
            <v-btn
                icon
                variant="text"
                size="x-small"
                @click="uploadStatus = 'idle'"
            >
              <v-icon>mdi-close</v-icon>
            </v-btn>
          </div>

          <v-progress-linear
              :model-value="progressValue"
              color="error"
              height="8"
              class="mt-2"
              stream
              rounded
          >
            <template #default>
              <span class="text-caption">{{ Math.ceil(progressValue) }}%</span>
            </template>
          </v-progress-linear>
        </v-alert>
      </div>
    </v-card-text>
  </div>
  <div v-else class="ma-4 error-card">
    <v-alert type="error">
      未选取蓝图
    </v-alert>
  </div>
</template>

<style scoped>

</style>