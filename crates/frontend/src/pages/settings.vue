<template>
  <div class="page">
    <div class="panel">
        <h1>Quizzy</h1>
        <p>Gives you popups with questions every now and again.</p>
        <input>
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
definePageMeta({
  layout: 'tray',
});
</script>
