package uk.hasali.zenith.screens.player

import android.net.Uri
import android.widget.Toast
import androidx.compose.animation.ExperimentalAnimationApi
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material.ExperimentalMaterialApi
import androidx.compose.material.Surface
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.viewinterop.AndroidView
import androidx.media3.common.*
import androidx.media3.common.util.UnstableApi
import androidx.media3.datasource.DefaultHttpDataSource
import androidx.media3.exoplayer.ExoPlayer
import androidx.media3.exoplayer.source.DefaultMediaSourceFactory
import androidx.media3.exoplayer.source.MergingMediaSource
import androidx.media3.exoplayer.source.SingleSampleMediaSource
import androidx.media3.exoplayer.trackselection.DefaultTrackSelector
import androidx.media3.session.MediaSession
import androidx.media3.ui.PlayerView
import kotlinx.coroutines.delay
import kotlinx.coroutines.launch

@Composable
fun LocalPlayer(
    item: VideoItem,
    onVideoProgress: (Long) -> Unit,
    onLaunchExternal: () -> Unit,
    onNavigateUp: () -> Unit,
) {
    KeepScreenOn {
        FullScreen {
            VideoPlayer(
                item = item,
                onVideoProgress = onVideoProgress,
                onVideoEnded = onNavigateUp,
                onLaunchExternal = onLaunchExternal,
                onBackPressed = onNavigateUp,
            )
        }
    }
}

@androidx.annotation.OptIn(UnstableApi::class)
@OptIn(ExperimentalAnimationApi::class, ExperimentalMaterialApi::class)
@Composable
private fun VideoPlayer(
    item: VideoItem,
    onVideoProgress: (Long) -> Unit,
    onVideoEnded: () -> Unit,
    onLaunchExternal: () -> Unit,
    onBackPressed: () -> Unit,
) {
    val context = LocalContext.current
    val scope = rememberCoroutineScope()

    var position by remember { mutableStateOf(0L) }
    var isPlaying by remember { mutableStateOf(true) }

    var selectedSubtitle by remember { mutableStateOf<SubtitleTrack?>(null) }

    val selector = remember { DefaultTrackSelector(context) }
    var textRenderer: Int? by remember { mutableStateOf(null) }

    fun onSelectSubtitle(subtitle: SubtitleTrack?) {
        val renderer = textRenderer ?: return

        if (subtitle == null) {
            selectedSubtitle = null
            selector.parameters = selector.buildUponParameters()
                .setRendererDisabled(renderer, true)
                .build()
        } else {
            val trackId = when (subtitle) {
                is SubtitleTrack.External -> "external:${subtitle.id}"
                is SubtitleTrack.Embedded -> (subtitle.index + 1).toString()
            }

            val mappedTrackInfo = selector.currentMappedTrackInfo ?: return
            val trackGroups = mappedTrackInfo.getTrackGroups(renderer)

            var group: TrackGroup? = null
            var track: Int? = null
            outer@for (i in 0 until trackGroups.length) {
                group = trackGroups[i]
                for (j in 0 until group.length) {
                    val format = group.getFormat(j)
                    if (format.id == trackId) {
                        track = j
                        break@outer
                    }
                }
            }

            if (group == null || track == null) {
                val toast = Toast.makeText(
                    context,
                    "Failed to find requested subtitle track",
                    Toast.LENGTH_SHORT
                )
                return toast.show()
            }

            val overrides = TrackSelectionOverrides.Builder()
                .setOverrideForType(TrackSelectionOverrides.TrackSelectionOverride(group, listOf(track)))
                .build()

            selector.parameters = selector.buildUponParameters()
                .setRendererDisabled(renderer, false)
                .setTrackSelectionOverrides(overrides)
                .build()

            selectedSubtitle = subtitle
        }
    }

    val player = remember {
        ExoPlayer.Builder(context)
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

    val session = remember {
        MediaSession.Builder(context, player)
            .build()
    }

    DisposableEffect(item) {
        val mediaItem = MediaItem.fromUri(item.url)

        val dataSourceFactory = DefaultHttpDataSource.Factory()
        val sources = mutableListOf(
            DefaultMediaSourceFactory(context)
                .createMediaSource(mediaItem)
        )

        item.subtitles.filterIsInstance<SubtitleTrack.External>()
            .forEach {
                val uri = Uri.parse(it.url)

                val subtitle = MediaItem.SubtitleConfiguration.Builder(uri)
                    .setMimeType(MimeTypes.TEXT_VTT)
                    .setLanguage(it.language)
                    .setLabel(it.title)
                    .build()

                val source = SingleSampleMediaSource.Factory(dataSourceFactory)
                    .setTrackId("external:${it.id}")
                    .createMediaSource(subtitle, C.TIME_UNSET)

                sources.add(source)
            }

        val mergedSource = MergingMediaSource(*sources.toTypedArray())

        player.setMediaSource(mergedSource, item.startPosition.toLong() * 1000)
        player.prepare()
        player.play()

        onDispose {
            player.stop()
        }
    }

    DisposableEffect(Unit) {
        onDispose {
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
            title = item.title,
            position = position,
            duration = item.duration.toLong(),
            isPlaying = isPlaying,
            subtitles = item.subtitles,
            selectedSubtitle = selectedSubtitle,
            onSeekStart = { player.pause() },
            onSeekEnd = {
                player.seekTo(it * 1000)
                player.play()
            },
            onTogglePlaying = { player.playWhenReady = !isPlaying },
            onSelectSubtitle = { onSelectSubtitle(it) },
            onLaunchExternal = onLaunchExternal,
            onBackPressed = onBackPressed,
        )
    }
}
