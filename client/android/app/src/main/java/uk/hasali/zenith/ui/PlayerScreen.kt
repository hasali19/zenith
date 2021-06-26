package uk.hasali.zenith.ui

import android.app.Activity
import android.view.WindowManager
import androidx.compose.animation.ExperimentalAnimationApi
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material.Surface
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.viewinterop.AndroidView
import androidx.core.view.WindowCompat
import androidx.core.view.WindowInsetsCompat
import androidx.core.view.WindowInsetsControllerCompat
import com.google.android.exoplayer2.ExoPlayer
import com.google.android.exoplayer2.MediaItem
import com.google.android.exoplayer2.Player
import com.google.android.exoplayer2.SimpleExoPlayer
import com.google.android.exoplayer2.ui.PlayerView
import com.google.android.exoplayer2.ui.StyledPlayerView
import kotlinx.coroutines.delay
import kotlinx.coroutines.launch
import uk.hasali.zenith.VideoInfo
import uk.hasali.zenith.ZenithApiClient

@Composable
fun PlayerScreen(id: Int) {
    val client = LocalZenithClient.current
    val navigator = LocalNavigator.current

    val info by produceState<VideoInfo?>(initialValue = null, id) {
        value = client.getVideoInfo(id)
    }

    KeepScreenOn {
        FullScreen {
            if (info != null) {
                VideoPlayer(
                    id = id,
                    client = client,
                    startPosition = info!!.position?.toInt() ?: 0,
                    onVideoEnded = { navigator.pop() },
                )
            }
        }
    }
}

@Composable
private fun KeepScreenOn(content: @Composable() () -> Unit) {
    val activity = LocalContext.current as? Activity
    val window = activity?.window

    if (window != null) {
        DisposableEffect(Unit) {
            window.addFlags(WindowManager.LayoutParams.FLAG_KEEP_SCREEN_ON)

            onDispose {
                window.clearFlags(WindowManager.LayoutParams.FLAG_KEEP_SCREEN_ON)
            }
        }
    }

    content()
}

@Composable
private fun FullScreen(content: @Composable() () -> Unit) {
    val activity = LocalContext.current as? Activity
    val window = activity?.window

    if (window != null) {
        DisposableEffect(Unit) {
            val controller = WindowCompat.getInsetsController(window, window.decorView)
            if (controller != null) {
                controller.hide(WindowInsetsCompat.Type.statusBars() or WindowInsetsCompat.Type.navigationBars())
                controller.systemBarsBehavior =
                    WindowInsetsControllerCompat.BEHAVIOR_SHOW_TRANSIENT_BARS_BY_SWIPE
            }

            onDispose {
                controller?.show(WindowInsetsCompat.Type.statusBars() or WindowInsetsCompat.Type.navigationBars())
            }
        }
    }

    content()
}

@OptIn(ExperimentalAnimationApi::class)
@Composable
private fun VideoPlayer(
    id: Int,
    client: ZenithApiClient,
    startPosition: Int,
    onVideoEnded: () -> Unit,
) {
    val context = LocalContext.current
    val scope = rememberCoroutineScope()

    val player = remember {
        SimpleExoPlayer.Builder(context)
            .build()
            .also { player ->
                player.addListener(object : Player.Listener {
                    override fun onPlaybackStateChanged(state: Int) {
                        if (state == ExoPlayer.STATE_ENDED) {
                            onVideoEnded()
                        }
                    }
                })

                scope.launch {
                    while (true) {
                        if (player.playWhenReady) {
                            val position = player.currentPosition / 1000
                            launch {
                                client.updateProgress(id, position)
                            }
                        }

                        delay(2000)
                    }
                }
            }
    }

    DisposableEffect(id) {
        val item = MediaItem.fromUri(client.getVideoUrl(id))

        player.setMediaItem(item)
        player.prepare()
        player.seekTo(startPosition.toLong() * 1000)
        player.play()

        onDispose {
            player.stop()
        }
    }

    DisposableEffect(Unit) {
        onDispose {
            player.release()
        }
    }

    Surface(
        color = Color.Black,
        modifier = Modifier.fillMaxSize(),
    ) {
        AndroidView(
            modifier = Modifier.fillMaxSize(),
            factory = { context -> StyledPlayerView(context).apply { setShowSubtitleButton(true) } },
            update = { playerView -> playerView.player = player },
        )
    }
}
