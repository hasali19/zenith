package uk.hasali.zenith.ui

import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.MoreVert
import androidx.compose.material.icons.filled.PlayArrow
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
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
fun EpisodeDetailsScreen(season: Season, episode: Episode) {
    val client = LocalZenithClient.current
    val navigator = LocalNavigator.current
    val scope = rememberCoroutineScope()

    val info by produceState<VideoInfo?>(null, episode) {
        value = client.getVideoInfo(episode.id)
    }

    fun convertVideo() {
        scope.launch {
            client.startTranscode(episode.id)
        }
    }

    fun onActionInvoked(action: Action) {
        when (action) {
            Action.Play -> navigator.push(Screen.Player(episode.id))
            Action.ConvertVideo -> convertVideo()
        }
    }

    Surface(
        modifier = Modifier
            .fillMaxSize()
            .navigationBarsPadding(),
    ) {
        BoxWithConstraints(modifier = Modifier.verticalScroll(rememberSaveableScrollState())) {
            Column(modifier = Modifier.padding()) {
                Image(
                    painter = rememberCoilPainter(request = episode.thumbnail, fadeIn = true),
                    contentDescription = "Backdrop",
                    modifier = Modifier.aspectRatio(16f / 9f)
                )

                Column(modifier = Modifier.padding(16.dp)) {
                    HeaderSection(season = season, episode = episode)
                    Spacer(modifier = Modifier.height(16.dp))
                    ActionsSection { action -> onActionInvoked(action) }
                    Spacer(modifier = Modifier.height(16.dp))
                    OverviewSection(content = episode.overview)
                    Spacer(modifier = Modifier.height(16.dp))

                    info?.let {
                        MediaInfoSection(info = it)
                    }
                }
            }
        }
    }
}

@Composable
private fun HeaderSection(season: Season, episode: Episode) {
    val seasonNumber = twoDigitNumber(season.seasonNumber)
    val episodeNumber = twoDigitNumber(episode.episodeNumber)
    val duration = displayDuration(episode.duration)

    Text(text = episode.name, style = MaterialTheme.typography.h4)
    Text(
        text = "S${seasonNumber}E${episodeNumber} - $duration",
        style = MaterialTheme.typography.caption,
    )
}

private enum class Action {
    Play,
    ConvertVideo,
}

@Composable
private fun ActionsSection(onActionInvoked: (Action) -> Unit) {
    val context = LocalContext.current

    @Composable
    fun PlayButton(onClick: () -> Unit) {
        Button(onClick = onClick) {
            Row(verticalAlignment = Alignment.CenterVertically) {
                Icon(Icons.Default.PlayArrow, contentDescription = "Play")
                Spacer(modifier = Modifier.width(12.dp))
                Text("Play")
            }
        }
    }

    Row(verticalAlignment = Alignment.CenterVertically) {
        PlayButton(onClick = {
            context.playClick()
            onActionInvoked(Action.Play)
        })

        Spacer(modifier = Modifier.width(8.dp))

        ActionsMenu(onConvertClick = {
            onActionInvoked(Action.ConvertVideo)
        })
    }
}

@Composable
private fun ActionsMenu(onConvertClick: () -> Unit) {
    val context = LocalContext.current
    var expanded by remember { mutableStateOf(false) }

    Box {
        IconButton(onClick = {
            context.playClick()
            expanded = true
        }) {
            Icon(Icons.Default.MoreVert, contentDescription = "More")
        }

        DropdownMenu(expanded = expanded, onDismissRequest = { expanded = false }) {
            DropdownMenuItem(onClick = {
                context.playClick()
                expanded = false
                onConvertClick()
            }) {
                Text("Convert")
            }
        }
    }
}

@Composable
private fun OverviewSection(content: String) {
    Text(text = "Overview", style = MaterialTheme.typography.h6)
    Spacer(modifier = Modifier.height(8.dp))
    Text(text = content, style = MaterialTheme.typography.body2)
}

@Composable
fun MediaInfoSection(info: VideoInfo) {
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

    Card(modifier = Modifier.fillMaxWidth()) {
        Column(modifier = Modifier.padding(12.dp)) {
            Text("Media Info", style = MaterialTheme.typography.h6)
            Spacer(modifier = Modifier.height(8.dp))

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
    }
}
