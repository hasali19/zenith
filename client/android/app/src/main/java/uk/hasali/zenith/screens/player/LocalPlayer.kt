package uk.hasali.zenith.screens.player

import android.net.Uri
import android.support.v4.media.session.MediaSessionCompat
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
import com.google.android.exoplayer2.*
import com.google.android.exoplayer2.ext.mediasession.MediaSessionConnector
import com.google.android.exoplayer2.source.DefaultMediaSourceFactory
import com.google.android.exoplayer2.source.MergingMediaSource
import com.google.android.exoplayer2.source.SingleSampleMediaSource
import com.google.android.exoplayer2.trackselection.DefaultTrackSelector
import com.google.android.exoplayer2.ui.PlayerView
import com.google.android.exoplayer2.upstream.DefaultHttpDataSource
import com.google.android.exoplayer2.util.MimeTypes
import kotlinx.coroutines.delay
import kotlinx.coroutines.launch
import uk.hasali.zenith.VideoInfo
import uk.hasali.zenith.SubtitleStreamInfo
import uk.hasali.zenith.VideoUserData
import uk.hasali.zenith.ui.LocalZenithClient

@Composable
fun LocalPlayer(
    url: String,
    title: String,
    info: VideoInfo,
    userData: VideoUserData,
    replay: Boolean,
    onVideoProgress: (Long) -> Unit,
    onLaunchExternal: () -> Unit,
    onNavigateUp: () -> Unit,
) {
    KeepScreenOn {
        FullScreen {
            val startPosition = if (replay) 0 else {
                userData.position?.toLong() ?: 0
            }

            VideoPlayer(
                url = url,
                title = title,
                startPosition = startPosition,
                duration = info.duration.toLong(),
                subtitles = info.subtitles.orEmpty(),
                onVideoProgress = onVideoProgress,
                onVideoEnded = onNavigateUp,
                onLaunchExternal = onLaunchExternal,
                onBackPressed = onNavigateUp,
            )
        }
    }
}

@OptIn(ExperimentalAnimationApi::class, ExperimentalMaterialApi::class)
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
    val client = LocalZenithClient.current
    val scope = rememberCoroutineScope()

    val session = remember { MediaSessionCompat(context, context.packageName) }
    val connector = remember { MediaSessionConnector(session) }

    var position by remember { mutableStateOf(0L) }
    var isPlaying by remember { mutableStateOf(true) }

    var selectedSubtitle by remember { mutableStateOf<SubtitleStreamInfo?>(null) }

    val selector = remember { DefaultTrackSelector(context) }
    var textRenderer: Int? by remember { mutableStateOf(null) }

    fun onSelectSubtitle(subtitle: SubtitleStreamInfo?) {
        val renderer = textRenderer ?: return

        if (subtitle == null) {
            selectedSubtitle = null
            selector.parameters = selector.buildUponParameters()
                .setRendererDisabled(renderer, true)
                .build()
        } else {
            val trackId = when (subtitle) {
                is SubtitleStreamInfo.External -> "external:${subtitle.id}"
                is SubtitleStreamInfo.Embedded -> (subtitle.index + 1).toString()
            }

            val mappedTrackInfo = selector.currentMappedTrackInfo ?: return
            val trackGroups = mappedTrackInfo.getTrackGroups(renderer)

            var track: Pair<Int, Int>? = null
            for (i in 0 until trackGroups.length) {
                val group = trackGroups[i]
                for (j in 0 until group.length) {
                    val format = group.getFormat(j)
                    if (format.id == trackId) {
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

            selectedSubtitle = subtitle
        }
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

        val dataSourceFactory = DefaultHttpDataSource.Factory()
        val sources = mutableListOf(
            DefaultMediaSourceFactory(context)
                .createMediaSource(item)
        )

        subtitles.filterIsInstance<SubtitleStreamInfo.External>()
            .forEach {
                val uri = Uri.parse(client.getSubtitleUrl(it.id))
                val subtitle = MediaItem.Subtitle(uri, MimeTypes.APPLICATION_SUBRIP, null)

                val source = SingleSampleMediaSource.Factory(dataSourceFactory)
                    .setTrackId("external:${it.id}")
                    .createMediaSource(subtitle, C.TIME_UNSET)

                sources.add(source)
            }

        val mergedSource = MergingMediaSource(*sources.toTypedArray())

        player.setMediaSource(mergedSource, startPosition * 1000)
        player.prepare()
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
            subtitles = subtitles,
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
