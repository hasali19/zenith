package uk.hasali.zenith.ui

import android.app.Activity
import android.content.res.ColorStateList
import android.support.v4.media.session.MediaSessionCompat
import android.text.format.DateUtils
import android.view.WindowManager
import android.widget.SeekBar
import android.widget.Toast
import androidx.compose.animation.*
import androidx.compose.animation.core.animateFloatAsState
import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.gestures.detectTapGestures
import androidx.compose.foundation.interaction.MutableInteractionSource
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
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
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.LocalLifecycleOwner
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import androidx.compose.ui.viewinterop.AndroidView
import androidx.core.graphics.BlendModeColorFilterCompat
import androidx.core.graphics.BlendModeCompat
import androidx.core.view.WindowCompat
import androidx.core.view.WindowInsetsCompat
import androidx.core.view.WindowInsetsControllerCompat
import androidx.lifecycle.Lifecycle
import androidx.lifecycle.LifecycleEventObserver
import androidx.lifecycle.LifecycleOwner
import coil.compose.rememberImagePainter
import com.google.accompanist.insets.statusBarsPadding
import com.google.android.exoplayer2.ExoPlayer
import com.google.android.exoplayer2.MediaItem
import com.google.android.exoplayer2.Player
import com.google.android.exoplayer2.SimpleExoPlayer
import com.google.android.exoplayer2.ext.mediasession.MediaSessionConnector
import com.google.android.exoplayer2.ui.PlayerView
import com.google.android.gms.cast.*
import com.google.android.gms.cast.framework.CastContext
import com.google.android.gms.cast.framework.CastSession
import com.google.android.gms.cast.framework.media.RemoteMediaClient
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Job
import kotlinx.coroutines.delay
import kotlinx.coroutines.launch
import uk.hasali.zenith.SubtitleStreamInfo
import uk.hasali.zenith.VideoInfo
import uk.hasali.zenith.ZenithApiClient
import uk.hasali.zenith.playClick

enum class MediaItemType {
    Movie,
    TvShow,
}

private fun MediaItemType.toCastMediaType() = when (this) {
    MediaItemType.Movie -> MediaMetadata.MEDIA_TYPE_MOVIE
    MediaItemType.TvShow -> MediaMetadata.MEDIA_TYPE_TV_SHOW
}

@Composable
fun PlayerScreen(
    id: Int,
    title: String,
    type: MediaItemType,
    backdrop: String?,
    playFromStart: Boolean,
) {
    val client = LocalZenithClient.current

    val info by produceState<VideoInfo?>(initialValue = null, id) {
        value = client.getVideoInfo(id)
    }

    if (info == null) return

    val context = LocalContext.current
    val castSession = remember {
        CastContext.getSharedInstance(context)
            .sessionManager
            .currentCastSession
    }

    info?.let {
        if (castSession != null && castSession.isConnected) {
            RemotePlayer(id = id,
                title = title,
                type = type,
                backdrop = backdrop,
                info = it,
                session = castSession)
        } else {
            LocalPlayer(id = id, title = title, info = it, playFromStart = playFromStart)
        }
    }
}

@Composable fun LifecycleObserver(onEvent: (owner: LifecycleOwner, event: Lifecycle.Event) -> Unit) {
    val owner = LocalLifecycleOwner.current
    val observer = remember(onEvent) {
        LifecycleEventObserver(onEvent)
    }

    DisposableEffect(owner, observer) {
        val lifecycle = owner.lifecycle
        lifecycle.addObserver(observer)
        onDispose {
            lifecycle.removeObserver(observer)
        }
    }
}

