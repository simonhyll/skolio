<template>
    <div class="quiz">
        <h4>{{ quiz.title }}</h4>
        <h5>{{ timeout }} sec</h5>
        <h1>{{ quiz.question }}</h1>
        <input ref="input" v-model="answer" :disabled="disabled" autofocus>
        <h1 v-if="correct">CORRECT</h1>
        <h1 v-else-if="answer || disabled">WRONG</h1>
    </div>
</template>

<script setup lang="ts">
import {ref,watch,nextTick} from 'vue'
import { appWindow } from "@tauri-apps/plugin-window";
definePageMeta({
  layout: 'quizzy',
});
const quiz = {
    title: "Quick!",
    question: '2 + 2',
    answer: '4'
}
const input = ref(<HTMLElement|null>null)
const answer = ref('')
const correct = ref(false)
const disabled = ref(false)
const timeout = ref(10)
const countdown = setInterval(()=>{
    timeout.value -= 1
    if(timeout.value <= 0) {
        disabled.value = true
        correct.value = false
        clearInterval(countdown)
        setTimeout(async ()=>{
            await appWindow.emit('fail')
            await appWindow.close()
        }, 1000)
    }
}, 1000)
const submit = async function(){
    correct.value = answer.value === quiz.answer
    if(correct.value) {
        disabled.value = true
        clearInterval(countdown)
        setTimeout(async ()=>{
            correct.value = false
            answer.value = ''
            disabled.value = false
            await appWindow.close()
        }, 1000)
    }
}
watch(answer, ()=>{
    submit()
})
nextTick(async ()=>{
    if(input.value)
        input.value.focus()
})
</script>

<style scoped>
.quiz {
    min-width: 5cm;
    text-align: center;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
}
h1, h4, h5 {
    text-align: center;
    margin:8px 0 0 0;
}
input {
    font-size: large;
    text-align: center;
    width: auto;
    min-width: 1cm;
    max-width: 100%;
    width:3cm;
    box-sizing: content-box;
    height: 1cm;
    margin:8px;
    border-radius: 4px;
}
</style>
