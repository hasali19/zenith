<template>
  <v-app>
    <v-navigation-drawer
      v-model="drawer"
      app
      clipped
      touchless
      :style="{ 'background-color': '#121212' }"
      :permanent="!isMobile"
    >
      <v-list nav>
        <v-list-item
          v-for="item in drawerItems"
          :key="item.to"
          :to="item.to"
          router
          exact
          active-class="active"
        >
          <v-list-item-action>
            <v-icon>{{ item.icon }}</v-icon>
          </v-list-item-action>
          <v-list-item-content>
            <v-list-item-title>{{ item.name }}</v-list-item-title>
          </v-list-item-content>
        </v-list-item>
      </v-list>
    </v-navigation-drawer>
    <v-app-bar clipped-left app>
      <v-app-bar-nav-icon v-if="isMobile" @click.stop="drawer = !drawer" />
      <img
        src="/zenith_full.png"
        class="ml-2"
        style="max-height: calc(100% - 24px)"
      />
    </v-app-bar>
    <v-main>
      <transition name="zoom" mode="out-in">
        <router-view></router-view>
      </transition>
    </v-main>
  </v-app>
</template>

<style scoped>
.zoom-enter-active,
.zoom-leave-active {
  animation-duration: 300ms;
  animation-fill-mode: both;
  animation-name: zoom;
}

.zoom-leave-active {
  animation-direction: reverse;
}

@keyframes zoom {
  from {
    opacity: 0;
    transform: scale3d(0.9, 0.9, 0.9);
  }

  100% {
    opacity: 1;
  }
}

.active {
  background-color: #fd11009f;
}
</style>

<script>
export default {
  data() {
    return {
      title: 'Zenith',
      drawer: false,
      drawerItems: [
        {
          name: 'Home',
          icon: 'mdi-home',
          to: '/',
        },
        {
          name: 'Movies',
          icon: 'mdi-movie',
          to: '/movies',
        },
        {
          name: 'Shows',
          icon: 'mdi-television',
          to: '/shows',
        },
      ],
    }
  },

  computed: {
    isMobile() {
      return this.$vuetify.breakpoint.mobile
    },
  },
}
</script>
