<template>
  <div class="page">
    <div class="panel" v-for="task in tasks">
        <h1>{{ task.title }}</h1>
        <p v-for="p in task.body">{{ p }}</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import {ref} from 'vue'
definePageMeta({
  layout: 'tray',
})
interface Task {
    title: string
    body: string[]
}
const tasks = ref(<Task[]>[])

for (let step = 0; step < 15; step++) {
    tasks.value.push({
        title: 'Do something',
        body: ['Paragraph 1.', 'Paragraph 2.']
    })
}
</script>

