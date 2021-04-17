<template>
  <v-app v-if="!fullscreen">
    <v-navigation-drawer v-model="drawer" clipped fixed app touchless>
      <v-list nav>
        <v-list-item to="/" router exact>
          <v-list-item-action>
            <v-icon>mdi-home</v-icon>
          </v-list-item-action>
          <v-list-item-content>
            <v-list-item-title>Home</v-list-item-title>
          </v-list-item-content>
        </v-list-item>
        <v-list-item to="/movies" router exact>
          <v-list-item-action>
            <v-icon>mdi-movie</v-icon>
          </v-list-item-action>
          <v-list-item-content>
            <v-list-item-title>Movies</v-list-item-title>
          </v-list-item-content>
        </v-list-item>
        <v-list-item to="/shows" router exact>
          <v-list-item-action>
            <v-icon>mdi-television</v-icon>
          </v-list-item-action>
          <v-list-item-content>
            <v-list-item-title>Shows</v-list-item-title>
          </v-list-item-content>
        </v-list-item>
      </v-list>
    </v-navigation-drawer>
    <v-app-bar clipped-left fixed app>
      <v-app-bar-nav-icon @click.stop="drawer = !drawer" />
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
</style>

<script>
export default {
  data() {
    return {
      drawer: false,
      title: 'Zenith',
    }
  },
  computed: {
    fullscreen() {
      return this.$route.path.startsWith('/player')
    },
  },
}
</script>
