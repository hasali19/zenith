package uk.hasali.zenith.screens.library.home

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import dagger.hilt.android.lifecycle.HiltViewModel
import kotlinx.coroutines.async
import kotlinx.coroutines.awaitAll
import kotlinx.coroutines.coroutineScope
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.combine
import kotlinx.coroutines.launch
import uk.hasali.zenith.api.Movie
import uk.hasali.zenith.api.Show
import uk.hasali.zenith.api.ZenithMediaService
import javax.inject.Inject

@HiltViewModel
class LibraryHomeViewModel @Inject constructor(
    private val client: ZenithMediaService,
) : ViewModel() {
    private val _movies = MutableStateFlow<List<Movie>>(emptyList())
    private val _shows = MutableStateFlow<List<Show>>(emptyList())
    private val _isRefreshing = MutableStateFlow(false)
    private val _isError = MutableStateFlow(false)

    val state =
        combine(_isRefreshing, _isError, _movies, _shows) { isRefreshing, isError, movies, shows ->
            LibraryHomeViewState(
                isRefreshing = isRefreshing,
                isError = isError,
                movies = movies,
                shows = shows,
            )
        }

    fun refresh() {
        viewModelScope.launch {
            _isRefreshing.value = true

            try {
                coroutineScope {
                    awaitAll(
                        async { _movies.value = client.getRecentMovies() },
                        async { _shows.value = client.getRecentShows() },
                    )
                }
                _isError.value = false
            } catch (t: Throwable) {
                _isError.value = true
            }

            _isRefreshing.value = false
        }
    }
}
