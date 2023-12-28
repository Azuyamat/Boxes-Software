<script setup lang="ts">

import {inject, onMounted, ref} from "vue";
import {invoke} from "@tauri-apps/api";
import Layout from "../Layout.vue";

const sidebar = inject("sidebar");
const goTo = sidebar.goTo;
const servers = ref([]);
const emojis = ["🐮", "🐨", "🐭", "🐰", "🐻"];
const randomEmoji = () => emojis[Math.floor(Math.random() * emojis.length)];

onMounted(async () => {
  servers.value = await invoke("get_servers") || [];
});

</script>

<template>
  <Layout>
    <div class="servers">
      <h1>Servers</h1>
      <ul>
        <li v-for="server in servers" :key="server.server_name" aria-label="server" @click="goTo(`/server/${server.server_name}`)">
          <span role="img" aria-label="server">{{ randomEmoji() }}</span>
          <h3>{{ server.server_name }}</h3>
        </li>
        <li @click="goTo(`/new-server`)">
          <span role="img" aria-label="add">📦</span>
          <h3>New Server</h3>
        </li>
      </ul>
    </div>
  </Layout>
</template>

<style scoped>
.servers {
  height: 100%;
  overflow-y: auto;
}
.servers ul {
  list-style: none;
  padding: 0;
  margin: 0;
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
  gap: 10px;
}
.servers li {
  display: flex;
  align-items: center;
  cursor: pointer;
  padding: .5rem 2rem;
  background: rgba(21, 21, 21, 0.41);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 5px;
  justify-content: center;
  gap: 10px;
}
.servers li span {
  background: rgba(105, 105, 105, 0.1);
  padding: 0.4rem;
  border-radius: 50%;
  border: 1px solid rgba(255, 255, 255, 0.1);
}
</style>