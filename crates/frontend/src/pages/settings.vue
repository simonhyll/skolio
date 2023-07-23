<template>
  <div class="page">
    <div class="panel">
        <h1>Quizzy</h1>
        <p>Gives you popups with questions every now and again.</p>
        <span>Counter</span>
        <input v-model="quizzySettings.timer.counter" type="number">
        <span>Initial</span>
        <input v-model="quizzySettings.timer.initial" type="number">
        <span>Decrement</span>
        <input v-model="quizzySettings.timer.decrement" type="number">
        <span>Increment</span>
        <input v-model="quizzySettings.timer.increment" type="number">
    </div>
    <div class="panel">
        <h1>Flashy</h1>
        <p>Sends you a notification every now and again with some information.</p>
        <input>
    </div>
    <div class="panel">
        <h1>Tasky</h1>
        <p>Pesters you to accomplish tasks.</p>
        <input>
    </div>
  </div>
</template>

<script setup lang="ts">
import {ref,watch, onMounted} from 'vue'
import { invoke } from "@tauri-apps/api/tauri";
import { appWindow } from "@tauri-apps/plugin-window";
definePageMeta({
  layout: 'tray',
});
interface QuizzyTimerSettings {
  counter: number
  initial: number
  decrement: number
  increment: number
}
interface QuizzySettings {
  timer: QuizzyTimerSettings
}
const quizzySettings = ref(<QuizzySettings>{
  timer: {
    counter: 0,
    initial: 0,
    decrement: 0,
    increment: 0
  }
})
onMounted(async ()=>{
  quizzySettings.value = await invoke('get_quizzy_state')
})
watch(quizzySettings, ()=>{
  appWindow.emit('settings', quizzySettings.value)
}, {deep:true})
</script>

<style scoped>
span {
  display: block;
}
</style>