@OptIn(ExperimentalMaterialApi::class)
@Composable
private fun RemotePlayer(
    id: Int,
    title: String,
    type: MediaItemType,
    backdrop: String?,
    info: VideoInfo,
    session: CastSession,
) {
    val context = LocalContext.current
    val client = LocalZenithClient.current
    val navigator = LocalNavigator.current
    val mediaClient = session.remoteMediaClient!!

    var position by remember { mutableStateOf(0L) }
    var isPlaying by remember { mutableStateOf(true) }

    val callback = remember {
        object : RemoteMediaClient.Callback() {
            override fun onStatusUpdated() {
                isPlaying = mediaClient.isPlaying
            }
        }
    }

    val progressListener = remember {
        RemoteMediaClient.ProgressListener { progress, _ ->
            position = progress / 1000
        }
    }

    DisposableEffect(Unit) {
        val metadata = MediaMetadata(type.toCastMediaType()).apply {
            putString(MediaMetadata.KEY_TITLE, title)
        }

        val subtitleTracks = info.subtitles
            .map {
                MediaTrack.Builder(it.index.toLong(), MediaTrack.TYPE_TEXT)
                    .setName(it.title ?: it.language)
                    .setSubtype(MediaTrack.SUBTYPE_SUBTITLES)
                    .setContentId(client.getSubtitleUrl(id, it.index))
                    .setContentType("text/vtt")
                    .setLanguage(it.language)
                    .build()
            }

        val mediaInfo = MediaInfo.Builder(client.getVideoUrl(id))
            .setStreamType(MediaInfo.STREAM_TYPE_BUFFERED)
            .setContentType("video/mp4")
            .setMetadata(metadata)
            .setMediaTracks(subtitleTracks)
            .build()

        val request = MediaLoadRequestData.Builder()
            .setMediaInfo(mediaInfo)
            .setAutoplay(true)
            .setActiveTrackIds(longArrayOf())
            .build()

        mediaClient.registerCallback(callback)
        mediaClient.addProgressListener(progressListener, 500)
        mediaClient.load(request)

        onDispose {
            mediaClient.unregisterCallback(callback)
            mediaClient.removeProgressListener(progressListener)
        }
    }

    LifecycleObserver { _, event ->
        if (event == Lifecycle.Event.ON_PAUSE) {
            mediaClient.unregisterCallback(callback)
            mediaClient.removeProgressListener(progressListener)
        } else if (event == Lifecycle.Event.ON_RESUME) {
            mediaClient.registerCallback(callback)
            mediaClient.addProgressListener(progressListener, 500)
        }
    }

    var showSubtitlesMenu by remember { mutableStateOf(false) }

    val onSelectSubtitle = { subtitle: SubtitleStreamInfo? ->
        val tracks = if (subtitle == null) {
            longArrayOf()
        } else {
            longArrayOf(subtitle.index.toLong())
        }

        mediaClient.setActiveMediaTracks(tracks)
            .setResultCallback { result ->
                Toast.makeText(context, result.status.statusMessage, Toast.LENGTH_SHORT)
                    .show()
            }

        mediaClient.setTextTrackStyle(TextTrackStyle().apply {
            backgroundColor = Color.Black.copy(alpha = 0.05f).toArgb()
            foregroundColor = Color.White.toArgb()
            edgeType = TextTrackStyle.EDGE_TYPE_OUTLINE
            edgeColor = Color.Black.toArgb()
            windowColor = Color.Blue.toArgb()
        })

        Unit
    }

    if (showSubtitlesMenu) {
        SubtitlesMenu(
            subtitles = info.subtitles,
            onSelectItem = onSelectSubtitle,
            onDismiss = { showSubtitlesMenu = false },
        )
    }

    CompositionLocalProvider(
        LocalContentColor provides Color.White,
    ) {
        Box(modifier = Modifier.fillMaxSize()) {
            Image(
                rememberImagePainter(backdrop, builder = { crossfade(true) }),
                contentDescription = "Backdrop",
                contentScale = ContentScale.Crop,
                modifier = Modifier.fillMaxSize(),
            )

            Box(
                modifier = Modifier
                    .fillMaxSize()
                    .background(Color.Black.copy(alpha = 0.4f)),
            )

            AppBar(
                title = title,
                onBackPressed = { navigator.pop() },
                onShowSubtitlesMenu = { showSubtitlesMenu = true },
                onClosePlayer = {
                    mediaClient.stop()
                    navigator.pop()
                },
                modifier = Modifier.statusBarsPadding(),
            )

            Row(
                verticalAlignment = Alignment.CenterVertically,
                modifier = Modifier
                    .align(Alignment.Center)
                    .pointerInput(Unit) { detectTapGestures { /* Consume tap events */ } },
            ) {
                SeekButton(Icons.Default.Replay10) {
                    mediaClient.seek(MediaSeekOptions.Builder()
                        .setPosition(maxOf(0, position - 10) * 1000)
                        .build())
                }

                PlayPauseButton(
                    isPlaying = isPlaying,
                    onClick = {
                        mediaClient.togglePlayback()
                    },
                )

                SeekButton(Icons.Default.Forward30) {
                    mediaClient.seek(MediaSeekOptions.Builder()
                        .setPosition(minOf(info.duration.toLong(), position + 30) * 1000)
                        .build())
                }
            }

            SeekBar(
                position = position,
                duration = info.duration.toLong(),
                onSeekStart = {
                    mediaClient.pause()
                },
                onSeekEnd = {
                    mediaClient.seek(MediaSeekOptions.Builder()
                        .setPosition(it * 1000)
                        .setResumeState(MediaSeekOptions.RESUME_STATE_PLAY)
                        .build())
                },
                modifier = Modifier.align(Alignment.BottomCenter),
            )
        }
    }
}

