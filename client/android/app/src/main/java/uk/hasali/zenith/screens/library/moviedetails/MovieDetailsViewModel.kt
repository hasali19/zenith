package uk.hasali.zenith.screens.library.moviedetails

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import dagger.hilt.android.lifecycle.HiltViewModel
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch
import uk.hasali.zenith.LibraryScreen
import uk.hasali.zenith.Movie
import uk.hasali.zenith.VideoUserDataPatch
import uk.hasali.zenith.ZenithApiClient
import uk.hasali.zenith.navigation.NavScreenProvider
import javax.inject.Inject

@HiltViewModel
class MovieDetailsViewModel @Inject constructor(
    screenProvider: NavScreenProvider,
    private val client: ZenithApiClient,
) : ViewModel() {
    private val screen: LibraryScreen.MovieDetails by screenProvider

    private val _movie = MutableStateFlow<Movie?>(null)
    val movie get() = _movie.asStateFlow()

    init {
        viewModelScope.launch {
            _movie.value = client.getMovie(screen.id)
        }
    }

    fun setWatched(isWatched: Boolean) {
        viewModelScope.launch {
            client.updateUserData(screen.id, VideoUserDataPatch(isWatched = isWatched))
        }
    }

    fun startTranscode() {
        viewModelScope.launch {
            client.startTranscode(screen.id)
        }
    }

    fun refreshMetadata() {
        viewModelScope.launch {
            client.refreshMetadata(screen.id)
        }
    }
}
