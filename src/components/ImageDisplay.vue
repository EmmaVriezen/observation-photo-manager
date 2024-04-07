<script setup lang="ts">
import { ref, watch } from 'vue'
import { convertFileSrc } from '@tauri-apps/api/tauri'

const props = defineProps({
  path: { type: String, required: true },
  crop: { type: Array, required: true }
})

const cropCanvas = ref()
const image = ref()
const width = ref()

const imageHeight: string = '400px'

const imageUrl = convertFileSrc(props.path)

watch(() => image.value, () => {
  console.log(image.value)
  if (image.value !== null && image.value !== undefined) {
    width.value = image.value.width + 'px'
  } else {
    width.value = '0px'
  }
})

const drawCrop = (): void => {
  const canvas = cropCanvas.value
  const ctx = canvas.getContext('2d')
  ctx.lineWidth = 1
  ctx.clearRect(0, 0, canvas.width, canvas.height)
  ctx.beginPath()
  ctx.rect(props.crop[0], props.crop[1], props.crop[2], props.crop[3])
  ctx.stroke()
}
</script>

<template>
  <div>
    <button type="button" @click="drawCrop">Show crop</button>
    <div class="canvas-wrapper">
      <img ref="image" :src="imageUrl" :height="imageHeight"/>
      <canvas v-if="width !== '0px'" class="crop-canvas" ref="cropCanvas" :width="width" :height="imageHeight"></canvas>
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
