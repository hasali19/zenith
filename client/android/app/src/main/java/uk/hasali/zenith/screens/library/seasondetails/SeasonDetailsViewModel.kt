package uk.hasali.zenith.screens.library.seasondetails

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import dagger.hilt.android.lifecycle.HiltViewModel
import kotlinx.coroutines.async
import kotlinx.coroutines.awaitAll
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.combine
import kotlinx.coroutines.launch
import uk.hasali.zenith.Episode
import uk.hasali.zenith.LibraryScreen
import uk.hasali.zenith.Season
import uk.hasali.zenith.ZenithApiClient
import uk.hasali.zenith.navigation.NavScreenProvider
import javax.inject.Inject

@HiltViewModel
class SeasonDetailsViewModel @Inject constructor(
    screenProvider: NavScreenProvider,
    private val client: ZenithApiClient,
) : ViewModel() {
    val screen: LibraryScreen.SeasonDetails by screenProvider

    private val _season = MutableStateFlow<Season?>(null)
    private val _episodes = MutableStateFlow<List<Episode>?>(null)

    val state = combine(_season, _episodes) { season, episodes ->
        SeasonDetailsViewState(
            season = season,
            episodes = episodes,
        )
    }

    init {
        viewModelScope.launch {
            awaitAll(
                async { _season.value = client.getSeason(screen.id) },
                async { _episodes.value = client.getEpisodes(screen.id) },
            )
        }
    }
}
