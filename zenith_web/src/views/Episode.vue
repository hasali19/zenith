<template>
  <div v-if="episode">
    <div style="position: relative">
      <v-img :src="episode.thumbnail" style="aspect-ratio: 16 / 9" />
      <v-overlay absolute :value="episode.is_watched">
        <v-icon x-large>mdi-check</v-icon>
      </v-overlay>
    </div>
    <div class="text-h4 mx-4 mt-6">
      {{ episode.episode_number }} - {{ episode.name }}
    </div>
    <div class="text-h6 mx-4" v-if="show">{{ show.name }}</div>
    <div class="text-caption mx-4 mb-2" v-if="season">
      {{ season.name }}
      <span class="mx-1">•</span>
      {{ duration }}
    </div>
    <div class="text-body-2 mx-4">{{ episode.overview }}</div>
    <v-btn class="my-4 ml-4 mr-1" color="accent" @click="onPlay">
      <v-icon left>mdi-play</v-icon> Play
    </v-btn>
    <v-btn class="my-4 mx-1" color="secondary" @click="onCast">
      <v-icon left>mdi-cast</v-icon> Cast
    </v-btn>
    <v-btn class="my-4 mx-1" color="secondary" @click="refreshMetadata">
      <v-icon>mdi-refresh</v-icon>
    </v-btn>
  </div>
</template>

<script lang="ts">
import Vue from 'vue'

import api, { TvEpisode, TvSeason, TvShow } from '@/api'
import gcast from '@/gcast'

export default Vue.extend({
  data() {
    return {
      show: null as TvShow | null,
      season: null as TvSeason | null,
      episode: null as TvEpisode | null,
    }
  },

  computed: {
    duration(): string | null {
      if (!this.episode) return null
      const duration = this.episode.duration
      if (duration <= 90 * 60) {
        return `${Math.floor(duration / 60)}m`
      } else {
        const hours = Math.floor(duration / 3600)
        const minutes = Math.floor((duration % 3600) / 60)
        return `${hours}h ${minutes}m`
      }
    },
  },

  async mounted() {
    this.episode = await api.tv.getEpisode(this.$route.params.id)
    api.tv.getShow(this.episode.show_id).then(show => (this.show = show))
    api.tv
      .getSeason(this.episode.season_id)
      .then(season => (this.season = season))
  },

  methods: {
    onPlay() {
      this.$router.push(`/player/${this.$route.params.id}`)
    },

    onCast() {
      gcast.connect().then(session => {
        const id = this.$route.params.id
        const url = origin + api.stream.getTranscodeUrl(id)
        const info = new window.chrome.cast.media.MediaInfo(url, 'video/mp4')
        const request = new window.chrome.cast.media.LoadRequest(info)

        session.loadMedia(request).then(() => {
          this.$router.push(`/cast/${this.$route.params.id}`)
        })
      })
    },

    refreshMetadata() {
      api.metadata.refresh(this.$route.params.id)
    },
  },
})
</script>
