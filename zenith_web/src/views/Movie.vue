<template>
  <div v-if="movie" class="root">
    <div v-if="$vuetify.breakpoint.mobile">
      <v-img :src="movie.backdrop" style="aspect-ratio: 16 / 9" />
      <div class="text-h4 mx-4 mt-6">{{ movie.title }}</div>
      <div class="text-caption mx-4 mb-2">
        {{ movie.releaseYear() }} • {{ duration }}
      </div>
      <div class="text-body-2 mx-4">{{ movie.overview }}</div>
      <v-btn class="ma-4 mr-1" color="accent" @click="onPlay">
        <v-icon left>mdi-play</v-icon> Play
      </v-btn>
      <v-btn class="ma-4 ml-1" color="secondary" @click="onCast">
        <v-icon left>mdi-cast</v-icon> Cast
      </v-btn>
    </div>
    <div v-else style="width: 100%; height: 100%">
      <div class="backdrop" :style="`background-image: url(${movie.backdrop})`">
        <div class="backdrop-overlay"></div>
      </div>
      <div class="content">
        <div class="main">
          <v-card max-width="260px" elevation="8">
            <v-img :src="movie.poster"></v-img>
          </v-card>
          <div class="main-content">
            <div class="text-h3">{{ movie.title }}</div>
            <div class="my-2 text-body-2">
              {{ movie.releaseYear() }} • {{ duration }}
            </div>
            <div class="my-6">{{ movie.overview }}</div>
            <v-btn color="accent" @click="onPlay">
              <v-icon left>mdi-play</v-icon> Play
            </v-btn>
            <v-btn @click="onCast">Cast</v-btn>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.root {
  width: 100%;
  height: 100%;
}

.backdrop {
  width: 100%;
  height: 100%;
  background-position: center;
  background-size: cover;
}

.backdrop-overlay {
  width: 100%;
  height: 100%;
  background-color: rgba(0, 0, 0, 0.7);
}

.content {
  width: 100%;
  height: 100%;
  position: absolute;
  top: 0;
  overflow-y: auto;
}

.main {
  display: flex;
  max-width: 1000px;
  margin: 92px;
}

.main-content {
  margin-left: 32px;
}
</style>

<script lang="ts">
import Vue from 'vue'

import api, { Movie } from '@/api'
import gcast from '@/gcast'

export default Vue.extend({
  data() {
    return {
      movie: null as Movie | null,
    }
  },

  computed: {
    duration(): string | null {
      if (!this.movie) return null
      const duration = this.movie.duration
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
    this.movie = await api.movies.getMovie(this.$route.params.id)
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
  },
})
</script>
