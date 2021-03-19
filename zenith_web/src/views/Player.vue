<template>
  <div
    class="root"
    @mousemove="onInteraction"
    @touchmove="onInteraction"
    @click="onInteraction"
  >
    <video ref="video" class="video" disableRemotePlayback></video>
    <transition name="fade">
      <div v-if="showControls" class="overlay">
        <player-controls
          :duration="duration"
          :position="offset + position"
          :state="state"
          @pause="video.pause()"
          @play="video.play()"
          @rewind="seekTo(offset + position - $event)"
          @fast-forward="seekTo(offset + position + $event)"
          @seek-start="startSeek"
          @seek-end="seekTo($event)"
          @toggle-fullscreen="toggleFullscreen"
        />
      </div>
    </transition>
  </div>
</template>

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
}

.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s;
}

.fade-enter,
.fade-leave-to {
  opacity: 0;
}
</style>

<script lang="ts">
import Vue from 'vue'

import api from '@/api'
import fullscreen from '@/fullscreen'

import PlayerControls from '@/components/PlayerControls.vue'

export default Vue.extend({
  components: { PlayerControls },

  data() {
    return {
      state: 'playing',
      duration: 0,
      offset: 0,
      position: 0,
      interval: undefined as number | undefined,
      controls: true,
      timeout: undefined as number | undefined,
    }
  },

  computed: {
    id() {
      return this.$route.params.id
    },
    video() {
      return this.$refs.video as HTMLVideoElement
    },
    showControls(): boolean {
      return this.controls || this.state === 'paused'
    },
  },

  async mounted() {
    const info = await api.stream.getInfo(this.$route.params.id)

    this.duration = info.duration

    this.interval = window.setInterval(() => {
      this.position = this.video.currentTime
    }, 200)

    this.video.addEventListener('pause', () => {
      this.state = 'paused'
    })

    this.video.addEventListener('play', () => {
      this.state = 'playing'
    })

    this.seekTo(0)
    this.delayedHideControls()
  },

  beforeDestroy() {
    if (this.interval) {
      window.clearInterval(this.interval)
    }
  },

  methods: {
    startSeek() {
      this.video.pause()
    },

    seekTo(position: number) {
      this.offset = Math.floor(Math.min(this.duration, Math.max(0, position)))
      this.position = 0
      this.video.src = api.stream.getTranscodeUrl(this.id, this.offset)
      this.video.play()
    },

    delayedHideControls() {
      if (this.timeout) window.clearTimeout(this.timeout)
      this.timeout = window.setTimeout(() => (this.controls = false), 3000)
    },

    toggleFullscreen() {
      fullscreen.toggle()
    },

    onInteraction() {
      this.controls = true
      this.delayedHideControls()
    },
  },
})
</script>
