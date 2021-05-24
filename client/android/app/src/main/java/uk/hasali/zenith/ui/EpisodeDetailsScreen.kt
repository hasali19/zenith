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
import kotlinx.coroutines.launch
import uk.hasali.zenith.*

@Composable
fun EpisodeDetailsScreen(
    client: ZenithApiClient,
    navigator: Navigator,
    season: Season,
    episode: Episode,
) {
    val context = LocalContext.current
    val scope = rememberCoroutineScope()

    val info by produceState<VideoInfo?>(null, episode) {
        value = client.getVideoInfo(episode.id)
    }

    fun convertVideo() {
        scope.launch {
            client.startTranscode(episode.id)
        }
    }

    Surface(modifier = Modifier.fillMaxSize()) {
        BoxWithConstraints(modifier = Modifier.verticalScroll(rememberScrollState())) {
            Column(modifier = Modifier.padding()) {
                Image(
                    painter = rememberCoilPainter(request = episode.thumbnail, fadeIn = true),
                    contentDescription = "Backdrop",
                    modifier = Modifier.aspectRatio(16f / 9f)
                )

                val seasonNumber = twoDigitNumber(season.seasonNumber)
                val episodeNumber = twoDigitNumber(episode.episodeNumber)
                val duration = displayDuration(episode.duration)

                Column(modifier = Modifier.padding(16.dp)) {
                    Text(episode.name, style = MaterialTheme.typography.h5)
                    Text(
                        "S${seasonNumber}E${episodeNumber} - $duration",
                        style = MaterialTheme.typography.caption,
                    )
                    Spacer(modifier = Modifier.height(16.dp))
                    Row(verticalAlignment = Alignment.CenterVertically) {
                        Button(
                            onClick = {
                                context.playClick()
                                navigator.push(Screen.Player(episode.id))
                            },
                        ) {
                            Row(verticalAlignment = Alignment.CenterVertically) {
                                Icon(Icons.Default.PlayArrow, contentDescription = "Play")
                                Spacer(modifier = Modifier.width(12.dp))
                                Text("Play")
                            }
                        }

                        Spacer(modifier = Modifier.width(8.dp))

                        ActionsMenu(
                            onConvertClick = { convertVideo() }
                        )
                    }
                    Spacer(modifier = Modifier.height(16.dp))
                    Text(episode.overview, style = MaterialTheme.typography.body2)
                    Spacer(modifier = Modifier.height(16.dp))

                    info?.let {
                        MediaInfo(info = it)
                    }
                }
            }
        }
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
fun MediaInfo(info: VideoInfo) {
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

    Card {
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

private fun twoDigitNumber(number: Int) = "$number".padStart(2, '0')

private fun displayDuration(duration: Double) =
    if (duration <= 90 * 60) {
        "${(duration / 60).toInt()}m";
    } else {
        val hours = (duration / 3600).toInt()
        val minutes = ((duration % 3600) / 60).toInt();
        "${hours}h ${minutes}m";
    }