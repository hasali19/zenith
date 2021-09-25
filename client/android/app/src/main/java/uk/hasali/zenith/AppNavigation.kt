package uk.hasali.zenith

import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.navigation.NavHostController
import androidx.navigation.NavType
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.currentBackStackEntryAsState
import androidx.navigation.navArgument
import androidx.navigation.navigation
import uk.hasali.zenith.screens.*
import uk.hasali.zenith.screens.player.MediaItemType
import uk.hasali.zenith.screens.player.PlayerScreen
import uk.hasali.zenith.ui.TopLevelScreenScaffold

@Composable
fun AppNavigation(nav: NavHostController) {
    val entry by nav.currentBackStackEntryAsState()

    val onBottomNavigationNavigate: (String) -> Unit = {
        nav.navigate(it) {
            launchSingleTop = true
            restoreState = true

            popUpTo("main/home") {
                saveState = true
            }
        }
    }

    NavHost(
        navController = nav,
        startDestination = "main",
    ) {
        navigation(route = "main", startDestination = "main/home") {
            composable("main/home") {
                TopLevelScreenScaffold(entry = entry, onNavigate = onBottomNavigationNavigate) {
                    HomeScreen(
                        onNavigateToMovie = { movie -> nav.navigate("movie_details/${movie.id}") },
                        onNavigateToShow = { show -> nav.navigate("show_details/${show.id}") },
                    )
                }
            }

            composable("main/movies") {
                TopLevelScreenScaffold(entry = entry, onNavigate = onBottomNavigationNavigate) {
                    MoviesScreen(
                        onNavigateToMovie = { movie -> nav.navigate("movie_details/${movie.id}") },
                    )
                }
            }

            composable("main/shows") {
                TopLevelScreenScaffold(entry = entry, onNavigate = onBottomNavigationNavigate) {
                    ShowsScreen(
                        onNavigateToShow = { show -> nav.navigate("show_details/${show.id}") },
                    )
                }
            }
        }

        composable("movie_details/{id}", arguments = listOf(
            navArgument("id") { type = NavType.IntType },
        )) {
            val args = it.arguments!!
            val id = args.getInt("id")

            MovieDetailsScreen(
                id = id,
                onPlay = { replay -> nav.navigate("player/movie/$id?replay=$replay") },
                onNavigateUp = { nav.popBackStack() },
            )
        }

        composable("show_details/{id}", arguments = listOf(
            navArgument("id") { type = NavType.IntType },
        )) {
            val args = it.arguments!!
            val id = args.getInt("id")

            ShowDetailsScreen(
                id = id,
                onNavigateToSeason = { season -> nav.navigate("season_details/${season.id}") },
                onNavigateUp = { nav.popBackStack() },
            )
        }

        composable("season_details/{id}", arguments = listOf(
            navArgument("id") { type = NavType.IntType },
        )) {
            val args = it.arguments!!
            val id = args.getInt("id")

            SeasonDetailsScreen(
                id = id,
                onNavigateToEpisode = { episode -> nav.navigate("episode_details/${episode.id}") },
                onNavigateUp = { nav.popBackStack() },
            )
        }

        composable("episode_details/{id}", arguments = listOf(
            navArgument("id") { type = NavType.IntType },
        )) {
            val args = it.arguments!!
            val id = args.getInt("id")

            EpisodeDetailsScreen(
                id = id,
                onPlay = { replay -> nav.navigate("player/show/$id?replay=$replay") },
                onNavigateUp = { nav.popBackStack() },
            )
        }

        composable("player/{type}/{id}?replay={replay}", arguments = listOf(
            navArgument("type") { type = NavType.StringType },
            navArgument("id") { type = NavType.IntType },
            navArgument("replay") {
                type = NavType.BoolType
                defaultValue = false
            },
        )) {
            val args = it.arguments!!

            val id = args.getInt("id")
            val replay = args.getBoolean("replay")
            val type = when (val type = args.getString("type")) {
                "movie" -> MediaItemType.Movie
                "show" -> MediaItemType.TvShow
                else -> throw IllegalArgumentException("Invalid item type: $type")
            }

            PlayerScreen(
                id = id,
                type = type,
                replay = replay,
                onNavigateUp = { nav.popBackStack() },
            )
        }

        composable("import_queue") {
            ImportQueueScreen(onNavigateUp = { nav.popBackStack() })
        }

        composable("transcode_queue") {
            TranscodeQueueScreen(onNavigateUp = { nav.popBackStack() })
        }
    }
}
