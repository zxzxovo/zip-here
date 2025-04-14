<script setup lang="ts">
import { computed, ref, watch, onMounted } from 'vue';
import FileDragInputBox from '../components/FileDragInputBox.vue';
import { open as dialogOpen } from '@tauri-apps/plugin-dialog';
import Card from '../components/Card.vue';
import RadioGroup from '../components/RadioGroup.vue';
import { useRoute } from 'vue-router';
import { decompressFiles } from '../utils/tauri-api';

// 添加路由对象以获取查询参数
const route = useRoute();

// 状态
const files = ref<string[]>([]);
const selectedOutputPath = ref('');
const handledOutputPath = computed(() => { 
    if (outputMethod.value === 'create_folder') {
        return `${selectedOutputPath.value}/Decompressed`;
    } else if (outputMethod.value === 'one_folder') {
        return `${selectedOutputPath.value}/All_Decompressed`;
    } else {
        return selectedOutputPath.value;
    }
});
const outputMethod = ref<'create_folder' | 'direct' | 'one_folder'>('create_folder');
const outputOption = ref<'select_path' | 'source_path' | 'desktop_path'>('source_path');
const isProcessing = ref(false);
const errorMessage = ref('');
const successMessage = ref('');
const password = ref('');
const usePassword = ref(false);

// 从路由参数获取文件
onMounted(() => {
    const filesParam = route.query.files;
    if (filesParam && typeof filesParam === 'string') {
        try {
            const parsedFiles = JSON.parse(filesParam);
            if (Array.isArray(parsedFiles)) {
                // 过滤只保留压缩文件格式
                const compressedFiles = parsedFiles.filter(file => {
                    const ext = file.split('.').pop()?.toLowerCase();
                    return ['zip', 'rar', '7z', 'tar', 'gz'].includes(ext || '');
                });
                
                if (compressedFiles.length > 0) {
                    files.value = compressedFiles;
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
                name: 'Archive',
                extensions: ['zip', 'rar', '7z', 'tar', 'gz']
            }]
        });

        if (selected !== null) {
            if (Array.isArray(selected)) {
                files.value = selected;
                updateSourcePath();
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
        updateSourcePath();
        files.value = paths;
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

        selectedOutputPath.value = filePath.substring(0, lastSlashIndex);
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
    selectedOutputPath.value = '';
};

// 开始解压
const startDecompress = async () => {
    if (files.value.length === 0) {
        errorMessage.value = '请选择要解压的文件';
        return;
    }

    if (selectedOutputPath.value === '' && outputOption.value === 'select_path') {
        errorMessage.value = '请选择输出路径';
        return;
    }

    isProcessing.value = true;
    errorMessage.value = '';
    successMessage.value = '';

    try {
        // 根据outputMethod生成正确的目标路径
        const targetPath = handledOutputPath.value;
        
        // 检查是否需要密码等选项
        const options: DecompressOptions = {};
        if (usePassword.value) {
            options.password = password.value;
        }
        
        console.log(`正在解压文件到: ${targetPath}`);
        
        // 调用后端解压API
        await decompressFiles(files.value, targetPath, '', options);

        successMessage.value = '文件解压成功!';
        setTimeout(() => {
            successMessage.value = '';
        }, 3000);
        
        // 解压成功后清除选择的文件
        clearFiles();
    } catch (error) {
        console.error('解压失败:', error);
        errorMessage.value = `解压失败: ${error instanceof Error ? error.message : String(error)}`;
    } finally {
        isProcessing.value = false;
    }
};

watch(outputOption, (newV, _oldV) => {
    if (newV === 'source_path') {
        // 使用源文件路径
        updateSourcePath();
    } else if (newV === 'desktop_path') {
        selectedOutputPath.value = `/Desktop`;
    } else if (newV === 'select_path') {
        selectOutputPath();
    }
});
</script>

<template>
    <div class="app-container">
        <!-- File Input -->
        <Card class="app-section card input-card">
            <template #header>
                <h3>选择压缩文件</h3>
            </template>
            <template #body>
                <div class="app-file-input">
                    <FileDragInputBox class="app-file-drag" @file-dropped="handleFileDrop" @click="handleFileSelect">
                        <template #content>
                            <div v-if="files.length === 0">
                                <p>拖拽文件到此处，或点击选择文件</p>
                                <p>支持 .zip、.rar、.7z、.tar.gz 等格式</p>
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

        <!-- Output Options -->
        <Card class="app-section card option-card">
            <template #header>
                <h3>输出选项</h3>
            </template>
            <template #body>
                <div class="app-options">
                    <div class="app-option-group">
                        <RadioGroup label="输出路径：" v-model="outputOption" :options="[
                            { value: 'select_path', label: '选择输出路径' },
                            { value: 'source_path', label: '使用压缩文件路径' },
                            { value: 'desktop_path', label: '输出到桌面' }
                        ]
                            " name="outputOption" />
                    </div>

                    <div class="app-option-group">
                        <RadioGroup label="解压方式：" v-model="outputMethod" :options="[
                            { value: 'create_folder', label: '为每个压缩包创建文件夹' },
                            { value: 'direct', label: '直接解压到选择的目录' },
                            { value: 'one_folder', label: '解压到一个文件夹' }
                        ]
                            " name="outputMethod" />
                    </div>

                    <div class="app-option-group">
                        <label>
                            <input type="checkbox" v-model="usePassword" />
                            使用密码
                        </label>
                        <input type="password" v-model="password" :disabled="!usePassword" placeholder="输入密码" />
                    </div>

                    <div class="app-path-display">
                        输出路径: {{ handledOutputPath }}
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
                <span v-if="isProcessing">解压中...</span>
                <span v-else>开始解压</span>
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