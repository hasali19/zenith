<template>
  <div v-if="show">
    <v-img :src="show.backdrop" style="aspect-ratio: 16 / 9" />
    <div class="text-h4 mx-4 mt-6 mb-2">{{ show.name }}</div>
    <div class="text-body-2 mx-4">{{ show.overview }}</div>
    <div class="mt-6">
      <div class="mx-4">
        <slide-group title="Seasons" :items="seasons">
          <template v-slot="{ item }">
            <poster-card
              :poster="item.poster"
              :primary="show.name"
              :secondary="item.name"
              @click="onSeasonClick(item.id)"
            />
          </template>
        </slide-group>
      </div>
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
