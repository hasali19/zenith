<template>
  <v-container>
    <div class="mt-4 mt-sm-8">
      <h1 class="text-h5 my-4">Recently Added Movies</h1>
      <slide-group style="overflow-x: auto; width: 100%">
        <poster-card
          v-for="movie in movies"
          :key="movie.id"
          :poster="movie.poster"
          :primary="movie.title"
          :secondary="movie.releaseYear()"
          @click="onMovieClick(movie.id)"
          style="width: 120px; margin: 0px 4px"
        />
      </slide-group>
    </div>
    <div class="mt-8">
      <h1 class="text-h5 my-4">Recently Added TV</h1>
      <slide-group style="overflow-x: auto; width: 100%">
        <poster-card
          v-for="show in shows"
          :key="show.id"
          :poster="show.poster"
          :primary="show.name"
          :secondary="show.startYear()"
          @click="onShowClick(show.id)"
          class="group-item mx-1"
        />
      </slide-group>
    </div>
  </v-container>
</template>

<style scoped>
.group-item {
  width: 120px;
}
</style>

<script lang="ts">
import Vue from 'vue'

import api, { Movie, TvShow } from '@/api'

import PosterCard from '@/components/PosterCard.vue'
import SlideGroup from '@/components/SlideGroup.vue'

export default Vue.extend({
  components: { PosterCard, SlideGroup },

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
