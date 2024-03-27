<script setup lang="ts">
import { convertFileSrc } from '@tauri-apps/api/tauri'

const props = defineProps({ path: { type: String, required: true } })

const imageUrl = convertFileSrc(props.path)

</script>

<template>
  <div>
    <p>{{ props.path }}</p>
    <img :src="imageUrl" height=400/>
  </div>
</template>
