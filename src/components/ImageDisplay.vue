<script setup lang="ts">
import { convertFileSrc } from '@tauri-apps/api/tauri'

const props = defineProps({ path: { type: String, required: true } })

const imageUrl = convertFileSrc(props.path)

</script>

<template>
  <p>{{ props.path }}</p>
  <img :src="imageUrl"/>
</template>
