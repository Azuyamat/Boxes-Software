import { createRouter, createWebHistory } from 'vue-router'
import Home from './components/pages/Home.vue'
import Server from './components/pages/Server.vue'
import Settings from './components/pages/Settings.vue'
import About from './components/pages/About.vue'

const routes = [
    { path: '/', component: Home },
    { path: '/settings', component: Settings },
    { path: '/about', component: About },
    { path: `/server/:id`, component: Server },
]

const router = createRouter({
    history: createWebHistory(),
    routes,
})

export default router