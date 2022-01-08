package uk.hasali.zenith.screens.library.itemdetails

import android.content.Context
import android.net.Uri
import android.provider.OpenableColumns
import android.text.format.DateUtils
import android.widget.Toast
import androidx.activity.compose.rememberLauncherForActivityResult
import androidx.activity.result.contract.ActivityResultContracts
import androidx.compose.animation.animateColorAsState
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import kotlinx.coroutines.launch
import uk.hasali.zenith.LanguageCodes
import uk.hasali.zenith.api.SubtitleInfo
import uk.hasali.zenith.api.VideoInfo
import uk.hasali.zenith.api.VideoUserData
import uk.hasali.zenith.ui.*
import kotlin.io.path.Path
import kotlin.io.path.extension

private fun Context.getFileName(uri: Uri): String? {
    if (!uri.scheme.equals("content")) {
        return null
    }

    return contentResolver
        .query(uri, null, null, null, null)
        .use { cursor ->
            var result: String? = null
            if (cursor != null && cursor.moveToFirst()) {
                val index = cursor.getColumnIndex(OpenableColumns.DISPLAY_NAME)
                if (index > -1) {
                    result = cursor.getString(index)
                }
            }
            result
        }
}

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
    onImportSubtitle: (String, ByteArray) -> Unit,
    onNavigateUp: () -> Unit,
) {
    val context = LocalContext.current
    val scope = rememberCoroutineScope()
    val position = userData.position ?: 0.0
    var isWatched by remember(userData) { mutableStateOf(userData.isWatched) }

    val subtitlePickerLauncher =
        rememberLauncherForActivityResult(ActivityResultContracts.GetContent()) {
            if (it != null) {
                val filename = context.getFileName(it)
                if (filename != null) {
                    val extension = Path(filename).extension
                    if (extension != "srt" && extension != "vtt") {
                        Toast.makeText(
                            context,
                            "Unsupported subtitle extension: $extension",
                            Toast.LENGTH_SHORT
                        )
                            .show()
                    } else {
                        val content = context.contentResolver.openInputStream(it).use { stream ->
                            stream?.readBytes()
                        }

                        if (content != null) {
                            onImportSubtitle(filename, content)
                        }
                    }
                }
            }
        }

    ItemDetailsScreen(
        backdrop = backdrop,
        poster = {
            Poster(
                url = poster,
                overlay = { if (position > 0.05 * info.duration) VideoPositionOverlay(position) },
            )
        },
        appBarActions = {
            CastButton()
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
                                subtitles = info.subtitles.orEmpty(),
                                onConvertVideo = onConvertVideo,
                                onRefreshMetadata = onRefreshMetadata,
                                onImportSubtitle = {
                                    subtitlePickerLauncher.launch("*/*")
                                },
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
        val resume = position != null && position > 0.05 * duration && position < 0.9 * duration

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

enum class ActionsSheetView {
    Main,
    Subtitles,
}

private data class ActionsSheetContent(
    val title: String,
    val subtitles: List<SubtitleInfo>,
    val onConvertVideo: () -> Unit,
    val onRefreshMetadata: () -> Unit,
    val onImportSubtitle: () -> Unit,
) : BottomSheetContent {
    @OptIn(ExperimentalMaterialApi::class)
    @Composable
    override fun BottomSheetContentScope.Content() {
        var view by remember { mutableStateOf(ActionsSheetView.Main) }

        Row(verticalAlignment = Alignment.CenterVertically) {
            if (view == ActionsSheetView.Subtitles) {
                IconButton(onClick = { view = ActionsSheetView.Main }) {
                    Icon(Icons.Default.ArrowBack, null)
                }
            }

            Text(
                text = title,
                maxLines = 1,
                overflow = TextOverflow.Ellipsis,
                style = MaterialTheme.typography.subtitle2,
                modifier = Modifier.padding(16.dp),
            )
        }

        Divider()

        when (view) {
            ActionsSheetView.Main -> {
                ListItem(modifier = Modifier.clickable {
                    hide()
                    onConvertVideo()
                }) {
                    Text("Convert video")
                }

                ListItem(modifier = Modifier.clickable {
                    hide()
                    onRefreshMetadata()
                }) {
                    Text("Refresh metadata")
                }

                ListItem(modifier = Modifier.clickable {
                    view = ActionsSheetView.Subtitles
                }) {
                    Text("Subtitles")
                }
            }

            ActionsSheetView.Subtitles -> {
                SubtitlesList(
                    subtitles = subtitles,
                    onImportClick = {
                        hide()
                        onImportSubtitle()
                    },
                )
            }
        }
    }
}

private data class SubtitleItem(
    val language: String,
    val title: String?,
    val info: SubtitleInfo,
)

@OptIn(ExperimentalMaterialApi::class)
@Composable
private fun SubtitlesList(
    subtitles: List<SubtitleInfo>,
    onImportClick: () -> Unit,
) {
    val items = remember(subtitles) {
        subtitles
            .map {
                SubtitleItem(
                    language = LanguageCodes.getDisplayNameForCode(it.language, "Unknown"),
                    title = it.title,
                    info = it,
                )
            }
            .sortedWith(
                compareBy(
                    { it.language },
                    { it.title },
                    { it.info.id },
                ),
            )
    }

    LazyColumn {
        items(items) {
            ListItem(secondaryText = if (it.title != null) ({ Text(it.title) }) else null) {
                Text(it.language)
            }
        }

        item {
            ListItem(modifier = Modifier.clickable(onClick = onImportClick)) {
                Text("Import File")
            }
        }
    }
}
