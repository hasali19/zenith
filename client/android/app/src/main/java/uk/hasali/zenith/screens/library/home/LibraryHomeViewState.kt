package uk.hasali.zenith.screens.library.home

import uk.hasali.zenith.api.MediaItem
import uk.hasali.zenith.api.Movie
import uk.hasali.zenith.api.Show

data class LibraryHomeViewState(
    val isRefreshing: Boolean = false,
    val isError: Boolean = false,
    val continueWatching: List<MediaItem> = emptyList(),
    val recentMovies: List<Movie> = emptyList(),
    val recentShows: List<Show> = emptyList(),
)
