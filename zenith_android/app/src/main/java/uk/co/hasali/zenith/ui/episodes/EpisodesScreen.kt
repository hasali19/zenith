package uk.co.hasali.zenith.ui.episodes

import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ArrowBack
import androidx.compose.material.icons.filled.Done
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.ColorFilter
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import dev.chrisbanes.accompanist.coil.CoilImage
import dev.chrisbanes.accompanist.insets.statusBarsPadding
import uk.co.hasali.zenith.api.TvEpisode
import uk.co.hasali.zenith.api.TvSeason
import uk.co.hasali.zenith.api.TvShow
import uk.co.hasali.zenith.ui.ZenithTheme

@Composable
fun EpisodesScreen(
    show: TvShow,
    season: TvSeason,
    episodes: List<TvEpisode>,
    onEpisodeClick: (TvEpisode) -> Unit,
    onBackPressed: () -> Unit,
) {
    ZenithTheme {
        Scaffold(
            topBar = { AppBar(show, season, onBackPressed) },
        ) {
            LazyColumn(contentPadding = PaddingValues(horizontal = 0.dp, vertical = 8.dp)) {
                items(episodes) { episode ->
                    EpisodeListItem(
                        season = season,
                        episode = episode,
                        onClick = { onEpisodeClick(episode) },
                    )
                }
            }
        }
    }
}

@Composable
fun AppBar(show: TvShow, season: TvSeason, onBackPressed: () -> Unit) {
    Surface(
        color = MaterialTheme.colors.primarySurface,
        elevation = 4.dp,
    ) {
        TopAppBar(
            title = {
                Column {
                    Text(text = show.name)
                    Text(
                        text = season.name ?: "Season ${season.seasonNumber}",
                        style = MaterialTheme.typography.caption
                    )
                }
            },
            backgroundColor = Color.Transparent,
            elevation = 0.dp,
            modifier = Modifier.statusBarsPadding(),
            navigationIcon = {
                IconButton(onClick = { onBackPressed() }) {
                    Icon(Icons.Default.ArrowBack, "Back")
                }
            }
        )
    }
}

@Composable
fun EpisodeListItem(season: TvSeason, episode: TvEpisode, onClick: () -> Unit) {
    val itemHeight = 78
    val itemWidth = itemHeight * (16f / 9f)

    Box(
        modifier = Modifier
            .fillMaxWidth()
            .clickable { onClick() }
    ) {
        Row(modifier = Modifier.padding(horizontal = 16.dp, vertical = 8.dp)) {
            Box(
                modifier = Modifier
                    .size(itemWidth.dp, itemHeight.dp)
                    .clip(MaterialTheme.shapes.small),
            ) {
                Thumbnail(url = episode.thumbnail)
                WatchedOverlay(visible = episode.isWatched)
            }

            Spacer(modifier = Modifier.preferredWidth(16.dp))

            val name = episode.name ?: String.format(
                "S%02dE%02d",
                season.seasonNumber,
                episode.episodeNumber
            )

            Column(modifier = Modifier.align(Alignment.CenterVertically)) {
                Text(
                    text = "${episode.episodeNumber} - $name",
                    style = MaterialTheme.typography.body2,
                    overflow = TextOverflow.Ellipsis,
                    maxLines = 1,
                )
                Text(
                    text = episode.overview.orEmpty(),
                    style = MaterialTheme.typography.caption,
                    overflow = TextOverflow.Ellipsis,
                    maxLines = 4,
                    color = Color.Gray,
                )
            }
        }
    }
}

@Composable
fun Thumbnail(url: String?, modifier: Modifier = Modifier) {
    Box(modifier = modifier) {
        if (url != null) {
            CoilImage(
                data = url,
                contentDescription = null,
                fadeIn = true,
            )
        }
    }
}

@Composable
fun WatchedOverlay(visible: Boolean) {
    if (!visible) return
    Box(
        contentAlignment = Alignment.Center,
        modifier = Modifier
            .fillMaxSize()
            .background(Color.White.copy(alpha = 0.2f))
    ) {
        Box(
            modifier = Modifier
                .preferredSize(32.dp, 32.dp)
                .clip(CircleShape)
                .background(MaterialTheme.colors.secondary)
                .padding(8.dp)
        ) {
            Image(
                imageVector = Icons.Default.Done,
                contentDescription = "Watched",
                colorFilter = ColorFilter.tint(MaterialTheme.colors.onSecondary),
                modifier = Modifier.fillMaxSize(),
            )
        }
    }
}
