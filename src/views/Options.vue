<script setup lang="ts">
import { computed, onMounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';

// 获取当前路由和路由器
const route = useRoute();
const router = useRouter();

// 计算当前激活的选项卡
const activeTab = computed(() => {
  return route.name || 'OptionsGeneral';
});

// 组件挂载时，如果没有子路由，则默认重定向到General页面
onMounted(() => {
  if (route.name === 'Options') {
    router.replace({ name: 'OptionsGeneral' });
  }
});
</script>

<template>
    <main class="op-container">
        <div class="options-bar">
            <router-link class="op-link" :to="{ name: 'OptionsGeneral' }">
                <div class="op-link-block">
                    <i class="settings-icon">⚙️</i>
                    <span>General</span>
                </div>
            </router-link>
            <router-link class="op-link" :to="{ name: 'OptionsFormats' }">
                <div class="op-link-block">
                    <i class="formats-icon">📦</i>
                    <span>Formats</span>
                </div>
            </router-link>
            <router-link class="op-link" :to="{ name: 'OptionsAbout' }">
                <div class="op-link-block">
                    <i class="about-icon">ℹ️</i>
                    <span>About</span>
                </div>
            </router-link>
        </div>

        <section class="options-content">
            <router-view></router-view>
        </section>
    </main>
</template>

<style lang="scss" scoped>
.op-container {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
    background-color: #f5f5f5;
    overflow-y: auto;
}

.options-bar {
    width: 100%;
    flex: 0 0 3.5rem;
    display: flex;
    flex-direction: row;
    flex-wrap: wrap;
    justify-content: center;
    align-items: center;
    background-color: #fff;
    font-size: 1rem;
    box-shadow: 0 1px 4px rgba(0, 0, 0, 0.1);
    margin-bottom: 1px;
}

.options-content {
    flex: 1 0 auto;
    display: flex;
    flex-direction: column;
    padding: 1rem;
    overflow-y: auto;
}

.op-link {
    flex: 1 0 auto;
    height: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
    text-decoration: none;
    color: #666;
    font-weight: 500;
    font-size: 0.9rem;
    transition: all 0.2s ease;
    
    &:hover {
        background-color: #f5f5f5;
    }
}

.op-link-block {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 1rem;
    border-radius: 2rem;
    transition: all 0.2s ease;
    
    i {
        font-size: 1.2rem;
        font-style: normal;
    }
}

.options-bar>.router-link-active {
    color: #1976d2;
    
    .op-link-block {
        background: #e3f2fd;
        color: #0d47a1;
        font-weight: 600;
    }
}
</style>
