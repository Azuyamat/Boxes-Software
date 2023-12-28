import { createRouter, createWebHistory } from 'vue-router'
import Home from './components/pages/Home.vue'
import Server from './components/pages/Server.vue'
import Settings from './components/pages/Settings.vue'
import About from './components/pages/About.vue'
import Servers from './components/pages/Servers.vue'
import NewServer from "./components/pages/NewServer.vue";

const routes = [
    { path: '/', component: Home },
    { path: '/settings', component: Settings },
    { path: '/about', component: About },
    { path: `/server/:id`, component: Server },
    { path: '/servers', component: Servers },
    { path: '/new-server', component: NewServer },
]

const router = createRouter({
    history: createWebHistory(),
    routes,
})

export default router