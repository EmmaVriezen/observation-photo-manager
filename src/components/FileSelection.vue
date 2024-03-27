<script setup lang="ts">
import { computed } from 'vue'
import { open } from '@tauri-apps/api/dialog'

const props = defineProps(['numberOfFiles'])
const emit = defineEmits(['setFileSelection'])

const fileSelectionButtonLabel = computed(() => {
  if (props.numberOfFiles === 1) {
    return props.numberOfFiles + ' file selected'
  } else if (props.numberOfFiles > 1) {
    return props.numberOfFiles + ' files selected'
  } else {
    return 'Select files...'
  }
})

const selectImages = async (): Promise<void> => {
  const selection = await open({
    multiple: true,
    filters: [{
      name: 'Image',
      extensions: ['png', 'jpeg', 'jpg']
    }]
  })
  if (Array.isArray(selection)) {
    emit('setFileSelection', selection)
  } else if (selection === null) {
    emit('setFileSelection', [])
  } else {
    emit('setFileSelection', [selection])
  }
}
</script>

<template>
  <button type="button" @click="selectImages">
    {{ fileSelectionButtonLabel }}
  </button>
</template>
