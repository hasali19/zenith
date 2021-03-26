import Vue from 'vue'

const CAST_SENDER_SCRIPT_SRC =
  'https://www.gstatic.com/cv/js/sender/v1/cast_sender.js?loadCastFramework=1'

const state = Vue.observable({
  ready: false,
  connected: false,
  paused: false,
  currentTime: 0,
})

window.__onGCastApiAvailable = function(isAvailable: boolean) {
  if (!isAvailable) {
    console.warn('cast api not available')
    return
  }

  cast.framework.CastContext.getInstance().setOptions({
    receiverApplicationId: chrome.cast.media.DEFAULT_MEDIA_RECEIVER_APP_ID,
    autoJoinPolicy: chrome.cast.AutoJoinPolicy.ORIGIN_SCOPED,
  })

  const player = new cast.framework.RemotePlayer()
  const controller = new cast.framework.RemotePlayerController(player)

  state.ready = isAvailable
  state.connected = player.isConnected

  controller.addEventListener(
    cast.framework.RemotePlayerEventType.IS_CONNECTED_CHANGED,
    () => {
      state.connected = player.isConnected
    },
  )

  controller.addEventListener(
    cast.framework.RemotePlayerEventType.IS_PAUSED_CHANGED,
    () => {
      state.paused = player.isPaused
    },
  )

  controller.addEventListener(
    cast.framework.RemotePlayerEventType.CURRENT_TIME_CHANGED,
    () => {
      state.currentTime = player.currentTime
    },
  )
}

export default {
  state,

  init() {
    const script = document.createElement('script')
    script.src = CAST_SENDER_SCRIPT_SRC
    document.head.appendChild(script)
  },

  async connect() {
    const context = cast.framework.CastContext.getInstance()
    return new Promise<cast.framework.CastSession>((resolve, reject) => {
      context.requestSession().then(
        () => {
          const session = context.getCurrentSession()
          if (session) {
            resolve(session)
          } else {
            reject('session not found')
          }
        },
        () => reject('failed to create session'),
      )
    })
  },
}
