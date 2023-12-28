<script setup lang="js">
import {useRoute, useRouter} from 'vue-router';
import {inject, ref, watch} from 'vue';
import {invoke} from "@tauri-apps/api";
import Layout from "../Layout.vue";

const jarImages = {
  "paper": "/paper-logo.png",
  "velocity": "/velocity-logo.png",
  "purpur": "/purpur-logo.svg",
  "unknown": "/pickaxe.png"
}

const router = useRouter();
const route = useRoute();
const serverName = ref(route.params.id);
const serverData = ref({});
const serverRunning = ref(false);
const jarImage = ref(jarImages.unknown);
const sidebar = inject("sidebar");
const prompt = inject("prompt");
const toast = inject("toast");

watch( () => route.params.id, async (newId) => {
  serverName.value = newId;
  let server = await getServer(newId);
  serverData.value = server;
  jarImage.value = jarImages[server.jar.name.toLowerCase()] || jarImages.unknown
  serverRunning.value = await isServerRunning();
}, {immediate: true});

async function getServer(name) {
  return await invoke("get_server", {name: name});
}

async function startServer() {
  toast("▶️", "Starting server");
  invoke("start_server", {name: serverName.value});
  toast("✅", "Server started");
  serverRunning.value = true;
}

async function stopServer() {
  toast("⏹️", "Stopping server");
  invoke("stop_server", {name: serverName.value});
  toast("✅", "Server stopped");
  serverRunning.value = false;
}

async function deleteServer() {
  let confirm = await prompt("??",
      "Are you sure you want to delete this server? This action cannot be undone. Type the server name to confirm: "+serverName.value);
  if (confirm !== serverName.value) return toast("❌", "Server name does not match");
  toast("🗑️", "Deleting server");
  await invoke("delete_server", {name: serverName.value});
  toast("✅", "Server deleted");
  router.push("/servers");
  sidebar.reloadSidebar();

}

async function isServerRunning() {
  return await invoke("is_server_running", {name: serverName.value});
}

async function promptName() {
  let newName = await prompt("Rename server", "Enter a new name for the server");
  if (!newName) return;
  toast("✏️", "Renaming server to " + newName);
  await invoke("rename_server", {oldName: serverName.value, newName: newName});
  serverName.value = newName;
  router.push(`/server/${newName}`)
  sidebar.reloadSidebar();
}
</script>

<template>
  <Layout>
    <div class="container">
      <header :style="`--image: url(${jarImage})`">
        <h2>{{ serverName }}</h2>
        <ul>
          <li>
            {{ serverData.jar?.name }} {{ serverData.jar?.version }} ({{ serverData.jar?.build }})
          </li>
        </ul>
      </header>
      <div class="controls">
        <button @click="startServer" v-if="!serverRunning">▶️ Start</button>
        <button @click="stopServer" v-else>⏹️ Stop</button>
        <button @click="deleteServer">🤚 Delete</button>
        <button @click="promptName">✏️ Rename</button>
      </div>
      <!--    Players-->
      <!--    Console-->
      <!--    Settings-->
    </div>
  </Layout>
</template>

<style scoped>
.container header {
  --image: url('/public/pickaxe.png');
  position: relative;
  display: flex;
  flex-direction: column;
  align-items: center;

  padding: 1rem 2rem;
  background: rgba(21, 21, 21, 0.41);
  border: 1px solid rgba(255, 255, 255, 0.1);
  width: 100%;
  border-radius: 5px;
  box-shadow: 0 1px 30px rgba(0, 0, 0, 0.07);
}
.container header::before {
  content: "";
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-image: var(--image);
  background-position: left center;
  background-repeat: no-repeat;
  background-size: contain;
  opacity: 0.5;
  z-index: -1;
}
.container header h2 {
  padding: 0;
  margin: 1rem 0 0 0;
}
.container header ul {
  list-style: none;
  padding: 0;
  margin: 1rem 0 0 0;
  display: flex;
  flex-direction: column;
  align-items: center;
}
#refresh {
  font-size: 0.8rem;
  color: rgb(174, 174, 174);
}
#refresh::before {
  content: "⏱";
  margin-right: 0.5rem;
}

.controls {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 1rem;
  margin: 1rem 0;
}
.container button {
  padding: 0.5rem 1rem;
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 5px;
  background: rgba(21, 21, 21, 0.41);
  color: #fff;
  font-size: 1rem;
  cursor: pointer;
  user-select: none;
}
.container button:hover {
  background: rgba(18, 18, 18, 0.61);
}
</style>