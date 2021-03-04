<template>
  <div class="root" @mousemove="onMouseMove" @touchmove="onMouseMove">
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
        <div class="time" style="margin: 16px 0 16px 16px">
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
        <div class="time" style="margin: 16px 0">
          {{ formattedRemaining }}
        </div>
        <v-btn
          icon
          style="margin-right: 16px; margin-left: 8px"
          @click="fullscreen = !fullscreen"
        >
          <v-icon>mdi-fullscreen</v-icon>
        </v-btn>
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
  layout: 'fullscreen',

  data() {
    return {
      duration: 1365,
      start: 0,
      position: 0,
      paused: false,
      interval: null as any,
      controls: false,
      timeout: null as any,
      fullscreen: false,
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

    fullscreen(val) {
      if (val) {
        document.documentElement.requestFullscreen()
      } else {
        document.exitFullscreen()
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
  position: absolute;
  width: 100%;
  height: 100%;
  top: 0;
  background-color: black;
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
  align-items: center;
}

.time {
  font-size: 0.8em;
}

.seekbar {
  -webkit-appearance: none;
  flex: 1;
  margin: 16px;
  background: transparent;
}

.seekbar:focus {
  outline: none;
}

.seekbar::-webkit-slider-thumb {
  -webkit-appearance: none;
  width: 16px;
  height: 16px;
  border-radius: 50%;
  background: rgb(202, 155, 0);
  cursor: pointer;
  margin-top: -6px; /* You need to specify a margin in Chrome, but in Firefox and IE it is automatic */
}

.seekbar::-webkit-slider-runnable-track {
  width: 100%;
  height: 3px;
  cursor: pointer;
  background: rgb(214, 214, 214);
  border-radius: 1px;
}
</style>
