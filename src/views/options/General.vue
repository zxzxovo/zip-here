<script setup lang="ts">
import { ref, onMounted } from 'vue';
import Card from '../../components/Card.vue';
import { 
  isWindowsOS, 
  addContextMenu, 
  removeContextMenu, 
  setFileAssociation, 
  removeFileAssociation 
} from '../../utils/tauri-api';

// 记住上次保存的设置
interface GeneralSettings {
  autoExtractBehavior: 'ask' | 'always' | 'never';
  defaultCompressFormat: string;
  defaultCompressionLevel: number;
  showFinishNotification: boolean;
  clearFilesAfterOperation: boolean;
  darkMode: boolean;
  // 新增设置项
  contextMenuEnabled: boolean;
  fileAssociationsEnabled: boolean;
  rightClickOpenMode: 'cli' | 'gui';
  fileAssociationOpenMode: 'gui' | 'viewer';
  associatedFormats: string;
}

// 默认设置
const defaultSettings: GeneralSettings = {
  autoExtractBehavior: 'ask',
  defaultCompressFormat: 'zip',
  defaultCompressionLevel: 6,
  showFinishNotification: true,
  clearFilesAfterOperation: true,
  darkMode: false,
  // 新增设置项的默认值
  contextMenuEnabled: false,
  fileAssociationsEnabled: false,
  rightClickOpenMode: 'gui',
  fileAssociationOpenMode: 'gui',
  associatedFormats: 'all'
};

// 设置状态
const settings = ref<GeneralSettings>({...defaultSettings});
const isDirty = ref(false);
const isSaving = ref(false);
const message = ref('');
const isWindows = ref(false);

// 加载设置
const loadSettings = async () => {
  try {
    // 检查当前平台是否为 Windows
    isWindows.value = await isWindowsOS();
    
    // 可以使用localStorage或者tauri的store来存储设置
    // 这里使用localStorage示例
    const savedSettings = localStorage.getItem('general_settings');
    if (savedSettings) {
      settings.value = JSON.parse(savedSettings);
    }
  } catch (error) {
    console.error('Failed to load settings:', error);
  }
};

// 保存设置
const saveSettings = async () => {
  try {
    isSaving.value = true;
    // 保存到localStorage
    localStorage.setItem('general_settings', JSON.stringify(settings.value));
    
    // 如果是Windows系统，处理右键菜单和文件关联设置
    if (isWindows.value) {
      await applyWindowsSettings();
    }
    
    isDirty.value = false;
    message.value = 'Settings saved successfully';
    
    // 3秒后清除消息
    setTimeout(() => {
      message.value = '';
    }, 3000);
  } catch (error) {
    console.error('Failed to save settings:', error);
    message.value = `Failed to save settings: ${error}`;
  } finally {
    isSaving.value = false;
  }
};

// 应用Windows设置
const applyWindowsSettings = async () => {
  try {
    // 处理右键菜单
    if (settings.value.contextMenuEnabled) {
      await addContextMenu(settings.value.rightClickOpenMode);
    } else {
      await removeContextMenu();
    }
    
    // 处理文件关联
    if (settings.value.fileAssociationsEnabled) {
      await setFileAssociation(
        settings.value.associatedFormats,
        settings.value.fileAssociationOpenMode
      );
    } else {
      await removeFileAssociation(settings.value.associatedFormats);
    }
  } catch (error) {
    console.error('Failed to apply Windows settings:', error);
    throw error;
  }
};

// 重置设置
const resetSettings = () => {
  settings.value = {...defaultSettings};
  isDirty.value = true;
};

// 监听设置变化
const updateSetting = () => {
  isDirty.value = true;
};

// 初始化时加载设置
onMounted(() => {
  loadSettings();
});
</script>

