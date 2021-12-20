package uk.hasali.zenith.screens.library.movies

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import dagger.hilt.android.lifecycle.HiltViewModel
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch
import uk.hasali.zenith.api.Movie
import uk.hasali.zenith.api.ZenithMediaService
import javax.inject.Inject

@HiltViewModel
class MoviesViewModel @Inject constructor(
    private val client: ZenithMediaService,
) : ViewModel() {
    private val _movies = MutableStateFlow<List<Movie>?>(null)
    val movies get() = _movies.asStateFlow()

    fun refresh() {
        viewModelScope.launch {
            _movies.value = client.getMovies()
        }
    }
}
