<template>
  <div>
    <v-card @click="$emit('click')">
      <v-img :src="poster" class="poster"></v-img>
    </v-card>
    <div class="my-2">
      <div class="subtitle-2 text-truncate">{{ primary }}</div>
      <div class="caption">{{ secondary }}</div>
    </div>
  </div>
</template>

<style scoped>
.poster {
  aspect-ratio: 2 / 3;
}
</style>

<script lang="ts">
import Vue from 'vue'
export default Vue.extend({
  props: {
    primary: {
      type: String,
      required: true,
    },
    secondary: [String, Number],
    poster: String,
  },
})
</script>
