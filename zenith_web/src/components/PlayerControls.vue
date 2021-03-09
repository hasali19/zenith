<template>
  <div class="root">
    <div class="main-controls">
      <div class="main-controls-buttons">
        <v-btn fab light color="grey" @click="$emit('rewind', 10)">
          <v-icon>mdi-rewind-10</v-icon>
        </v-btn>
        <v-btn fab x-large light color="grey lighten-2" @click="toggleState">
          <v-icon v-if="state === 'paused'">mdi-play</v-icon>
          <v-icon v-else-if="state === 'playing'">mdi-pause</v-icon>
        </v-btn>
        <v-btn fab light color="grey" @click="$emit('fast-forward', 30)">
          <v-icon>mdi-fast-forward-30</v-icon>
        </v-btn>
      </div>
    </div>
    <div class="bottom-controls">
      <time-text
        style="margin: 0px 8px"
        :value="position"
        :show-hours="duration >= 3600"
      />
      <seek-bar
        style="margin: 0px 8px"
        :duration="duration"
        :position="seekbarPos"
        @seek-start="onSeekStart"
        @seek-end="onSeekEnd"
      />
      <time-text
        style="margin: 0px 8px"
        :value="duration - position"
        :show-hours="duration >= 3600"
      />
      <v-btn
        style="margin: 0px 8px 0px 0px"
        icon
        @click="$emit('toggle-fullscreen')"
      >
        <v-icon>mdi-fullscreen</v-icon>
      </v-btn>
    </div>
  </div>
</template>

<style scoped>
.root {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  background: linear-gradient(#0000, 90%, #000);
}

.main-controls {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
}

.main-controls-buttons {
  width: 100%;
  max-width: 600px;
  display: flex;
  align-items: center;
  justify-content: space-evenly;
}

.bottom-controls {
  display: flex;
  align-items: center;
  padding: 8px;
}
</style>

<script lang="ts">
import Vue from 'vue'

import SeekBar from './SeekBar.vue'
import TimeText from './TimeText.vue'

export default Vue.extend({
  components: { SeekBar, TimeText },

  props: {
    duration: {
      type: Number,
      required: true,
    },
    position: {
      type: Number,
      required: true,
    },
    state: {
      type: String,
      required: true,
      validator(value) {
        return ['playing', 'paused'].indexOf(value) !== -1
      },
    },
  },

  data() {
    return {
      seeking: false,
      seekbarPos: this.position,
    }
  },

  watch: {
    position(value) {
      if (!this.seeking) {
        this.seekbarPos = value
      }
    },

    seeking(value) {
      if (!value) {
        this.seekbarPos = this.position
      }
    },
  },

  methods: {
    toggleState() {
      if (this.state === 'playing') {
        this.$emit('pause')
      } else if (this.state === 'paused') {
        this.$emit('play')
      }
    },

    onSeekStart() {
      this.seeking = true
      this.$emit('seek-start')
    },

    onSeekEnd(position: number) {
      this.seeking = false
      this.$emit('seek-end', position)
    },
  },
})
</script>
