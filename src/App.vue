<script setup lang="ts">
import NavBar from './components/NavBar.vue';

</script>

<template>
  <div class="app-container">

    <nav class="app_nav">
      <NavBar />
    </nav>

    <main class="content-container">
        <router-view></router-view>

    </main>

  </div>
</template>

<style scoped>
.app-container {
  width: 100%;
  height: 100vh;
  display: flex;
  flex-direction: column;
  background-color: #f0f0f0;
  overflow: hidden;
}

.app_nav {
  width: 100%;
  flex: 0 0 auto;
  height: 100px;
  display: flex;
  flex-direction: column;
  background-color: #fff;
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.1);
  position: sticky;
  position: -webkit-sticky;
  top: 0;
}

.content-container {
  width: 100%;
  flex: 1 1 auto;
  justify-content: center;
  overflow: hidden;
}
</style>

<style>
* {
  box-sizing: border-box;
  margin: 0;
  padding: 0;
}

:root {
  user-select: none;
  -moz-user-select: none;
  -webkit-user-drag: none;
  -webkit-user-select: none;
}

a {
  text-decoration: none;
}
</style>