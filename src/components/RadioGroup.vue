<script setup lang="ts">

defineProps<{
    options: Array<{ value: string | number, label: string, click?: () => void }>,
    modelValue: string | number,
    name?: string,
    disabled?: boolean,
    label?: string
}>();

// 定义emit用于v-model
const emit = defineEmits<{
    'update:modelValue': [value: string | number]
}>();

const handleChange = (value: string | number) => {
    emit('update:modelValue', value);
};
</script>

<template>
    <div class="radio-container">

        <span v-if="label" class="radio-group-label">{{ label }}</span>
        <label v-for="option in options" :key="option.value" class="radio-label"
            :class="{ 'radio-disabled': disabled }">
            <input type="radio" :name="name" :value="option.value" :checked="modelValue === option.value"
                @change="handleChange(option.value)" :disabled="disabled" class="radio-input" />
            <span class="radio-text">{{ option.label }}</span>
        </label>
    </div>
</template>

<style lang="scss" scoped>
.radio-container {
    display: flex;
    flex-direction: row;
    align-items: center;
    gap: 16px;
}

.radio-group-label {
    font-size: 0.9rem;
    color: #333;
    margin-right: 8px;
    white-space: nowrap;
}

.radio-label {
    display: flex;
    align-items: center;
    cursor: pointer;
    padding: 4px 0;

    &.radio-disabled {
        cursor: not-allowed;
        opacity: 0.6;
    }
}

.radio-input {
    margin-right: 6px;
    cursor: pointer;

    &:disabled {
        cursor: not-allowed;
    }
}

.radio-text {
    user-select: none;
}
</style>
