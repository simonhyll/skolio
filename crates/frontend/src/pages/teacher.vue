<template>
  <div class="page">
    <div class="panel settings-box">
      <select name="gptOptions">
        <option v-for="option in gptOptions" :value="option">{{ option }}</option>
      </select>
    </div>
    <div ref="messagesFrame" class="messages">
      <div v-for="message in messages" class="panel" style="padding:8px">
          <span class="message-text">{{ message.text }}</span>
          <span class="message-date">{{ message.date.toLocaleDateString() + ' ' + message.date.toLocaleTimeString() }}</span>
      </div>
    </div>
    <div class="spacer"></div>
    <div class="panel message-box">
        <textarea v-model="message" @keydown.enter.prevent="sendMessage"></textarea>
    </div>
  </div>
</template>

<script setup lang="ts">
import {ref, watch,nextTick} from 'vue'
definePageMeta({
  layout: 'tray',
});
interface ChatMessage {
  text: string
  date: Date
}
const messages = ref(<ChatMessage[]>[])
const messagesFrame = ref(null);
const message = ref('')
const gptOptions = ['davinci','chat']
const sendMessage = async function () {
  messages.value.push({
    text: message.value,
    date: new Date()
  })
  message.value = ''
}
for (let step = 0; step < 15; step++) {
    message.value = 'x'
    sendMessage()
}
watch(messages, ()=>{
  nextTick(()=>{
    if(messagesFrame.value) {
        messagesFrame.value.scrollTop = messagesFrame.value.scrollHeight;
    }

  })
}, {deep:true})
</script>

<style scoped>
.settings-box {
  min-height: 1cm;
  display: flex;
}
.settings-box select {
  min-height: 1cm;
  flex-grow: 1;
  border-radius: 6px;
  outline: none;
}
.message-box {
  padding:6px;
  display: flex;
  flex-direction: row;
}
.message-box textarea {
  padding:6px;
  border:none;
  flex-grow: 1;
  resize: vertical;
}
.messages {
  flex-grow: 1;
  overflow: auto;
}
.message-text {
  display: block;
}
.message-date {
  display: block;
  font-size: small;
}
</style>
