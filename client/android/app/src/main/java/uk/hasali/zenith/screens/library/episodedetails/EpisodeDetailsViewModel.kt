package uk.hasali.zenith.screens.library.episodedetails

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import dagger.hilt.android.lifecycle.HiltViewModel
import kotlinx.coroutines.async
import kotlinx.coroutines.awaitAll
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.combine
import kotlinx.coroutines.launch
import uk.hasali.zenith.*
import uk.hasali.zenith.navigation.NavScreenProvider
import javax.inject.Inject

@HiltViewModel
class EpisodeDetailsViewModel @Inject constructor(
    screenProvider: NavScreenProvider,
    private val client: ZenithApiClient,
) : ViewModel() {
    private val screen: LibraryScreen.EpisodeDetails by screenProvider

    private val _show = MutableStateFlow<Show?>(null)
    private val _season = MutableStateFlow<Season?>(null)
    private val _episode = MutableStateFlow<Episode?>(null)

    val state = combine(_show, _season, _episode) { show, season, episode ->
        EpisodeDetailsViewState(
            show = show,
            season = season,
            episode = episode,
        )
    }

    fun refresh() {
        viewModelScope.launch {
            val episode = client.getEpisode(screen.id)
                .also { _episode.value = it }

            awaitAll(
                async { _show.value = client.getShow(episode.showId) },
                async { _season.value = client.getSeason(episode.seasonId) },
            )
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
