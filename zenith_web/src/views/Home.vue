<template>
  <v-container>
    <div class="mt-4 mt-sm-8">
      <slide-group title="Recently Added Movies" :items="movies">
        <template v-slot="{ item }">
          <poster-card
            :poster="item.poster"
            :primary="item.title"
            :secondary="item.releaseYear()"
            @click="onMovieClick(item.id)"
          />
        </template>
      </slide-group>
    </div>
    <div class="mt-8">
      <slide-group title="Recently Added TV" :items="shows">
        <template v-slot="{ item }">
          <poster-card
            :poster="item.poster"
            :primary="item.name"
            :secondary="item.startYear()"
            @click="onShowClick(item.id)"
          />
        </template>
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
      this.$router.push({ path: `/movies/${id}` })
    },

    onShowClick(id: number) {
      this.$router.push({ path: `/shows/${id}` })
    },
  },
})
</script>
