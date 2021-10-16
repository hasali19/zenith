package uk.hasali.zenith

import android.os.Parcelable
import androidx.compose.animation.*
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Dns
import androidx.compose.material.icons.filled.Info
import androidx.compose.material.icons.filled.VideoLibrary
import androidx.compose.runtime.*
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.saveable.rememberSaveableStateHolder
import androidx.compose.ui.Modifier
import com.google.accompanist.insets.navigationBarsPadding
import kotlinx.parcelize.Parcelize
import uk.hasali.zenith.navigation.ContentHost
import uk.hasali.zenith.navigation.StackNavigator
import uk.hasali.zenith.navigation.rememberStackNavigator
import uk.hasali.zenith.screens.*
import uk.hasali.zenith.screens.player.MediaItemType
import uk.hasali.zenith.screens.player.PlayerScreen

sealed class Screen(val route: String) {
    override fun equals(other: Any?) = other is Screen && other.route == route
    override fun hashCode() = route.hashCode()
    override fun toString() = "Screen=$route"
}

sealed interface TopLevelScreen : Parcelable {
    @Parcelize
    object Main : Screen("main"), TopLevelScreen

    @Parcelize
    class Player(val id: Int, val type: MediaItemType, val position: Double?) :
        Screen("player/$id/$type/$position"), TopLevelScreen
}

sealed interface BottomNavigationArea : Parcelable {
    @Parcelize
    object Library : Screen("main/library"), BottomNavigationArea

    @Parcelize
    object Management : Screen("main/management"), BottomNavigationArea

    @Parcelize
    object About : Screen("main/about"), BottomNavigationArea
}

sealed interface LibraryScreen : Parcelable {
    @Parcelize
    object Home : Screen("main/library/home"), LibraryScreen

    @Parcelize
    object Movies : Screen("main/library/movies"), LibraryScreen

    @Parcelize
    object Shows : Screen("main/library/shows"), LibraryScreen

    @Parcelize
    class MovieDetails(val id: Int) : Screen("main/library/movies/$id"), LibraryScreen

    @Parcelize
    class ShowDetails(val id: Int) : Screen("main/library/shows/$id"), LibraryScreen

    @Parcelize
    class SeasonDetails(val id: Int) : Screen("main/library/seasons/$id"), LibraryScreen

    @Parcelize
    class EpisodeDetails(val id: Int) : Screen("main/library/episodes/$id"), LibraryScreen
}

sealed interface ManagementScreen : Parcelable {
    @Parcelize
    object Home : Screen("main/management/home"), ManagementScreen

    @Parcelize
    object ImportQueue : Screen("main/management/import"), ManagementScreen

    @Parcelize
    object TranscodeQueue : Screen("main/management/transcode"), ManagementScreen
}

@OptIn(ExperimentalAnimationApi::class)
@Composable
fun AppNavigation(navigator: StackNavigator<TopLevelScreen>) {
    navigator.ContentHost { screen ->
        when (screen) {
            is TopLevelScreen.Main -> {
                TopLevelMain(
                    onNavigateToPlayer = { id, type, pos ->
                        navigator.push(TopLevelScreen.Player(id, type, pos))
                    },
                )
            }

            is TopLevelScreen.Player -> PlayerScreen(
                id = screen.id,
                type = screen.type,
                startPosition = screen.position,
                onNavigateUp = { navigator.pop() },
            )
        }
    }
}

