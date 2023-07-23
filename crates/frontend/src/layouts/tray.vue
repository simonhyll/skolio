<template>
    <div class="background">
        <div class="menuArea">
          <TrayMenu></TrayMenu>
        </div>
        <div class="pageArea">
          <NuxtPage></NuxtPage>
        </div>
    </div>
</template>
<script setup lang="ts">
import {nextTick} from 'vue'
import { appWindow } from "@tauri-apps/plugin-window";
nextTick(()=>{
  appWindow.emit('ready')
})
</script>
<style scoped>
.background {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: row;
    flex-grow: 1;
}
.menuArea {
    display: flex;
    flex-direction: column;
    width: 55px;
    min-width: 55px;
    background: #eee;
    overflow: hidden;
}
.pageArea {
    display: flex;
    flex-direction: column;
    flex-grow: 1;
    overflow: hidden;
}
</style>
