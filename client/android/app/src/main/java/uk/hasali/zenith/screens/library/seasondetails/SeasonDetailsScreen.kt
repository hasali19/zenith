package uk.hasali.zenith.screens.library.seasondetails

import androidx.compose.foundation.ExperimentalFoundationApi
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.items
import androidx.compose.material.Icon
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Check
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.layout.Layout
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import uk.hasali.zenith.Episode
import uk.hasali.zenith.Season
import uk.hasali.zenith.Show
import uk.hasali.zenith.navigation.hiltViewModel
import uk.hasali.zenith.ui.*

@Composable
fun SeasonDetailsScreen(
    model: SeasonDetailsViewModel = hiltViewModel(),
    onNavigateToEpisode: (Episode) -> Unit,
    onNavigateUp: () -> Unit,
) {
    val state by rememberFlowWithLifecycle(model.state)
        .collectAsState(SeasonDetailsViewState())

    SeasonDetailsScreen(
        show = state.show,
        season = state.season,
        episodes = state.episodes,
        onNavigateToEpisode = onNavigateToEpisode,
        onNavigateUp = onNavigateUp,
    )
}

@OptIn(ExperimentalFoundationApi::class)
@Composable
private fun SeasonDetailsScreen(
    show: Show?,
    season: Season?,
    episodes: List<Episode>?,
    onNavigateToEpisode: (Episode) -> Unit,
    onNavigateUp: () -> Unit,
) {
    when {
        show == null || season == null || episodes == null -> CenteredLoadingIndicator()
        else -> ItemDetailsScreen(
            backdrop = season.backdrop,
            poster = { Poster(url = season.poster) },
            headerContent = {
                Column {
                    Text(show.name, style = MaterialTheme.typography.h6)
                    Text(season.title(), style = MaterialTheme.typography.caption)
                }
            },
            overview = season.overview,
            isWatched = false,
            onNavigateUp = onNavigateUp,
        ) { width ->
            item {
                Text(
                    text = "Episodes",
                    style = MaterialTheme.typography.subtitle2,
                    color = if (MaterialTheme.colors.isLight) Color.Black else Color.White,
                    modifier = Modifier.padding(top = 8.dp, bottom = 8.dp, start = 16.dp, end = 16.dp),
                )
            }

            val nColumns = maxOf((width / 200.dp).toInt(), 1)

            items(episodes.windowed(nColumns, nColumns, true)) {
                Layout(
                    modifier = Modifier.padding(horizontal = 8.dp),
                    content = {
                        for (episode in it) {
                            EpisodeItem(episode = episode) {
                                onNavigateToEpisode(episode)
                            }
                        }
                    },
                ) { measurables, constraints ->
                    val placeables = measurables.map {
                        it.measure(constraints.copy(maxWidth = constraints.maxWidth / nColumns))
                    }

                    val height = placeables.maxByOrNull { it.height }?.height ?: 0

                    layout(constraints.maxWidth, height) {
                        var x = 0
                        for (placeable in placeables) {
                            placeable.place(x, 0)
                            x += placeable.width
                        }
                    }
                }
            }
        }
    }
}

private val SEASON_TITLE_REGEX = Regex("(?:Season|Series) +\\d+")

private fun Season.title(): String =
    if (name != null) {
        if (SEASON_TITLE_REGEX.matches(name)) {
            name
        } else {
            "Season $seasonNumber - $name"
        }
    } else {
        "Season $seasonNumber"
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
    Column(modifier = Modifier.padding(8.dp)) {
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
