<script setup lang="ts">
import { computed, ref, watch, onMounted } from 'vue';
import RadioGroup from '../components/RadioGroup.vue';
import FileDragInputBox from '../components/FileDragInputBox.vue';
import { open as dialogOpen } from '@tauri-apps/plugin-dialog';
import Card from '../components/Card.vue';
import { useRoute } from 'vue-router';

// 添加路由对象以获取查询参数
const route = useRoute();

// 状态
const files = ref<string[]>([]);
const selectedOutputPath = ref('');
const handledOutputPath = computed(() => {
    if (outputPath.value === 'source_path') {
        return `${sourcePath.value}/${fileName.value}.${compressFormat.value}`;
    } else if (outputPath.value === 'desktop_path') {
        return `/Desktop/${fileName.value}.${compressFormat.value}`;
    } else {
        return `${selectedOutputPath.value}/${fileName.value}.${compressFormat.value}`;
    }
});
const compressFormat = ref<'zip' | 'rar' | '7z' | 'tar' | 'gz'>('zip');
const compressionLevel = ref<'store' | 'fast' | 'normal' | 'maximum'>('normal');
const outputPath = ref<'select_path' | 'source_path' | 'desktop_path'>('source_path');
const fileName = ref('compressed');
const sourcePath = ref('');
const isProcessing = ref(false);
const errorMessage = ref('');
const successMessage = ref('');
const showAdvanced = ref(false);

// 从路由参数获取文件
onMounted(() => {
    const filesParam = route.query.files;
    if (filesParam && typeof filesParam === 'string') {
        try {
            const parsedFiles = JSON.parse(filesParam);
            if (Array.isArray(parsedFiles)) {
                files.value = parsedFiles;
                if (files.value.length > 0) {
                    updateSourcePath();
                }
            }
        } catch (error) {
            console.error('解析文件参数出错:', error);
        }
    }
});

// 压缩格式选项
const formatOptions = [
    { value: 'zip', label: 'ZIP格式' },
    { value: 'rar', label: 'RAR格式' },
    { value: '7z', label: '7Z格式' },
    { value: 'tar', label: 'TAR格式' },
    { value: 'gz', label: 'GZ格式' }
];

// 处理文件选择
const handleFileSelect = async () => {
    try {
        const selected = await dialogOpen({
            multiple: true,
            filters: [{
                name: 'All Files',
                extensions: ['*']
            }]
        });

        if (selected !== null) {
            if (Array.isArray(selected)) {
                files.value = selected;
                if (files.value.length > 0) {
                    updateSourcePath();
                }
            } else {
                files.value = [selected];
                updateSourcePath();
            }
            errorMessage.value = '';
        }
    } catch (error) {
        errorMessage.value = `选择文件出错: ${error}`;
    }
};

// 处理文件拖放
const handleFileDrop = async (e: DragEvent) => {
    if (e.dataTransfer?.files) {
        // TODO: 处理文件路径获取，Tauri有特定API来处理这个
        // files.value = Array.from(e.dataTransfer.files).map(file => file.path || file.name);
        errorMessage.value = '';
        updateSourcePath();
    }
};

// 更新源文件路径
const updateSourcePath = () => {
    if (files.value.length > 0) {
        const filePath = files.value[0];
        // 获取文件所在目录
        const lastSlashIndex = filePath.lastIndexOf('\\') > -1 ?
            filePath.lastIndexOf('\\') :
            filePath.lastIndexOf('/');

        sourcePath.value = filePath.substring(0, lastSlashIndex);
    }
};

// 选择输出路径
const selectOutputPath = async () => {
    try {
        const selected = await dialogOpen({
            directory: true,
            multiple: false
        });

        if (selected !== null && typeof selected === 'string') {
            selectedOutputPath.value = selected;
            errorMessage.value = '';
        }
    } catch (error) {
        errorMessage.value = `选择输出路径出错: ${error}`;
    }
};

// 清除选择的文件
const clearFiles = () => {
    files.value = [];
    sourcePath.value = '';
};

// 开始压缩
const startCompress = async () => {
    if (files.value.length === 0) {
        errorMessage.value = '请选择要压缩的文件';
        return;
    }

    if (selectedOutputPath.value === '' && outputPath.value === 'select_path') {
        errorMessage.value = '请选择输出路径';
        return;
    }

    isProcessing.value = true;
    errorMessage.value = '';

    try {
        // TODO: 调用Tauri的Rust后端函数进行实际压缩
        // 示例：
        // await invoke('compress_files', { 
        //     files: files.value,
        //     outputPath: handledOutputPath.value,
        //     format: compressFormat.value,
        //     level: compressionLevel.value
        // });

        // 模拟压缩过程
        await new Promise(resolve => setTimeout(resolve, 2000));

        successMessage.value = '文件压缩成功!';
        setTimeout(() => {
            successMessage.value = '';
        }, 3000);
    } catch (error) {
        errorMessage.value = `压缩失败: ${error}`;
    } finally {
        isProcessing.value = false;
    }
};

