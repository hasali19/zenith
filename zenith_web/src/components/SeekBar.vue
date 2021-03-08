<template>
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
</template>

<style scoped>
.seekbar {
  -webkit-appearance: none;
  flex: 1;
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
  margin-top: -6px;
}

.seekbar::-webkit-slider-runnable-track {
  width: 100%;
  height: 3px;
  cursor: pointer;
  background: rgb(214, 214, 214);
  border-radius: 1px;
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
