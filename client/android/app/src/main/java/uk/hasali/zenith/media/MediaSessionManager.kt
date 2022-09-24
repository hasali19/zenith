package uk.hasali.zenith.media

import android.content.Context
import com.google.android.gms.cast.framework.CastContext
import com.google.android.gms.cast.framework.CastSession
import com.google.android.gms.cast.framework.SessionManagerListener
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

    private var castContext: CastContext? = null

    private val castSessionManagerListener = object : SessionManagerListener<CastSession> {
        override fun onSessionEnded(p0: CastSession, p1: Int) {}

        override fun onSessionEnding(p0: CastSession) {}

        override fun onSessionResumeFailed(p0: CastSession, p1: Int) {}

        override fun onSessionResumed(p0: CastSession, p1: Boolean) {
            val session = castContext?.sessionManager?.currentCastSession!!
            val client = session.remoteMediaClient!!
            if (player == null) {
                val remotePlayer = RemoteVideoPlayer(context, client)
                setPlayer(remotePlayer)
            }
        }

        override fun onSessionResuming(p0: CastSession, p1: String) {}

        override fun onSessionStartFailed(p0: CastSession, p1: Int) {}

        override fun onSessionStarted(p0: CastSession, p1: String) {}

        override fun onSessionStarting(p0: CastSession) {}

        override fun onSessionSuspended(p0: CastSession, p1: Int) {}
    }

    private val listeners = mutableListOf<Listener>()
    private var player: VideoPlayer? = null

    init {
        CastContext.getSharedInstance(context) { it.run() }
            .addOnCompleteListener {
                castContext = it.result
            }
    }

    fun init() {
        castContext?.sessionManager?.addSessionManagerListener(
            castSessionManagerListener,
            CastSession::class.java
        )
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
        castContext?.sessionManager?.removeSessionManagerListener(
            castSessionManagerListener,
            CastSession::class.java
        )
    }

    private fun setPlayer(value: VideoPlayer?) {
        player?.dispose()
        player = value
        listeners.forEach {
            it.onPlayerChanged()
        }
    }

    private fun createPlayer(): VideoPlayer {
        val currentCastSession = castContext?.sessionManager?.currentCastSession
        val mediaClient = currentCastSession?.remoteMediaClient
        return if (mediaClient == null) {
            LocalVideoPlayer(context)
        } else {
            RemoteVideoPlayer(context, mediaClient)
        }
    }
}
