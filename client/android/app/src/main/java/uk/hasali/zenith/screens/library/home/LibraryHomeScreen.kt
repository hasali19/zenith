package uk.hasali.zenith.screens.library.home

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyRow
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.*
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.lifecycle.Lifecycle
import com.google.accompanist.swiperefresh.SwipeRefresh
import com.google.accompanist.swiperefresh.rememberSwipeRefreshState
import kotlinx.datetime.Instant
import kotlinx.datetime.TimeZone
import kotlinx.datetime.toLocalDateTime
import uk.hasali.zenith.api.Movie
import uk.hasali.zenith.api.Show
import uk.hasali.zenith.navigation.hiltViewModel
import uk.hasali.zenith.ui.*

@Composable
fun LibraryHomeScreen(
    model: LibraryHomeViewModel = hiltViewModel(),
    onNavigateToMovies: () -> Unit,
    onNavigateToShows: () -> Unit,
    onNavigateToMovie: (Movie) -> Unit,
    onNavigateToShow: (Show) -> Unit,
) {
    val state by rememberFlowWithLifecycle(model.state)
        .collectAsState(LibraryHomeViewState())

    LifecycleEffect(Lifecycle.State.RESUMED) {
        model.refresh()
    }

    Scaffold(
        topBar = {
            AppBar(title = "Zenith") {
                CastButton()
            }
        },
    ) {
        if (state.isError) {
            Box(modifier = Modifier.fillMaxSize()) {
                Column(
                    horizontalAlignment = Alignment.CenterHorizontally,
                    modifier = Modifier.align(Alignment.Center),
                ) {
                    Text("Failed to get data from server")
                    OutlinedButton(enabled = !state.isRefreshing, onClick = { model.refresh() }) {
                        Text("Retry")
                    }
                }
            }
        } else {
            LibraryHomeScreen(
                movies = state.movies,
                shows = state.shows,
                isRefreshing = state.isRefreshing,
                onRefresh = model::refresh,
                onNavigateToMovies = onNavigateToMovies,
                onNavigateToShows = onNavigateToShows,
                onNavigateToMovie = onNavigateToMovie,
                onNavigateToShow = onNavigateToShow,
            )
        }
    }
}

@Composable
private fun LibraryHomeScreen(
    movies: List<Movie>,
    shows: List<Show>,
    isRefreshing: Boolean,
    onRefresh: () -> Unit,
    onNavigateToMovies: () -> Unit,
    onNavigateToShows: () -> Unit,
    onNavigateToMovie: (Movie) -> Unit,
    onNavigateToShow: (Show) -> Unit,
) {
    SwipeRefresh(state = rememberSwipeRefreshState(isRefreshing), onRefresh = onRefresh) {
        Column(
            modifier = Modifier
                .fillMaxSize()
                .verticalScroll(state = rememberScrollState()),
        ) {
            Row(modifier = Modifier.padding(top = 12.dp, start = 12.dp, end = 12.dp)) {
                OutlinedButton(modifier = Modifier.weight(1f), onClick = onNavigateToMovies) {
                    Text("Movies")
                }

                Spacer(modifier = Modifier.width(16.dp))

                OutlinedButton(modifier = Modifier.weight(1f), onClick = onNavigateToShows) {
                    Text("Shows")
                }
            }

            if (movies.isNotEmpty()) {
                Section(
                    title = "Recently Added Movies",
                    items = movies,
                    poster = { it.poster },
                    name = { it.title },
                    date = { it.releaseDate },
                    isWatched = { it.userData.isWatched },
                    onItemClick = onNavigateToMovie,
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
                    onItemClick = onNavigateToShow,
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
    onItemClick: (T) -> Unit,
) {
    Column {
        Text(
            text = title,
            style = MaterialTheme.typography.subtitle1,
            fontWeight = FontWeight.Bold,
            modifier = Modifier
                .fillMaxWidth()
                .padding(top = 8.dp, bottom = 4.dp)
                .padding(horizontal = 12.dp, vertical = 8.dp),
        )

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
                    onClick = { onItemClick(item) },
                    modifier = Modifier
                        .width(120.dp)
                        .padding(4.dp),
                )
            }
        }
    }
}
