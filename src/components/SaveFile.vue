<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'

const props = defineProps({
  sourcePath: { type: String, required: true },
  cropped: { type: Boolean, required: true },
  destinationPath: { type: String, required: true },
  crop: { type: Array<string>, required: true }
})
const emit = defineEmits(['setCropped'])

const successMessage = ref('')

async function save (): Promise<void> {
  successMessage.value = await invoke('save_image',
    {
      source: props.sourcePath,
      target: props.destinationPath,
      x1: parseInt(props.crop[0]),
      y1: parseInt(props.crop[1]),
      x2: parseInt(props.crop[2]),
      y2: parseInt(props.crop[3])
    }
  )
  emit('setCropped')
}
</script>

<template>
  <button type="button" @click="save">Save {{ props.cropped ? "&check;" : "" }}</button>
  <p>{{ successMessage }}</p>
</template>
