package uk.hasali.zenith.ui

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyRow
import androidx.compose.foundation.lazy.items
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.produceState
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.dp
import kotlinx.datetime.Instant
import kotlinx.datetime.TimeZone
import kotlinx.datetime.toLocalDateTime
import uk.hasali.zenith.Season
import uk.hasali.zenith.Show

@Composable
fun ShowDetailsScreen(show: Show) {
    val client = LocalZenithClient.current
    val navigator = LocalNavigator.current

    val seasons by produceState(initialValue = emptyList<Season>()) {
        value = client.getSeasons(show.id)
    }

    ItemDetailsScreen(
        backdrop = show.backdrop,
        poster = show.poster,
        headerContent = { HeaderContent(show = show) },
        overview = show.overview,
        isWatched = show.unwatchedEpisodes == 0,
    ) {
        SeasonsSection(
            show = show,
            seasons = seasons,
            onItemClick = { navigator.push(Screen.SeasonDetails(show, it)) },
        )
    }
}

@Composable
private fun HeaderContent(show: Show) {
    Column {
        val year = Instant.fromEpochSeconds(show.startDate)
            .toLocalDateTime(TimeZone.UTC)
            .year

        Text(show.name, style = MaterialTheme.typography.h6)
        Text(year.toString(), style = MaterialTheme.typography.caption)
    }
}

@Composable
private fun SeasonsSection(show: Show, seasons: List<Season>, onItemClick: (Season) -> Unit) {
    Text(
        text = "Seasons",
        style = MaterialTheme.typography.subtitle2,
        color = if (MaterialTheme.colors.isLight) Color.Black else Color.White,
        modifier = Modifier.padding(horizontal = 16.dp),
    )
    Spacer(modifier = Modifier.height(8.dp))
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
                primary = season.name,
                secondary = show.name,
                onClick = { onItemClick(season) },
                modifier = Modifier
                    .width(120.dp)
                    .padding(4.dp)
            )
        }
    }
}
