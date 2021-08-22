package uk.hasali.zenith.screens.player

import android.support.v4.media.session.MediaSessionCompat
import android.widget.Toast
import androidx.compose.animation.ExperimentalAnimationApi
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material.Surface
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.viewinterop.AndroidView
import com.google.android.exoplayer2.*
import com.google.android.exoplayer2.ext.mediasession.MediaSessionConnector
import com.google.android.exoplayer2.trackselection.DefaultTrackSelector
import com.google.android.exoplayer2.ui.PlayerView
import kotlinx.coroutines.delay
import kotlinx.coroutines.launch
import uk.hasali.zenith.SubtitleStreamInfo
import uk.hasali.zenith.VideoInfo

@Composable
fun LocalPlayer(
    url: String,
    title: String,
    info: VideoInfo,
    replay: Boolean,
    onVideoProgress: (Long) -> Unit,
    onLaunchExternal: () -> Unit,
    onNavigateUp: () -> Unit,
) {
    KeepScreenOn {
        FullScreen {
            val startPosition = if (replay) 0 else {
                info.position?.toLong() ?: 0
            }

            VideoPlayer(
                url = url,
                title = title,
                startPosition = startPosition,
                duration = info.duration.toLong(),
                subtitles = info.subtitles,
                onVideoProgress = onVideoProgress,
                onVideoEnded = onNavigateUp,
                onLaunchExternal = onLaunchExternal,
                onBackPressed = onNavigateUp,
            )
        }
    }
}

@OptIn(ExperimentalAnimationApi::class)
@Composable
private fun VideoPlayer(
    url: String,
    title: String,
    startPosition: Long,
    duration: Long,
    subtitles: List<SubtitleStreamInfo>,
    onVideoProgress: (Long) -> Unit,
    onVideoEnded: () -> Unit,
    onLaunchExternal: () -> Unit,
    onBackPressed: () -> Unit,
) {
    val context = LocalContext.current
    val scope = rememberCoroutineScope()

    val session = remember { MediaSessionCompat(context, context.packageName) }
    val connector = remember { MediaSessionConnector(session) }

    var position by remember { mutableStateOf(0L) }
    var isPlaying by remember { mutableStateOf(true) }

    var showSubtitlesMenu by remember { mutableStateOf(false) }

    val selector = remember { DefaultTrackSelector(context) }
    var textRenderer: Int? by remember { mutableStateOf(null) }

    fun onSelectSubtitle(subtitle: SubtitleStreamInfo?) {
        val renderer = textRenderer ?: return

        if (subtitle == null) {
            selector.parameters = selector.buildUponParameters()
                .setRendererDisabled(renderer, true)
                .build()
        } else {
            when (subtitle) {
                is SubtitleStreamInfo.External -> {
                    val toast = Toast.makeText(
                        context,
                        "External subtitles are not yet supported",
                        Toast.LENGTH_SHORT
                    )
                    return toast.show()
                }
                is SubtitleStreamInfo.Embedded -> {
                    val mappedTrackInfo = selector.currentMappedTrackInfo ?: return
                    val trackGroups = mappedTrackInfo.getTrackGroups(renderer)

                    var track: Pair<Int, Int>? = null
                    for (i in 0 until trackGroups.length) {
                        val group = trackGroups[i]
                        for (j in 0 until group.length) {
                            val format = group.getFormat(j)
                            // TODO: Investigate if there's a better way to find the right track
                            if (format.id?.toIntOrNull() == subtitle.index + 1) {
                                track = Pair(i, j)
                            }
                        }
                    }

                    if (track == null) {
                        val toast = Toast.makeText(
                            context,
                            "Failed to find requested subtitle track",
                            Toast.LENGTH_SHORT
                        )
                        return toast.show()
                    }

                    selector.parameters = selector.buildUponParameters()
                        .setRendererDisabled(renderer, false)
                        .setSelectionOverride(
                            renderer,
                            trackGroups,
                            DefaultTrackSelector.SelectionOverride(track.first, track.second),
                        )
                        .build()
                }
            }
        }
    }

    if (showSubtitlesMenu) {
        SubtitlesMenu(
            subtitles = subtitles,
            onSelectItem = { onSelectSubtitle(it) },
            onDismiss = { showSubtitlesMenu = false },
        )
    }

    val player = remember {
        SimpleExoPlayer.Builder(context)
            .setTrackSelector(selector)
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

                for (i in 0 until player.rendererCount) {
                    if (player.getRendererType(i) == C.TRACK_TYPE_TEXT) {
                        textRenderer = i
                        break
                    }
                }

                // Disable the text renderer initially
                textRenderer.let { textRenderer ->
                    if (textRenderer == null) {
                        Toast.makeText(context, "Missing text renderer", Toast.LENGTH_LONG)
                            .show()
                    } else {
                        selector.parameters = selector.buildUponParameters()
                            .setRendererDisabled(textRenderer, true)
                            .build()
                    }
                }

                scope.launch {
                    var counter = 0
                    while (true) {
                        counter += 1

                        if (player.playWhenReady) {
                            position = player.currentPosition / 1000
                            if (counter == 4) {
                                onVideoProgress(position)
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

    DisposableEffect(url) {
        val item = MediaItem.fromUri(url)

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
                showSubtitlesMenu = true
            },
            onLaunchExternal = onLaunchExternal,
            onBackPressed = onBackPressed,
        )
    }
}
