<script setup lang="ts">

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