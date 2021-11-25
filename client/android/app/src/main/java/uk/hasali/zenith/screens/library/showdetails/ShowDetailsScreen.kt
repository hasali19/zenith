package uk.hasali.zenith.screens.library.showdetails

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyRow
import androidx.compose.foundation.lazy.items
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.dp
import kotlinx.datetime.Instant
import kotlinx.datetime.TimeZone
import kotlinx.datetime.toLocalDateTime
import uk.hasali.zenith.Season
import uk.hasali.zenith.Show
import uk.hasali.zenith.navigation.hiltViewModel
import uk.hasali.zenith.ui.*

@Composable
fun ShowDetailsScreen(
    model: ShowDetailsViewModel = hiltViewModel(),
    onNavigateToSeason: (Season) -> Unit,
    onNavigateUp: () -> Unit
) {
    val state by rememberFlowWithLifecycle(model.state)
        .collectAsState(ShowDetailsViewState())

    ShowDetailsScreen(
        show = state.show,
        seasons = state.seasons,
        onNavigateToSeason = onNavigateToSeason,
        onNavigateUp = onNavigateUp,
    )
}

@Composable
private fun ShowDetailsScreen(
    show: Show?,
    seasons: List<Season>?,
    onNavigateToSeason: (Season) -> Unit,
    onNavigateUp: () -> Unit,
) {
    when {
        show == null || seasons == null -> CenteredLoadingIndicator()
        else -> ItemDetailsScreen(
            backdrop = show.backdrop,
            poster = { Poster(url = show.poster) },
            headerContent = { HeaderContent(show = show) },
            overview = show.overview,
            isWatched = show.userData.unwatched == 0,
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
}

@Composable
private fun HeaderContent(show: Show) {
    Column {
        val dateVal = show.startDate
        val year = if (dateVal == null) null else
            Instant.fromEpochSeconds(dateVal)
                .toLocalDateTime(TimeZone.UTC)
                .year

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
