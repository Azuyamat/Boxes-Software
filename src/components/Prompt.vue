<script setup>
import {nextTick, ref} from 'vue'

const show = ref(false)
const header = ref('')
const message = ref('')
const input = ref('')
const inputElement = ref(null)
let resolve;

function prompt(title, msg) {
  header.value = title
  message.value = msg
  show.value = true
  input.value = ''
  nextTick(() => {
    inputElement.value.focus()
  })
  return new Promise((res) => {
    resolve = res
  });
}

const submit = () => {
  show.value = false
  resolve(input.value)
}

const close = () => {
  show.value = false
  resolve(null)
}

defineExpose({ prompt })
</script>

<template>
  <div v-if="show" class="prompt-box">
    <div>
      <h1>{{ header }}</h1>
      <p>{{ message }}</p>
      <input type="text" v-model="input" @keyup.enter="submit" ref="inputElement">
      <ul class="buttons">
        <li><button @click="submit">Submit</button></li>
        <li><button @click="close">Cancel</button></li>
      </ul>
    </div>
  </div>
</template>

<style scoped>
.prompt-box {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: rgba(0, 0, 0, 0.46);
  z-index: 100;
  backdrop-filter: blur(10px);
}
.prompt-box div {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  background: rgba(21, 21, 21, 0.41);
  border: 1px solid rgba(255, 255, 255, 0.1);
  width: 80%;
  border-radius: 5px;
  box-shadow: 0 1px 30px rgba(0, 0, 0, 0.07);
  padding: 1rem 2rem;
}
.prompt-box div h1 {
  margin: 0;
  font-size: 1.5rem;
}
.prompt-box div p {
  margin: .5rem 0;
  font-size: 1rem;
}
.prompt-box div input {
  width: 100%;
  padding: .5rem;
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 5px;
  background: rgba(255, 255, 255, 0.1);
  color: white;
  font-size: 1rem;
}
.prompt-box div input:focus {
  outline: none;
}
.prompt-box div ul {
  list-style: none;
  padding: 0;
  margin: 0;
  display: flex;
  justify-content: flex-end;
  gap: 10px;
}
.prompt-box div button {
  cursor: pointer;
  margin-top: .5rem;
  padding: .5rem;
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 5px;
  background: rgba(255, 255, 255, 0.1);
  color: white;
}
</style>