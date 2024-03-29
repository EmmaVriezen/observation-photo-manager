<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'

const props = defineProps(['sourcePath', 'destinationPath'])
const successMessage = ref('')

async function save (): Promise<void> {
  successMessage.value = await invoke('save_image',
    {
      source: props.sourcePath,
      target: props.destinationPath
    }
  )
}
</script>

<template>
  <button type="button" @click="save">Save</button>
  <p>{{ successMessage }}</p>
</template>
