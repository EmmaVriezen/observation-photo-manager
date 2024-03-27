<script setup lang="ts">
import { ref } from "vue";
import { open } from '@tauri-apps/api/dialog';

const fileSelection = ref("Select files...");
const count = ref(0);

const selected = async () => {
    let selection = await open({
        multiple: true,
        filters: [{
            name: "Image",
            extensions: ['png', 'jpeg', 'jpg']
        }]
    });
    if (Array.isArray(selection)) {
        fileSelection.value = selection[0];
        count.value = selection.length;
    } else if (selection === null) {
        fileSelection.value = "Select files...";
        count.value = 0;
    } else {
        fileSelection.value = selection;
        count.value = 1;
    }
}
</script>

<template>
    <p>Number of selected images: {{ count }}</p>
    <button type="button" @click="selected">{{ fileSelection }}</button>
</template>
