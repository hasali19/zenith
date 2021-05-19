package uk.hasali.zenith.ui

import androidx.compose.foundation.ExperimentalFoundationApi
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.lazy.GridCells
import androidx.compose.foundation.lazy.LazyVerticalGrid
import androidx.compose.material.Scaffold
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.produceState
import androidx.compose.ui.unit.dp
import kotlinx.datetime.Instant
import kotlinx.datetime.TimeZone
import kotlinx.datetime.toLocalDateTime
import uk.hasali.zenith.Navigator
import uk.hasali.zenith.Screen
import uk.hasali.zenith.Show
import uk.hasali.zenith.ZenithApiClient

@OptIn(ExperimentalFoundationApi::class)
@Composable
fun ShowsScreen(client: ZenithApiClient, navigator: Navigator) {
    val shows by produceState(initialValue = emptyList<Show>()) {
        value = client.getShows()
    }

    Scaffold(topBar = { AppBar(navigator = navigator) }) {
        LazyVerticalGrid(
            cells = GridCells.Adaptive(120.dp),
            contentPadding = PaddingValues(4.dp),
        ) {
            items(shows.size) { i ->
                val show = shows[i]
                val year = Instant.fromEpochSeconds(show.startDate)
                    .toLocalDateTime(TimeZone.UTC)
                    .year

                MediaItemWithPoster(
                    poster = show.poster,
                    primary = show.name,
                    secondary = year.toString(),
                    onClick = { navigator.push(Screen.ShowDetails(shows[i])) },
                )
            }
        }
    }
}
