import { createRouter, createWebHashHistory } from 'vue-router'

const routes = [
  {
    path: '/',
    alias: '/index',
    name: 'Index',
    component: () => import('../views/Index.vue')
  },
  {
    path: '/service/:name',
    name: 'Service',
    component: () => import('../views/Service.vue')
  },
]

const router = createRouter({
  history: createWebHashHistory(),
  routes
})

export default router