let isFullscreen = false

document.addEventListener('fullscreenchange', () => {
  if (document.fullscreenElement) {
    isFullscreen = true
  } else {
    isFullscreen = false
  }
})

export default {
  get() {
    return isFullscreen
  },

  toggle() {
    if (this.get()) {
      this.exit()
    } else {
      this.enter()
    }
  },

  enter() {
    document.documentElement.requestFullscreen()
  },

  exit() {
    document.exitFullscreen()
  },
}
