<template>
  <div class="px-4">
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
  </div>
</template>

<script lang="ts">
import Vue from 'vue'
export default Vue.extend({
  data() {
    return {
      movies: [],
    }
  },

  async mounted() {
    const res = await fetch('/api/movies')
    const data = await res.json()
    this.movies = data
  },

  methods: {
    onItemClick(id: number) {
      this.$router.push({ path: `/movies/${id}` })
    },
  },
})
</script>

<style scoped lang="scss">
@import '~vuetify/src/styles/settings/_variables';

.grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(120px, 1fr));
  grid-gap: 8px;
}

@media #{map-get($display-breakpoints, 'md-and-up')} {
  .grid {
    grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
  }
}

.poster {
  aspect-ratio: 2 / 3;
}
</style>
