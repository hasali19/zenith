package uk.hasali.zenith.media

import kotlinx.coroutines.flow.StateFlow

interface MediaSession {
    val player: StateFlow<VideoPlayer>
    val state: StateFlow<VideoPlayer.State>

    fun dispose()
}
