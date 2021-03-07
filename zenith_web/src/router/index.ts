import Vue from 'vue'
import VueRouter, { RouteConfig } from 'vue-router'

import Home from '../views/Home.vue'
import Movies from '../views/Movies.vue'
import Shows from '../views/Shows.vue'
import Show from '../views/Show.vue'
import Player from '../views/Player.vue'

Vue.use(VueRouter)

const routes: Array<RouteConfig> = [
  {
    path: '/',
    name: 'Home',
    component: Home,
  },
  {
    path: '/movies',
    component: Movies,
  },
  {
    path: '/shows',
    component: Shows,
  },
  {
    path: '/shows/:id',
    component: Show,
  },
  {
    path: '/player/:id',
    component: Player,
  },
]

const router = new VueRouter({
  mode: 'history',
  base: process.env.BASE_URL,
  routes,
})

export default router
