<template>
  <div style="overflow-x: auto; width: 100%">
    <div class="slide-group my-2">
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
