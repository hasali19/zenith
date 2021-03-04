<template>
  <div class="root" @mousemove="onMouseMove">
    <video ref="video" class="video" :src="url" autoplay></video>
    <div v-if="controls" class="overlay">
      <div class="main-controls">
        <v-btn
          fab
          x-large
          light
          color="grey lighten-2"
          @click="paused = !paused"
        >
          <v-icon v-if="paused">mdi-play</v-icon>
          <v-icon v-else>mdi-pause</v-icon>
        </v-btn>
      </div>
      <div class="bottom-controls">
        <div style="margin: 16px 0 16px 16px">
          {{ formattedPosition }}
        </div>
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
        <div style="margin: 16px 16px 16px 0">
          {{ formattedRemaining }}
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import Vue from 'vue'

function formatTimeSegment(value: number) {
  return value.toString().padStart(2, '0')
}

function formatTime(value: number, duration: number) {
  const hours = formatTimeSegment(Math.floor(value / 3600))
  const mins = formatTimeSegment(Math.floor((value % 3600) / 60))
  const secs = formatTimeSegment(Math.floor((value % 3600) % 60))

  if (duration >= 3600) {
    return `${hours}:${mins}:${secs}`
  } else {
    return `${mins}:${secs}`
  }
}

export default Vue.extend({
  data() {
    return {
      duration: 1365,
      start: 0,
      position: 0,
      paused: false,
      interval: null as any,
      controls: false,
      timeout: null as any,
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

    formattedPosition(): string {
      return formatTime(this.totalPosition, this.duration)
    },

    formattedRemaining(): string {
      return formatTime(this.duration - this.totalPosition, this.duration)
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

    onMouseMove() {
      this.controls = true

      if (this.timeout) {
        window.clearTimeout(this.timeout)
      }

      this.timeout = window.setTimeout(() => (this.controls = false), 3000)
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

.main-controls {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
}

.bottom-controls {
  display: flex;
}

.seekbar {
  margin: 16px;
  flex: 1;
}
</style>
