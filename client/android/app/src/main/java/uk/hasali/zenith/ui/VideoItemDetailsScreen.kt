package uk.hasali.zenith.ui

import android.text.format.DateUtils
import androidx.activity.compose.BackHandler
import androidx.compose.animation.animateColorAsState
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import com.google.accompanist.insets.navigationBarsPadding
import kotlinx.coroutines.launch
import uk.hasali.zenith.VideoInfo
import uk.hasali.zenith.VideoUserData
import uk.hasali.zenith.playClick

@OptIn(ExperimentalMaterialApi::class)
@Composable
fun VideoItemDetailsScreen(
    backdrop: String?,
    poster: String?,
    overview: String?,
    headerContent: @Composable () -> Unit,
    info: VideoInfo,
    userData: VideoUserData,
    onSetWatched: (Boolean) -> Unit,
    onPlay: (replay: Boolean) -> Unit,
    onTranscode: () -> Unit,
    onRefreshMetadata: () -> Unit,
    onNavigateUp: () -> Unit,
) {
    val scope = rememberCoroutineScope()
    val sheetState = rememberModalBottomSheetState(ModalBottomSheetValue.Hidden)

    val position = userData.position ?: 0.0
    var isWatched by remember(userData) { mutableStateOf(userData.isWatched) }

    fun onActionInvoked(action: Action) {
        when (action) {
            Action.Play -> onPlay(false)
            Action.PlayFromStart -> onPlay(true)
            Action.ConvertVideo -> onTranscode()
            Action.RefreshMetadata -> onRefreshMetadata()
            Action.MediaInfo -> scope.launch { sheetState.show() }
        }
    }

    BackHandler(enabled = sheetState.isVisible) {
        scope.launch { sheetState.hide() }
    }

    ModalBottomSheetLayout(
        sheetState = sheetState,
        sheetContent = { MediaInfoSheet(info = info) },
    ) {
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
                    position = userData.position,
                    isWatched = isWatched,
                    onSetWatched = {
                        isWatched = it
                        onSetWatched(it)
                    },
                    onActionInvoked = { action ->
                        onActionInvoked(action)
                    },
                )
            },
            overview = overview,
            isWatched = isWatched,
            onNavigateUp = onNavigateUp,
        )
    }
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

private enum class Action {
    Play,
    PlayFromStart,
    ConvertVideo,
    RefreshMetadata,
    MediaInfo,
}

@Composable
private fun ActionsSection(
    position: Double?,
    isWatched: Boolean,
    onSetWatched: (Boolean) -> Unit,
    onActionInvoked: (Action) -> Unit
) {
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
                resume = position ?: 0.0 > 0,
                onClick = {
                    context.playClick()
                    onActionInvoked(Action.Play)
                },
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

                if ((position?.toFloat() ?: 0f) > 0) {
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
            entry("Format", info.format!!)
            entry("Path", info.path)

            heading("Video")
            entry("Codec", info.video!!.codec)
            entry("Profile", info.video.profile)
            entry("Resolution", "${info.video.width}x${info.video.height}")

            heading("Audio")
            entry("Codec", info.audio!!.codec)
        }
    }
}
