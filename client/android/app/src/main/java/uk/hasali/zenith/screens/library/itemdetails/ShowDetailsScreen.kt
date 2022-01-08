package uk.hasali.zenith.screens.library.itemdetails

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.lazy.LazyRow
import androidx.compose.foundation.lazy.items
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.dp
import kotlinx.datetime.Instant
import kotlinx.datetime.TimeZone
import kotlinx.datetime.toLocalDateTime
import uk.hasali.zenith.api.Season
import uk.hasali.zenith.api.Show
import uk.hasali.zenith.ui.BottomSheetController
import uk.hasali.zenith.ui.MediaItemWithPoster

@Composable
fun ShowDetailsScreen(
    show: Show,
    seasons: List<Season>,
    bottomSheetController: BottomSheetController,
    onRefreshMetadata: () -> Unit,
    onNavigateToSeason: (Season) -> Unit,
    onNavigateUp: () -> Unit,
) {
    CollectionItemDetailsScreen(
        title = show.name,
        backdrop = show.backdrop,
        poster = show.poster,
        headerContent = { HeaderContent(show) },
        overview = show.overview,
        isWatched = show.userData.unwatched == 0,
        bottomSheetController = bottomSheetController,
        onRefreshMetadata = onRefreshMetadata,
        onNavigateUp = onNavigateUp,
    ) {
        if (seasons.isNotEmpty()) {
            item {
                SeasonsSection(
                    show = show,
                    seasons = seasons,
                    onItemClick = onNavigateToSeason,
                )
            }
        }
    }
}

@Composable
private fun HeaderContent(show: Show) {
    Column {
        val year = show.startYear()

        Text(show.name, style = MaterialTheme.typography.h6)

        if (year != null)
            Text(year.toString(), style = MaterialTheme.typography.caption)
    }
}

@Composable
private fun SeasonsSection(show: Show, seasons: List<Season>, onItemClick: (Season) -> Unit) {
    Text(
        text = "Seasons",
        style = MaterialTheme.typography.subtitle2,
        color = if (MaterialTheme.colors.isLight) Color.Black else Color.White,
        modifier = Modifier.padding(top = 8.dp, start = 16.dp, end = 16.dp, bottom = 8.dp),
    )
    SeasonsList(
        show = show,
        seasons = seasons,
        onItemClick = onItemClick,
    )
}

@Composable
private fun SeasonsList(show: Show, seasons: List<Season>, onItemClick: (Season) -> Unit) {
    LazyRow(contentPadding = PaddingValues(12.dp, 0.dp)) {
        items(seasons) { season ->
            MediaItemWithPoster(
                poster = season.poster,
                primary = season.name ?: "Season ${season.seasonNumber}",
                secondary = show.name,
                isWatched = season.userData.unwatched == 0,
                onClick = { onItemClick(season) },
                modifier = Modifier
                    .width(120.dp)
                    .padding(4.dp)
            )
        }
    }
}
