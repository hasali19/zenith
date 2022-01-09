package uk.hasali.zenith.screens.library.home

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyRow
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.MaterialTheme
import androidx.compose.material.OutlinedButton
import androidx.compose.material.Scaffold
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import androidx.lifecycle.Lifecycle
import com.google.accompanist.swiperefresh.SwipeRefresh
import com.google.accompanist.swiperefresh.rememberSwipeRefreshState
import uk.hasali.zenith.api.*
import uk.hasali.zenith.navigation.hiltViewModel
import uk.hasali.zenith.ui.*

@Composable
fun LibraryHomeScreen(
    model: LibraryHomeViewModel = hiltViewModel(),
    onNavigateToMovies: () -> Unit,
    onNavigateToShows: () -> Unit,
    onNavigateToItem: (id: Int) -> Unit,
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
                continueWatching = state.continueWatching,
                movies = state.recentMovies,
                shows = state.recentShows,
                isRefreshing = state.isRefreshing,
                onRefresh = model::refresh,
                onNavigateToMovies = onNavigateToMovies,
                onNavigateToShows = onNavigateToShows,
                onNavigateToItem = onNavigateToItem,
            )
        }
    }
}

@Composable
private fun LibraryHomeScreen(
    continueWatching: List<MediaItem>,
    movies: List<Movie>,
    shows: List<Show>,
    isRefreshing: Boolean,
    onRefresh: () -> Unit,
    onNavigateToMovies: () -> Unit,
    onNavigateToShows: () -> Unit,
    onNavigateToItem: (id: Int) -> Unit,
) {
    SwipeRefresh(state = rememberSwipeRefreshState(isRefreshing), onRefresh = onRefresh) {
        Column(
            modifier = Modifier
                .fillMaxSize()
                .verticalScroll(state = rememberScrollState())
                .padding(bottom = 8.dp),
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

            if (continueWatching.isNotEmpty()) {
                Section(title = "Continue Watching") {
                    ContinueWatchingList(
                        items = continueWatching,
                        onItemClick = { onNavigateToItem(it.id) },
                    )
                }
            }

            if (movies.isNotEmpty()) {
                Section(
                    title = "Recently Added Movies",
                    items = movies,
                    poster = { it.poster },
                    name = { it.title },
                    year = { it.releaseYear() },
                    isWatched = { it.userData.isWatched },
                    onItemClick = { onNavigateToItem(it.id) },
                )
            }

            if (shows.isNotEmpty()) {
                Section(
                    title = "Recently Updated Shows",
                    items = shows,
                    poster = { it.poster },
                    name = { it.name },
                    year = { it.startYear() },
                    isWatched = { it.userData.unwatched == 0 },
                    onItemClick = { onNavigateToItem(it.id) },
                )
            }
        }
    }
}

@Composable
private fun Section(title: String, content: @Composable () -> Unit) {
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

        content()
    }
}

@Composable
private fun ContinueWatchingList(items: List<MediaItem>, onItemClick: (MediaItem) -> Unit) {
    LazyRow(contentPadding = PaddingValues(horizontal = 8.dp)) {
        items(items) { item ->
            val title: String
            val subtitle: String?
            val image: String?
            val userData: VideoUserData
            val videoInfo: VideoInfo

            when (item) {
                is Movie -> {
                    title = item.title
                    subtitle = item.releaseYear()?.toString()
                    image = item.backdrop
                    userData = item.userData
                    videoInfo = item.videoInfo
                }
                is Episode -> {
                    title = item.showName
                    subtitle = item.seasonEpisodeString()
                    image = item.thumbnail
                    userData = item.userData
                    videoInfo = item.videoInfo
                }
                else -> throw IllegalArgumentException("Invalid item type")
            }

            val progress =
                ((userData.position ?: 0.0) / videoInfo.duration).toFloat()

            Thumbnail(
                url = image,
                overlay = {
                    Column(modifier = Modifier.fillMaxSize()) {
                        Column(
                            verticalArrangement = Arrangement.Bottom,
                            modifier = Modifier
                                .fillMaxWidth()
                                .weight(1f)
                        ) {
                            Text(
                                text = title,
                                style = MaterialTheme.typography.subtitle2,
                                overflow = TextOverflow.Ellipsis,
                                maxLines = 1,
                                modifier = Modifier.padding(horizontal = 8.dp),
                            )

                            if (subtitle != null) {
                                Text(
                                    text = subtitle,
                                    style = MaterialTheme.typography.caption,
                                    overflow = TextOverflow.Ellipsis,
                                    maxLines = 1,
                                    modifier = Modifier.padding(horizontal = 8.dp)
                                )
                            }
                        }

                        Box(
                            modifier = Modifier
                                .padding(8.dp)
                                .fillMaxWidth()
                                .height(2.dp)
                                .clip(RoundedCornerShape(50))
                                .background(Color.White)
                        ) {
                            Box(
                                modifier = Modifier
                                    .fillMaxWidth(progress)
                                    .fillMaxHeight()
                                    .background(MaterialTheme.colors.primary)
                            )
                        }
                    }
                },
                modifier = Modifier
                    .width(280.dp)
                    .padding(4.dp),
                onClick = { onItemClick(item) },
            )
        }
    }
}

@Composable
private fun <T> Section(
    title: String,
    items: List<T>,
    poster: (T) -> String?,
    name: (T) -> String,
    year: (T) -> Int?,
    isWatched: (T) -> Boolean = { false },
    onItemClick: (T) -> Unit,
) {
    Section(title = title) {
        LazyRow(contentPadding = PaddingValues(horizontal = 8.dp)) {
            items(items) { item ->
                MediaItemWithPoster(
                    poster = poster(item),
                    primary = name(item),
                    secondary = year(item)?.toString() ?: "",
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