<template>
  <div class="general-settings">
    <Card class="app-section card">
      <template #header>
        <h3>General Settings</h3>
      </template>
      <template #body>
        <div class="settings-form">
          <!-- Auto Extract Behavior -->
          <div class="settings-group">
            <label>Auto Extract Behavior:</label>
            <select v-model="settings.autoExtractBehavior" @change="updateSetting" class="app-select">
              <option value="ask">Ask before extracting</option>
              <option value="always">Always extract automatically</option>
              <option value="never">Never extract automatically</option>
            </select>
          </div>
          
          <!-- Default Compress Format -->
          <div class="settings-group">
            <label>Default Compress Format:</label>
            <select v-model="settings.defaultCompressFormat" @change="updateSetting" class="app-select">
              <option value="zip">ZIP</option>
              <option value="7z">7-Zip</option>
              <option value="tar">TAR</option>
              <option value="gz">GZIP</option>
            </select>
          </div>
          
          <!-- Default Compression Level -->
          <div class="settings-group">
            <label>Default Compression Level: {{ settings.defaultCompressionLevel }}</label>
            <input 
              type="range" 
              v-model.number="settings.defaultCompressionLevel" 
              @change="updateSetting"
              min="1" 
              max="9" 
              class="app-range"
            />
          </div>
          
          <!-- Show Finish Notification -->
          <div class="settings-group checkbox-group">
            <input 
              type="checkbox" 
              id="show-notification" 
              v-model="settings.showFinishNotification" 
              @change="updateSetting"
            />
            <label for="show-notification">Show notification when operation completes</label>
          </div>
          
          <!-- Clear Files After Operation -->
          <div class="settings-group checkbox-group">
            <input 
              type="checkbox" 
              id="clear-files" 
              v-model="settings.clearFilesAfterOperation" 
              @change="updateSetting"
            />
            <label for="clear-files">Clear file selection after operation</label>
          </div>
          
          <!-- Dark Mode -->
          <div class="settings-group checkbox-group">
            <input 
              type="checkbox" 
              id="dark-mode" 
              v-model="settings.darkMode" 
              @change="updateSetting"
            />
            <label for="dark-mode">Dark Mode</label>
          </div>
          
          <!-- Message -->
          <div v-if="message" class="settings-message">
            {{ message }}
          </div>
          
          <!-- Buttons -->
          <div class="settings-actions">
            <button 
              @click="saveSettings" 
              class="app-btn app-btn-primary" 
              :disabled="!isDirty || isSaving"
            >
              {{ isSaving ? 'Saving...' : 'Save Settings' }}
            </button>
            <button 
              @click="resetSettings" 
              class="app-btn" 
              :disabled="isSaving"
            >
              Reset to Default
            </button>
          </div>
        </div>
      </template>
    </Card>
    
    <!-- Windows Integration Settings (仅在Windows平台显示) -->
    <Card v-if="isWindows" class="app-section card">
      <template #header>
        <h3>Windows Integration</h3>
      </template>
      <template #body>
        <div class="settings-form">
          <!-- Context Menu Integration -->
          <div class="settings-group checkbox-group">
            <input 
              type="checkbox" 
              id="context-menu" 
              v-model="settings.contextMenuEnabled" 
              @change="updateSetting"
            />
            <label for="context-menu">Add to Windows right-click menu</label>
          </div>
          
          <!-- Context Menu Open Mode -->
          <div class="settings-group indent-group" v-if="settings.contextMenuEnabled">
            <label>When opening from right-click menu:</label>
            <div class="radio-group">
              <div class="radio-option">
                <input 
                  type="radio" 
                  id="right-click-gui" 
                  value="gui" 
                  v-model="settings.rightClickOpenMode"
                  @change="updateSetting"
                />
                <label for="right-click-gui">Open in GUI mode</label>
              </div>
              <div class="radio-option">
                <input 
                  type="radio" 
                  id="right-click-cli" 
                  value="cli" 
                  v-model="settings.rightClickOpenMode"
                  @change="updateSetting"
                />
                <label for="right-click-cli">Use command-line mode (faster)</label>
              </div>
            </div>
          </div>
          
          <!-- File Associations -->
          <div class="settings-group checkbox-group">
            <input 
              type="checkbox" 
              id="file-associations" 
              v-model="settings.fileAssociationsEnabled" 
              @change="updateSetting"
            />
            <label for="file-associations">Associate compressed file formats with ZipHere</label>
          </div>
          
          <!-- Associated Formats -->
          <div class="settings-group indent-group" v-if="settings.fileAssociationsEnabled">
            <label>File formats to associate:</label>
            <select v-model="settings.associatedFormats" @change="updateSetting" class="app-select">
              <option value="all">All supported formats</option>
              <option value="zip">ZIP files only</option>
              <option value="7z">7-Zip files only</option>
              <option value="zip,7z">ZIP and 7-Zip files</option>
              <option value="zip,tar,gz,bz2,xz,zst,7z">All archive formats</option>
            </select>
          </div>
          
          <!-- File Association Open Mode -->
          <div class="settings-group indent-group" v-if="settings.fileAssociationsEnabled">
            <label>When opening associated files:</label>
            <div class="radio-group">
              <div class="radio-option">
                <input 
                  type="radio" 
                  id="file-assoc-gui" 
                  value="gui" 
                  v-model="settings.fileAssociationOpenMode"
                  @change="updateSetting"
                />
                <label for="file-assoc-gui">Open in GUI mode</label>
              </div>
              <div class="radio-option">
                <input 
                  type="radio" 
                  id="file-assoc-viewer" 
                  value="viewer" 
                  v-model="settings.fileAssociationOpenMode"
                  @change="updateSetting"
                />
                <label for="file-assoc-viewer">Open in file viewer mode</label>
              </div>
            </div>
          </div>
          
        </div>
      </template>
    </Card>
  </div>
</template>

<style lang="scss" scoped>
.general-settings {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  padding: 0 1rem;
  overflow-y: auto;
}

.card {
  background-color: #fff;
  width: 100%;
  margin-bottom: 1rem;
  box-shadow: 0 1px 3px rgba(0,0,0,0.12), 0 1px 2px rgba(0,0,0,0.24);
  border-radius: 6px;
}

.settings-form {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  padding: 0.5rem 0;
}

.settings-group {
  display: flex;
  flex-direction: column;
  gap: 0.3rem;
}

.checkbox-group {
  flex-direction: row;
  align-items: center;
  gap: 0.5rem;
}

.indent-group {
  margin-left: 1.5rem;
  margin-top: -0.5rem;
}

.radio-group {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  margin-top: 0.3rem;
}

.radio-option {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.app-select {
  padding: 0.5rem;
  border: 1px solid #ddd;
  border-radius: 4px;
  background-color: white;
  cursor: pointer;
}

.app-range {
  margin: 0.5rem 0;
}

.settings-actions {
  display: flex;
  gap: 1rem;
  margin-top: 1rem;
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

.app-btn-primary {
  background-color: #1976d2;
  color: white;
  border: none;

  &:hover {
    background-color: #1565c0;
  }
}

.settings-message {
  padding: 0.8rem;
  background-color: #e8f5e9;
  color: #2e7d32;
  border: 1px solid #a5d6a7;
  border-radius: 4px;
}
</style>