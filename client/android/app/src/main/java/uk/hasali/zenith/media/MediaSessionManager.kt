package uk.hasali.zenith.media

import android.content.Context
import com.google.android.gms.cast.framework.CastContext
import dagger.hilt.android.qualifiers.ApplicationContext
import dagger.hilt.android.scopes.ActivityRetainedScoped
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.MainScope
import kotlinx.coroutines.cancel
import kotlinx.coroutines.flow.*
import javax.inject.Inject

@ActivityRetainedScoped
class MediaSessionManager @Inject constructor(
    @ApplicationContext
    private val context: Context,
) {
    private val scope = MainScope()

    private val _current = MutableStateFlow<MediaSession?>(null)
    val current: StateFlow<MediaSession?>
        get() = _current

    fun play(item: VideoItem, startAt: Long) {
        // If there is an existing session, end it
        _current.value?.dispose()

        // Create a new session
        _current.value = createSession().also { session ->
            session.player.value.setItem(item, startAt)
        }
    }

    fun stop() {
        _current.value?.dispose()
        _current.value = null
    }

    fun dispose() {
        scope.cancel()
    }

    private fun createSession(): MediaSession {
        return MediaSessionImpl()
    }

    private fun createPlayer(): VideoPlayer {
        val castContext = CastContext.getSharedInstance(context)
        val castSessionManager = castContext.sessionManager
        val castSession = castSessionManager.currentCastSession
        return if (castSession == null) {
            LocalVideoPlayer(context)
        } else {
            RemoteVideoPlayer(context, castSession)
        }
    }

    @OptIn(ExperimentalCoroutinesApi::class)
    private inner class MediaSessionImpl : MediaSession {
        private val _player = MutableStateFlow(createPlayer())
        override val player: StateFlow<VideoPlayer>
            get() = _player

        override val state = _player
            .transform { emitAll(it.state) }
            .stateIn(scope, SharingStarted.Eagerly, VideoPlayer.State.Active)

        override fun dispose() {
            _player.value.dispose()
        }
    }
}
