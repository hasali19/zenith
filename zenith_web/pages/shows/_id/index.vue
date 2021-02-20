<template>
  <v-container fluid>
    <div class="text-h4">{{ show.name }}</div>
    <div v-for="season in seasons" :key="season.id">
      <div class="text-h5 mt-4 mb-4">{{ season.name }}</div>
      <div class="episode-grid">
        <div v-for="episode in season.episodes" :key="episode.id">
          <v-card @click="onEpisodeClick(episode.id)">
            <v-img :src="episode.thumbnail"></v-img>
          </v-card>
          <div class="mt-2 mb-4">
            <div class="subtitle-2 text-truncate">{{ episode.episode_number }} - {{ episode.name }}</div>
            <div
              class="caption"
              style="line-height: 1rem; max-height: 3rem; overflow: hidden"
            >{{ episode.overview }}</div>
          </div>
        </div>
      </div>
    </div>
  </v-container>
</template>

<script lang="ts">
import Vue from 'vue'
export default Vue.extend({
  async asyncData({ params }) {
    const show = await fetch(`/api/tv/shows/${params.id}`).then((res) =>
      res.json()
    )

    const seasons = await fetch(
      `/api/tv/shows/${params.id}/seasons`
    ).then((res) => res.json())

    for (const season of seasons) {
      season.episodes = await fetch(
        `/api/tv/seasons/${season.id}/episodes`
      ).then((res) => res.json())
    }

    return { show, seasons }
  },

  methods: {
    onEpisodeClick(id: number) {
      this.$router.push({ path: `/player/${id}` })
    },
  },
})
</script>

<style scoped>
.episode-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
  grid-gap: 16px;
}
</style>
