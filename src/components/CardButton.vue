<script setup lang="ts">
import Card from './Card.vue';

</script>

<template>
    <div class="card-button-container">
        <Card class="card-button"
            @click="$emit('click')">
            <template #body>
                <slot name="content"></slot>
            </template>
        </Card>
    </div>
</template>

<style lang="scss" scoped>
.card-button-container {
    justify-content: center;
    align-items: center;
}

.card-button {
    padding: 1.2rem;
    border-radius: 0.4rem;
    background-color: #fff;
}

.card-button:hover {
    background-color: #f0f0f0;
    cursor: pointer;
}

.card-button:active {
    background-color: #e0e0e0;
}

</style>