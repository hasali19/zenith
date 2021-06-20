package uk.hasali.zenith.ui

import androidx.compose.animation.Crossfade
import androidx.compose.foundation.ExperimentalFoundationApi
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.GridCells
import androidx.compose.foundation.lazy.LazyRow
import androidx.compose.foundation.lazy.LazyVerticalGrid
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Home
import androidx.compose.material.icons.filled.Movie
import androidx.compose.material.icons.filled.Tv
import androidx.compose.runtime.*
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.unit.dp
import com.google.accompanist.insets.navigationBarsPadding
import kotlinx.datetime.Instant
import kotlinx.datetime.TimeZone
import kotlinx.datetime.toLocalDateTime
import uk.hasali.zenith.Movie
import uk.hasali.zenith.Show
import uk.hasali.zenith.ZenithApiClient
import uk.hasali.zenith.playClick

private enum class MainScreen {
    Home,
    Movies,
    Shows,
}

@Composable
fun MainScreen(client: ZenithApiClient, navigator: Navigator) {
    var screen by rememberSaveable { mutableStateOf(MainScreen.Home) }

    Scaffold(
        modifier = Modifier.navigationBarsPadding(),
        topBar = { AppBar(navigator = navigator) },
        bottomBar = {
            MainBottomNavigation(screen = screen) {
                screen = it
            }
        }
    ) { padding ->
        Box(modifier = Modifier.padding(padding)) {
            Crossfade(targetState = screen) {
                when (it) {
                    MainScreen.Home -> HomeScreen(client = client, navigator = navigator)
                    MainScreen.Movies -> MoviesScreen(client = client, navigator = navigator)
                    MainScreen.Shows -> ShowsScreen(client = client, navigator = navigator)
                }
            }
        }
    }
}

@Composable
private fun HomeScreen(client: ZenithApiClient, navigator: Navigator) {
    val movies by produceState(emptyList<Movie>()) {
        value = client.getRecentMovies()
    }

    val shows by produceState(initialValue = emptyList<Show>()) {
        value = client.getRecentShows()
    }

    Column(
        modifier = Modifier
            .fillMaxWidth()
            .verticalScroll(state = rememberSaveableScrollState()),
    ) {
        if (movies.isNotEmpty()) {
            Text(
                text = "Recently Added Movies",
                style = MaterialTheme.typography.h6,
                modifier = Modifier
                    .padding(horizontal = 12.dp)
                    .padding(top = 16.dp, bottom = 8.dp),
            )
            LazyRow(
                state = rememberSaveableLazyListState(),
                contentPadding = PaddingValues(horizontal = 8.dp),
            ) {
                items(movies.size) { i ->
                    val item = movies[i]
                    val year = Instant.fromEpochSeconds(item.releaseDate)
                        .toLocalDateTime(TimeZone.UTC)
                        .year

                    MediaItemWithPoster(
                        poster = item.poster,
                        primary = item.title,
                        secondary = year.toString(),
                        onClick = { navigator.push(Screen.Player(item.id)) },
                        modifier = Modifier
                            .width(120.dp)
                            .padding(4.dp),
                    )
                }
            }
        }

        if (shows.isNotEmpty()) {
            Text(
                text = "Recently Updated Shows",
                style = MaterialTheme.typography.h6,
                modifier = Modifier
                    .padding(horizontal = 12.dp)
                    .padding(top = 16.dp, bottom = 8.dp),
            )
            LazyRow(
                state = rememberSaveableLazyListState(),
                contentPadding = PaddingValues(horizontal = 8.dp),
            ) {
                items(shows.size) { i ->
                    val item = shows[i]
                    val year = Instant.fromEpochSeconds(item.startDate)
                        .toLocalDateTime(TimeZone.UTC)
                        .year

                    MediaItemWithPoster(
                        poster = item.poster,
                        primary = item.name,
                        secondary = year.toString(),
                        onClick = { navigator.push(Screen.ShowDetails(item)) },
                        modifier = Modifier
                            .width(120.dp)
                            .padding(4.dp),
                    )
                }
            }
        }
    }
}

@Composable
private fun MoviesScreen(client: ZenithApiClient, navigator: Navigator) {
    val movies by produceState(emptyList<Movie>()) {
        value = client.getMovies()
    }

    ListScreen(
        items = movies,
        poster = { it.poster },
        name = { it.title },
        date = { it.releaseDate },
        onClick = { navigator.push(Screen.Player(it.id)) },
    )
}

@Composable
private fun ShowsScreen(client: ZenithApiClient, navigator: Navigator) {
    val shows by produceState(initialValue = emptyList<Show>()) {
        value = client.getShows()
    }

    ListScreen(
        items = shows,
        poster = { it.poster },
        name = { it.name },
        date = { it.startDate },
        onClick = { navigator.push(Screen.ShowDetails(it)) },
    )
}

@Composable
private fun MainBottomNavigation(screen: MainScreen, onChangeScreen: (MainScreen) -> Unit) {
    @Composable
    fun RowScope.NavigationItem(name: String, icon: ImageVector, value: MainScreen) {
        val context = LocalContext.current
        BottomNavigationItem(
            selected = screen == value,
            icon = { Icon(icon, contentDescription = name) },
            label = { Text(name) },
            onClick = {
                context.playClick()
                onChangeScreen(value)
            },
        )
    }

    BottomNavigation {
        NavigationItem(name = "Home", icon = Icons.Default.Home, value = MainScreen.Home)
        NavigationItem(name = "Movies", icon = Icons.Default.Movie, value = MainScreen.Movies)
        NavigationItem(name = "Shows", icon = Icons.Default.Tv, value = MainScreen.Shows)
    }
}

@OptIn(ExperimentalFoundationApi::class)
@Composable
private fun <T> ListScreen(
    items: List<T>,
    poster: (T) -> String,
    name: (T) -> String,
    date: (T) -> Long,
    onClick: (T) -> Unit,
) {
    LazyVerticalGrid(
        cells = GridCells.Adaptive(120.dp),
        contentPadding = PaddingValues(4.dp),
        state = rememberSaveableLazyListState(),
    ) {
        items(items.size) { i ->
            val item = items[i]
            val year = Instant.fromEpochSeconds(date(item))
                .toLocalDateTime(TimeZone.UTC)
                .year

            MediaItemWithPoster(
                poster = poster(item),
                primary = name(item),
                secondary = year.toString(),
                onClick = { onClick(item) },
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(4.dp),
            )
        }
    }
}
