<script setup lang="ts">
import { ref } from 'vue';
import BaseModal from '../molecules/BaseModal.vue';
import BaseButton from '../atoms/BaseButton.vue';
import FormGroup from '../molecules/FormGroup.vue';
import BaseInput from '../atoms/BaseInput.vue';

interface Props {
    show: boolean;
}

defineProps<Props>();
const emit = defineEmits<{
    close: [];
    create: [data: { username: string; password: string; role: string }];
}>();

const username = ref('');
const password = ref('');
const role = ref('participant');

function handleCreate() {
    if (!username.value || !password.value) {
        alert('Username and password are required');
        return;
    }

    emit('create', {
        username: username.value,
        password: password.value,
        role: role.value
    });

    // Reset form
    username.value = '';
    password.value = '';
    role.value = 'participant';
}

function handleClose() {
    emit('close');
    // Reset form
    username.value = '';
    password.value = '';
    role.value = 'participant';
}
</script>

<template>
    <BaseModal :show="show" title="Register New User" size="md" @close="handleClose">
        <p class="text-xs text-gray-900 dark:text-eling-dark-text/50 mb-4">
            Create a new candidate or admin account.
        </p>

        <div class="space-y-4">
            <FormGroup label="Username" required>
                <BaseInput
                    v-model="username"
                    placeholder="Enter username"
                />
            </FormGroup>

            <FormGroup label="Role" required>
                <select 
                    v-model="role"
                    class="input-glass w-full py-2.5 text-sm appearance-none"
                >
                    <option value="participant">Participant (Candidate)</option>
                    <option value="admin">Administrator</option>
                </select>
            </FormGroup>

            <FormGroup label="Password" required>
                <BaseInput
                    v-model="password"
                    type="password"
                    placeholder="••••••••"
                />
            </FormGroup>
        </div>

        <template #footer>
            <div class="flex flex-row-reverse gap-3">
                <BaseButton 
                    size="md"
                    @click="handleCreate"
                >
                    Create User
                </BaseButton>
                <BaseButton 
                    variant="ghost" 
                    size="md"
                    @click="handleClose"
                >
                    Cancel
                </BaseButton>
            </div>
        </template>
    </BaseModal>
</template>
