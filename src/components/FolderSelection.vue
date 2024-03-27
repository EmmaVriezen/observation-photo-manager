<script setup lang="ts">
import { ref, computed } from 'vue'
import { open } from '@tauri-apps/api/dialog'

const selectedFolder = ref('')

const folderSelectionButtonLabel = computed(() => {
  if (selectedFolder.value === '') {
    return 'Select folder...'
  } else {
    return selectedFolder.value
  }
})

const selectFolder = async (): Promise<void> => {
  const selection = await open({ directory: true, multiple: false })
  if (Array.isArray(selection)) {
    console.error('Selecting multiple folders is forbidden.')
  } else if (selection === null) {
    selectedFolder.value = ''
  } else {
    selectedFolder.value = selection
  }
}
</script>

<template>
  <button type="button" @click="selectFolder">{{ folderSelectionButtonLabel }}</button>
</template>
