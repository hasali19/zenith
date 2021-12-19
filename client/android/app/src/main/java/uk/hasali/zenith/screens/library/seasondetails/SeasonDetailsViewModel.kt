package uk.hasali.zenith.screens.library.seasondetails

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import dagger.hilt.android.lifecycle.HiltViewModel
import kotlinx.coroutines.async
import kotlinx.coroutines.awaitAll
import kotlinx.coroutines.coroutineScope
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.combine
import kotlinx.coroutines.launch
import uk.hasali.zenith.*
import uk.hasali.zenith.navigation.NavScreenProvider
import javax.inject.Inject

@HiltViewModel
class SeasonDetailsViewModel @Inject constructor(
    screenProvider: NavScreenProvider,
    private val client: ZenithApiClient,
) : ViewModel() {
    val screen: LibraryScreen.SeasonDetails by screenProvider

    private val _show = MutableStateFlow<Show?>(null)
    private val _season = MutableStateFlow<Season?>(null)
    private val _episodes = MutableStateFlow<List<Episode>?>(null)

    val state = combine(_show, _season, _episodes) { show, season, episodes ->
        SeasonDetailsViewState(
            show = show,
            season = season,
            episodes = episodes,
        )
    }

    fun refresh() {
        viewModelScope.launch {
            awaitAll(
                async {
                    _season.value = client.getSeason(screen.id).also {
                        _show.value = client.getShow(it.showId)
                    }
                },
                async { _episodes.value = client.getEpisodes(screen.id) },
            )
        }
    }

    fun refreshMetadata() {
        viewModelScope.launch {
            client.refreshMetadata(screen.id)
        }
    }
}
