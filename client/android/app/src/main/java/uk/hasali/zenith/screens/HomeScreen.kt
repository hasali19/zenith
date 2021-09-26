package uk.hasali.zenith.screens

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyRow
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.google.accompanist.swiperefresh.SwipeRefresh
import com.google.accompanist.swiperefresh.rememberSwipeRefreshState
import kotlinx.coroutines.launch
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
    val scope = rememberCoroutineScope()
    val client = LocalZenithClient.current

    var isRefreshing by remember { mutableStateOf(true) }
    var movies by remember { mutableStateOf(emptyList<Movie>()) }
    var shows by remember { mutableStateOf(emptyList<Show>()) }

    suspend fun refresh() {
        isRefreshing = true
        movies = client.getRecentMovies()
        shows = client.getRecentShows()
        isRefreshing = false
    }

    LaunchedEffect(Unit) {
        refresh()
    }

    HomeScreen(
        movies = movies,
        shows = shows,
        isRefreshing = isRefreshing,
        onRefresh = { scope.launch { refresh() } },
        onNavigateToMovie = onNavigateToMovie,
        onNavigateToShow = onNavigateToShow,
    )
}

@Composable
private fun HomeScreen(
    movies: List<Movie>,
    shows: List<Show>,
    isRefreshing: Boolean,
    onRefresh: () -> Unit,
    onNavigateToMovie: (Movie) -> Unit,
    onNavigateToShow: (Show) -> Unit,
) {
    SwipeRefresh(state = rememberSwipeRefreshState(isRefreshing), onRefresh = onRefresh) {
        Column(
            modifier = Modifier
                .fillMaxSize()
                .verticalScroll(state = rememberScrollState()),
        ) {
            if (movies.isNotEmpty()) {
                Section(
                    title = "Recently Added Movies",
                    items = movies,
                    poster = { it.poster },
                    name = { it.title },
                    date = { it.releaseDate },
                    isWatched = { it.userData.isWatched },
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
                    isWatched = { it.userData.unwatched == 0 },
                    onClick = onNavigateToShow,
                )
            }
        }
    }
}

@Composable
private fun <T> Section(
    title: String,
    items: List<T>,
    poster: (T) -> String?,
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
