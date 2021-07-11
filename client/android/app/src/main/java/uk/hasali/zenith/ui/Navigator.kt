package uk.hasali.zenith.ui

import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.saveable.SaveableStateHolder
import androidx.compose.runtime.setValue
import androidx.lifecycle.ViewModel
import uk.hasali.zenith.Episode
import uk.hasali.zenith.Movie
import uk.hasali.zenith.Season
import uk.hasali.zenith.Show

sealed class Screen {
    object ImportQueue : Screen()
    object TranscodeQueue : Screen()
    object Main : Screen()
    data class MovieDetails(val movie: Movie) : Screen()
    data class ShowDetails(val show: Show) : Screen()
    data class SeasonDetails(val show: Show, val season: Season) : Screen()
    data class EpisodeDetails(val show: Show, val season: Season, val episode: Episode) : Screen()
    data class Player(val id: Int, val title: String) : Screen()
}

@Composable
fun Screen.Composable() {
    when (this) {
        is Screen.ImportQueue -> ImportQueueScreen()
        is Screen.TranscodeQueue -> TranscodeQueueScreen()
        is Screen.Main -> MainScreen()
        is Screen.MovieDetails -> MovieDetailsScreen(movie = movie)
        is Screen.ShowDetails -> ShowDetailsScreen(show = show)
        is Screen.SeasonDetails -> SeasonDetailsScreen(
            show = show,
            season = season,
        )
        is Screen.EpisodeDetails -> EpisodeDetailsScreen(
            show = show,
            season = season,
            episode = episode,
        )
        is Screen.Player -> PlayerScreen(id = id, title = title)
    }
}

class Navigator(
    private val saveableStateHolder: SaveableStateHolder,
    navigator: Navigator? = null,
) : ViewModel() {
    private var stack: List<Screen> by mutableStateOf(navigator?.stack ?: listOf(Screen.Main))

    private var _push = true
    val push get() = _push

    val currentScreen
        get() = stack.last()

    fun push(screen: Screen) {
        _push = true
        stack = stack + screen
    }

    fun pop(): Boolean {
        return if (stack.size > 1) {
            saveableStateHolder.removeState(currentScreen.hashCode())
            stack = stack.dropLast(1)
            _push = false
            true
        } else {
            false
        }
    }

    @Composable
    fun SaveableStateProvider(screen: Screen, content: @Composable () -> Unit) {
        saveableStateHolder.SaveableStateProvider(screen.hashCode()) {
            content()
        }
    }
}