@Composable
private fun TopLevelMain(onNavigateToPlayer: (Int, MediaItemType, Double?) -> Unit) {
    val holder = rememberSaveableStateHolder()
    var area by rememberSaveable { mutableStateOf<BottomNavigationArea>(BottomNavigationArea.Library) }

    val libraryNavigator = rememberStackNavigator<LibraryScreen>(LibraryScreen.Home)
    val managementNavigator = rememberStackNavigator<ManagementScreen>(ManagementScreen.Home)

    // TODO: Implement a proper tab navigator
    Column(modifier = Modifier.navigationBarsPadding()) {
        Box(modifier = Modifier.weight(1f)) {
            Crossfade(area) { area ->
                holder.SaveableStateProvider(area) {
                    when (area) {
                        is BottomNavigationArea.Library -> LibraryArea(
                            libraryNavigator,
                            onNavigateToPlayer
                        )
                        is BottomNavigationArea.Management -> ManagementArea(managementNavigator)
                        is BottomNavigationArea.About -> AboutScreen()
                    }
                }
            }
        }

        BottomNavigation {
            BottomNavigationItem(
                selected = area == BottomNavigationArea.Library,
                onClick = {
                    if (area != BottomNavigationArea.Library)
                        area = BottomNavigationArea.Library
                    else
                        libraryNavigator.popAll()
                },
                icon = { Icon(Icons.Default.VideoLibrary, null) },
                label = { Text("Library") },
            )
            BottomNavigationItem(
                selected = area == BottomNavigationArea.Management,
                onClick = {
                    if (area != BottomNavigationArea.Management)
                        area = BottomNavigationArea.Management
                    else
                        managementNavigator.popAll()
                },
                icon = { Icon(Icons.Default.Dns, null) },
                label = { Text("Manage") },
            )
            BottomNavigationItem(
                selected = area == BottomNavigationArea.About,
                onClick = { area = BottomNavigationArea.About },
                icon = { Icon(Icons.Default.Info, null) },
                label = { Text("About") },
            )
        }
    }
}

@Composable
private fun LibraryArea(
    navigator: StackNavigator<LibraryScreen>,
    onNavigateToPlayer: (Int, MediaItemType, Double?) -> Unit,
) {
    navigator.ContentHost { screen ->
        when (screen) {
            is LibraryScreen.Home -> LibraryHomeScreen(
                onNavigateToMovies = { navigator.push(LibraryScreen.Movies) },
                onNavigateToShows = { navigator.push(LibraryScreen.Shows) },
                onNavigateToMovie = { navigator.push(LibraryScreen.MovieDetails(it.id)) },
                onNavigateToShow = { navigator.push(LibraryScreen.ShowDetails(it.id)) },
            )

            is LibraryScreen.Movies -> MoviesScreen(
                onNavigateToMovie = { navigator.push(LibraryScreen.MovieDetails(it.id)) },
                onNavigateUp = { navigator.pop() },
            )

            is LibraryScreen.Shows -> ShowsScreen(
                onNavigateToShow = { navigator.push(LibraryScreen.ShowDetails(it.id)) },
                onNavigateUp = { navigator.pop() },
            )

            is LibraryScreen.MovieDetails -> MovieDetailsScreen(
                id = screen.id,
                onPlay = { onNavigateToPlayer(screen.id, MediaItemType.Movie, it) },
                onNavigateUp = { navigator.pop() },
            )

            is LibraryScreen.ShowDetails -> ShowDetailsScreen(
                id = screen.id,
                onNavigateToSeason = { navigator.push(LibraryScreen.SeasonDetails(it.id)) },
                onNavigateUp = { navigator.pop() },
            )

            is LibraryScreen.SeasonDetails -> SeasonDetailsScreen(
                id = screen.id,
                onNavigateToEpisode = { navigator.push(LibraryScreen.EpisodeDetails(it.id)) },
                onNavigateUp = { navigator.pop() },
            )

            is LibraryScreen.EpisodeDetails -> EpisodeDetailsScreen(
                id = screen.id,
                onPlay = { onNavigateToPlayer(screen.id, MediaItemType.TvShow, it) },
                onNavigateUp = { navigator.pop() },
            )
        }
    }
}

@Composable
private fun ManagementArea(navigator: StackNavigator<ManagementScreen>) {
    navigator.ContentHost { screen ->
        when (screen) {
            is ManagementScreen.Home -> ManagementHomeScreen(
                onNavigateToImportQueue = { navigator.push(ManagementScreen.ImportQueue) },
                onNavigateToTranscodeQueue = { navigator.push(ManagementScreen.TranscodeQueue) },
            )

            is ManagementScreen.ImportQueue -> ImportQueueScreen(
                onNavigateUp = { navigator.pop() },
            )

            is ManagementScreen.TranscodeQueue -> TranscodeQueueScreen(
                onNavigateUp = { navigator.pop() },
            )
        }
    }
}
