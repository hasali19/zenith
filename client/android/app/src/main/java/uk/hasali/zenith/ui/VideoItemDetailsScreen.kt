package uk.hasali.zenith.ui

import android.text.format.DateUtils
import androidx.compose.foundation.layout.*
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Info
import androidx.compose.material.icons.filled.MoreVert
import androidx.compose.material.icons.filled.PlayArrow
import androidx.compose.material.icons.filled.Replay
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import com.google.accompanist.insets.navigationBarsPadding
import kotlinx.coroutines.launch
import uk.hasali.zenith.VideoInfo
import uk.hasali.zenith.playClick

@OptIn(ExperimentalMaterialApi::class)
@Composable
fun VideoItemDetailsScreen(
    backdrop: String?,
    poster: String?,
    overview: String?,
    isWatched: Boolean,
    headerContent: @Composable () -> Unit,
    info: VideoInfo,
    onPlay: (replay: Boolean) -> Unit,
    onTranscode: () -> Unit,
    onRefreshMetadata: () -> Unit,
    onNavigateUp: () -> Unit,
) {
    val scope = rememberCoroutineScope()
    val sheetState = rememberModalBottomSheetState(ModalBottomSheetValue.Hidden)

    fun onActionInvoked(action: Action) {
        when (action) {
            Action.Play -> onPlay(false)
            Action.PlayFromStart -> onPlay(true)
            Action.ConvertVideo -> onTranscode()
            Action.RefreshMetadata -> onRefreshMetadata()
            Action.MediaInfo -> scope.launch { sheetState.show() }
        }
    }

    ModalBottomSheetLayout(
        sheetState = sheetState,
        sheetContent = { MediaInfoSheet(info = info) },
    ) {
        ItemDetailsScreen(
            backdrop = backdrop,
            poster = poster,
            headerContent = headerContent,
            actionsRow = {
                ActionsSection(info = info) { action ->
                    onActionInvoked(action)
                }
            },
            overview = overview,
            isWatched = isWatched,
            onNavigateUp = onNavigateUp,
        )
    }
}

private enum class Action {
    Play,
    PlayFromStart,
    ConvertVideo,
    RefreshMetadata,
    MediaInfo,
}

@Composable
private fun ActionsSection(info: VideoInfo?, onActionInvoked: (Action) -> Unit) {
    val context = LocalContext.current

    @Composable
    fun PlayButton(resume: Boolean, onClick: () -> Unit) {
        Button(onClick = onClick, modifier = Modifier.width(150.dp)) {
            Row(
                verticalAlignment = Alignment.CenterVertically,
                horizontalArrangement = Arrangement.Center,
            ) {
                Icon(Icons.Default.PlayArrow, contentDescription = "Play")
                Spacer(modifier = Modifier.width(12.dp))
                Text(if (resume) "Resume" else "Play")
                // Without this spacer the button content ends up looking
                // slightly off center
                Spacer(modifier = Modifier.width(8.dp))
            }
        }
    }

    Column {
        Row(verticalAlignment = Alignment.CenterVertically) {
            PlayButton(
                resume = info?.position ?: 0.0 > 0,
                onClick = {
                    context.playClick()
                    onActionInvoked(Action.Play)
                },
            )

            Spacer(modifier = Modifier.width(8.dp))

            Row(horizontalArrangement = Arrangement.End, modifier = Modifier.weight(1f)) {
                if ((info?.position?.toFloat() ?: 0f) > 0) {
                    IconButton(
                        onClick = {
                            context.playClick()
                            onActionInvoked(Action.PlayFromStart)
                        },
                    ) {
                        Icon(Icons.Default.Replay, contentDescription = "Replay")
                    }
                }

                IconButton(
                    onClick = {
                        context.playClick()
                        onActionInvoked(Action.MediaInfo)
                    },
                ) {
                    Icon(Icons.Default.Info, contentDescription = "Media info")
                }

                ActionsMenu(onActionInvoked = onActionInvoked)
            }
        }

        if (info?.position != null && info.position > 0) {
            Text(
                text = "Resume from ${DateUtils.formatElapsedTime(info.position.toLong())}",
                textAlign = TextAlign.Center,
                style = MaterialTheme.typography.caption,
                modifier = Modifier.widthIn(min = 150.dp),
            )
        }
    }
}

@Composable
private fun ActionsMenu(onActionInvoked: (Action) -> Unit) {
    val context = LocalContext.current
    var expanded by remember { mutableStateOf(false) }

    @Composable
    fun MenuItem(action: Action, label: String) {
        DropdownMenuItem(onClick = {
            context.playClick()
            expanded = false
            onActionInvoked(action)
        }) {
            Text(label)
        }
    }

    Box {
        IconButton(
            onClick = {
                context.playClick()
                expanded = true
            },
        ) {
            Icon(Icons.Default.MoreVert, contentDescription = "More")
        }

        DropdownMenu(expanded = expanded, onDismissRequest = { expanded = false }) {
            MenuItem(Action.ConvertVideo, "Convert")
            MenuItem(Action.RefreshMetadata, "Refresh Metadata")
        }
    }
}

@Composable
private fun MediaInfoSheet(info: VideoInfo) {
    Column(modifier = Modifier.navigationBarsPadding()) {
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

        DescriptionList(modifier = Modifier.padding(16.dp)) {
            entry("Format", info.format)
            entry("Path", info.path)

            heading("Video")
            entry("Codec", info.video.codec)
            entry("Profile", info.video.profile)
            entry("Resolution", "${info.video.width}x${info.video.height}")

            heading("Audio")
            entry("Codec", info.audio.codec)
        }
    }
}
