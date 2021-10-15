package uk.hasali.zenith

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.RowScope
import androidx.compose.material.BottomNavigation
import androidx.compose.material.BottomNavigationItem
import androidx.compose.material.Icon
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Home
import androidx.compose.material.icons.filled.Movie
import androidx.compose.material.icons.filled.Tv
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
import uk.hasali.zenith.ui.TopLevelScreenScaffold

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
                onTopLevelNavigate = { topLevelNav.navigate(it) },
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

        composable("import_queue") {
            ImportQueueScreen(onNavigateUp = { topLevelNav.popBackStack() })
        }

        composable("transcode_queue") {
            TranscodeQueueScreen(onNavigateUp = { topLevelNav.popBackStack() })
        }

        composable("about") {
            AboutScreen(onNavigateUp = { topLevelNav.popBackStack() })
        }
    }
}

@Composable
private fun BottomNavigation(nav: NavHostController) {
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
                        if (nav.backQueue.any { it.destination.route?.contains(route) == true }) {
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
        NavigationItem(name = "Home", icon = Icons.Default.Home, route = "main/home")
        NavigationItem(name = "Movies", icon = Icons.Default.Movie, route = "main/movies")
        NavigationItem(name = "Shows", icon = Icons.Default.Tv, route = "main/shows")
    }
}

@Composable
private fun MainNavigation(
    onNavigateToPlayer: (id: Int, type: String, position: Double?) -> Unit,
    onTopLevelNavigate: (String) -> Unit,
) {
    val nav = rememberNavController()

    Column(modifier = Modifier.navigationBarsPadding()) {
        NavHost(nav, startDestination = "main", modifier = Modifier.weight(1f)) {
            navigation(route = "main", startDestination = "main/home") {
                composable("main/home") {
                    TopLevelScreenScaffold(onNavigate = onTopLevelNavigate) {
                        HomeScreen(
                            onNavigateToMovie = { movie -> nav.navigate("movie_details/${movie.id}") },
                            onNavigateToShow = { show -> nav.navigate("show_details/${show.id}") },
                        )
                    }
                }

                composable("main/movies") {
                    TopLevelScreenScaffold(onNavigate = onTopLevelNavigate) {
                        MoviesScreen(
                            onNavigateToMovie = { movie -> nav.navigate("movie_details/${movie.id}") },
                        )
                    }
                }

                composable("main/shows") {
                    TopLevelScreenScaffold(onNavigate = onTopLevelNavigate) {
                        ShowsScreen(
                            onNavigateToShow = { show -> nav.navigate("show_details/${show.id}") },
                        )
                    }
                }
            }

            composable(
                "movie_details/{id}", arguments = listOf(
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
                "show_details/{id}", arguments = listOf(
                    navArgument("id") { type = NavType.IntType },
                )
            ) {
                val args = it.arguments!!
                val id = args.getInt("id")

                ShowDetailsScreen(
                    id = id,
                    onNavigateToSeason = { season -> nav.navigate("season_details/${season.id}") },
                    onNavigateUp = { nav.popBackStack() },
                )
            }

            composable(
                "season_details/{id}", arguments = listOf(
                    navArgument("id") { type = NavType.IntType },
                )
            ) {
                val args = it.arguments!!
                val id = args.getInt("id")

                SeasonDetailsScreen(
                    id = id,
                    onNavigateToEpisode = { episode -> nav.navigate("episode_details/${episode.id}") },
                    onNavigateUp = { nav.popBackStack() },
                )
            }

            composable(
                "episode_details/{id}", arguments = listOf(
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

        BottomNavigation(nav = nav)
    }
}
