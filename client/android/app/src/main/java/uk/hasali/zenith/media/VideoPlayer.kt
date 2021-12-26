package uk.hasali.zenith.media

import androidx.media3.common.Player
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.StateFlow

interface VideoPlayer {
    val isLocal: Boolean
        get() = false

    val player: Player?
        get() = null

    val currentItem: StateFlow<VideoItem?>
    val subtitleTrack: StateFlow<SubtitleTrack?>

    enum class State {
        Active,
        Ended,
    }

    val state: StateFlow<State>
    val isPlaying: StateFlow<Boolean>
    val playWhenReady: StateFlow<Boolean>

    fun pollPosition(delayMs: Int = 500): Flow<Long>

    fun setItem(item: VideoItem, startAt: Long)
    fun setSubtitleTrack(subtitle: SubtitleTrack?)
    fun setPlayWhenReady(playWhenReady: Boolean)

    fun seekTo(position: Long)

    /**
     * Stops playback and releases all resources.
     *
     * This may be called multiple times so implementations should be idempotent.
     */
    fun dispose()
}
