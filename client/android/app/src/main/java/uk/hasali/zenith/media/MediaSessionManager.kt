package uk.hasali.zenith.media

import android.content.Context
import com.google.android.gms.cast.framework.CastContext
import com.google.android.gms.cast.framework.CastState
import com.google.android.gms.cast.framework.CastStateListener
import dagger.hilt.android.qualifiers.ApplicationContext
import dagger.hilt.android.scopes.ActivityRetainedScoped
import javax.inject.Inject

@ActivityRetainedScoped
class MediaSessionManager @Inject constructor(
    @ApplicationContext
    private val context: Context,
) {
    interface Listener {
        fun onPlayerChanged() {}
    }

    private val castContext = CastContext.getSharedInstance(context)
    private val castStateListener = CastStateListener { state ->
        val currentPlayer = player

        fun switchToPlayer(player: VideoPlayer) {
            val currentItem = currentPlayer?.currentItem?.value
            if (currentItem != null) {
                player.setItem(currentItem, startAt = currentPlayer.position)
            }
            setPlayer(player)
        }

        if (state == CastState.CONNECTED) {
            if (currentPlayer !is RemoteVideoPlayer) {
                val session = castContext.sessionManager.currentCastSession!!
                val remotePlayer = RemoteVideoPlayer(context, session)
                switchToPlayer(remotePlayer)
            }
        } else {
            if (currentPlayer is RemoteVideoPlayer) {
                val localPlayer = LocalVideoPlayer(context)
                switchToPlayer(localPlayer)
            }
        }
    }

    private val listeners = mutableListOf<Listener>()
    private var player: VideoPlayer? = null

    fun init() {
        castContext.addCastStateListener(castStateListener)
        castStateListener.onCastStateChanged(castContext.castState)
    }

    fun addListener(listener: Listener) {
        listeners.add(listener)
    }

    fun removeListener(listener: Listener) {
        listeners.remove(listener)
    }

    fun getCurrentPlayer(): VideoPlayer? {
        return player
    }

    fun getOrCreatePlayer(): VideoPlayer {
        player?.let {
            return it
        }

        return createPlayer().also { player ->
            setPlayer(player)
        }
    }

    fun endCurrentSession() {
        setPlayer(null)
    }

    fun dispose() {
        castContext.removeCastStateListener(castStateListener)
    }

    private fun setPlayer(value: VideoPlayer?) {
        player?.dispose()
        player = value
        listeners.forEach {
            it.onPlayerChanged()
        }
    }

    private fun createPlayer(): VideoPlayer {
        val currentCastSession = castContext.sessionManager.currentCastSession
        return if (currentCastSession == null) {
            LocalVideoPlayer(context)
        } else {
            RemoteVideoPlayer(context, currentCastSession)
        }
    }
}
