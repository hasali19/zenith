package uk.hasali.zenith.screens

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyRow
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.produceState
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import kotlinx.datetime.Instant
import kotlinx.datetime.TimeZone
import kotlinx.datetime.toLocalDateTime
import uk.hasali.zenith.Movie
import uk.hasali.zenith.Show
import uk.hasali.zenith.ui.LocalZenithClient
import uk.hasali.zenith.ui.MediaItemWithPoster

@Composable
fun HomeScreen(
    onNavigateToMovie: (Movie) -> Unit,
    onNavigateToShow: (Show) -> Unit,
) {
    val client = LocalZenithClient.current

    val movies by produceState(emptyList<Movie>()) {
        value = client.getRecentMovies()
    }

    val shows by produceState(initialValue = emptyList<Show>()) {
        value = client.getRecentShows()
    }

    Column(
        modifier = Modifier
            .fillMaxWidth()
            .verticalScroll(state = rememberScrollState()),
    ) {
        if (movies.isNotEmpty()) {
            Section(
                title = "Recently Added Movies",
                items = movies,
                poster = { it.poster },
                name = { it.title },
                date = { it.releaseDate },
                isWatched = { it.isWatched },
                onClick = onNavigateToMovie,
            )
        }

        if (shows.isNotEmpty()) {
            Section(
                title = "Recently Updated Shows",
                items = shows,
                poster = { it.poster },
                name = { it.name },
                date = { it.startDate },
                isWatched = { it.unwatchedEpisodes == 0 },
                onClick = onNavigateToShow,
            )
        }
    }
}

@Composable
private fun <T> Section(
    title: String,
    items: List<T>,
    poster: (T) -> String,
    name: (T) -> String,
    date: (T) -> Long?,
    isWatched: (T) -> Boolean = { false },
    onClick: (T) -> Unit,
) {
    Column(modifier = Modifier.padding(top = 16.dp)) {
        Text(
            text = title,
            style = MaterialTheme.typography.h6,
            modifier = Modifier.padding(horizontal = 12.dp),
        )

        Spacer(modifier = Modifier.height(8.dp))

        LazyRow(contentPadding = PaddingValues(horizontal = 8.dp)) {
            items(items) { item ->
                val dateVal = date(item)
                val year = if (dateVal == null) null else
                    Instant.fromEpochSeconds(dateVal)
                        .toLocalDateTime(TimeZone.UTC)
                        .year

                MediaItemWithPoster(
                    poster = poster(item),
                    primary = name(item),
                    secondary = year?.toString() ?: "",
                    isWatched = isWatched(item),
                    onClick = { onClick(item) },
                    modifier = Modifier
                        .width(120.dp)
                        .padding(4.dp),
                )
            }
        }
    }
}