@Composable
private fun AppBar(
    title: String,
    onBackPressed: () -> Unit,
    onShowSubtitlesMenu: () -> Unit,
    modifier: Modifier = Modifier,
    onClosePlayer: (() -> Unit)? = null,
) {
    val context = LocalContext.current

    TopAppBar(
        navigationIcon = {
            IconButton(onClick = {
                context.playClick()
                onBackPressed()
            }) {
                Icon(Icons.Default.ArrowBack, contentDescription = "Back")
            }
        },
        title = {
            Text(
                text = title,
                maxLines = 1,
                overflow = TextOverflow.Ellipsis,
            )
        },
        backgroundColor = Color.Transparent,
        elevation = 0.dp,
        actions = {
            IconButton(
                onClick = {
                    context.playClick()
                    onShowSubtitlesMenu()
                },
            ) {
                Icon(Icons.Default.ClosedCaption, contentDescription = "Captions")
            }

            if (onClosePlayer != null) {
                IconButton(onClick = {
                    context.playClick()
                    onClosePlayer()
                }) {
                    Icon(Icons.Default.Close, contentDescription = "Close")
                }
            }
        },
        modifier = modifier,
    )
}

@OptIn(ExperimentalMaterialApi::class)
@Composable
private fun SubtitlesMenu(
    subtitles: List<SubtitleStreamInfo>,
    onSelectItem: (SubtitleStreamInfo?) -> Unit,
    onDismiss: () -> Unit,
) {
    val context = LocalContext.current

    AlertDialog(
        onDismissRequest = onDismiss,
        text = {
            Column {
                Text("Subtitles")
                Spacer(modifier = Modifier.height(16.dp))
                LazyColumn(
                    modifier = Modifier
                        .fillMaxWidth()
                        .heightIn(max = 400.dp),
                ) {
                    item {
                        ListItem(
                            text = { Text("None") },
                            modifier = Modifier.clickable {
                                context.playClick()
                                onSelectItem(null)
                                onDismiss()
                            },
                        )
                    }

                    items(subtitles) {
                        ListItem(
                            text = {
                                val label = it.title
                                    ?: it.language
                                    ?: "Track ${it.index}"

                                Text(label)
                            },
                            modifier = Modifier.clickable {
                                context.playClick()
                                onSelectItem(it)
                                onDismiss()
                            },
                        )
                    }
                }
            }
        },
        confirmButton = {
            TextButton(
                onClick = {
                    context.playClick()
                    onDismiss()
                },
            ) {
                Text("Close")
            }
        },
    )
}

@Composable
private fun LocalPlayer(id: Int, title: String, info: VideoInfo, playFromStart: Boolean) {
    val client = LocalZenithClient.current
    val navigator = LocalNavigator.current

    KeepScreenOn {
        FullScreen {
            val startPosition = if (playFromStart) 0 else {
                info.position?.toLong() ?: 0
            }

            VideoPlayer(
                id = id,
                title = title,
                client = client,
                startPosition = startPosition,
                duration = info.duration.toLong(),
                onVideoEnded = { navigator.pop() },
            )
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
    startPosition: Long,
    duration: Long,
    onVideoEnded: () -> Unit,
) {
    val context = LocalContext.current
    val scope = rememberCoroutineScope()

    val session = remember { MediaSessionCompat(context, context.packageName) }
    val connector = remember { MediaSessionConnector(session) }

    var position by remember { mutableStateOf(0L) }
    var isPlaying by remember { mutableStateOf(true) }

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
        player.seekTo(startPosition * 1000)
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
            },
            onShowSubtitlesMenu = {
                // TODO
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
    onShowSubtitlesMenu: () -> Unit,
) {
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
                AppBar(
                    title = title,
                    onBackPressed = { navigator.pop() },
                    onShowSubtitlesMenu = onShowSubtitlesMenu,
                )
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

                    PlayPauseButton(isPlaying = isPlaying, onClick = {
                        controls.showAndHideDelayed()
                        onTogglePlaying()
                    })

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
private fun PlayPauseButton(
    isPlaying: Boolean,
    onClick: () -> Unit,
    modifier: Modifier = Modifier,
) {
    val context = LocalContext.current

    Box(
        modifier = modifier
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
