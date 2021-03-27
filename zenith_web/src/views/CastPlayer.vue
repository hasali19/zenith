<template>
  <div class="root">
    <div style="flex: 1"></div>
    <div class="row-centered">
      <v-icon size="200" v-if="!connected">mdi-cast</v-icon>
      <v-icon size="200" v-else color="blue">mdi-cast-connected</v-icon>
    </div>
    <div class="row-centered" v-if="!connected">
      <div>
        <v-select
          v-if="info"
          label="Subtitles"
          :items="subtitles"
          item-text="text"
          item-value="value"
          v-model="subtitle"
          solo
        />
      </div>
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

import api, { StreamInfo } from '@/api'
import gcast from '@/gcast'

export default Vue.extend({
  data() {
    return {
      info: null as StreamInfo | null,
      subtitle: null as number | null,
    }
  },

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

    subtitles(): { text: string; value: number }[] {
      return (
        this.info?.subtitles?.map(s => ({
          text: s.title || s.language || 'Unknown',
          value: s.index,
        })) ?? []
      )
    },
  },

  async mounted() {
    const id = this.$route.params.id
    this.info = await api.stream.getInfo(id)
  },

  methods: {
    onCast() {
      gcast.connect().then(session => {
        const id = this.$route.params.id
        const url = origin + api.stream.getTranscodeUrl(id)
        const info = new window.chrome.cast.media.MediaInfo(url, 'video/mp4')

        if (this.subtitle !== null) {
          const id = this.$route.params.id
          const type = window.chrome.cast.media.TrackType.TEXT
          const subtitle = new window.chrome.cast.media.Track(1, type)
          const item = this.info?.subtitles.find(s => s.index === this.subtitle)

          if (!item) {
            throw new Error('invalid subtitle track index')
          }

          subtitle.trackContentId =
            origin + `/api/stream/${id}/subtitles/${this.subtitle}`

          subtitle.trackContentType = 'text/vtt'
          subtitle.subtype = window.chrome.cast.media.TextTrackType.SUBTITLES
          subtitle.name = item.title || item.language || ''
          subtitle.language = item.language || ''

          info.tracks = [subtitle]
        }

        const request = new window.chrome.cast.media.LoadRequest(info)

        session.loadMedia(request).then(() => {
          if (this.subtitle !== null) {
            const media = session.getMediaSession()
            if (media) {
              media.editTracksInfo(
                new window.chrome.cast.media.EditTracksInfoRequest([1]),
                () => undefined,
                e => console.error(e),
              )
            }
          }
        })
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
