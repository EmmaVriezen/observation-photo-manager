<script setup lang="ts">
import { ref, computed } from 'vue'
import { open } from '@tauri-apps/api/dialog'

const emit = defineEmits(['setFileSelection'])

const selectedFiles = ref<string[]>([])

const fileCount = computed(() => selectedFiles.value.length)

const fileSelectionButtonLabel = computed(() => {
  if (fileCount.value === 1) {
    return fileCount.value + ' file selected'
  } else if (fileCount.value > 1) {
    return fileCount.value + ' files selected'
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
    selectedFiles.value = selection
    emit('setFileSelection', selection)
  } else if (selection === null) {
    selectedFiles.value = []
    emit('setFileSelection', [])
  } else {
    selectedFiles.value = [selection]
    emit('setFileSelection', [selection])
  }
}
</script>

<template>
  <button type="button" @click="selectImages">{{ fileSelectionButtonLabel }}</button>
  <p>{{ selectedFiles }}</p>
</template>
