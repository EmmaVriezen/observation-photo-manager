<script setup lang="ts">
import ImageDisplay from './ImageDisplay.vue'
import SaveFile from './SaveFile.vue'
import { ref, watch } from 'vue'

const props = defineProps({
  imagePaths: { type: Array<string>, required: true },
  savePath: { type: String, required: true }
})

const index = ref<number>(0)
const xTop = ref<string>('0')
const yTop = ref<string>('0')
const xLow = ref<string>('0')
const yLow = ref<string>('0')

watch(() => props.imagePaths, () => {
  index.value = 0
})

const setCrop = (crop: number[]): void => {
  xTop.value = (crop[2] > 0 ? crop[0] : crop[0] + crop[2]).toString()
  yTop.value = (crop[3] > 0 ? crop[1] : crop[1] + crop[3]).toString()
  xLow.value = (crop[2] > 0 ? crop[2] : -crop[2]).toString()
  yLow.value = (crop[3] > 0 ? crop[3] : -crop[3]).toString()
}

const goLeft = (): void => {
  if (index.value > 0) {
    index.value--
  } else {
    index.value = props.imagePaths.length - 1
  }
}

const goRight = (): void => {
  if (index.value < props.imagePaths.length - 1) {
    index.value++
  } else {
    index.value = 0
  }
}
</script>

<template>
  <p>{{ index }}</p>
  <div class="carousel">
    <button
      type="button"
      :disabled="props.imagePaths.length === 1"
      @click="goLeft">
        &lt;
      </button>
    <ImageDisplay
      v-if=props.imagePaths.length
      :path="props.imagePaths[index]"
      :key="props.imagePaths[index]"
      @set-crop="(arr) => setCrop(arr)"/>
    <button
      type="button"
      :disabled="props.imagePaths.length === 1"
      @click="goRight">
        &gt;
    </button>
  </div>
  <SaveFile
    v-if="props.imagePaths.length"
    :source-path="props.imagePaths[index]"
    :destination-path="props.savePath"
    :crop="[xTop, yTop, xLow, yLow]"/>
</template>

<style scoped>
.carousel {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

button[disabled] {
  background-color: rgb(162, 162, 162);
  color: rgb(205, 204, 204);
  cursor: not-allowed;
}
</style>
