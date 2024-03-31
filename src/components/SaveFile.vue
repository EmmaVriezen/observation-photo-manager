<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'

const props = defineProps({
  sourcePath: { type: String, required: true },
  destinationPath: { type: String, required: true },
  crop: { type: Array<string>, required: true }
})
const successMessage = ref('')

async function save (): Promise<void> {
  console.log(typeof props.crop[0])
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
}
</script>

<template>
  <button type="button" @click="save">Save</button>
  <p>{{ successMessage }}</p>
</template>
