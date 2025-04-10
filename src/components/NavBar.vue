<script setup lang="ts">
import { Window } from '@tauri-apps/api/window';

const appWindows = new Window("main");

const minimize = () => {
    appWindows.minimize();
}
const toggleMaximize = () => {
    appWindows.toggleMaximize();
}
const close = () => {
    appWindows.close();
}


</script>

<template>
    <div class="nav-container">

        <div data-tauri-drag-region="true" class="app-bar">

            <div data-tauri-drag-region="true" class="app-title">
                Zip Here
            </div>

            <div class="bar-group">
                <div class="titlebar-button" @click="minimize">
                    <img src="../assets/minimize.svg" alt="minimize" />
                </div>
                <div class="titlebar-button" @click="toggleMaximize">
                    <img src="../assets/maximize.svg" alt="maximize" />
                </div>
                <div class="titlebar-button" @click="close">
                    <img src="../assets/close.svg" alt="close" />
                </div>
            </div>

        </div>

        <div data-tauri-drag-region="true" class="nav-bar">
            <router-link class="nav-link" :to="{ name: 'Home' }">
                <div class="nav-link-block">
                    Home
                </div>
            </router-link>
            <router-link class="nav-link" :to="{ name: 'Compress' }">
                <div class="nav-link-block">
                    Compress
                </div>
            </router-link>
            <router-link class="nav-link" :to="{ name: 'Decompress' }">
                <div class="nav-link-block">
                    Decompress
                </div>
            </router-link>
            <router-link class="nav-link" :to="{ name: 'Options' }">
                <div class="nav-link-block">
                    Options
                </div>
            </router-link>
        </div>

    </div>
</template>

<style scoped lang="scss">
.nav-container {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
}

.app-bar {
    width: 100%;
    flex: 0 0 42px;
    display: flex;
    flex-direction: row;
    flex-flow: nowrap;
    justify-content: flex-end;
    align-items: end;
}

.app-bar>.app-title {
    flex: 1 0 auto;
    height: 100%;
    display: flex;
    text-align: left;
    align-items: center;
    font-size: 1rem;
    font-weight: bold;
    color: #37474F;
    margin-left: 1rem;
}

.app-bar>.bar-group {
    display: flex;
    justify-content: center;
    align-items: center;
}

.titlebar-button {
    display: inline-flex;
    justify-content: center;
    align-items: center;
    width: 42px;
    height: 42px;
    border-radius: 2px;
    transition-duration: .2s;
}

.titlebar-button:hover {
    background: #CFD8DC;
}

.titlebar-button:last-child:hover {
    background: #EF5350;
}

.nav-bar {
    width: 100%;
    flex: 1 1 auto;
    display: flex;
    flex-direction: row;
    justify-content: center;
    font-weight: bold;
}

.nav-bar>.nav-link {
    width: 7rem;
    height: 100%;
    margin: 0 1rem;
    align-content: center;
    text-align: center;
    color: #666;
}

.nav-link>.nav-link-block {
    padding: 4px;
}

.nav-bar>.router-link-active>.nav-link-block {
    background: #26A69A;
    transition-duration: .2s;
    border-radius: 2rem;
    color: #fcfcfc;
}

</style>