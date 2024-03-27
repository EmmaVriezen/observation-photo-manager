<script setup lang="ts">
import { ref, computed } from "vue";
import { open } from '@tauri-apps/api/dialog';

const fileSelection = ref<String[]>([]);

const count = computed(() => fileSelection.value.length);

const buttonLabel = computed(() => {
    if (count.value === 1) {
        return count.value + " file selected";
    } else if (count.value) {
        return count.value + " files selected";
    } else {
        return "Select files...";
    }
});

const selectImages = async () => {
    let selection = await open({
        multiple: true,
        filters: [{
            name: "Image",
            extensions: ['png', 'jpeg', 'jpg']
        }]
    });
    if (Array.isArray(selection)) {
        fileSelection.value = selection;
    } else if (selection === null) {
        fileSelection.value = [];
    } else {
        fileSelection.value = [selection];
    }
}
</script>

<template>
    <button type="button" @click="selectImages">{{ buttonLabel }}</button>
    <p>{{ fileSelection }}</p>
</template>
