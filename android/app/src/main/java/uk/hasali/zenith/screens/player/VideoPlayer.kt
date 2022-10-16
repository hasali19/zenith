package uk.hasali.zenith.screens.player

import androidx.annotation.OptIn
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.LocalLifecycleOwner
import androidx.compose.ui.viewinterop.AndroidView
import androidx.lifecycle.Lifecycle
import androidx.lifecycle.repeatOnLifecycle
import androidx.media3.common.util.UnstableApi
import androidx.media3.ui.AspectRatioFrameLayout
import androidx.media3.ui.PlayerView
import coil.compose.AsyncImage
import coil.request.ImageRequest
import uk.hasali.zenith.LocalPictureInPictureController
import uk.hasali.zenith.media.VideoPlayer
import uk.hasali.zenith.media.pollPosition

@Composable
fun VideoPlayer(
    player: VideoPlayer,
    onClosePressed: () -> Unit,
) {
    val pictureInPictureController = LocalPictureInPictureController.current

    val isPlaying by player.playWhenReady.collectAsState()
    val isInPictureInPictureMode by pictureInPictureController.isInPictureInPictureMode.collectAsState()

    DisposableEffect(player, pictureInPictureController, isPlaying) {
        pictureInPictureController.setEnterOnUserLeaveHint(player.isLocal && isPlaying)
        onDispose {
            pictureInPictureController.setEnterOnUserLeaveHint(false)
        }
    }

    var scaleMode by remember { mutableStateOf(ScaleMode.Fit) }

    VideoPlayerView(
        player = player,
        scaleMode = scaleMode,
    )

    if (!isInPictureInPictureMode) {
        VideoPlayerOverlay(
            player = player,
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

    if (player.isLocal) {
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
            AsyncImage(
                model = ImageRequest.Builder(LocalContext.current)
                    .data(it.backdrop)
                    .crossfade(true)
                    .build(),
                contentDescription = null,
                contentScale = ContentScale.Crop,
            )
        }
    }
}

@Composable
fun VideoPlayerOverlay(
    player: VideoPlayer,
    scaleMode: ScaleMode,
    onSetScaleMode: (ScaleMode) -> Unit,
    onClosePressed: () -> Unit,
) {
    val lifecycle = LocalLifecycleOwner.current.lifecycle
    val visibility = rememberControlsVisibility(!player.isLocal)

    val item by player.currentItem.collectAsState()
    val subtitleTrack by player.subtitleTrack.collectAsState()
    val state by player.state.collectAsState()
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
        state = state,
        isLoading = playWhenReady && !isPlaying && state == VideoPlayer.State.Active,
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
        onReplay = { player.restart() },
        onSelectSubtitle = { player.setSubtitleTrack(it) },
        onSetScaleMode = onSetScaleMode,
        onClosePressed = onClosePressed,
        visibility = visibility,
    )
}