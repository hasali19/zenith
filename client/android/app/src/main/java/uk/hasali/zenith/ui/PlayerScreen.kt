package uk.hasali.zenith.ui

import android.app.Activity
import android.content.res.ColorStateList
import android.support.v4.media.session.MediaSessionCompat
import android.text.format.DateUtils
import android.view.WindowManager
import android.widget.SeekBar
import androidx.compose.animation.*
import androidx.compose.animation.core.animateFloatAsState
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.gestures.detectTapGestures
import androidx.compose.foundation.interaction.MutableInteractionSource
import androidx.compose.foundation.layout.*
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.*
import androidx.compose.material.ripple.rememberRipple
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.toArgb
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.compose.ui.input.pointer.pointerInput
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import androidx.compose.ui.viewinterop.AndroidView
import androidx.core.graphics.BlendModeColorFilterCompat
import androidx.core.graphics.BlendModeCompat
import androidx.core.view.WindowCompat
import androidx.core.view.WindowInsetsCompat
import androidx.core.view.WindowInsetsControllerCompat
import com.google.android.exoplayer2.ExoPlayer
import com.google.android.exoplayer2.MediaItem
import com.google.android.exoplayer2.Player
import com.google.android.exoplayer2.SimpleExoPlayer
import com.google.android.exoplayer2.ext.mediasession.MediaSessionConnector
import com.google.android.exoplayer2.ui.PlayerView
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Job
import kotlinx.coroutines.delay
import kotlinx.coroutines.launch
import uk.hasali.zenith.VideoInfo
import uk.hasali.zenith.ZenithApiClient
import uk.hasali.zenith.playClick

@Composable
fun PlayerScreen(id: Int, title: String) {
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
                    title = title,
                    client = client,
                    startPosition = info!!.position?.toInt() ?: 0,
                    onVideoEnded = { navigator.pop() },
                )
            }
        }
    }
}

