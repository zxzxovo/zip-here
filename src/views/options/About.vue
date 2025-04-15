<script setup lang="ts">
import { ref, onMounted } from 'vue';
import Card from '../../components/Card.vue';
import { getVersionInfo } from '../../utils/tauri-api';
import type { VersionInfo } from '../../utils/tauri-api';

// 版本信息
const versionInfo = ref<VersionInfo | null>(null);
const loading = ref(false);
const errorMessage = ref('');

// 加载版本信息
const loadVersionInfo = async () => {
  loading.value = true;
  errorMessage.value = '';
  
  try {
    versionInfo.value = await getVersionInfo();
  } catch (error) {
    console.error('Failed to load version information:', error);
    errorMessage.value = `Failed to load version information: ${error}`;
  } finally {
    loading.value = false;
  }
};

// 初始化时加载版本信息
onMounted(() => {
  loadVersionInfo();
});
</script>

<template>
  <div class="about-page">
    <Card class="app-section card">
      <template #header>
        <h3>About ZipHere</h3>
      </template>
      <template #body>
        <div v-if="loading" class="about-loading">
          Loading application information...
        </div>
        
        <div v-else-if="errorMessage" class="about-error">
          {{ errorMessage }}
        </div>
        
        <div v-else-if="versionInfo" class="about-content">
          <div class="app-logo">
            <img src="/tauri.svg" alt="ZipHere Logo" width="100" />
          </div>
          
          <h2 class="app-title">ZipHere</h2>
          
          <div class="app-version">
            Version: {{ versionInfo.version }}
          </div>
          
          <div class="app-build-time">
            Build Time: {{ versionInfo.build_time }}
          </div>
          
          <div class="app-description">
            {{ versionInfo.description }}
          </div>
          
          <div class="app-author">
            Created by: {{ versionInfo.author }}
          </div>
          
          <div class="app-technologies">
            <h4>Built with:</h4>
            <ul>
              <li>Tauri - Native desktop applications with a web frontend</li>
              <li>Vue.js - Progressive JavaScript framework</li>
              <li>Rust - A language empowering everyone to build reliable and efficient software</li>
            </ul>
          </div>
          
          <div class="app-legal">
            <p>© 2025 ZipHere. All rights reserved.</p>
            <p>This software is provided as-is without warranty of any kind.</p>
          </div>
        </div>
      </template>
    </Card>
  </div>
</template>

<style lang="scss" scoped>
.about-page {
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

.about-loading, .about-error {
  padding: 1.5rem;
  text-align: center;
}

.about-error {
  color: #c62828;
}

.about-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 1.5rem;
  text-align: center;
}

.app-logo {
  margin-bottom: 1rem;
}

.app-title {
  font-size: 2rem;
  font-weight: bold;
  margin-bottom: 0.5rem;
  color: #1976d2;
}

.app-version, .app-build-time {
  margin-bottom: 0.5rem;
  font-family: monospace;
}

.app-description {
  margin: 1rem 0;
  max-width: 600px;
  line-height: 1.5;
}

.app-author {
  margin-bottom: 1.5rem;
  font-style: italic;
}

.app-technologies {
  margin: 1rem 0;
  text-align: left;
  width: 100%;
  max-width: 600px;
  
  h4 {
    margin-bottom: 0.5rem;
  }
  
  ul {
    padding-left: 1.5rem;
    
    li {
      margin-bottom: 0.3rem;
    }
  }
}

.app-legal {
  margin-top: 2rem;
  font-size: 0.8rem;
  color: #666;
  border-top: 1px solid #eee;
  padding-top: 1rem;
  width: 100%;
}
</style>