<script setup lang="ts">
import FileSelection from './components/FileSelection.vue'
import FolderSelection from './components/FolderSelection.vue'
import ImageCarousel from './components/ImageCarousel.vue'
import { ref } from 'vue'

const saveDir = ref('')
const selectedFiles = ref<string[]>([])

const setFileSelection = (files: string[]): void => {
  selectedFiles.value = files
}

</script>

<template>
  <div class="container">
    <h1>Observation Photo Manager</h1>
    <FileSelection
      :numberOfFiles="selectedFiles.length"
      @set-file-selection="(arr) => setFileSelection(arr)"
    />
    <FolderSelection
      :selectedFolder="saveDir"
      @set-folder="(str) => saveDir = str"
    />
    <ImageCarousel
      v-if="selectedFiles.length > 0"
      :imagePaths="selectedFiles"
      :savePath="saveDir"
    />
  </div>
</template>
