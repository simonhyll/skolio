<template>
  <div class="page">
    <div v-for="subject in subjects" class="panel">
        <h1>{{ subject.title }}</h1>
    </div>
  </div>
</template>

<script setup lang="ts">
definePageMeta({
  layout: 'tray',
});
const subjects = [
  {
    title: 'Svenska'
  },
  {
    title: 'Engelska'
  },
  {
    title: 'Matematik'
  },
  {
    title: 'Programmering'
  },
  {
    title: 'Fysik'
  },
  {
    title: 'Kemi'
  },
  {
    title: 'Biologi'
  },
]
</script>
