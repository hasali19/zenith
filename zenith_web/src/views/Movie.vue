<template>
  <div v-if="movie">
    <v-img :src="movie.backdrop" style="aspect-ratio: 16 / 9" />
    <div class="text-h4 mx-4 mt-6">{{ movie.title }}</div>
    <div class="text-caption mx-4 mb-2">
      {{ movie.releaseYear() }} • {{ duration }}
    </div>
    <div class="text-body-2 mx-4">{{ movie.overview }}</div>
    <v-btn class="ma-4" color="accent" @click="onPlay">
      <v-icon left>mdi-play</v-icon> Play
    </v-btn>
  </div>
</template>

<script lang="ts">
import Vue from 'vue'

import api, { Movie } from '@/api'

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
  },
})
</script>
