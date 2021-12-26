package uk.hasali.zenith.screens.library.itemdetails

import androidx.compose.foundation.ExperimentalFoundationApi
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.items
import androidx.compose.material.Icon
import androidx.compose.material.LocalContentColor
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Check
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import uk.hasali.zenith.api.Episode
import uk.hasali.zenith.api.Season
import uk.hasali.zenith.api.Show
import uk.hasali.zenith.ui.BottomSheetController
import uk.hasali.zenith.ui.Thumbnail
import uk.hasali.zenith.ui.formatDuration

@OptIn(ExperimentalFoundationApi::class)
@Composable
fun SeasonDetailsScreen(
    show: Show,
    season: Season,
    episodes: List<Episode>,
    bottomSheetController: BottomSheetController,
    onRefreshMetadata: () -> Unit,
    onNavigateToEpisode: (Episode) -> Unit,
    onNavigateUp: () -> Unit,
) {
    CollectionItemDetailsScreen(
        title = season.title(),
        backdrop = season.backdrop,
        poster = season.poster,
        headerContent = {
            Column {
                Text(show.name, style = MaterialTheme.typography.h6)
                Text(season.title(), style = MaterialTheme.typography.caption)
            }
        },
        overview = season.overview,
        isWatched = season.userData.unwatched == 0,
        bottomSheetController = bottomSheetController,
        onRefreshMetadata = onRefreshMetadata,
        onNavigateUp = onNavigateUp,
    ) {
        item {
            Text(
                text = "Episodes",
                style = MaterialTheme.typography.subtitle2,
                color = if (MaterialTheme.colors.isLight) Color.Black else Color.White,
                modifier = Modifier.padding(
                    top = 8.dp,
                    bottom = 8.dp,
                    start = 16.dp,
                    end = 16.dp
                ),
            )
        }

        items(episodes) {
            EpisodeItem(it) {
                onNavigateToEpisode(it)
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
    Row(
        modifier = Modifier
            .clickable(onClick = onClick)
            .padding(horizontal = 16.dp, vertical = 4.dp),
    ) {
        Thumbnail(
            url = episode.thumbnail,
            modifier = Modifier.width(160.dp),
            overlay = { WatchedOverlay(visible = episode.userData.isWatched) },
        )

        Spacer(modifier = Modifier.width(4.dp))

        Column(
            modifier = Modifier
                .weight(1f)
                .padding(4.dp),
        ) {
            Text(
                text = "${episode.episodeNumber} - ${episode.name}",
                maxLines = 1,
                overflow = TextOverflow.Ellipsis,
                style = MaterialTheme.typography.subtitle2
            )

            Text(
                text = formatDuration(episode.videoInfo.duration),
                maxLines = 1,
                overflow = TextOverflow.Ellipsis,
                color = LocalContentColor.current.copy(alpha = 0.5f),
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
