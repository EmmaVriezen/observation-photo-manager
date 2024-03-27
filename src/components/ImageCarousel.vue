<script setup lang="ts">
import ImageDisplay from './ImageDisplay.vue'
import { ref, watch } from 'vue'

const props = defineProps({ imagePaths: { type: Array<string>, required: true } })
const index = ref<number>(0)

watch(() => props.imagePaths, () => {
  index.value = 0
})

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
      :key="props.imagePaths[index]"/>
    <button
      type="button"
      :disabled="props.imagePaths.length === 1"
      @click="goRight">
        &gt;
    </button>
  </div>
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