watch(outputPath, (newV, _oldV) => {
    if (newV === 'select_path') {
        selectOutputPath();
    }
});

// 切换高级选项显示
const toggleAdvanced = () => {
    showAdvanced.value = !showAdvanced.value;
};
</script>

<template>
    <div class="app-container">
        <!-- File Input -->
        <Card class="app-section card input-card">
            <template #header>
                <h3>选择要压缩的文件</h3>
            </template>
            <template #body>
                <div class="app-file-input">
                    <FileDragInputBox class="app-file-drag" @drop="handleFileDrop" @click="handleFileSelect">
                        <template #content>
                            <div v-if="files.length === 0">
                                <p>拖拽文件到此处，或点击选择文件</p>
                                <p>可选择多个文件或文件夹进行压缩</p>
                            </div>
                            <div v-else>
                                <p>已选择 {{ files.length }} 个文件:</p>
                                <ul class="app-file-list">
                                    <li v-for="(file, index) in files" :key="index">
                                        {{ file.split('/').pop()?.split('\\').pop() }}
                                    </li>
                                </ul>
                                <button @click.stop="clearFiles" class="app-btn app-btn-small">清除选择</button>
                            </div>
                        </template>
                    </FileDragInputBox>
                </div>
            </template>
        </Card>

        <!-- Basic Options -->
        <Card class="app-section card option-card">
            <template #header>
                <h3>基本选项</h3>
            </template>
            <template #body>
                <div class="app-options">
                    <div class="app-option-group">
                        <label for="format-select">压缩格式：</label>
                        <select 
                            id="format-select" 
                            v-model="compressFormat" 
                            class="app-select">
                            <option v-for="option in formatOptions" :key="option.value" :value="option.value">
                                {{ option.label }}
                            </option>
                        </select>
                    </div>

                    <div class="app-option-group">
                        <RadioGroup label="输出路径：" v-model="outputPath" :options="[
                            { value: 'select_path', label: '选择输出路径' },
                            { value: 'source_path', label: '使用源文件路径' },
                            { value: 'desktop_path', label: '输出到桌面' }
                        ]" name="outputPath" />
                    </div>

                    <div class="app-option-group">
                        <label>文件名：</label>
                        <input v-model="fileName" type="text" class="app-input" placeholder="输入压缩包名称">
                    </div>

                    <div class="app-path-display">
                        输出路径: {{ handledOutputPath }}
                    </div>

                    <button @click="toggleAdvanced" class="app-btn app-btn-text">
                        {{ showAdvanced ? '隐藏高级选项 ▲' : '显示高级选项 ▼' }}
                    </button>
                </div>
            </template>
        </Card>

        <!-- Advanced Options (Expandable) -->
        <Card v-if="showAdvanced" class="app-section card advanced-card">
            <template #header>
                <h3>高级选项</h3>
            </template>
            <template #body>
                <div class="app-options">
                    <div class="app-option-group">
                        <RadioGroup label="压缩级别：" v-model="compressionLevel" :options="[
                            { value: 'store', label: '存储 (不压缩)' },
                            { value: 'fast', label: '快速压缩' },
                            { value: 'normal', label: '标准压缩' },
                            { value: 'maximum', label: '最大压缩' }
                        ]" name="compressionLevel" />
                    </div>

                    <!-- TODO: 可以添加更多高级选项，如加密、分卷等 -->
                </div>
            </template>
        </Card>

        <!-- Messages -->
        <div v-if="errorMessage" class="app-message app-error">{{ errorMessage }}</div>
        <div v-if="successMessage" class="app-message app-success">{{ successMessage }}</div>

        <!-- Button -->
        <div class="app-action">
            <button @click="startCompress" class="app-btn app-btn-primary" :disabled="isProcessing || files.length === 0">
                <span v-if="isProcessing">压缩中...</span>
                <span v-else>开始压缩</span>
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
    flex: 1 1 auto;
    min-height: 180px;
}

.option-card {
    flex: 1 1 auto;
}

.advanced-card {
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

.app-file-list {
    max-height: 80px;
    overflow-y: auto;
    margin: 0.5rem 0;
    text-align: center;
    list-style-type: none;
    padding-left: 0;
}

.app-file-list li {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    padding: 0.2rem 0;
}

.app-options {
    display: flex;
    flex-direction: column;
    gap: 0.6rem;
    width: 100%;
}

.app-option-group {
    display: flex;
    align-items: center;
    gap: 0.5rem;
}

.app-input {
    padding: 0.3rem;
    border: 1px solid #ddd;
    border-radius: 4px;
    flex-grow: 1;
}

.app-select {
    padding: 0.3rem;
    border: 1px solid #ddd;
    border-radius: 4px;
    background-color: white;
    cursor: pointer;
    min-width: 150px;
    flex-grow: 1;
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

.app-btn-text {
    background: none;
    border: none;
    color: #1976d2;
    text-decoration: underline;
    padding: 0.2rem;

    &:hover {
        background: none;
        color: #1565c0;
    }
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
</style>