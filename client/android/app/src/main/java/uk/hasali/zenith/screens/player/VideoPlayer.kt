package uk.hasali.zenith.screens.player

import androidx.annotation.OptIn
import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material.Surface
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.platform.LocalLifecycleOwner
import androidx.compose.ui.viewinterop.AndroidView
import androidx.lifecycle.Lifecycle
import androidx.lifecycle.repeatOnLifecycle
import androidx.media3.common.Player
import androidx.media3.common.util.UnstableApi
import androidx.media3.ui.AspectRatioFrameLayout
import androidx.media3.ui.PlayerView
import coil.compose.rememberImagePainter
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
fun VideoPlayer(
    player: VideoPlayer,
    autoHideControls: Boolean,
    onClosePressed: () -> Unit,
) {
    Surface(
        color = Color.Black,
        modifier = Modifier.fillMaxSize(),
    ) {
        var scaleMode by remember { mutableStateOf(ScaleMode.Fit) }

        VideoPlayerView(
            player = player,
            scaleMode = scaleMode,
        )

        VideoPlayerOverlay(
            player = player,
            autoHideControls = autoHideControls,
            scaleMode = scaleMode,
            onSetScaleMode = { scaleMode = it },
            onClosePressed = onClosePressed,
        )
    }
}

enum class ScaleMode {
    Fit,
    Zoom,
}

private fun ScaleMode.toResizeMode() = when (this) {
    ScaleMode.Fit -> AspectRatioFrameLayout.RESIZE_MODE_FIT
    ScaleMode.Zoom -> AspectRatioFrameLayout.RESIZE_MODE_ZOOM
}

@OptIn(UnstableApi::class)
@Composable
fun VideoPlayerView(player: VideoPlayer, scaleMode: ScaleMode) {
    val item by player.currentItem.collectAsState()

    if (player.usePlayerView) {
        AndroidView(
            modifier = Modifier.fillMaxSize(),
            factory = { context -> PlayerView(context).apply { useController = false } },
            update = { playerView ->
                playerView.player = player.player
                playerView.resizeMode = scaleMode.toResizeMode()
            },
        )
    } else {
        item?.let {
            val painter = rememberImagePainter(it.backdrop) {
                crossfade(true)
            }

            Image(
                painter = painter,
                contentDescription = null,
                contentScale = ContentScale.Crop,
            )
        }
    }
}

@Composable
fun VideoPlayerOverlay(
    player: VideoPlayer,
    autoHideControls: Boolean,
    scaleMode: ScaleMode,
    onSetScaleMode: (ScaleMode) -> Unit,
    onClosePressed: () -> Unit,
) {
    val lifecycle = LocalLifecycleOwner.current.lifecycle
    val visibility = rememberControlsVisibility(!autoHideControls)

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
        isLoading = playWhenReady && !isPlaying,
        isPlaying = playWhenReady,
        subtitles = item?.subtitles.orEmpty(),
        selectedSubtitle = subtitleTrack,
        scaleMode = scaleMode,
        onSeekStart = { isSeeking = true },
        onSeekEnd = {
            player.seekTo(it * 1000)
            position = it
            isSeeking = false
        },
        onTogglePlaying = { player.setPlayWhenReady(!playWhenReady) },
        onSelectSubtitle = { player.setSubtitleTrack(it) },
        onSetScaleMode = onSetScaleMode,
        onClosePressed = onClosePressed,
        visibility = visibility,
    )
}
