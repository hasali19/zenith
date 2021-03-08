<template>
  <v-container>
    <div>
      <h1 class="text-h5 my-4">Recently Added Movies</h1>
      <v-sheet outlined rounded class="pt-2">
        <v-slide-group style="margin: 8px 4px" show-arrows="desktop">
          <v-slide-item
            v-for="movie in movies"
            :key="movie.id"
            style="margin: 0px 4px"
          >
            <div style="width: 120px">
              <v-card @click="onMovieClick(movie.id)">
                <v-img :src="movie.poster" class="poster"></v-img>
              </v-card>
              <div class="my-2">
                <div class="subtitle-2 text-truncate">{{ movie.title }}</div>
                <div class="caption">
                  {{ movie.releaseYear() }}
                </div>
              </div>
            </div>
          </v-slide-item>
        </v-slide-group>
      </v-sheet>
    </div>
    <div class="mt-8">
      <h1 class="text-h5 my-4">Recently Added TV</h1>
      <v-sheet outlined rounded class="pt-2">
        <v-slide-group style="margin: 8px 4px" show-arrows="desktop">
          <v-slide-item
            v-for="show in shows"
            :key="show.id"
            style="margin: 0px 4px"
          >
            <div style="width: 120px">
              <v-card @click="onShowClick(show.id)">
                <v-img :src="show.poster" class="poster"></v-img>
              </v-card>
              <div class="my-2">
                <div class="subtitle-2 text-truncate">{{ show.name }}</div>
                <div class="caption">
                  {{ show.startYear() }}
                </div>
              </div>
            </div>
          </v-slide-item>
        </v-slide-group>
      </v-sheet>
    </div>
  </v-container>
</template>

<style scoped></style>

<script lang="ts">
import Vue from 'vue'

import api, { Movie, TvShow } from '@/api'

export default Vue.extend({
  data() {
    return {
      movies: [] as Movie[],
      shows: [] as TvShow[],
    }
  },

  async mounted() {
    this.movies = await api.movies.getRecent()
    this.shows = await api.tv.getRecentShows()
  },

  methods: {
    onMovieClick(id: number) {
      this.$router.push({ path: `/player/${id}` })
    },

    onShowClick(id: number) {
      this.$router.push({ path: `/shows/${id}` })
    },
  },
})
</script>
