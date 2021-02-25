<template>
  <v-container fluid>
    <h1 class="display-1 mt-4 mb-4">Movies</h1>
    <div class="grid">
      <div v-for="movie in movies" :key="movie.id">
        <v-card @click="onItemClick(movie.id)">
          <v-img :src="movie.poster" class="poster"></v-img>
        </v-card>
        <div class="mt-2 mb-4">
          <div class="subtitle-2 text-truncate">{{ movie.title }}</div>
          <div class="caption">
            {{ new Date(movie.release_date * 1000).getFullYear() }}
          </div>
        </div>
      </div>
    </div>
  </v-container>
</template>

<script lang="ts">
import Vue from 'vue'
export default Vue.extend({
  data() {
    return {
      movies: [],
    }
  },

  async fetch() {
    const res = await fetch('/api/movies')
    const data = await res.json()
    this.movies = data
  },

  methods: {
    onItemClick(id: number) {
      this.$router.push({ path: `/player/${id}` })
    },
  },
})
</script>

<style scoped>
.grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(120px, 1fr));
  grid-gap: 8px;
}

.poster {
  aspect-ratio: 2 / 3;
}
</style>
