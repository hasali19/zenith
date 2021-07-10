package uk.hasali.zenith.ui

import androidx.compose.foundation.layout.*
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Info
import androidx.compose.material.icons.filled.MoreVert
import androidx.compose.material.icons.filled.PlayArrow
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.unit.dp
import kotlinx.coroutines.launch
import uk.hasali.zenith.*

@Composable
fun EpisodeDetailsScreen(show: Show, season: Season, episode: Episode) {
    val client = LocalZenithClient.current
    val navigator = LocalNavigator.current
    val scope = rememberCoroutineScope()

    val info by produceState<VideoInfo?>(null, episode) {
        value = client.getVideoInfo(episode.id)
    }

    var showMediaInfo by remember { mutableStateOf(false) }

    fun onActionInvoked(action: Action) {
        when (action) {
            Action.Play -> navigator.push(Screen.Player(episode.id))
            Action.ConvertVideo -> scope.launch {
                client.startTranscode(episode.id)
            }
            Action.RefreshMetadata -> scope.launch {
                client.refreshMetadata(episode.id)
            }
            Action.MediaInfo -> showMediaInfo = true
        }
    }

    if (showMediaInfo) {
        info?.let {
            MediaInfoDialog(info = it, onDismiss = { showMediaInfo = false })
        }
    }

    ItemDetailsScreen(
        backdrop = episode.thumbnail,
        poster = season.poster,
        headerContent = { HeaderContent(show = show, season = season, episode = episode) },
        actionsRow = { ActionsSection { action -> onActionInvoked(action) } },
        overview = episode.overview,
        isWatched = episode.isWatched,
    )
}

@Composable
private fun HeaderContent(show: Show, season: Season, episode: Episode) {
    val seasonNumber = twoDigitNumber(season.seasonNumber)
    val episodeNumber = twoDigitNumber(episode.episodeNumber)
    val duration = displayDuration(episode.duration)

    Column {
        Text(text = show.name, style = MaterialTheme.typography.caption)
        Text(text = episode.name, style = MaterialTheme.typography.h6)
        Spacer(modifier = Modifier.height(8.dp))
        Text(
            text = "S${seasonNumber}E${episodeNumber} - $duration",
            style = MaterialTheme.typography.caption,
        )
    }
}

private enum class Action {
    Play,
    ConvertVideo,
    RefreshMetadata,
    MediaInfo,
}

@Composable
private fun ActionsSection(onActionInvoked: (Action) -> Unit) {
    val context = LocalContext.current

    @Composable
    fun PlayButton(onClick: () -> Unit) {
        Button(onClick = onClick, modifier = Modifier.width(150.dp)) {
            Row(verticalAlignment = Alignment.CenterVertically,
                horizontalArrangement = Arrangement.Center) {
                Icon(Icons.Default.PlayArrow, contentDescription = "Play")
                Spacer(modifier = Modifier.width(12.dp))
                Text("Play")
                // Without this spacer the button content ends up looking
                // slightly off center
                Spacer(modifier = Modifier.width(8.dp))
            }
        }
    }

    Row(verticalAlignment = Alignment.CenterVertically) {
        PlayButton(onClick = {
            context.playClick()
            onActionInvoked(Action.Play)
        })

        Spacer(modifier = Modifier.width(8.dp))

        Row(horizontalArrangement = Arrangement.End, modifier = Modifier.weight(1f)) {
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
