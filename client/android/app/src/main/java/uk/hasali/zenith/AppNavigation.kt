package uk.hasali.zenith

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.RowScope
import androidx.compose.material.BottomNavigation
import androidx.compose.material.BottomNavigationItem
import androidx.compose.material.Icon
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Dns
import androidx.compose.material.icons.filled.Info
import androidx.compose.material.icons.filled.VideoLibrary
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.compose.ui.platform.LocalContext
import androidx.navigation.NavHostController
import androidx.navigation.NavType
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.currentBackStackEntryAsState
import androidx.navigation.compose.rememberNavController
import androidx.navigation.navArgument
import androidx.navigation.navigation
import com.google.accompanist.insets.navigationBarsPadding
import uk.hasali.zenith.screens.*
import uk.hasali.zenith.screens.player.MediaItemType
import uk.hasali.zenith.screens.player.PlayerScreen

@Composable
fun AppNavigation(topLevelNav: NavHostController) {
    val navigateToPlayer = { id: Int, type: String, position: Double? ->
        var route = "player/$type/$id"
        if (position != null) route += "?position=$position"
        topLevelNav.navigate(route)
    }

    NavHost(
        navController = topLevelNav,
        startDestination = "main",
    ) {
        composable("main") {
            MainNavigation(
                onNavigateToPlayer = navigateToPlayer,
            )
        }

        composable("player/{type}/{id}?position={position}", arguments = listOf(
            navArgument("type") { type = NavType.StringType },
            navArgument("id") { type = NavType.IntType },
            navArgument("position") {
                type = NavType.FloatType
                defaultValue = 0.0
            }
        )) {
            val args = it.arguments!!

            val id = args.getInt("id")
            val position = args.getFloat("position")
            val type = when (val type = args.getString("type")) {
                "movie" -> MediaItemType.Movie
                "show" -> MediaItemType.TvShow
                else -> throw IllegalArgumentException("Invalid item type: $type")
            }

            PlayerScreen(
                id = id,
                type = type,
                startPosition = position.toDouble(),
                onNavigateUp = { topLevelNav.popBackStack() },
            )
        }
    }
}

@Composable
private fun MainNavigation(
    onNavigateToPlayer: (id: Int, type: String, position: Double?) -> Unit,
) {
    val nav = rememberNavController()

    Column(modifier = Modifier.navigationBarsPadding()) {
        NavHost(nav, startDestination = "library", modifier = Modifier.weight(1f)) {
            navigation(route = "library", startDestination = "library/home") {
                composable("library/home") {
                    LibraryHomeScreen(
                        onNavigateToMovies = { nav.navigate("library/movies") },
                        onNavigateToShows = { nav.navigate("library/shows") },
                        onNavigateToMovie = { movie -> nav.navigate("library/movie_details/${movie.id}") },
                        onNavigateToShow = { show -> nav.navigate("library/show_details/${show.id}") },
                    )
                }

                composable("library/movies") {
                    MoviesScreen(
                        onNavigateToMovie = { movie -> nav.navigate("library/movie_details/${movie.id}") },
                        onNavigateUp = { nav.popBackStack() },
                    )
                }

                composable("library/shows") {
                    ShowsScreen(
                        onNavigateToShow = { show -> nav.navigate("library/show_details/${show.id}") },
                        onNavigateUp = { nav.popBackStack() },
                    )
                }

                composable(
                    "library/movie_details/{id}", arguments = listOf(
                        navArgument("id") { type = NavType.IntType },
                    )
                ) {
                    val args = it.arguments!!
                    val id = args.getInt("id")

                    MovieDetailsScreen(
                        id = id,
                        onPlay = { position -> onNavigateToPlayer(id, "movie", position) },
                        onNavigateUp = { nav.popBackStack() },
                    )
                }

                composable(
                    "library/show_details/{id}", arguments = listOf(
                        navArgument("id") { type = NavType.IntType },
                    )
                ) {
                    val args = it.arguments!!
                    val id = args.getInt("id")

                    ShowDetailsScreen(
                        id = id,
                        onNavigateToSeason = { season -> nav.navigate("library/season_details/${season.id}") },
                        onNavigateUp = { nav.popBackStack() },
                    )
                }

                composable(
                    "library/season_details/{id}", arguments = listOf(
                        navArgument("id") { type = NavType.IntType },
                    )
                ) {
                    val args = it.arguments!!
                    val id = args.getInt("id")

                    SeasonDetailsScreen(
                        id = id,
                        onNavigateToEpisode = { episode -> nav.navigate("library/episode_details/${episode.id}") },
                        onNavigateUp = { nav.popBackStack() },
                    )
                }

                composable(
                    "library/episode_details/{id}", arguments = listOf(
                        navArgument("id") { type = NavType.IntType },
                    )
                ) {
                    val args = it.arguments!!
                    val id = args.getInt("id")

                    EpisodeDetailsScreen(
                        id = id,
                        onPlay = { position -> onNavigateToPlayer(id, "show", position) },
                        onNavigateUp = { nav.popBackStack() },
                    )
                }
            }

            navigation(route = "management", startDestination = "management/home") {
                composable("management/home") {
                    ManagementHomeScreen(
                        onNavigateToImportQueue = { nav.navigate("management/import_queue") },
                        onNavigateToTranscodeQueue = { nav.navigate("management/transcode_queue") },
                    )
                }

                composable("management/import_queue") {
                    ImportQueueScreen(onNavigateUp = { nav.popBackStack() })
                }

                composable("management/transcode_queue") {
                    TranscodeQueueScreen(onNavigateUp = { nav.popBackStack() })
                }
            }

            composable("about") {
                AboutScreen()
            }
        }

        BottomNavigationBar(nav = nav)
    }
}

@Composable
private fun BottomNavigationBar(nav: NavHostController) {
    val context = LocalContext.current
    val entry by nav.currentBackStackEntryAsState()
    val currentRoute = entry?.destination?.route

    @Composable
    fun RowScope.NavigationItem(name: String, icon: ImageVector, route: String) {
        BottomNavigationItem(
            selected = currentRoute?.startsWith(route) == true,
            icon = { Icon(icon, contentDescription = name) },
            label = { Text(name) },
            onClick = {
                context.playClick()
                if (currentRoute != route) {
                    nav.navigate(route) {
                        if (nav.backQueue.any { it.destination.route == route }) {
                            popUpTo(route) {
                                inclusive = true
                            }
                        }
                    }
                }
            },
        )
    }

    BottomNavigation {
        NavigationItem(name = "Library", icon = Icons.Default.VideoLibrary, route = "library")
        NavigationItem(name = "Manage", icon = Icons.Default.Dns, route = "management")
        NavigationItem(name = "About", icon = Icons.Default.Info, route = "about")
    }
}
