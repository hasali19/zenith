package uk.hasali.zenith.ui

import androidx.compose.foundation.ExperimentalFoundationApi
import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.GridCells
import androidx.compose.foundation.lazy.LazyVerticalGrid
import androidx.compose.foundation.lazy.items
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Check
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.produceState
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.LocalDensity
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import com.google.accompanist.coil.rememberCoilPainter
import uk.hasali.zenith.*

@OptIn(ExperimentalFoundationApi::class)
@Composable
fun SeasonDetailsScreen(client: ZenithApiClient, navigator: Navigator, show: Show, season: Season) {
    val episodes by produceState(initialValue = emptyList<Episode>()) {
        value = client.getEpisodes(season.id)
    }

    Scaffold(topBar = { AppBar(title = season.name, navigator = navigator) }) {
        LazyVerticalGrid(cells = GridCells.Adaptive(200.dp), contentPadding = PaddingValues(4.dp)) {
            items(episodes) { episode ->
                EpisodeItem(
                    episode = episode,
                    onClick = {
                        navigator.push(Screen.EpisodeDetails(show, season, episode))
                    },
                )
            }
        }
    }
}

@Composable
private fun WatchedOverlay(visible: Boolean) {
    if (!visible) return

    Box(
        modifier = Modifier
            .fillMaxSize()
            .background(Color.Black.copy(alpha = 0.4f))
    ) {
        Icon(
            imageVector = Icons.Default.Check,
            contentDescription = "Watched",
            modifier = Modifier.align(Alignment.Center),
        )
    }
}

@Composable
private fun EpisodeItem(episode: Episode, onClick: () -> Unit) {
    Column(modifier = Modifier.padding(4.dp)) {
        Thumbnail(
            url = episode.thumbnail,
            modifier = Modifier.fillMaxWidth(),
            overlay = { WatchedOverlay(visible = episode.isWatched) },
            onClick = onClick,
        )

        Column(modifier = Modifier.padding(vertical = 8.dp)) {
            Text(
                text = "${episode.episodeNumber} - ${episode.name}",
                maxLines = 1,
                overflow = TextOverflow.Ellipsis,
                style = MaterialTheme.typography.subtitle2
            )

            Text(
                text = displayDuration(episode.duration),
                maxLines = 1,
                overflow = TextOverflow.Ellipsis,
                color = Color.LightGray.copy(alpha = 0.8f),
                style = MaterialTheme.typography.caption
            )

            Text(
                text = episode.overview,
                maxLines = 3,
                overflow = TextOverflow.Ellipsis,
                style = MaterialTheme.typography.caption
            )
        }
    }
}
