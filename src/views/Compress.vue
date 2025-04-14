<script setup lang="ts">
import { computed, ref, watch, onMounted } from 'vue';
import RadioGroup from '../components/RadioGroup.vue';
import FileDragInputBox from '../components/FileDragInputBox.vue';
import { open as dialogOpen } from '@tauri-apps/plugin-dialog';
import Card from '../components/Card.vue';
import { useRoute } from 'vue-router';
import { compressFiles, getFormatOptions, getSupportedFormats, saveFile } from '../utils/tauri-api';
import type { FormatOption } from '../utils/tauri-api';

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
const compressFormat = ref('zip');
const compressionLevel = ref(6);
const outputPath = ref<'select_path' | 'source_path' | 'desktop_path'>('source_path');
const fileName = ref('compressed');
const sourcePath = ref('');
const isProcessing = ref(false);
const errorMessage = ref('');
const successMessage = ref('');
const showAdvanced = ref(false);
const password = ref('');
const usePassword = ref(false);

// 格式选项
const formatOptions = ref<FormatOption[]>([]);
const supportedFormats = ref<string[]>([]);
const currentFormatOption = ref<FormatOption | null>(null);

// 从后端加载支持的压缩格式和选项
const loadFormats = async () => {
    try {
        // 获取支持的格式
        supportedFormats.value = await getSupportedFormats();
        
        // 加载每个格式的详细信息
        const options = await Promise.all(
            supportedFormats.value.map(async format => {
                try {
                    return await getFormatOptions(format);
                } catch (error) {
                    console.error(`加载格式选项失败: ${format}`, error);
                    return null;
                }
            })
        );
        
        formatOptions.value = options.filter((opt): opt is FormatOption => opt !== null);
        
        // 更新选中格式的详情
        updateFormatDetails();
    } catch (error) {
        console.error('加载格式信息失败:', error);
        errorMessage.value = `加载格式信息失败: ${error}`;
    }
};

// 更新当前选择格式的详细信息
const updateFormatDetails = () => {
    const selected = formatOptions.value.find(opt => opt.extension === compressFormat.value);
    currentFormatOption.value = selected || null;
    
    // 如果格式不支持密码，禁用密码选项
    if (selected && !selected.supports_password) {
        usePassword.value = false;
    }
    
    // 如果格式不支持压缩级别，使用默认级别
    if (selected) {
        if (selected.supports_level && selected.default_level !== undefined) {
            compressionLevel.value = selected.default_level;
        }
    }
};

// 从路由参数获取文件
onMounted(async () => {
    // 加载格式信息
    await loadFormats();
    
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
const handleFileDrop = async (paths: string[]) => {
    if (Array.isArray(paths)) {
        console.log('Files dropped:', paths);
        files.value = paths;
        updateSourcePath();
        errorMessage.value = '';
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
        const selected = await saveFile(
            `${fileName.value}.${compressFormat.value}`,
            [{ name: '压缩文件', extensions: [compressFormat.value] }]
        );

        if (selected !== null) {
            // 从路径中提取文件名和目录
            const lastSlashIndex = selected.lastIndexOf('\\') > -1 ?
                selected.lastIndexOf('\\') :
                selected.lastIndexOf('/');
            
            selectedOutputPath.value = selected.substring(0, lastSlashIndex);
            
            // 提取不带扩展名的文件名
            const fileNameWithExt = selected.substring(lastSlashIndex + 1);
            const dotIndex = fileNameWithExt.lastIndexOf('.');
            if (dotIndex > -1) {
                fileName.value = fileNameWithExt.substring(0, dotIndex);
            } else {
                fileName.value = fileNameWithExt;
            }
            
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
    successMessage.value = '';

    try {
        // 调用Tauri API进行压缩
        await compressFiles(
            files.value,
            handledOutputPath.value,
            {
                format: compressFormat.value,
                level: compressionLevel.value,
                password: usePassword.value ? password.value : undefined
            }
        );

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

// 监听输出路径变化
watch(outputPath, (newV, _oldV) => {
    if (newV === 'select_path') {
        selectOutputPath();
    }
});

// 监听压缩格式变化
watch(compressFormat, () => {
    updateFormatDetails();
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
                    <FileDragInputBox class="app-file-drag" @file-dropped="handleFileDrop" @click="handleFileSelect">
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
                            <option v-for="option in formatOptions" :key="option.id" :value="option.extension">
                                {{ option.name }}
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
                    <!-- 压缩级别选项 -->
                    <div v-if="currentFormatOption && currentFormatOption.supports_level" class="app-option-group">
                        <label for="compression-level">压缩级别:</label>
                        <input 
                            id="compression-level" 
                            type="range" 
                            v-model.number="compressionLevel"
                            :min="currentFormatOption.min_level"
                            :max="currentFormatOption.max_level"
                        >
                        <span>{{ compressionLevel }}</span>
                    </div>

                    <!-- 密码选项 -->
                    <div v-if="currentFormatOption && currentFormatOption.supports_password" class="app-option-group">
                        <div class="checkbox-group">
                            <input type="checkbox" id="use-password" v-model="usePassword">
                            <label for="use-password">使用密码保护</label>
                        </div>
                    </div>

                    <div v-if="usePassword" class="app-option-group">
                        <label for="password">密码:</label>
                        <input type="password" id="password" v-model="password" class="app-input" placeholder="输入密码">
                    </div>
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
    margin-top: 0.8rem;
    flex: 1 1 auto;
    min-height: 220px;
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

.checkbox-group {
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