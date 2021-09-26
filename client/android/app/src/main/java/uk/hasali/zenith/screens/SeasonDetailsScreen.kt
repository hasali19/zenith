package uk.hasali.zenith.screens

import androidx.compose.foundation.ExperimentalFoundationApi
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.GridCells
import androidx.compose.foundation.lazy.LazyVerticalGrid
import androidx.compose.foundation.lazy.items
import androidx.compose.material.Icon
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Scaffold
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Check
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.produceState
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import com.google.accompanist.insets.navigationBarsPadding
import uk.hasali.zenith.Episode
import uk.hasali.zenith.Season
import uk.hasali.zenith.ui.*

@Composable
fun SeasonDetailsScreen(
    id: Int,
    onNavigateToEpisode: (Episode) -> Unit,
    onNavigateUp: () -> Unit,
) {
    val client = LocalZenithClient.current

    val season by produceState<Season?>(null, id) {
        value = client.getSeason(id)
    }

    val episodes by produceState<List<Episode>?>(null, id) {
        value = client.getEpisodes(id)
    }

    SeasonDetailsScreen(
        season = season,
        episodes = episodes,
        onNavigateToEpisode = onNavigateToEpisode,
        onNavigateUp = onNavigateUp,
    )
}

@OptIn(ExperimentalFoundationApi::class)
@Composable
private fun SeasonDetailsScreen(
    season: Season?,
    episodes: List<Episode>?,
    onNavigateToEpisode: (Episode) -> Unit,
    onNavigateUp: () -> Unit,
) {
    Scaffold(
        topBar = { AppBar(title = season?.name, onBackPressed = onNavigateUp) },
        modifier = Modifier.navigationBarsPadding(),
    ) {
        when (episodes) {
            null -> CenteredLoadingIndicator()
            else -> {
                LazyVerticalGrid(
                    cells = GridCells.Adaptive(200.dp),
                    contentPadding = PaddingValues(4.dp),
                ) {
                    items(episodes) { episode ->
                        EpisodeItem(
                            episode = episode,
                            onClick = { onNavigateToEpisode(episode) },
                        )
                    }
                }
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
            tint = Color.White,
        )
    }
}

@Composable
private fun EpisodeItem(episode: Episode, onClick: () -> Unit) {
    Column(modifier = Modifier.padding(4.dp)) {
        Thumbnail(
            url = episode.thumbnail,
            modifier = Modifier.fillMaxWidth(),
            overlay = { WatchedOverlay(visible = episode.userData.isWatched) },
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
                text = displayDuration(episode.videoInfo.duration),
                maxLines = 1,
                overflow = TextOverflow.Ellipsis,
                color = Color.LightGray.copy(alpha = 0.8f),
                style = MaterialTheme.typography.caption
            )

            Text(
                text = episode.overview ?: "",
                maxLines = 3,
                overflow = TextOverflow.Ellipsis,
                style = MaterialTheme.typography.caption
            )
        }
    }
}
