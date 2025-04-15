<script setup lang="ts">
import { computed, ref, watch, onMounted } from 'vue';
import RadioGroup from '../components/RadioGroup.vue';
import FileDragInputBox from '../components/FileDragInputBox.vue';
import { open as dialogOpen } from '@tauri-apps/plugin-dialog';
import Card from '../components/Card.vue';
import { useRoute } from 'vue-router';
import { decompressFiles, selectDirectory, getDesktopPath } from '../utils/tauri-api';
import { useI18n } from '../i18n';

// Get internationalization instance
const { t } = useI18n();

// Route object for query parameters
const route = useRoute();

// State
const files = ref<string[]>([]);
const inputFilePath = ref('');
const outputPath = ref<'source_path' | 'desktop_path' | 'select_path'>('source_path');
const selectedOutputPath = ref('');
const desktopPath = ref('');
const sourcePath = ref('');
const isProcessing = ref(false);
const errorMessage = ref('');
const successMessage = ref('');
const password = ref('');
const usePassword = ref(false);

// Computed output path based on selection
const handledOutputPath = computed(() => {
    if (outputPath.value === 'source_path') {
        return sourcePath.value;
    } else if (outputPath.value === 'desktop_path') {
        return desktopPath.value;
    } else {
        return selectedOutputPath.value;
    }
});

// Handle file selection
const handleFileSelect = async () => {
    try {
        const selected = await dialogOpen({
            multiple: false,
            filters: [{
                name: 'Archives',
                extensions: ['zip', 'tar', 'gz', 'xz', 'bz2', 'zst', '7z']
            }]
        });

        if (selected !== null) {
            if (Array.isArray(selected)) {
                files.value = selected;
                inputFilePath.value = selected[0] || '';
                updateSourcePath();
            } else {
                files.value = [selected];
                inputFilePath.value = selected;
                updateSourcePath();
            }
            errorMessage.value = '';
        }
    } catch (error) {
        errorMessage.value = `${t('error')}: ${error}`;
    }
};

// Handle file drop
const handleFileDrop = async (paths: string[]) => {
    if (Array.isArray(paths) && paths.length > 0) {
        files.value = [paths[0]]; // Only taking the first file for decompression
        inputFilePath.value = paths[0];
        updateSourcePath();
        errorMessage.value = '';
    }
};

// Update source path when file changes
const updateSourcePath = () => {
    if (inputFilePath.value) {
        const filePath = inputFilePath.value;
        // Get directory containing the file
        const lastSlashIndex = filePath.lastIndexOf('\\') > -1 ?
            filePath.lastIndexOf('\\') :
            filePath.lastIndexOf('/');

        sourcePath.value = filePath.substring(0, lastSlashIndex);
    }
};

// Select output path
const selectOutputPath = async () => {
    try {
        const selected = await selectDirectory(false);

        if (selected !== null && typeof selected === 'string') {
            selectedOutputPath.value = selected;
            errorMessage.value = '';
        }
    } catch (error) {
        errorMessage.value = `${t('error')}: ${error}`;
    }
};

// Clear selected file
const clearFiles = () => {
    files.value = [];
    inputFilePath.value = '';
    sourcePath.value = '';
};

// Start decompression
const startDecompress = async () => {
    if (files.value.length === 0) {
        errorMessage.value = t('pleaseSelectArchive');
        return;
    }

    // Validate selected path
    if (outputPath.value === 'select_path' && !selectedOutputPath.value) {
        errorMessage.value = t('pleaseSelectOutputPath');
        return;
    }

    isProcessing.value = true;
    errorMessage.value = '';
    successMessage.value = '';

    try {
        const options = usePassword.value ? { password: password.value } : undefined;

        await decompressFiles(
            [inputFilePath.value], 
            handledOutputPath.value, 
            '', // Auto-detect format
            options
        );

        successMessage.value = t('decompressSuccess');
        setTimeout(() => {
            successMessage.value = '';
        }, 3000);
    } catch (error) {
        console.error('Decompression failed:', error);
        errorMessage.value = t('decompressError', [error instanceof Error ? error.message : String(error)]);
    } finally {
        isProcessing.value = false;
    }
};

// Watch for output path changes
watch(outputPath, (newV, _oldV) => {
    if (newV === 'source_path') {
        // Use source file path
        updateSourcePath();
    } else if (newV === 'desktop_path') {
        // Desktop path is handled in the computed property
    } else if (newV === 'select_path') {
        selectOutputPath();
    }
});

// Initialize component
onMounted(async () => {
    // Get desktop path
    desktopPath.value = await getDesktopPath();
    
    // Check for files passed in query parameters
    const filesParam = route.query.files;
    if (filesParam && typeof filesParam === 'string') {
        try {
            const parsedFiles = JSON.parse(filesParam);
            if (Array.isArray(parsedFiles) && parsedFiles.length > 0) {
                files.value = [parsedFiles[0]]; // Just use the first file for decompression
                inputFilePath.value = parsedFiles[0];
                updateSourcePath();
            }
        } catch (error) {
            console.error('Failed to parse files parameter:', error);
        }
    }
});
</script>

