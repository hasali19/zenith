package uk.hasali.zenith.screens.player

import androidx.annotation.OptIn
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material.Surface
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.platform.LocalLifecycleOwner
import androidx.compose.ui.viewinterop.AndroidView
import androidx.lifecycle.Lifecycle
import androidx.lifecycle.repeatOnLifecycle
import androidx.media3.common.Player
import androidx.media3.common.util.UnstableApi
import androidx.media3.ui.PlayerView
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.collect

interface VideoPlayer {
    val usePlayerView: Boolean
        get() = false

    val player: Player?
        get() = null

    val currentItem: StateFlow<VideoItem?>
    val subtitleTrack: StateFlow<SubtitleTrack?>

    val isPlaying: StateFlow<Boolean>
    val playWhenReady: StateFlow<Boolean>

    fun pollPosition(resolution: Int = 500): Flow<Long>

    fun setItem(item: VideoItem)
    fun setSubtitleTrack(subtitle: SubtitleTrack?)
    fun setPlayWhenReady(playWhenReady: Boolean)

    fun stop()

    fun seekTo(position: Long)

    fun dispose()
}

@Composable
fun VideoPlayer(player: VideoPlayer) {
    Surface(
        color = Color.Black,
        modifier = Modifier.fillMaxSize(),
    ) {
        VideoPlayerView(player = player)
        VideoPlayerOverlay(player = player)
    }
}

@OptIn(UnstableApi::class)
@Composable
fun VideoPlayerView(player: VideoPlayer) {
    if (player.usePlayerView) {
        AndroidView(
            modifier = Modifier.fillMaxSize(),
            factory = { context -> PlayerView(context).apply { useController = false } },
            update = { playerView -> playerView.player = player.player },
        )
    }
}

@Composable
fun VideoPlayerOverlay(player: VideoPlayer) {
    val lifecycle = LocalLifecycleOwner.current.lifecycle
    val visibility = rememberControlsVisibility()

    val item by player.currentItem.collectAsState()
    val subtitleTrack by player.subtitleTrack.collectAsState()
    val isPlaying by player.isPlaying.collectAsState()
    val playWhenReady by player.playWhenReady.collectAsState()

    var position by remember { mutableStateOf(0L) }
    var isSeeking by remember { mutableStateOf(false) }

    LaunchedEffect(visibility.isVisible, isPlaying, isSeeking) {
        if (visibility.isVisible && isPlaying && !isSeeking) {
            lifecycle.repeatOnLifecycle(Lifecycle.State.RESUMED) {
                player.pollPosition()
                    .collect {
                        position = it / 1000
                    }
            }
        }
    }

    Controls(
        title = item?.title.orEmpty(),
        position = position,
        duration = item?.duration?.toLong() ?: 0,
        isPlaying = playWhenReady,
        subtitles = item?.subtitles.orEmpty(),
        selectedSubtitle = subtitleTrack,
        onSeekStart = { isSeeking = true },
        onSeekEnd = {
            player.seekTo(it * 1000)
            position = it
            isSeeking = false
        },
        onTogglePlaying = { player.setPlayWhenReady(!playWhenReady) },
        onSelectSubtitle = { player.setSubtitleTrack(it) },
        onLaunchExternal = { },
        onBackPressed = { },
        visibility = visibility,
    )
}
