<script setup lang="ts">


</script>

<template>
    <main class="op-container">
        <div class="options-bar">
            <router-link class="op-link" :to="{ name: 'OptionsGeneral' }">
                <div class="op-link-block">
                    General
                </div>
            </router-link>
            <router-link class="op-link" :to="{ name: 'OptionsFormats' }">
                <div class="op-link-block">
                    Formats
                </div>
            </router-link>
            <router-link class="op-link" :to="{ name: 'OptionsAbout' }">
                <div class="op-link-block">
                    About
                </div>
            </router-link>
        </div>

        <section class="options-content">
            <router-view></router-view>
        </section>
    </main>
</template>

<style scoped>
.op-container {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
}

.options-bar {
    width: 100%;
    flex: 0 0 3rem;
    display: flex;
    flex-direction: row;
    flex-wrap: wrap;
    justify-content: center;
    align-items: center;
    background-color: #fff;
    font-size: 1rem;
    box-shadow: 0 1px 2px rgba(0, 0, 0, 0.1);
}

.options-content {
    flex: 1 0 auto;
    display: flex;
    justify-content: center;
    align-items: center;
}

.op-link {
    flex: 1 0 auto;
    height: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
    text-decoration: none;
    color: #666;
    font-weight: bold;
    font-size: 0.8rem;
}

.op-link>.op-link-block {
    padding: 2px;
}

.options-bar>.router-link-active>.op-link-block {
    background: #42A5F5;
    padding: 2px;
    transition-duration: .2s;
    border-radius: 2rem;
    color: #fcfcfc;
}
</style>
