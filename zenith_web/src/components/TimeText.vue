<template>
  <div class="text">
    {{ text }}
  </div>
</template>

<style scoped>
.text {
  font-size: 0.8em;
  user-select: none;
}
</style>

<script lang="ts">
import Vue from 'vue'

function formatTimeSegment(value: number) {
  return value.toString().padStart(2, '0')
}

function formatTime(value: number, showHours: boolean) {
  const hours = formatTimeSegment(Math.floor(value / 3600))
  const mins = formatTimeSegment(Math.floor((value % 3600) / 60))
  const secs = formatTimeSegment(Math.floor((value % 3600) % 60))

  if (showHours) {
    return `${hours}:${mins}:${secs}`
  } else {
    return `${mins}:${secs}`
  }
}

export default Vue.extend({
  props: {
    value: {
      type: Number,
      required: true,
    },
    showHours: {
      type: Boolean,
      default: true,
    },
  },

  computed: {
    text() {
      return formatTime(this.value, this.showHours)
    },
  },
})
</script>
