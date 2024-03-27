<script setup lang="ts">
import FileSelection from './components/FileSelection.vue'
import FolderSelection from './components/FolderSelection.vue'
import ImageDisplay from './components/ImageDisplay.vue'
import { ref } from 'vue'

const saveDir = ref('')
const selectedFiles = ref<string[]>([])

</script>

<template>
  <div class="container">
    <h1>Observation Photo Manager</h1>
    <FileSelection
      :numberOfFiles="selectedFiles.length"
      @set-file-selection="(arr) => selectedFiles = arr"/>
    <FolderSelection
      :selectedFolder="saveDir"
      @set-folder="(str) => saveDir = str"/>
    <div v-for="file in selectedFiles" :key="file">
      <ImageDisplay :path="file"/>
    </div>
  </div>
</template>
