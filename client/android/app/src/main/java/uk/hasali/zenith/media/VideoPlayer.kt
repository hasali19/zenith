package uk.hasali.zenith.media

import androidx.media3.common.Player
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.StateFlow

interface VideoPlayer {
    val usePlayerView: Boolean
        get() = false

    val player: Player?
        get() = null

    val currentItem: StateFlow<VideoItem?>
    val subtitleTrack: StateFlow<SubtitleTrack?>

    val isPlaying: StateFlow<Boolean>
    val playWhenReady: StateFlow<Boolean>

    fun setVideoEndedCallback(callback: () -> Unit)
    fun removeVideoEndedCallback(callback: () -> Unit)

    fun pollPosition(resolution: Int = 500): Flow<Long>

    fun setItem(item: VideoItem)
    fun setSubtitleTrack(subtitle: SubtitleTrack?)
    fun setPlayWhenReady(playWhenReady: Boolean)

    fun stop()

    fun seekTo(position: Long)

    fun dispose()
}
