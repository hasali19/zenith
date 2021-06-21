package uk.hasali.zenith.ui

import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.setValue
import androidx.lifecycle.ViewModel
import uk.hasali.zenith.Episode
import uk.hasali.zenith.Season
import uk.hasali.zenith.Show

sealed class Screen {
    object ImportQueue : Screen()
    object TranscodeQueue : Screen()
    object Main : Screen()
    data class ShowDetails(val show: Show) : Screen()
    data class SeasonDetails(val show: Show, val season: Season) : Screen()
    data class EpisodeDetails(val show: Show, val season: Season, val episode: Episode) : Screen()
    data class Player(val id: Int) : Screen()
}

class Navigator : ViewModel() {
    var stack by mutableStateOf(listOf<Screen>(Screen.Main))

    fun push(screen: Screen) {
        stack = stack + screen
    }

    fun pop(): Boolean {
        return if (stack.size > 1) {
            stack = stack.dropLast(1)
            true
        } else {
            false
        }
    }
}
