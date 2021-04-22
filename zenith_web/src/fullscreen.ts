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

  toggle(element: HTMLElement = document.documentElement) {
    if (this.get()) {
      this.exit()
    } else {
      this.enter(element)
    }
  },

  enter(element: HTMLElement = document.documentElement) {
    element.requestFullscreen()
  },

  exit() {
    document.exitFullscreen()
  },
}
