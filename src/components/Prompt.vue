<script setup lang="ts">
import {ref, provide} from "vue";

const prompt = ref(null);

const promptUser = (message) => {
  prompt.value = message
}

provide('promptUser', promptUser)
</script>

<template>
  <div class="prompt-ctn" v-if="prompt">
    <div class="box">
      <p>{{ prompt }}</p>
      <input type="text">
    </div>
  </div>
</template>

<style scoped>
.prompt-ctn {
  position: fixed;
  top: 0;
  left: 0;
  height: 100vh;
  width: 100vw;
  background: rgba(49, 49, 49, 0.47);
  backdrop-filter: blur(10px);

  display: flex;
  align-content: center;
  justify-content: center;
}
.box {
  height: 50%;
  width: 80%;
  margin: auto;
  padding: 1rem;
  background: #151515;
  border-radius: 5px;
  border: 1px solid rgba(255, 255, 255, 0.2);

  display: flex;
  flex-direction: column;
  justify-content: flex-start;
}
.box input[type="text"] {
  outline: none;
  border: 1px solid rgba(255, 255, 255, 0.2);
  background: #151515;
  width: 100%;
  padding: 1rem;
  border-radius: 5px;
  color: white;
}
</style>