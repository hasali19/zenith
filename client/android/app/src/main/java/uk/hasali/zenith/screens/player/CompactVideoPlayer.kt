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
import kotlinx.coroutines.flow.emitAll
import kotlinx.coroutines.flow.transformLatest
import uk.hasali.zenith.media.MediaSessionManager
import uk.hasali.zenith.media.VideoPlayer

@OptIn(ExperimentalAnimationApi::class, ExperimentalCoroutinesApi::class)
@Composable
fun CompactVideoPlayer(
    mediaSessionManager: MediaSessionManager,
    onNavigateToPlayer: () -> Unit,
) {
    var player by remember { mutableStateOf<VideoPlayer?>(null) }

    LaunchedEffect(mediaSessionManager) {
        mediaSessionManager.current
            .transformLatest {
                if (it != null) {
                    emitAll(it.player)
                } else {
                    emit(null)
                }
            }
            .collect {
                println(it)
                player = it
            }
    }

    player?.let {
        CompactVideoPlayer(
            player = it,
            onEndSession = { mediaSessionManager.stop() },
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

    Surface(
        elevation = 2.dp,
        onClick = onNavigateToPlayer,
        modifier = Modifier
            .fillMaxWidth()
            .height(64.dp),
    ) {
        val item by player.currentItem.collectAsState()
        val isPlaying by player.isPlaying.collectAsState()
        val playWhenReady by player.playWhenReady.collectAsState()
        var position by remember { mutableStateOf(0L) }
        val duration = item?.duration ?: 0

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
            Row(verticalAlignment = Alignment.CenterVertically, modifier = Modifier.weight(1f)) {
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
                    item?.let { item ->
                        Text(
                            text = item.title,
                            style = MaterialTheme.typography.subtitle2,
                            maxLines = 1,
                            overflow = TextOverflow.Ellipsis,
                        )

                        if (item.subtitle != null) {
                            Text(
                                text = item.subtitle,
                                color = LocalContentColor.current.copy(alpha = 0.7f),
                                style = MaterialTheme.typography.body2,
                                maxLines = 1,
                                overflow = TextOverflow.Ellipsis,
                            )
                        }
                    }
                }

                IconButton(onClick = { player.setPlayWhenReady(!playWhenReady) }) {
                    Icon(if (playWhenReady) Icons.Default.Pause else Icons.Default.PlayArrow, null)
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
                        .fillMaxWidth(position.toFloat() / duration.toFloat())
                        .fillMaxHeight()
                        .background(MaterialTheme.colors.onBackground),
                )
            }
        }
    }
}
