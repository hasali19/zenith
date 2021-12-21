package uk.hasali.zenith

import android.os.Parcelable
import androidx.compose.animation.Crossfade
import androidx.compose.animation.ExperimentalAnimationApi
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Dns
import androidx.compose.material.icons.filled.Settings
import androidx.compose.material.icons.filled.VideoLibrary
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.saveable.rememberSaveableStateHolder
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import com.google.accompanist.insets.navigationBarsPadding
import kotlinx.parcelize.Parcelize
import uk.hasali.zenith.navigation.ContentHost
import uk.hasali.zenith.navigation.StackNavigator
import uk.hasali.zenith.navigation.rememberStackNavigator
import uk.hasali.zenith.screens.SettingsScreen
import uk.hasali.zenith.screens.library.home.LibraryHomeScreen
import uk.hasali.zenith.screens.library.itemdetails.ItemDetailsScreen
import uk.hasali.zenith.screens.library.movies.MoviesScreen
import uk.hasali.zenith.screens.library.shows.ShowsScreen
import uk.hasali.zenith.screens.management.ImportQueueScreen
import uk.hasali.zenith.screens.management.ManagementHomeScreen
import uk.hasali.zenith.screens.management.TranscodeQueueScreen
import uk.hasali.zenith.screens.player.VideoItemType
import uk.hasali.zenith.screens.player.VideoPlayerScreen
import uk.hasali.zenith.ui.BottomSheetController
import uk.hasali.zenith.ui.rememberBottomSheetController

sealed class Screen(val route: String) {
    override fun equals(other: Any?) = other is Screen && other.route == route
    override fun hashCode() = route.hashCode()
    override fun toString() = "Screen=$route"
}

sealed interface PrimaryScreen : Parcelable {
    @Parcelize
    object Main : Screen("main"), PrimaryScreen

    @Parcelize
    data class VideoPlayer(val id: Int, val type: VideoItemType, val position: Double?) :
        Screen("video_player"), PrimaryScreen
}

sealed interface BottomNavigationArea : Parcelable {
    @Parcelize
    object Library : Screen("main/library"), BottomNavigationArea

    @Parcelize
    object Management : Screen("main/management"), BottomNavigationArea

    @Parcelize
    object Settings : Screen("main/about"), BottomNavigationArea
}

sealed interface LibraryScreen : Parcelable {
    @Parcelize
    object Home : Screen("main/library/home"), LibraryScreen

    @Parcelize
    object Movies : Screen("main/library/movies"), LibraryScreen

    @Parcelize
    object Shows : Screen("main/library/shows"), LibraryScreen

    @Parcelize
    class ItemDetails(val id: Int) : Screen("main/library/items/$id"), LibraryScreen
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
fun AppNavigation(onLaunchSelectServer: () -> Unit) {
    val navigator = rememberStackNavigator<PrimaryScreen>(PrimaryScreen.Main)

    navigator.ContentHost {
        when (it) {
            is PrimaryScreen.Main -> MainNavigation(
                onNavigateToPlayer = { id, type, position ->
                    navigator.push(PrimaryScreen.VideoPlayer(id, type, position))
                },
                onLaunchSelectServer = onLaunchSelectServer,
            )

            is PrimaryScreen.VideoPlayer -> VideoPlayerScreen()
        }
    }
}

@OptIn(ExperimentalMaterialApi::class)
@Composable
private fun MainNavigation(
    onNavigateToPlayer: (Int, VideoItemType, Double?) -> Unit,
    onLaunchSelectServer: () -> Unit,
) {
    val holder = rememberSaveableStateHolder()
    var area by rememberSaveable { mutableStateOf<BottomNavigationArea>(BottomNavigationArea.Library) }

    val libraryNavigator = rememberStackNavigator<LibraryScreen>(LibraryScreen.Home)
    val managementNavigator = rememberStackNavigator<ManagementScreen>(ManagementScreen.Home)

    val bottomSheetController = rememberBottomSheetController()

    ModalBottomSheetLayout(
        sheetState = bottomSheetController.state,
        sheetContent = { bottomSheetController.Content(this) },
        modifier = Modifier.navigationBarsPadding(),
    ) {
        Column {
            Box(modifier = Modifier.weight(1f)) {
                Crossfade(area) { area ->
                    holder.SaveableStateProvider(area) {
                        when (area) {
                            is BottomNavigationArea.Library -> LibraryArea(
                                navigator = libraryNavigator,
                                bottomSheetController = bottomSheetController,
                                onNavigateToPlayer = onNavigateToPlayer,
                            )
                            is BottomNavigationArea.Management -> ManagementArea(
                                navigator = managementNavigator,
                            )
                            is BottomNavigationArea.Settings -> SettingsScreen(
                                onLaunchSelectServer = onLaunchSelectServer,
                            )
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
                    selected = area == BottomNavigationArea.Settings,
                    onClick = { area = BottomNavigationArea.Settings },
                    icon = { Icon(Icons.Default.Settings, null) },
                    label = { Text("Settings") },
                )
            }
        }
    }
}

@Composable
private fun LibraryArea(
    navigator: StackNavigator<LibraryScreen>,
    bottomSheetController: BottomSheetController,
    onNavigateToPlayer: (Int, VideoItemType, Double?) -> Unit,
) {
    navigator.ContentHost { screen ->
        when (screen) {
            is LibraryScreen.Home -> LibraryHomeScreen(
                onNavigateToMovies = { navigator.push(LibraryScreen.Movies) },
                onNavigateToShows = { navigator.push(LibraryScreen.Shows) },
                onNavigateToItem = { navigator.push(LibraryScreen.ItemDetails(it)) },
            )

            is LibraryScreen.Movies -> MoviesScreen(
                onNavigateToItem = { navigator.push(LibraryScreen.ItemDetails(it)) },
                onNavigateUp = { navigator.pop() },
            )

            is LibraryScreen.Shows -> ShowsScreen(
                onNavigateToItem = { navigator.push(LibraryScreen.ItemDetails(it)) },
                onNavigateUp = { navigator.pop() },
            )

            is LibraryScreen.ItemDetails -> ItemDetailsScreen(
                bottomSheetController = bottomSheetController,
                onPlay = { onNavigateToPlayer(screen.id, VideoItemType.TvShow, it) },
                onNavigateToItem = { navigator.push(LibraryScreen.ItemDetails(it)) },
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
