<template>
  <div style="overflow-x: auto; width: 100%">
    <div class="slide-group my-2" :style="{ padding }">
      <slot></slot>
    </div>
  </div>
</template>

<style scoped>
.slide-group {
  display: flex;
  box-sizing: border-box;
  min-width: 100%;
  float: left;
}
</style>

<script lang="ts">
import Vue from 'vue'
export default Vue.extend({
  props: {
    padding: String,
  },
})
</script>
