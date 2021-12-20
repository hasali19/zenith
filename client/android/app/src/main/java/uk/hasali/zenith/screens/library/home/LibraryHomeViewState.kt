package uk.hasali.zenith.screens.library.home

import uk.hasali.zenith.api.Movie
import uk.hasali.zenith.api.Show

data class LibraryHomeViewState(
    val isRefreshing: Boolean = false,
    val movies: List<Movie> = emptyList(),
    val shows: List<Show> = emptyList(),
)
