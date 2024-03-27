<script setup lang="ts">
import { computed } from 'vue'
import { open } from '@tauri-apps/api/dialog'

const props = defineProps(['selectedFolder'])
const emit = defineEmits(['setFolder'])

const folderSelectionButtonLabel = computed(() => {
  if (props.selectedFolder === '') {
    return 'Select folder...'
  } else {
    return props.selectedFolder
  }
})

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
    {{ folderSelectionButtonLabel }}
  </button>
</template>
