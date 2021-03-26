<template>
  <v-container>
    <div v-if="season" class="text-h4 mt-2 mb-4">{{ season.name }}</div>
    <div class="episode-grid">
      <div v-for="episode in episodes" :key="episode.id">
        <v-card @click="onEpisodeClick(episode.id)">
          <v-img :src="episode.thumbnail" class="thumbnail"></v-img>
        </v-card>
        <div class="mt-2 mb-4">
          <div class="subtitle-2 text-truncate">
            {{ episode.episode_number }} - {{ episode.name }}
          </div>
          <div class="caption text--secondary">
            {{ duration(episode.duration) }}
          </div>
          <div class="caption overview">
            {{ episode.overview }}
          </div>
        </div>
      </div>
    </div>
  </v-container>
</template>

<style scoped>
.episode-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
  grid-gap: 16px;
}

.thumbnail {
  aspect-ratio: 16 / 9;
}

.overview {
  display: -webkit-box;
  -webkit-box-orient: vertical;
  -webkit-line-clamp: 3;
  overflow: hidden;
}
</style>

<script lang="ts">
import Vue from 'vue'
export default Vue.extend({
  data() {
    return {
      season: null,
      episodes: [],
    }
  },

  async mounted() {
    const season = await fetch(
      `/api/tv/seasons/${this.$route.params.id}`,
    ).then(res => res.json())

    this.season = season

    const episodes = await fetch(
      `/api/tv/seasons/${this.$route.params.id}/episodes`,
    ).then(res => res.json())

    this.episodes = episodes
  },

  methods: {
    duration(duration: number): string {
      if (duration <= 90 * 60) {
        return `${Math.floor(duration / 60)}m`
      } else {
        const hours = Math.floor(duration / 3600)
        const minutes = Math.floor((duration % 3600) / 60)
        return `${hours}h ${minutes}m`
      }
    },

    onEpisodeClick(id: number) {
      this.$router.push({ path: `/episodes/${id}` })
    },
  },
})
</script>
