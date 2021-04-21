<template>
  <v-app v-if="!fullscreen">
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
      <transition name="fade" mode="out-in">
        <router-view></router-view>
      </transition>
    </v-main>
  </v-app>
  <v-app v-else>
    <router-view></router-view>
  </v-app>
</template>

<style scoped>
.fade-enter-active,
.fade-leave-active {
  transition-duration: 0.3s;
  transition-property: opacity;
  transition-timing-function: ease;
}

.fade-enter,
.fade-leave-active {
  opacity: 0;
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
    fullscreen() {
      return this.$route.path.startsWith('/player')
    },

    isMobile() {
      return this.$vuetify.breakpoint.mobile
    },
  },
}
</script>
