<script setup lang="ts">
import { ref } from 'vue'
import { convertFileSrc } from '@tauri-apps/api/tauri'

const props = defineProps({
  path: { type: String, required: true }
})
const emit = defineEmits(['setCrop'])

const cropCanvas = ref()
const image = ref()
const imageLoaded = ref(false)
const isDrawing = ref(false)
const x = ref(0)
const y = ref(0)
const width = ref<string>('0px')

const imageHeight: string = '400px'
const imageUrl = convertFileSrc(props.path)

const drawCrop = (event: Event): void => {
  if (isDrawing.value) {
    const canvas = cropCanvas.value
    const ctx = canvas.getContext('2d')
    const width = event.layerX - x.value
    const height = event.layerY - y.value
    ctx.lineWidth = 1
    ctx.clearRect(0, 0, canvas.width, canvas.height)
    ctx.beginPath()
    ctx.rect(x.value, y.value, width, height)
    ctx.stroke()
  }
}

const startDrawing = (event: Event): void => {
  x.value = event.layerX
  y.value = event.layerY
  isDrawing.value = true
  const canvas = cropCanvas.value
  const ctx = canvas.getContext('2d')
  ctx.clearRect(0, 0, canvas.width, canvas.height)
}

const stopDrawing = (event: Event): void => {
  isDrawing.value = false
  const width = event.layerX - x.value
  const height = event.layerY - y.value
  emit('setCrop', [x.value, y.value, width, height])
}

const whenImgLoaded = (): void => {
  width.value = image.value.width + 'px'
  imageLoaded.value = true
}
</script>

<template>
  <div>
    <div class="canvas-wrapper">
      <img
        ref="image"
        :src="imageUrl"
        :height="imageHeight"
        @load="whenImgLoaded"/>
      <canvas
        v-if="imageLoaded"
        class="crop-canvas"
        ref="cropCanvas"
        :width="width"
        :height="imageHeight"
        @mousedown="startDrawing"
        @mousemove="drawCrop"
        @mouseup="stopDrawing">
      </canvas>
    </div>
  </div>
</template>

<style>
.crop-canvas {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
}

.canvas-wrapper {
  position: relative;
}
</style>
