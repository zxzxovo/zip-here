<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { getSupportedFormats, getFormatOptions } from '../../utils/tauri-api';
import type { FormatOption } from '../../utils/tauri-api';
import Card from '../../components/Card.vue';

// 格式列表
const formatOptions = ref<FormatOption[]>([]);
const loading = ref(false);
const errorMessage = ref('');

// 加载格式信息
const loadFormatsInfo = async () => {
  loading.value = true;
  errorMessage.value = '';
  
  try {
    // 获取支持的格式列表
    const formats = await getSupportedFormats();
    
    // 加载每个格式的详细信息
    const options = await Promise.all(
      formats.map(async format => {
        try {
          return await getFormatOptions(format);
        } catch (error) {
          console.error(`Failed to load format options: ${format}`, error);
          return null;
        }
      })
    );
    
    formatOptions.value = options.filter((opt): opt is FormatOption => opt !== null);
  } catch (error) {
    console.error('Failed to load format information:', error);
    errorMessage.value = `Failed to load format information: ${error}`;
  } finally {
    loading.value = false;
  }
};

// 初始化时加载格式信息
onMounted(() => {
  loadFormatsInfo();
});

// 获取格式的能力说明
const getFormatCapabilities = (format: FormatOption): string[] => {
  const capabilities: string[] = [];
  
  if (format.can_compress) capabilities.push('Compression');
  if (format.can_decompress) capabilities.push('Decompression');
  if (format.supports_password) capabilities.push('Password Protection');
  if (format.supports_level) capabilities.push('Compression Levels');
  
  return capabilities;
};
</script>

<template>
  <div class="formats-page">
    <Card class="app-section card">
      <template #header>
        <h3>Supported Formats</h3>
      </template>
      <template #body>
        <div v-if="loading" class="formats-loading">
          Loading format information...
        </div>
        
        <div v-else-if="errorMessage" class="formats-error">
          {{ errorMessage }}
        </div>
        
        <div v-else class="formats-container">
          <p class="formats-intro">
            ZipHere supports the following compression and decompression formats:
          </p>
          
          <div class="formats-grid">
            <div v-for="format in formatOptions" :key="format.id" class="format-card">
              <div class="format-header">
                <span class="format-name">{{ format.name }}</span>
                <span class="format-extension">.{{ format.extension }}</span>
              </div>
              
              <div class="format-details">
                <div class="format-capabilities">
                  <div v-for="capability in getFormatCapabilities(format)" :key="capability" class="format-capability">
                    {{ capability }}
                  </div>
                </div>
                
                <div v-if="format.supports_level" class="format-level-info">
                  <span>Compression Levels: {{ format.min_level }} - {{ format.max_level }}</span>
                  <span>(Default: {{ format.default_level }})</span>
                </div>
              </div>
            </div>
          </div>
          
          <div class="formats-note">
            <p>Note: Support for some formats may depend on system libraries and configurations.</p>
          </div>
        </div>
      </template>
    </Card>
  </div>
</template>

<style lang="scss" scoped>
.formats-page {
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

.formats-loading, .formats-error {
  padding: 1.5rem;
  text-align: center;
}

.formats-error {
  color: #c62828;
}

.formats-container {
  padding: 0.5rem 0;
}

.formats-intro {
  margin-bottom: 1rem;
}

.formats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
  gap: 1rem;
  margin-bottom: 1rem;
}

.format-card {
  border: 1px solid #ddd;
  border-radius: 4px;
  padding: 1rem;
  background-color: #f9f9f9;
}

.format-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 0.5rem;
  padding-bottom: 0.5rem;
  border-bottom: 1px solid #eee;
}

.format-name {
  font-weight: bold;
  font-size: 1.1rem;
}

.format-extension {
  color: #666;
  font-family: monospace;
  background: #f0f0f0;
  padding: 0.1rem 0.3rem;
  border-radius: 3px;
}

.format-capabilities {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
  margin-bottom: 0.5rem;
}

.format-capability {
  background-color: #e3f2fd;
  color: #0d47a1;
  padding: 0.2rem 0.5rem;
  border-radius: 4px;
  font-size: 0.8rem;
}

.format-level-info {
  margin-top: 0.5rem;
  font-size: 0.9rem;
  color: #555;
  display: flex;
  flex-direction: column;
}

.formats-note {
  margin-top: 1rem;
  padding-top: 0.5rem;
  border-top: 1px solid #eee;
  font-size: 0.9rem;
  color: #666;
  font-style: italic;
}
</style>