<template>
    <div class="app-container">
        <!-- File Input -->
        <Card class="app-section card input-card">
            <template #header>
                <h3>{{ t('selectArchiveToDecompress') }}</h3>
            </template>
            <template #body>
                <div class="app-file-input">
                    <FileDragInputBox class="app-file-drag" @file-dropped="handleFileDrop" @click="handleFileSelect">
                        <template #content>
                            <div v-if="files.length === 0">
                                <p>{{ t('dragArchiveHere') }}</p>
                                <p>{{ t('supportedFormats') }}</p>
                            </div>
                            <div v-else>
                                <p>{{ t('selectedFiles', [files.length]) }}</p>
                                <div class="app-file-info">
                                    {{ inputFilePath.split('/').pop()?.split('\\').pop() }}
                                </div>
                                <button @click.stop="clearFiles" class="app-btn app-btn-small">{{ t('clearSelection') }}</button>
                            </div>
                        </template>
                    </FileDragInputBox>
                </div>
            </template>
        </Card>

        <!-- Decompression Options -->
        <Card class="app-section card option-card">
            <template #header>
                <h3>{{ t('decompressTitle') }}</h3>
            </template>
            <template #body>
                <div class="app-options">
                    <div class="app-option-group">
                        <RadioGroup 
                            :label="t('outputPath')" 
                            v-model="outputPath" 
                            :options="[
                                { value: 'select_path', label: t('selectOutputPath') },
                                { value: 'source_path', label: t('useSourcePath') },
                                { value: 'desktop_path', label: t('outputToDesktop') }
                            ]" 
                            name="outputPath" 
                        />
                    </div>

                    <div class="app-path-display">
                        {{ t('resultPath', [handledOutputPath]) }}
                    </div>

                    <div class="app-option-group">
                        <label>
                            <input type="checkbox" v-model="usePassword" />
                            {{ t('usePasswordToDecompress') }}
                        </label>
                        <input
                            v-if="usePassword"
                            type="password"
                            v-model="password"
                            :placeholder="t('enterPassword')"
                            class="app-input"
                        />
                    </div>
                </div>
            </template>
        </Card>

        <!-- Messages -->
        <div v-if="errorMessage" class="app-message app-error">{{ errorMessage }}</div>
        <div v-if="successMessage" class="app-message app-success">{{ successMessage }}</div>

        <!-- Button -->
        <div class="app-action">
            <button @click="startDecompress" class="app-btn app-btn-primary" :disabled="isProcessing || files.length === 0">
                <span v-if="isProcessing">{{ t('decompressing') }}</span>
                <span v-else>{{ t('decompressButtonText') }}</span>
            </button>
        </div>
    </div>
</template>

<style lang="scss" scoped>
.app-container {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
    padding: 0 2rem;
    overflow-y: auto;
    align-items: center;
    gap: 1.2rem;
}

.card {
    background-color: #fff;
    width: 100%;
    box-shadow: 0 1px 3px rgba(0,0,0,0.12), 0 1px 2px rgba(0,0,0,0.24);
    border-radius: 6px;
}

.input-card {
    margin-top: 0.8rem;
    flex: 1 1 auto;
    min-height: 220px;
}

.option-card {
    flex: 1 1 auto;
}

.app-file-input {
    width: 100%;
    height: 150px;
}

.app-file-drag {
    height: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
    text-align: center;
    padding: 1rem;
    border: 2px dashed #ccc;
    border-radius: 4px;
    transition: all 0.2s;
    
    &:hover {
        border-color: #1976d2;
    }
}

.app-file-info {
    margin: 0.5rem 0;
    padding: 0.3rem 0.6rem;
    background-color: #f5f5f5;
    border-radius: 4px;
    max-width: 100%;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
}

.app-options {
    display: flex;
    flex-direction: column;
    gap: 0.8rem;
    width: 100%;
}

.app-option-group {
    display: flex;
    flex-direction: column;
    gap: 0.4rem;
}

.app-input {
    padding: 0.5rem;
    border: 1px solid #ddd;
    border-radius: 4px;
    background-color: white;
    font-size: 0.9rem;
}

.app-path-display {
    font-size: 0.8rem;
    color: #666;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 100%;
    padding: 0.3rem 0;
}

.app-message {
    padding: 0.8rem;
    border-radius: 4px;
    width: 100%;
    text-align: center;
    font-weight: 500;
}

.app-error {
    background-color: #ffebee;
    color: #c62828;
    border: 1px solid #ef9a9a;
}

.app-success {
    background-color: #e8f5e9;
    color: #2e7d32;
    border: 1px solid #a5d6a7;
}

.app-action {
    display: flex;
    justify-content: center;
    margin-top: 1rem;
    margin-bottom: 1rem;
}

.app-btn {
    padding: 0.5rem 1rem;
    border: 1px solid #ddd;
    border-radius: 4px;
    background-color: #f5f5f5;
    cursor: pointer;
    transition: all 0.2s;

    &:hover {
        background-color: #e0e0e0;
    }

    &:disabled {
        opacity: 0.6;
        cursor: not-allowed;
    }
}

.app-btn-small {
    padding: 0.25rem 0.5rem;
    font-size: 0.8rem;
}

.app-btn-primary {
    background-color: #1976d2;
    color: white;
    border: none;

    &:hover {
        background-color: #1565c0;
    }
}
</style>