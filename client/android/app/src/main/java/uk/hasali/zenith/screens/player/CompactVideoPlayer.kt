package uk.hasali.zenith.screens.player

import androidx.compose.animation.ExperimentalAnimationApi
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Close
import androidx.compose.material.icons.filled.Pause
import androidx.compose.material.icons.filled.PlayArrow
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.platform.LocalLifecycleOwner
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import androidx.lifecycle.Lifecycle
import androidx.lifecycle.repeatOnLifecycle
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.channels.awaitClose
import kotlinx.coroutines.channels.trySendBlocking
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.callbackFlow
import uk.hasali.zenith.media.MediaSessionManager
import uk.hasali.zenith.media.VideoPlayer
import uk.hasali.zenith.media.pollPosition
import uk.hasali.zenith.ui.rememberFlowWithLifecycle

@Composable
private fun rememberCurrentVideoPlayer(mediaSessionManager: MediaSessionManager): Flow<VideoPlayer?> {
    return rememberFlowWithLifecycle(
        callbackFlow {
            send(mediaSessionManager.getCurrentPlayer())

            val listener = object : MediaSessionManager.Listener {
                override fun onPlayerChanged() {
                    trySendBlocking(mediaSessionManager.getCurrentPlayer())
                }
            }

            mediaSessionManager.addListener(listener)
            awaitClose {
                mediaSessionManager.removeListener(listener)
            }
        }
    )
}

@OptIn(ExperimentalAnimationApi::class, ExperimentalCoroutinesApi::class)
@Composable
fun CompactVideoPlayer(
    mediaSessionManager: MediaSessionManager,
    onNavigateToPlayer: () -> Unit,
) {
    val player by rememberCurrentVideoPlayer(mediaSessionManager)
        .collectAsState(initial = null)

    player?.let {
        CompactVideoPlayer(
            player = it,
            onEndSession = { mediaSessionManager.endCurrentSession() },
            onNavigateToPlayer = onNavigateToPlayer,
        )
    }
}

@OptIn(ExperimentalMaterialApi::class)
@Composable
private fun CompactVideoPlayer(
    player: VideoPlayer,
    onEndSession: () -> Unit,
    onNavigateToPlayer: () -> Unit,
) {
    val lifecycle = LocalLifecycleOwner.current.lifecycle
    val item by player.currentItem.collectAsState()

    item?.let { currentItem ->
        Surface(
            elevation = 2.dp,
            onClick = onNavigateToPlayer,
            modifier = Modifier
                .fillMaxWidth()
                .height(64.dp),
        ) {
            val isPlaying by player.isPlaying.collectAsState()
            val playWhenReady by player.playWhenReady.collectAsState()
            var position by remember { mutableStateOf(0L) }
            val duration = currentItem.duration
            val progress = if (duration > 0) position.toFloat() / duration.toFloat() else 0f

            LaunchedEffect(isPlaying) {
                if (isPlaying) {
                    lifecycle.repeatOnLifecycle(Lifecycle.State.RESUMED) {
                        player.pollPosition()
                            .collect {
                                position = it / 1000
                            }
                    }
                }
            }

            Column(modifier = Modifier.fillMaxSize()) {
                Row(
                    verticalAlignment = Alignment.CenterVertically,
                    modifier = Modifier.weight(1f)
                ) {
                    Box(
                        modifier = Modifier
                            .fillMaxHeight()
                            .aspectRatio(16f / 9f)
                            .background(Color.Black)
                    ) {
                        VideoPlayerView(player = player, scaleMode = ScaleMode.Zoom)
                    }

                    Column(
                        modifier = Modifier
                            .weight(1f)
                            .fillMaxHeight()
                            .padding(8.dp),
                    ) {
                        Text(
                            text = currentItem.title,
                            style = MaterialTheme.typography.subtitle2,
                            maxLines = 1,
                            overflow = TextOverflow.Ellipsis,
                        )

                        if (currentItem.subtitle != null) {
                            Text(
                                text = currentItem.subtitle,
                                color = LocalContentColor.current.copy(alpha = 0.7f),
                                style = MaterialTheme.typography.body2,
                                maxLines = 1,
                                overflow = TextOverflow.Ellipsis,
                            )
                        }
                    }

                    IconButton(onClick = { player.setPlayWhenReady(!playWhenReady) }) {
                        Icon(
                            if (playWhenReady) Icons.Default.Pause else Icons.Default.PlayArrow,
                            null
                        )
                    }

                    IconButton(onClick = onEndSession) {
                        Icon(Icons.Default.Close, null)
                    }
                }

                Box(
                    modifier = Modifier
                        .fillMaxWidth()
                        .height(4.dp)
                        .background(MaterialTheme.colors.onBackground.copy(alpha = 0.3f)),
                ) {
                    Box(
                        modifier = Modifier
                            .fillMaxWidth(progress)
                            .fillMaxHeight()
                            .background(MaterialTheme.colors.onBackground),
                    )
                }
            }
        }
    }

}
