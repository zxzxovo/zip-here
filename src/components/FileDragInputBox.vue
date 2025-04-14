<script setup lang="ts">
import { listen, UnlistenFn } from '@tauri-apps/api/event';
import { onMounted, onUnmounted } from 'vue';
const emit = defineEmits( ['file-dropped']);

let unlisten: UnlistenFn | undefined;
onMounted(async () => {
    // 监听文件拖放事件
    unlisten = await listen<{ paths: string[] }>('tauri://drag-drop', (event) => {
        const files = event.payload.paths;
        if(files && files.length > 0) {
            emit('file-dropped', files);
        }
    });
});

onUnmounted(() => {
    // 清除监听器
    if(unlisten) {
        unlisten();
    }
});

</script>

<template>
    <div class="file-drag-input-box" @dragover.prevent @drop.prevent>
        <div class="file-drag-input-box__content">
            <slot name="content"></slot>
        </div>
    </div>
</template>

<style lang="scss" scoped>
.file-drag-input-box {
    width: 100%;
    height: 100%;
    border: 2px dashed #aaa;
    text-align: center;
    align-content: center;
    border-radius: 0.8rem;
    transition: background-color 0.3s;
    cursor: pointer; 
}

.file-drag-input-box:hover {
    background-color: #f0f0f0;
}
</style>