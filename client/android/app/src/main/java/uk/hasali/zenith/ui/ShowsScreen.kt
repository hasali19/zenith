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
import io.ktor.client.*
import io.ktor.client.request.*
import kotlinx.datetime.Instant
import kotlinx.datetime.TimeZone
import kotlinx.datetime.toLocalDateTime
import uk.hasali.zenith.*

@OptIn(ExperimentalFoundationApi::class)
@Composable
fun ShowsScreen(client: HttpClient, navigator: Navigator) {
    val shows by produceState(initialValue = emptyList<Show>()) {
        value = client.get("https://zenith.hasali.uk/api/tv/shows")
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
