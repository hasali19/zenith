<template>
  <div class="root">
    <div class="d-flex">
      <h1 class="text-h5 flex-grow-1">{{ title }}</h1>
      <v-btn @click="prev" icon :disabled="page === 0">
        <v-icon>mdi-chevron-left</v-icon>
      </v-btn>
      <v-btn @click="next" icon :disabled="page === pageCount - 1">
        <v-icon>mdi-chevron-right</v-icon>
      </v-btn>
    </div>
    <div class="wrapper my-2" :style="{ transform }">
      <div
        class="item"
        v-for="(item, index) in items"
        :key="index"
        :style="{
          minWidth: `${100 / itemCount}%`,
          width: `${100 / itemCount}%`,
        }"
      >
        <slot v-bind:item="item"></slot>
      </div>
    </div>
  </div>
</template>

<style scoped>
.root {
  width: 100%;
  overflow-x: hidden;
}

.wrapper {
  margin: 0px -4px;
  display: flex;
  box-sizing: border-box;
  width: calc(100% + 8px);
  float: left;
  transition: transform 300ms;
}

.item {
  padding: 4px;
}
</style>

<script lang="ts">
import Vue from 'vue'
export default Vue.extend({
  props: {
    title: String,
    items: Array,
  },

  data() {
    return {
      page: 0,
    }
  },

  computed: {
    itemCount(): number {
      switch (this.$vuetify.breakpoint.name) {
        case 'xs':
          return 3

        case 'sm':
          return 5

        case 'md':
          return 7

        case 'lg':
          return 9

        case 'xl':
          return 11

        default:
          return 0
      }
    },

    pageCount(): number {
      return Math.ceil(this.items.length / this.itemCount)
    },

    transform(): string {
      return `translateX(${this.page * -100}%)`
    },
  },

  methods: {
    prev() {
      this.page = Math.max(0, this.page - 1)
    },

    next() {
      this.page = Math.min(this.page + 1, this.pageCount - 1)
    },
  },
})
</script>
