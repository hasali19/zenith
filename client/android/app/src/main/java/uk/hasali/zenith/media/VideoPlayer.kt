package uk.hasali.zenith.media

import androidx.media3.common.Player
import kotlinx.coroutines.delay
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.flow

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

    val position: Long

    fun setItem(item: VideoItem, startAt: Long)
    fun setSubtitleTrack(subtitle: SubtitleTrack?)
    fun setPlayWhenReady(playWhenReady: Boolean)

    fun seekTo(position: Long)

    fun dispose()
}

fun VideoPlayer.pollPosition(delayMs: Int = 500): Flow<Long> {
    return flow {
        while (true) {
            emit(position)
            delay(delayMs.toLong())
        }
    }
}
