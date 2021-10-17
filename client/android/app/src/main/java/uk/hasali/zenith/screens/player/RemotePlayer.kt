package uk.hasali.zenith.screens.player

import android.widget.Toast
import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.BoxScope
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Forward30
import androidx.compose.material.icons.filled.Replay10
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.toArgb
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.platform.LocalContext
import androidx.lifecycle.Lifecycle
import coil.annotation.ExperimentalCoilApi
import coil.compose.rememberImagePainter
import com.google.android.gms.cast.*
import com.google.android.gms.cast.framework.CastSession
import com.google.android.gms.cast.framework.media.RemoteMediaClient
import kotlinx.coroutines.launch
import uk.hasali.zenith.ui.LifecycleObserver

private fun VideoItemType.toCastMediaType() = when (this) {
    VideoItemType.Movie -> MediaMetadata.MEDIA_TYPE_MOVIE
    VideoItemType.TvShow -> MediaMetadata.MEDIA_TYPE_TV_SHOW
}

@OptIn(ExperimentalCoilApi::class, ExperimentalMaterialApi::class)
@Composable
fun RemotePlayer(
    item: VideoItem,
    session: CastSession,
    onNavigateUp: () -> Unit,
) {
    val context = LocalContext.current
    val scope = rememberCoroutineScope()
    val mediaClient = session.remoteMediaClient!!

    var position by remember { mutableStateOf(0L) }
    var isPlaying by remember { mutableStateOf(true) }
    var selectedSubtitle by remember { mutableStateOf<SubtitleTrack?>(null) }

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

    DisposableEffect(item) {
        val metadata = MediaMetadata(item.type.toCastMediaType()).apply {
            putString(MediaMetadata.KEY_TITLE, item.title)
        }

        val subtitleTracks = item.subtitles
            .map {
                MediaTrack.Builder(it.id.toLong(), MediaTrack.TYPE_TEXT)
                    .setName(it.title ?: it.language)
                    .setSubtype(MediaTrack.SUBTYPE_SUBTITLES)
                    .setContentId(it.url)
                    .setContentType("text/vtt")
                    .setLanguage(it.language)
                    .build()
            }

        val mediaInfo = MediaInfo.Builder(item.url)
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

    val onSelectSubtitle = { subtitle: SubtitleTrack? ->
        val tracks = if (subtitle == null) {
            longArrayOf()
        } else {
            longArrayOf(subtitle.id.toLong())
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

        selectedSubtitle = subtitle
    }

    val sheetState = rememberModalBottomSheetState(ModalBottomSheetValue.Hidden)

    ModalBottomSheetLayout(
        sheetState = sheetState,
        scrimColor = MaterialTheme.colors.surface.copy(alpha = 0.32f),
        sheetContent = {
            SubtitlesMenu(
                subtitles = item.subtitles.orEmpty(),
                current = selectedSubtitle,
                onSelectSubtitle = {
                    onSelectSubtitle(it)
                    scope.launch {
                        sheetState.hide()
                    }
                },
            )
        },
    ) {
        CompositionLocalProvider(LocalContentColor provides Color.White) {
            Box(modifier = Modifier.fillMaxSize()) {
                Image(
                    rememberImagePainter(item.backdrop) { crossfade(true) },
                    contentDescription = "Backdrop",
                    contentScale = ContentScale.Crop,
                    modifier = Modifier.fillMaxSize(),
                )

                Box(
                    modifier = Modifier
                        .fillMaxSize()
                        .background(Color.Black.copy(alpha = 0.4f)),
                )

                Controls(
                    title = item.title,
                    position = position,
                    duration = item.duration.toLong(),
                    isPlaying = isPlaying,
                    onTogglePlaying = { mediaClient.togglePlayback() },
                    onSeekStart = { mediaClient.pause() },
                    onSeekEnd = { position, resume ->
                        mediaClient.seek(
                            MediaSeekOptions.Builder()
                                .setPosition(position)
                                .setResumeState(
                                    if (resume)
                                        MediaSeekOptions.RESUME_STATE_PLAY
                                    else
                                        MediaSeekOptions.RESUME_STATE_UNCHANGED
                                )
                                .build()
                        )
                    },
                    onNavigateUp = onNavigateUp,
                    onShowSubtitlesMenu = { scope.launch { sheetState.show() } },
                    onClosePlayer = {
                        mediaClient.stop()
                        onNavigateUp()
                    },
                )
            }
        }
    }
}

@Composable
private fun BoxScope.Controls(
    title: String,
    position: Long,
    duration: Long,
    isPlaying: Boolean,
    onTogglePlaying: () -> Unit,
    onSeekStart: () -> Unit,
    onSeekEnd: (Long, Boolean) -> Unit,
    onNavigateUp: () -> Unit,
    onShowSubtitlesMenu: () -> Unit,
    onClosePlayer: () -> Unit,
) {
    AppBar(
        title = title,
        onBackPressed = onNavigateUp,
        onShowSubtitlesMenu = onShowSubtitlesMenu,
        onClosePlayer = onClosePlayer,
    )

    Row(
        verticalAlignment = Alignment.CenterVertically,
        modifier = Modifier.align(Alignment.Center),
    ) {
        SeekButton(Icons.Default.Replay10) {
            onSeekEnd(maxOf(0, position - 10) * 1000, false)
        }

        PlayPauseButton(
            isPlaying = isPlaying,
            onClick = onTogglePlaying,
        )

        SeekButton(Icons.Default.Forward30) {
            onSeekEnd(minOf(duration, position + 30) * 1000, false)
        }
    }

    SeekBar(
        position = position,
        duration = duration,
        onSeekStart = onSeekStart,
        onSeekEnd = { onSeekEnd(it * 1000, true) },
        modifier = Modifier.align(Alignment.BottomCenter),
    )
}
