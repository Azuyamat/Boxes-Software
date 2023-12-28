<script setup lang="ts">

import {ref} from "vue";

const show = ref(false)
const header = ref('')
const message = ref('')

const toast = (title, msg) => {
  header.value = title
  message.value = msg
  show.value = true
  setTimeout(() => {
    show.value = false
    header.value = ''
    message.value = ''
  }, 3000)
}

defineExpose({ toast })
</script>

<template>
  <div v-if="show" class="toast">
    <h1>{{ header }}</h1>
    <p>{{ message }}</p>
  </div>
</template>

<style scoped>
.toast {
  position: fixed;
  bottom: 0;
  right: 0;
  background: rgba(23, 23, 23, 0.51);
  color: #ffffff;
  padding: .5rem 1rem;
  margin: 1rem;
  border-radius: 0.5rem;
  backdrop-filter: blur(10px);
  box-shadow: 0 1px 30px rgba(0, 0, 0, 0.5);
  border: 1px solid rgba(255, 255, 255, 0.1);
  display: flex;
  flex-direction: row;
  gap: 10px;
  max-width: 50%;
}
.toast h1 {
  margin: .5rem 0;
  font-size: 1rem;
}
.toast p {
  margin: .5rem 0;
  font-size: 1rem;
  word-break: break-all;
}
</style>