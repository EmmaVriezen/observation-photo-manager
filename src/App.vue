<script setup lang="ts">
import FileSelection from './components/FileSelection.vue'
import FolderSelection from './components/FolderSelection.vue'
import ImageCarousel from './components/ImageCarousel.vue'
import { type Photo } from './interfaces/photo.ts'
import { ref } from 'vue'

const saveDir = ref('')
const selectedFiles = ref<Photo[]>([])

const setFileSelection = (files: string[]): void => {
  const photos: Photo[] = []
  files.forEach(file => {
    const photo = { file, cropped: false }
    photos.push(photo)
  })
  selectedFiles.value = photos
}
</script>

<template>
  <div class="container">
    <h1>Observation Photo Manager</h1>
    <FileSelection
      :numberOfFiles="selectedFiles.length"
      @set-file-selection="(arr) => setFileSelection(arr)"/>
    <FolderSelection
      :selectedFolder="saveDir"
      @set-folder="(str) => saveDir = str"/>
    <ImageCarousel
      v-if="selectedFiles.length > 0"
      :images="selectedFiles"
      :savePath="saveDir"/>
  </div>
</template>
