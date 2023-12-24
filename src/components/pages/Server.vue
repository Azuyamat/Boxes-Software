<script setup lang="js">
import {useRoute} from 'vue-router';
import {inject, onMounted, ref, watch} from 'vue';
import {invoke} from "@tauri-apps/api";

const jarImages = {
  "paper": "/paper-logo.png",
  "velocity": "/velocity-logo.png",
  "purpur": "/purpur-logo.svg",
  "unknown": "/pickaxe.png"
}

const route = useRoute();
const serverName = ref(route.params.id);
const serverData = ref({});
const jarImage = ref(jarImages.unknown);

onMounted(async () => {
  const promptUser = inject('promptUser');
  let server = await getServer(serverName.value);
  serverData.value = server;
  jarImage.value = jarImages[server.jar_name.toLowerCase()] || jarImages.unknown
});

watch( () => route.params.id, async (newId) => {
  serverName.value = newId;
  let server = await getServer(newId);
  serverData.value = server;
  jarImage.value = jarImages[server.jar_name.toLowerCase()] || jarImages.unknown;
});

async function getServer(name) {
  return await invoke("get_server", {name: name});
}
</script>

<template>
  <div class="container">
    <header :style="`--image: url(${jarImage})`">
      <h2>{{ serverName }}</h2>
      <ul>
        <li>
          {{ serverData.jar_name }} {{ serverData.version }} ({{ serverData.build }})
        </li>
        <li>
          Last refreshed <span id="refresh">3 minutes ago</span>
        </li>
      </ul>
    </header>
    <div class="controls">
      <button>▶️ Start</button>
      <button>⏹️ Stop</button>
      <button>🤚 Delete</button>
      <button @click="changeName">✏️ Rename</button>
    </div>
<!--    Players-->
<!--    Console-->
<!--    Settings-->
  </div>
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