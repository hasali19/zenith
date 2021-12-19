package uk.hasali.zenith.screens.library.home

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import dagger.hilt.android.lifecycle.HiltViewModel
import kotlinx.coroutines.async
import kotlinx.coroutines.awaitAll
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.combine
import kotlinx.coroutines.launch
import uk.hasali.zenith.Movie
import uk.hasali.zenith.Show
import uk.hasali.zenith.ZenithApiClient
import javax.inject.Inject

@HiltViewModel
class LibraryHomeViewModel @Inject constructor(
    private val client: ZenithApiClient,
) : ViewModel() {
    private val _movies = MutableStateFlow<List<Movie>>(emptyList())
    private val _shows = MutableStateFlow<List<Show>>(emptyList())
    private val _isRefreshing = MutableStateFlow(false)

    val state = combine(_isRefreshing, _movies, _shows) { isRefreshing, movies, shows ->
        LibraryHomeViewState(
            isRefreshing = isRefreshing,
            movies = movies,
            shows = shows,
        )
    }

    fun refresh() {
        viewModelScope.launch {
            _isRefreshing.value = true

            awaitAll(
                async { _movies.value = client.getRecentMovies() },
                async { _shows.value = client.getRecentShows() },
            )

            _isRefreshing.value = false
        }
    }
}
