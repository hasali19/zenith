package uk.hasali.zenith.ui

import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Check
import androidx.compose.material.icons.filled.Info
import androidx.compose.material.icons.filled.MoreVert
import androidx.compose.material.icons.filled.PlayArrow
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.LocalDensity
import androidx.compose.ui.text.SpanStyle
import androidx.compose.ui.text.buildAnnotatedString
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.withStyle
import androidx.compose.ui.unit.dp
import com.google.accompanist.coil.rememberCoilPainter
import com.google.accompanist.insets.navigationBarsPadding
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

    Surface(
        modifier = Modifier
            .fillMaxSize()
            .navigationBarsPadding(),
    ) {
        BoxWithConstraints(modifier = Modifier.verticalScroll(rememberSaveableScrollState())) {
            Box(modifier = Modifier.aspectRatio(16f / 9f)) {
                Image(
                    painter = rememberCoilPainter(request = episode.thumbnail, fadeIn = true),
                    contentDescription = "Backdrop",
                    contentScale = ContentScale.Crop,
                    modifier = Modifier.fillMaxWidth(),
                )

                if (episode.isWatched) {
                    Box(
                        modifier = Modifier
                            .fillMaxSize()
                            .background(Color.Black.copy(alpha = 0.4f))
                    ) {
                        Icon(
                            imageVector = Icons.Default.Check,
                            contentDescription = "Watched",
                            modifier = Modifier
                                .size(32.dp)
                                .align(Alignment.Center),
                            tint = Color.White,
                        )
                    }
                }
            }

            val backdropHeight = with(LocalDensity.current) {
                (constraints.maxWidth * 9f / 16f).toDp()
            }

            Column(
                modifier = Modifier
                    .padding(top = backdropHeight - 48.dp, bottom = 16.dp)
                    .padding(horizontal = 16.dp),
            ) {
                HeaderSection(show = show, season = season, episode = episode)
                Spacer(modifier = Modifier.height(16.dp))
                ActionsSection { action -> onActionInvoked(action) }
                Spacer(modifier = Modifier.height(16.dp))
                OverviewSection(content = episode.overview)
                Spacer(modifier = Modifier.height(16.dp))
            }
        }
    }
}

@Composable
private fun HeaderSection(show: Show, season: Season, episode: Episode) {
    Row {
        Poster(url = season.poster, modifier = Modifier.width(150.dp))
        Spacer(modifier = Modifier.width(16.dp))
        Column(modifier = Modifier.align(Alignment.CenterVertically)) {
            val seasonNumber = twoDigitNumber(season.seasonNumber)
            val episodeNumber = twoDigitNumber(episode.episodeNumber)
            val duration = displayDuration(episode.duration)

            Text(text = show.name, style = MaterialTheme.typography.caption)
            Text(text = episode.name, style = MaterialTheme.typography.h6)
            Spacer(modifier = Modifier.height(8.dp))
            Text(
                text = "S${seasonNumber}E${episodeNumber} - $duration",
                style = MaterialTheme.typography.caption,
            )
        }
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

@Composable
private fun OverviewSection(content: String) {
    Text(text = "Overview", style = MaterialTheme.typography.subtitle2, color = Color.Black)
    Spacer(modifier = Modifier.height(8.dp))
    Text(text = content, style = MaterialTheme.typography.body2)
}

@Composable
fun MediaInfoDialog(info: VideoInfo, onDismiss: () -> Unit) {
    val context = LocalContext.current

    @Composable
    fun Field(name: String, value: String) {
        Text(buildAnnotatedString {
            withStyle(style = MaterialTheme.typography.body2.toSpanStyle()) {
                withStyle(style = SpanStyle(fontWeight = FontWeight.Bold)) {
                    append(name)
                    append(':')
                }

                append(' ')
                append(value)
            }
        })
    }

    AlertDialog(
        onDismissRequest = onDismiss,
        title = { Text("Media Info") },
        text = {
            Column {
                Text("File", style = MaterialTheme.typography.subtitle2)
                Field("Path", info.path)
                Field("Format", info.format)
                Spacer(modifier = Modifier.height(8.dp))

                Text("Video", style = MaterialTheme.typography.subtitle2)
                Field("Codec", info.video.codec)
                Field("Profile", info.video.profile)
                Field("Resolution", "${info.video.width}x${info.video.height}")
                Spacer(modifier = Modifier.height(8.dp))

                Text("Audio", style = MaterialTheme.typography.subtitle2)
                Field("Codec", info.audio.codec)
                Spacer(modifier = Modifier.height(8.dp))
            }
        },
        confirmButton = {
            TextButton(onClick = {
                context.playClick()
                onDismiss()
            }) {
                Text("Close")
            }
        },
    )
}