@Composable
private fun KeepScreenOn(content: @Composable () -> Unit) {
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
private fun FullScreen(content: @Composable () -> Unit) {
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
    title: String,
    client: ZenithApiClient,
    startPosition: Int,
    onVideoEnded: () -> Unit,
) {
    val context = LocalContext.current
    val scope = rememberCoroutineScope()

    val session = remember { MediaSessionCompat(context, context.packageName) }
    val connector = remember { MediaSessionConnector(session) }

    var position by remember { mutableStateOf(0L) }
    var duration by remember { mutableStateOf(0L) }
    var isPlaying by remember { mutableStateOf(true) }

    val player = remember {
        SimpleExoPlayer.Builder(context)
            .build()
            .also { player ->
                player.addListener(object : Player.Listener {
                    override fun onPlaybackStateChanged(state: Int) {
                        if (state == ExoPlayer.STATE_READY) {
                            duration = player.duration / 1000
                        }

                        if (state == ExoPlayer.STATE_ENDED) {
                            onVideoEnded()
                        }
                    }

                    override fun onPlayWhenReadyChanged(playWhenReady: Boolean, reason: Int) {
                        isPlaying = playWhenReady
                    }
                })

                scope.launch {
                    var counter = 0
                    while (true) {
                        counter += 1

                        if (player.playWhenReady) {
                            position = player.currentPosition / 1000
                            if (counter == 4) {
                                launch {
                                    client.updateProgress(id, position)
                                }
                            }
                        }

                        if (counter == 4) {
                            counter = 0
                        }

                        delay(500)
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
        connector.setPlayer(player)

        onDispose {
            connector.setPlayer(null)
            session.release()
            player.release()
        }
    }

    Surface(
        color = Color.Black,
        modifier = Modifier.fillMaxSize(),
    ) {
        AndroidView(
            modifier = Modifier.fillMaxSize(),
            factory = { context -> PlayerView(context).apply { useController = false } },
            update = { playerView -> playerView.player = player },
        )

        Controls(
            title = title,
            position = position,
            duration = duration,
            isPlaying = isPlaying,
            onSeekStart = { player.pause() },
            onSeekEnd = {
                player.seekTo(it * 1000)
                player.play()
            },
            onTogglePlaying = {
                player.playWhenReady = !isPlaying
            }
        )
    }
}

class ControlsController(private val scope: CoroutineScope) {
    private var _visible by mutableStateOf(false)
    val visible get() = _visible

    private var job: Job? = null
    private var isPlaying = true

    /**
     * Toggles the current visibility. If the controls are shown, a coroutine
     * is started to hide them after a delay.
     */
    fun toggle() {
        if (!_visible) {
            showAndHideDelayed()
        } else {
            _visible = false
        }
    }

    /**
     * Shows the controls immediately, and starts a coroutine to hide
     * them after a delay.
     */
    fun showAndHideDelayed() {
        _visible = true
        job?.cancel()
        job = scope.launch {
            delay(5000)
            _visible = false
        }
    }

    /**
     * Cancels the current delayed hide job (if any).
     */
    fun cancelHide() {
        job?.cancel()
        job = null
    }

    /**
     * Notifies controller of playback state changes.
     *
     * If the video has been paused, the current hide job is cancelled
     * and the controls are set to be visible indefinitely.
     *
     * If the video is resumed, controls will be hidden after a delay.
     */
    fun setPlaying(isPlaying: Boolean) {
        if (!isPlaying && this.isPlaying) {
            cancelHide()
            _visible = true
        } else if (isPlaying && !this.isPlaying) {
            showAndHideDelayed()
        }

        this.isPlaying = isPlaying
    }
}

@Composable
private fun rememberControlsController(): ControlsController {
    val scope = rememberCoroutineScope()
    val controls = remember { ControlsController(scope) }

    // Set controls as initially visible
    DisposableEffect(Unit) {
        controls.showAndHideDelayed()
        onDispose { }
    }

    return controls
}

@OptIn(ExperimentalAnimationApi::class)
@Composable
private fun Controls(
    title: String,
    position: Long,
    duration: Long,
    isPlaying: Boolean,
    onSeekStart: () -> Unit,
    onSeekEnd: (Long) -> Unit,
    onTogglePlaying: () -> Unit,
) {
    val context = LocalContext.current
    val navigator = LocalNavigator.current

    val controls = rememberControlsController()
    val opacity by animateFloatAsState(if (controls.visible) 0.4f else 0f)

    SideEffect {
        controls.setPlaying(isPlaying)
    }

    CompositionLocalProvider(
        LocalContentColor provides Color.White,
    ) {
        Box(
            modifier = Modifier
                .fillMaxSize()
                .background(Color.Black.copy(alpha = opacity))
                .pointerInput(Unit) {
                    detectTapGestures(onTap = { controls.toggle() })
                },
        ) {
            AnimatedVisibility(
                visible = controls.visible,
                enter = slideInVertically() + fadeIn(),
                exit = slideOutVertically() + fadeOut(),
                modifier = Modifier.align(Alignment.TopCenter),
            ) {
                Row(
                    verticalAlignment = Alignment.CenterVertically,
                    modifier = Modifier
                        .fillMaxWidth()
                        .height(56.dp)
                        .padding(horizontal = 16.dp),
                ) {
                    IconButton(onClick = {
                        context.playClick()
                        navigator.pop()
                    }) {
                        Icon(Icons.Default.ArrowBack, contentDescription = "Back")
                    }
                    Spacer(modifier = Modifier.width(8.dp))
                    Text(
                        text = title,
                        style = MaterialTheme.typography.h6,
                        maxLines = 1,
                        overflow = TextOverflow.Ellipsis,
                        modifier = Modifier.weight(1f),
                    )
                }
            }

            AnimatedVisibility(
                visible = controls.visible,
                enter = fadeIn() + expandIn(expandFrom = Alignment.Center),
                exit = fadeOut() + shrinkOut(shrinkTowards = Alignment.Center),
                modifier = Modifier.align(Alignment.Center)
            ) {
                Row(
                    verticalAlignment = Alignment.CenterVertically,
                    modifier = Modifier.pointerInput(Unit) { detectTapGestures { /* Consume tap events */ } },
                ) {
                    SeekButton(Icons.Default.Replay10) {
                        controls.showAndHideDelayed()
                        onSeekEnd(maxOf(0, position - 10))
                    }

                    PlayPauseButton(isPlaying = isPlaying) {
                        controls.showAndHideDelayed()
                        onTogglePlaying()
                    }

                    SeekButton(Icons.Default.Forward30) {
                        controls.showAndHideDelayed()
                        onSeekEnd(minOf(duration, position + 30))
                    }
                }
            }

            AnimatedVisibility(
                visible = controls.visible,
                enter = slideInVertically(initialOffsetY = { it / 2 }) + fadeIn(),
                exit = slideOutVertically(targetOffsetY = { it / 2 }) + fadeOut(),
                modifier = Modifier.align(Alignment.BottomCenter),
            ) {
                SeekBar(
                    position = position,
                    duration = duration,
                    onSeekStart = {
                        controls.cancelHide()
                        onSeekStart()
                    },
                    onSeekEnd = {
                        controls.showAndHideDelayed()
                        onSeekEnd(it)
                    },
                )
            }
        }
    }
}

@Composable
private fun PlayPauseButton(isPlaying: Boolean, onClick: () -> Unit) {
    val context = LocalContext.current

    Box(
        modifier = Modifier
            .size(72.dp)
            .clickable(
                interactionSource = remember { MutableInteractionSource() },
                indication = rememberRipple(bounded = false, radius = 40.dp),
            ) {
                context.playClick()
                onClick()
            },
    ) {
        Icon(
            if (isPlaying) Icons.Default.Pause else Icons.Default.PlayArrow,
            contentDescription = if (isPlaying) "Pause" else "Play",
            modifier = Modifier
                .align(Alignment.Center)
                .padding(8.dp)
                .fillMaxSize(),
        )
    }
}

@Composable
private fun SeekButton(imageVector: ImageVector, onClick: () -> Unit) {
    val context = LocalContext.current

    Box(
        modifier = Modifier
            .size(56.dp)
            .clickable(
                interactionSource = remember { MutableInteractionSource() },
                indication = rememberRipple(bounded = false, radius = 40.dp),
            ) {
                context.playClick()
                onClick()
            },
    ) {
        Icon(
            imageVector = imageVector,
            contentDescription = null,
            modifier = Modifier
                .align(Alignment.Center)
                .padding(8.dp)
                .fillMaxSize(),
        )
    }
}

@Composable
private fun SeekBar(
    position: Long,
    duration: Long,
    onSeekStart: () -> Unit,
    onSeekEnd: (Long) -> Unit,
    modifier: Modifier = Modifier,
) {
    val primaryColor = MaterialTheme.colors.primary.toArgb()
    var sliderPosition by remember { mutableStateOf(0L) }

    DisposableEffect(position) {
        sliderPosition = position
        onDispose { }
    }

    Row(modifier = modifier.padding(16.dp)) {
        TimeText(
            time = sliderPosition,
            align = TextAlign.Start,
        )

        AndroidView(
            modifier = Modifier.weight(1f),
            factory = {
                SeekBar(it).apply {
                    // Set progress and thumb colors from Compose Material theme
                    progressTintList = ColorStateList.valueOf(primaryColor)
                    thumb.colorFilter = BlendModeColorFilterCompat
                        .createBlendModeColorFilterCompat(primaryColor, BlendModeCompat.SRC_ATOP)

                    setOnSeekBarChangeListener(object : SeekBar.OnSeekBarChangeListener {
                        override fun onProgressChanged(
                            seekBar: SeekBar?,
                            progress: Int,
                            fromUser: Boolean,
                        ) {
                            sliderPosition = progress.toLong()
                        }

                        override fun onStartTrackingTouch(seekBar: SeekBar?) {
                            onSeekStart()
                        }

                        override fun onStopTrackingTouch(seekBar: SeekBar?) {
                            onSeekEnd(progress.toLong())
                        }
                    })
                }
            },
            update = {
                it.max = duration.toInt()
                it.progress = position.toInt()
            }
        )

        TimeText(
            time = duration - sliderPosition,
            align = TextAlign.Start,
        )
    }
}

@Composable
private fun TimeText(
    time: Long,
    modifier: Modifier = Modifier,
    align: TextAlign = TextAlign.Start,
) {
    Text(
        text = DateUtils.formatElapsedTime(time),
        color = Color.White,
        textAlign = align,
        style = MaterialTheme.typography.caption,
        modifier = modifier,
    )
}
