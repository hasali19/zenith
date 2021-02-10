package uk.co.hasali.zenith.ui.showdetails

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.LazyRow
import androidx.compose.foundation.lazy.items
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ArrowBack
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.dp
import dev.chrisbanes.accompanist.insets.statusBarsPadding
import uk.co.hasali.zenith.api.TvSeason
import uk.co.hasali.zenith.api.TvShow
import uk.co.hasali.zenith.ui.Backdrop
import uk.co.hasali.zenith.ui.PosterCard
import uk.co.hasali.zenith.ui.ZenithTheme

@Composable
fun TvShowDetailsScreen(
    show: TvShow,
    seasons: List<TvSeason>,
    onSeasonClick: (TvSeason) -> Unit,
    onBackPressed: () -> Unit,
) {
    ZenithTheme {
        Surface(color = MaterialTheme.colors.background) {
            Box {
                LazyColumn {
                    item {
                        Backdrop(url = show.backdrop)

                        Column(modifier = Modifier.padding(16.dp)) {
                            Text(
                                text = show.name,
                                style = MaterialTheme.typography.h5
                            )

                            Spacer(modifier = Modifier.preferredHeight(16.dp))
                            Text(text = show.overview ?: "")
                            Spacer(modifier = Modifier.preferredHeight(16.dp))

                            SeasonList(
                                show = show,
                                seasons = seasons,
                                onItemClick = { onSeasonClick(it) }
                            )
                        }
                    }
                }
            }

            Box(modifier = Modifier.statusBarsPadding()) {
                TopAppBar(
                    title = { /* No title */ },
                    backgroundColor = Color.Transparent,
                    elevation = 0.dp,
                    navigationIcon = {
                        IconButton(onClick = { onBackPressed() }) {
                            Icon(Icons.Default.ArrowBack, "Back")
                        }
                    },
                )
            }
        }
    }
}

@Composable
fun SeasonList(show: TvShow, seasons: List<TvSeason>, onItemClick: (TvSeason) -> Unit) {
    Text(text = "Seasons", style = MaterialTheme.typography.h6)
    Spacer(modifier = Modifier.preferredHeight(16.dp))
    LazyRow {
        items(seasons) { season ->
            PosterCard(
                posterUrl = season.poster,
                primaryText = show.name,
                secondaryText = season.name
                    ?: "Season ${season.seasonNumber}",
                modifier = Modifier
                    .padding(4.dp)
                    .preferredWidth(92.dp)
                    .clickable { onItemClick(season) }
            )
        }
    }
}
