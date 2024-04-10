<script setup lang="ts">
import { ref } from 'vue'
import { convertFileSrc } from '@tauri-apps/api/tauri'

const props = defineProps({
  path: { type: String, required: true },
  crop: { type: Array, required: true }
})

const cropCanvas = ref()
const image = ref()
const imageLoaded = ref(false)
const width = ref<string>('0px')

const imageHeight: string = '400px'
const imageUrl = convertFileSrc(props.path)

const drawCrop = (): void => {
  const canvas = cropCanvas.value
  const ctx = canvas.getContext('2d')
  ctx.lineWidth = 1
  ctx.clearRect(0, 0, canvas.width, canvas.height)
  ctx.beginPath()
  ctx.rect(props.crop[0], props.crop[1], props.crop[2], props.crop[3])
  ctx.stroke()
}

const whenImgLoaded = (): void => {
  width.value = image.value.width + 'px'
  imageLoaded.value = true
}
</script>

<template>
  <div>
    <button type="button" @click="drawCrop">Show crop</button>
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
        :height="imageHeight">
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
