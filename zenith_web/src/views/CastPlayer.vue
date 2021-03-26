<template>
  <div class="root">
    <div style="flex: 1"></div>
    <div class="row-centered">
      <v-icon size="200" v-if="!connected">mdi-cast</v-icon>
      <v-icon size="200" v-else color="blue">mdi-cast-connected</v-icon>
    </div>
    <div class="row-centered">
      <v-btn v-if="!connected" @click="onCast">Cast</v-btn>
      <v-btn v-else @click="onDisconnect">Stop</v-btn>
    </div>
    <div style="flex: 1"></div>
    <div class="row-centered bottom-controls">
      <v-btn fab large light color="grey lighten-2">
        <v-icon v-if="connected && paused">mdi-play</v-icon>
        <v-icon v-else>mdi-pause</v-icon>
      </v-btn>
    </div>
  </div>
</template>

<style scoped>
.root {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  justify-content: center;
}

.row-centered {
  display: flex;
  justify-content: center;
}

.bottom-controls {
  padding: 16px;
}
</style>

<script lang="ts">
import Vue from 'vue'

import api from '@/api'
import gcast from '@/gcast'

export default Vue.extend({
  computed: {
    ready() {
      return gcast.state.ready
    },

    connected() {
      return gcast.state.connected
    },

    paused() {
      return gcast.state.paused
    },

    currentTime() {
      return gcast.state.currentTime
    },
  },

  methods: {
    onCast() {
      gcast.connect().then(session => {
        const id = this.$route.params.id
        const url = origin + api.stream.getTranscodeUrl(id)
        const info = new window.chrome.cast.media.MediaInfo(url, 'video/mp4')
        const request = new window.chrome.cast.media.LoadRequest(info)
        session.loadMedia(request)
      })
    },

    onDisconnect() {
      const context = window.cast.framework.CastContext.getInstance()
      const session = context.getCurrentSession()
      if (session) {
        session.endSession(true)
      }
    },
  },
})
</script>
