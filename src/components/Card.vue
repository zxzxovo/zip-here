<script setup lang="ts">

</script>

<template>
    <div class="card-container">

        <div v-if="$slots.header" class="card-header">
            <slot name="header"></slot>
        </div>

        <div class="card-body">
            <slot name="body"></slot>
        </div>

    </div>
</template>

<style scoped lang="scss">
.card-container {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
    border-radius: 0.4rem;
    border: 1px solid rgba($color: #000000, $alpha: 0.1);
    box-shadow: 0 1px 2px rgba(0, 0, 0, 0.1);
    padding: 1rem;
}
.card-header {
    width: 100%;
    flex: 0 0 2rem;
    display: flex;
    justify-content: center;
    align-items: center;
    border-bottom: 1px solid #e0e0e0;
}
.card-body {
    width: 100%;
    flex: 1 1 auto;
    display: flex;
    justify-content: center;
    align-items: center;
}
</style>