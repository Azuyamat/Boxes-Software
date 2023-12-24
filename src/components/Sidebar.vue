<script setup lang="js">
import {invoke} from '@tauri-apps/api';
import {onMounted, ref} from "vue";
import {useRouter} from "vue-router";

const router = useRouter();
const sidebarVisible = ref(false);

const getServers = async () => {
  return await invoke("get_servers");
};

const servers = ref([]);

onMounted(async () => {
  servers.value = await getServers();
});

const toggleSidebar = () => {
  sidebarVisible.value = !sidebarVisible.value;
};

const goTo = (path) => {
  router.push(path);
};
</script>

<template>
  <aside :class="{ 'sidebar': true, 'shown': sidebarVisible }">
    <header>
      <span role="img" title="Boxes" @click="toggleSidebar">📦</span>
    </header>
    <ul>
      <h2>Menu</h2>
      <li aria-label="home" @click="goTo(`/`)">
        <span role="img" aria-label="rocket">🚀</span>
        <router-link to="/">Home</router-link>
      </li>
      <li aria-label="settings" @click="goTo(`/settings`)">
        <span role="img" aria-label="settings">⚙️</span>
        <router-link to="/settings">Settings</router-link>
      </li>
      <li aria-label="about" @click="goTo(`/about`)">
        <span role="img" aria-label="about">📖</span>
        <router-link to="/about">About</router-link>
      </li>
      <li class="separator" />
      <h2>Servers</h2>
      <li v-for="server in servers" :key="2" aria-label="server" @click="goTo(`/server/${server.server_name}`)">
        <span role="img" aria-label="server">🖥️</span>
        <router-link :to="`/server/${server.server_name}`">{{ server.server_name }}</router-link>
      </li>
      <li class="separator" />
    </ul>
    <footer>
      <h2>🤓</h2>
      <p>
        You're objectively a nerd
      </p>
    </footer>
  </aside>
</template>

<style scoped>
.sidebar {
  position: fixed;
  top: 0;
  left: 0;

  width: 18rem;
  height: 100vh;
  padding: 1rem;
  margin: 0;
  background-color: rgba(23, 23, 23, 0.42);
  box-shadow: 0 1px 30px rgba(0, 0, 0, 0.5);
  visibility: hidden;
  border-right: 1px solid rgba(255, 255, 255, 0.1);
  overflow-y: auto;
  z-index: 1000;
}

.sidebar::-webkit-scrollbar {
  width: 0.5rem;
}

.sidebar.shown {
  visibility: visible;
  animation: appear 0.5s ease-in-out;
  backdrop-filter: blur(10px);
}

.sidebar header {
  position: sticky;
  top: 0;
  filter: drop-shadow(0 1px 10px rgba(0, 0, 0, 0.5));
  visibility: visible;
  display: flex;
  align-items: center;
  justify-content: left;
  margin: 0;
  gap: 15%;
  z-index: 1000;
}

.sidebar header span {
  --size: 50px;
  font-size: 2rem;
  background: rgb(21, 21, 21);
  height: var(--size);
  width: var(--size);
  border-radius: 50%;
  text-align: center;
  line-height: var(--size);
  border: 1px solid rgba(255, 255, 255, 0.1);
  user-select: none;
  position: relative;
  backdrop-filter: blur(10px);
}

.sidebar header span:hover {
  cursor: pointer;
  background: rgb(23, 23, 23);
  border: 1px solid rgba(255, 255, 255, 0.1);
}

.sidebar > ul {
  list-style: none;
  padding: 0;
  margin: 1rem 0;
}

.sidebar > ul > li > h3 {
  margin: 0;
  padding: 0;
  font-size: inherit;
  font-weight: 400;
  color: #fff;
}

.sidebar > ul > li {
  display: flex;
  align-items: center;
  justify-content: left;
  gap: 1rem;
  margin: 1rem 0;
  font-size: 1.2rem;
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 5px;
  padding: 0.5rem;
  width: 100%;
  cursor: pointer;
  user-select: none;
}

.sidebar > ul > li:hover {
  background: rgba(26, 26, 26, 0.96);
  backdrop-filter: blur(10px);
}

.sidebar ul li[aria-label="server"]:hover {
  background: radial-gradient(circle at center, rgb(115, 191, 253), transparent);
}
.sidebar ul li[aria-label="home"]:hover {
  background: radial-gradient(circle at center, rgba(81, 81, 81, 0.4), transparent);
}
.sidebar ul li[aria-label="settings"]:hover {
  background: radial-gradient(circle at center, rgba(81, 81, 81, 0.4), transparent);
}
.sidebar ul li[aria-label="about"]:hover {
  background: radial-gradient(circle at center, rgba(81, 81, 81, 0.4), transparent);
}

.separator {
  height: 1px;
  padding: 0 !important;
}
</style>