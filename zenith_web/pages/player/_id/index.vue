<template>
  <div class="root">
    <video ref="video" class="video" :src="url" autoplay></video>
    <div class="overlay">
      <div style="flex: 1"></div>
      <div class="bottom-controls">
        <input
          type="range"
          class="seekbar"
          min="0"
          :max="duration"
          :value="totalPosition"
          @mousedown="onSeekStart"
          @touchstart="onSeekStart"
          @change="onSeekEnd"
        />
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import Vue from 'vue'

export default Vue.extend({
  data() {
    return {
      duration: 1365,
      start: 0,
      position: 0,
      paused: false,
      interval: null as any,
    }
  },

  computed: {
    id(): number {
      return parseInt(this.$route.params.id)
    },

    url(): string {
      return `/api/stream/${this.$route.params.id}/transcode?start=${this.start}`
    },

    totalPosition(): number {
      return this.start + this.position
    },
  },

  watch: {
    paused(val) {
      const video = this.$refs.video as HTMLVideoElement
      if (val) {
        video.pause()
      } else {
        video.play()
      }
    },
  },

  mounted() {
    this.interval = window.setInterval(this.updatePosition, 200)
  },

  beforeDestroy() {
    window.clearInterval(this.interval)
  },

  methods: {
    updatePosition() {
      if (!this.paused) {
        const video = this.$refs.video as HTMLVideoElement
        this.position = video.currentTime
      }
    },

    onSeekStart() {
      this.paused = true
    },

    onSeekEnd(e: any) {
      this.start = parseFloat(e.target.value)
      this.position = 0
      this.paused = false
    },
  },
})
</script>

<style scoped>
.root {
  width: 100%;
  height: 100%;
}

.video {
  width: 100%;
  height: 100%;
}

.overlay {
  position: absolute;
  width: 100%;
  height: 100%;
  top: 0;
  display: flex;
  flex-direction: column;
}

.bottom-controls {
  display: flex;
}

.seekbar {
  margin: 16px;
  flex: 1;
}
</style>
