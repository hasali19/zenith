<template>
  <div v-if="show">
    <v-img :src="show.backdrop" style="aspect-ratio: 16 / 9" />
    <div class="text-h4 mx-4 mt-6 mb-2">{{ show.name }}</div>
    <div class="text-body-2 mx-4">{{ show.overview }}</div>
    <div class="mt-6">
      <div class="text-h5 mx-4">Seasons</div>
      <slide-group padding="0px 12px" style="overflow-x: auto; width: 100%">
        <poster-card
          v-for="season in seasons"
          :key="season.id"
          :poster="season.poster"
          :primary="show.name"
          :secondary="season.name"
          @click="onSeasonClick(season.id)"
          style="width: 120px; margin: 0px 4px"
        />
      </slide-group>
    </div>
  </div>
</template>

<script lang="ts">
import Vue from 'vue'

import api, { TvShow } from '@/api'

import PosterCard from '@/components/PosterCard.vue'
import SlideGroup from '@/components/SlideGroup.vue'

export default Vue.extend({
  components: { PosterCard, SlideGroup },

  data() {
    return {
      show: null as TvShow | null,
      seasons: [],
    }
  },

  async mounted() {
    this.show = await api.tv.getShow(this.$route.params.id)

    const seasons = await fetch(
      `/api/tv/shows/${this.$route.params.id}/seasons`,
    ).then(res => res.json())

    this.seasons = seasons
  },

  methods: {
    onSeasonClick(id: number) {
      this.$router.push({ path: `/seasons/${id}` })
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

.thumbnail {
  aspect-ratio: 16 / 9;
}
</style>
