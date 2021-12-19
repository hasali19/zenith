package uk.hasali.zenith.ui

import android.text.format.DateUtils
import androidx.compose.animation.animateColorAsState
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.CheckCircleOutline
import androidx.compose.material.icons.filled.HourglassBottom
import androidx.compose.material.icons.filled.Info
import androidx.compose.material.icons.filled.MoreVert
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import kotlinx.coroutines.launch
import uk.hasali.zenith.VideoInfo
import uk.hasali.zenith.VideoUserData

@Composable
fun VideoItemDetailsScreen(
    name: String,
    backdrop: String?,
    poster: String?,
    overview: String?,
    headerContent: @Composable () -> Unit,
    info: VideoInfo,
    userData: VideoUserData,
    bottomSheetController: BottomSheetController,
    onSetWatched: (Boolean) -> Unit,
    onPlay: (position: Double?) -> Unit,
    onConvertVideo: () -> Unit,
    onRefreshMetadata: () -> Unit,
    onNavigateUp: () -> Unit,
) {
    val scope = rememberCoroutineScope()
    val position = userData.position ?: 0.0
    var isWatched by remember(userData) { mutableStateOf(userData.isWatched) }

    ItemDetailsScreen(
        backdrop = backdrop,
        poster = {
            Poster(
                url = poster,
                overlay = { if (position > 0) VideoPositionOverlay(position) },
            )
        },
        headerContent = headerContent,
        actionsRow = {
            ActionsSection(
                duration = info.duration,
                position = userData.position,
                isWatched = isWatched,
                onPlay = onPlay,
                onSetWatched = {
                    isWatched = it
                    onSetWatched(it)
                },
                onShowMediaInfo = {
                    scope.launch {
                        bottomSheetController.show(MediaInfoSheetContent(info))
                    }
                },
                onShowMoreActions = {
                    scope.launch {
                        bottomSheetController.show(
                            ActionsSheetContent(
                                title = name,
                                onConvertVideo = onConvertVideo,
                                onRefreshMetadata = onRefreshMetadata,
                            )
                        )
                    }
                }
            )
        },
        overview = overview,
        isWatched = isWatched,
        onNavigateUp = onNavigateUp,
    )
}

@Composable
private fun VideoPositionOverlay(position: Double) {
    Box {
        Row(
            horizontalArrangement = Arrangement.Center,
            verticalAlignment = Alignment.CenterVertically,
            modifier = Modifier
                .fillMaxWidth()
                .align(Alignment.BottomCenter)
                .background(Color.Black.copy(alpha = 0.5f))
                .padding(vertical = 4.dp),
        ) {
            Icon(Icons.Default.HourglassBottom, null, modifier = Modifier.size(12.dp))
            Spacer(modifier = Modifier.width(4.dp))
            Text(
                text = DateUtils.formatElapsedTime(position.toLong()),
                textAlign = TextAlign.Center,
                style = MaterialTheme.typography.caption,
            )
        }
    }
}

@Composable
private fun ActionsSection(
    duration: Double,
    position: Double?,
    isWatched: Boolean,
    onPlay: (position: Double?) -> Unit,
    onSetWatched: (Boolean) -> Unit,
    onShowMediaInfo: () -> Unit,
    onShowMoreActions: () -> Unit,
) {
    Row(verticalAlignment = Alignment.CenterVertically) {
        val resume = position != null && position > 0 && position < 0.9 * duration

        PlayButton(
            resume = resume,
            onClick = { onPlay(if (resume) position else null) },
        )

        Spacer(modifier = Modifier.width(8.dp))

        Row(horizontalArrangement = Arrangement.End, modifier = Modifier.weight(1f)) {
            IconToggleButton(checked = isWatched, onCheckedChange = onSetWatched) {
                val tint by animateColorAsState(
                    if (isWatched) {
                        MaterialTheme.colors.secondary
                    } else {
                        LocalContentColor.current
                    }
                )

                Icon(Icons.Default.CheckCircleOutline, null, tint = tint)
            }

            IconButton(onClick = onShowMediaInfo) {
                Icon(Icons.Default.Info, contentDescription = "Media info")
            }

            IconButton(onClick = onShowMoreActions) {
                Icon(Icons.Default.MoreVert, contentDescription = "More")
            }
        }
    }
}

private data class MediaInfoSheetContent(val info: VideoInfo) : BottomSheetContent {
    @Composable
    override fun BottomSheetContentScope.Content() {
        Row(
            verticalAlignment = Alignment.CenterVertically,
            modifier = Modifier.padding(16.dp),
        ) {
            Icon(Icons.Default.Info, null)
            Spacer(modifier = Modifier.width(8.dp))
            Text(
                text = "Media Info",
                style = MaterialTheme.typography.subtitle2,
            )
        }

        Divider()

        DescriptionList(
            modifier = Modifier
                .padding(16.dp)
                .verticalScroll(rememberScrollState()),
        ) {
            if (info.format != null) {
                entry("Format", info.format)
            }

            entry("Path", info.path)

            if (info.video != null) {
                heading("Video (#${info.video.index})")
                entry("Codec", info.video.codec)
                entry("Resolution", "${info.video.width}x${info.video.height}")
            }

            for (stream in info.audio.orEmpty()) {
                heading("Audio (#${stream.index})")
                entry("Codec", stream.codec)
                entry("Language", stream.language ?: "Unknown")
            }
        }
    }
}

private data class ActionsSheetContent(
    val title: String,
    val onConvertVideo: () -> Unit,
    val onRefreshMetadata: () -> Unit,
) : BottomSheetContent {
    @OptIn(ExperimentalMaterialApi::class)
    @Composable
    override fun BottomSheetContentScope.Content() {
        Text(
            text = title,
            maxLines = 1,
            overflow = TextOverflow.Ellipsis,
            style = MaterialTheme.typography.subtitle2,
            modifier = Modifier.padding(16.dp),
        )

        Divider()

        ListItem(modifier = Modifier.clickable {
            hide()
            onConvertVideo()
        }) {
            Text("Convert video")
        }

        ListItem(modifier = Modifier.clickable {
            hide()
            onConvertVideo()
        }) {
            Text("Refresh metadata")
        }
    }
}
