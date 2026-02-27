import { createRouter, createWebHistory } from 'vue-router'
import LandingView from '@/views/LandingView.vue'
import LibraryView from '@/views/LibraryView.vue'
import ReaderView from '@/views/ReaderView.vue'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: '/', component: LandingView },
    { path: '/library', component: LibraryView },
    { path: '/reader', component: ReaderView },
  ],
})

export default router
