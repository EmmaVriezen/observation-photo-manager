<script setup lang="ts">
import { open } from '@tauri-apps/api/dialog'

const emit = defineEmits(['setFolder'])

const selectFolder = async (): Promise<void> => {
  const selection = await open({ directory: true, multiple: false })
  if (Array.isArray(selection)) {
    console.error('Selecting multiple folders is forbidden.')
  } else if (selection === null) {
    emit('setFolder', '')
  } else {
    emit('setFolder', selection)
  }
}
</script>

<template>
  <button type="button" @click="selectFolder">
    <slot></slot>
  </button>
</template>
