<script setup lang="ts">
import Sidebar from "./components/Sidebar.vue";
import Prompt from "./components/Prompt.vue";
</script>

<template>
  <Sidebar />
  <router-view />
  <Prompt />
</template>