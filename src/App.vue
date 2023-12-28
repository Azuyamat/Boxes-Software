<script setup lang="ts">
import Sidebar from "./components/Sidebar.vue";
import Prompt from "./components/Prompt.vue";
import Toast from "./components/Toast.vue";
import {onMounted, provide, ref} from "vue";

const sidebar = ref(null);
const prompt = ref(null);
const toast = ref(null);

onMounted(() => {
  provide("sidebar", sidebar.value);
  provide("prompt", prompt.value.prompt);
  provide("toast", toast.value.toast);
});

</script>

<template>
  <Sidebar ref="sidebar"/>
  <router-view />
  <Prompt ref="prompt" />
  <Toast ref="toast" />
</template>