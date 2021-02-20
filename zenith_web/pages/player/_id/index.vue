<template>
  <div class="root">
    <video class="video" ref="video" controls></video>
  </div>
</template>

<script lang="ts">
import Vue from 'vue'
import Hls from 'hls.js'

export default Vue.extend({
  data() {
    return {
      ready: false,
    }
  },

  computed: {
    id(): number {
      return parseInt(this.$route.params.id)
    },

    url() {
      return `/api/hls/${this.$route.params.id}/main.m3u8`
    },
  },

  mounted() {
    fetch(`/api/hls/${this.id}/prepare`, { method: 'POST' }).then(() => {
      const video = this.$refs.video as HTMLVideoElement
      if (video.canPlayType('application/vnd.apple.mpegURL')) {
        video.src = this.url
      } else if (Hls.isSupported()) {
        const hls = new Hls()
        hls.loadSource(this.url)
        hls.attachMedia(video)
        video.play()
      }
    })
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
</style>
