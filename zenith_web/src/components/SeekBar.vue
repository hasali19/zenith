<template>
  <div class="flex-grow-1 flex-shrink-1 d-flex" style="position: relative">
    <input
      type="range"
      class="seekbar"
      min="0"
      :max="duration"
      :value="position"
      @mousedown="$emit('seek-start')"
      @touchstart="$emit('seek-start')"
      @change="$emit('seek-end', parseFloat($event.target.value))"
    />
    <div
      class="track watched"
      :style="{
        width: `${(position / duration) * 100}%`,
      }"
    />
    <div
      class="track remaining"
      :style="{
        left: `${(position / duration) * 100}%`,
        width: `${100 - (position / duration) * 100}%`,
      }"
    />
  </div>
</template>

<style lang="scss" scoped>
.root {
  flex: 1;
}

.seekbar {
  -webkit-appearance: none;
  flex: 1;
  background: transparent;
  z-index: 1;
}

.seekbar:focus {
  outline: none;
}

.seekbar::-webkit-slider-thumb {
  -webkit-appearance: none;
  width: 14px;
  height: 14px;
  border-radius: 50%;
  background: var(--v-accent-base);
  cursor: pointer;
  margin-top: -5px;
}

.seekbar::-webkit-slider-runnable-track {
  width: 100%;
  height: 3px;
  cursor: pointer;
  background: transparent;
}

.track {
  position: absolute;
  height: 3px;
  z-index: 0;
}

.track.watched {
  background-color: var(--v-accent-base);
  border-top-left-radius: 2px;
  border-bottom-left-radius: 2px;
}

.track.remaining {
  background-color: rgb(214, 214, 214);
  border-top-right-radius: 2px;
  border-bottom-right-radius: 2px;
}
</style>

<script lang="ts">
import Vue from 'vue'
export default Vue.extend({
  props: {
    duration: {
      type: Number,
      required: true,
    },
    position: {
      type: Number,
      required: true,
    },
  },

  model: {
    prop: 'position',
    event: 'seek-end',
  },
})
</script>
