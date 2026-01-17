<script setup lang="ts">
interface Props {
    modelValue?: string | number;
    type?: 'text' | 'password' | 'email' | 'number' | 'date' | 'search';
    placeholder?: string;
    disabled?: boolean;
    required?: boolean;
    min?: string | number;
    max?: string | number;
    step?: string | number;
}

withDefaults(defineProps<Props>(), {
    type: 'text',
    disabled: false,
    required: false
});

const emit = defineEmits<{
    'update:modelValue': [value: string | number];
}>();

const handleInput = (event: Event) => {
    const target = event.target as HTMLInputElement;
    emit('update:modelValue', target.value);
};
</script>

<template>
    <input
        :value="modelValue"
        :type="type"
        :placeholder="placeholder"
        :disabled="disabled"
        :required="required"
        :min="min"
        :max="max"
        :step="step"
        @input="handleInput"
        class="input-glass w-full py-2.5 text-sm"
    />
</template>
