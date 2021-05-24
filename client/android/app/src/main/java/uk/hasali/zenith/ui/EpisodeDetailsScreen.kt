package uk.hasali.zenith.ui

import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.PlayArrow
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.unit.dp
import com.google.accompanist.coil.rememberCoilPainter
import uk.hasali.zenith.Episode
import uk.hasali.zenith.Season
import uk.hasali.zenith.ZenithApiClient
import uk.hasali.zenith.playClick

@Composable
fun EpisodeDetailsScreen(
    client: ZenithApiClient,
    navigator: Navigator,
    season: Season,
    episode: Episode,
) {
    val context = LocalContext.current

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
                    Spacer(modifier = Modifier.height(16.dp))
                    Text(episode.overview, style = MaterialTheme.typography.body2)
                }
            }
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