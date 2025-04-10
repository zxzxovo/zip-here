<script setup lang="ts">
import CardButton from '../components/CardButton.vue';
import { useRouter } from 'vue-router';
import FileDragInputBox from '../components/FileDragInputBox.vue';
import { ref } from 'vue';
import { open as dialogOpen } from '@tauri-apps/plugin-dialog';

const router = useRouter();
const files = ref<string[]>([]);

const goCompress = () => {
    router.push({
        path: '/compress',
        query: { files: JSON.stringify(files.value) }
    });
};

const goDecompress = () => {
    router.push({
        path: '/decompress',
        query: { files: JSON.stringify(files.value) }
    });
};

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
            } else {
                files.value = [selected];
            }
        }
    } catch (error) {
        console.error(`选择文件出错: ${error}`);
    }
};

// 处理文件拖放
const handleFileDrop = async (e: DragEvent) => {
    if (e.dataTransfer?.files) {
        // 注意：在实际应用中，需要使用Tauri API获取文件路径
        // 这里只是添加了处理逻辑结构
        console.log('Files dropped');
        // TODO: 使用Tauri API处理文件路径
    }
};

// 清除选择的文件
const clearFiles = () => {
    files.value = [];
};
</script>

<template>
    <main class="home-container">
        <div class="home-drag-box">
            <FileDragInputBox class="app-file-drag" @drop="handleFileDrop" @click="handleFileSelect">
                <template #content>
                    <div v-if="files.length === 0">
                        <h1>欢迎使用 Zip Here</h1>
                        <p>拖拽文件到此处进行压缩或解压</p>
                        <p>或点击选择文件</p>
                    </div>
                    <div v-else>
                        <h2>已选择 {{ files.length }} 个文件:</h2>
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

        <div class="home-button-group">
            <CardButton class="button" @click="goCompress">
                <template #content>
                    开始压缩
                </template>
            </CardButton>
            <CardButton class="button" @click="goDecompress">
                <template #content>
                    开始解压
                </template>
            </CardButton>
        </div>
    </main>
</template>

<style lang="scss" scoped>
.home-container {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
    justify-content: center;
}

.home-drag-box {
    width: 100%;
    flex: 1 1 auto;
    padding: 2rem;
    display: flex;
    justify-content: center;
    align-items: center;
    background-color: #f0f0f0;
}

.app-file-drag {
    width: 100%;
    height: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
    text-align: center;
    padding: 1rem;
}

.app-file-list {
    max-height: 150px;
    overflow-y: auto;
    margin: 0.5rem 0;
    text-align: left;
    list-style-type: none;
    padding: 0;
}

.app-file-list li {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    padding: 0.2rem 0;
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
}

.app-btn-small {
    padding: 0.25rem 0.5rem;
    font-size: 0.8rem;
    margin-top: 0.5rem;
}

.home-button-group {
    display: flex;
    flex-direction: row;
    justify-content: center;
    align-items: center;
    margin-bottom: 2rem;
    gap: 1.5rem;
}

.home-button-group>.button {
    width: 10rem;
}
</style>
