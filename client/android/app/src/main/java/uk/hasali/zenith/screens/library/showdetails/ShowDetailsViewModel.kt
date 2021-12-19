package uk.hasali.zenith.screens.library.showdetails

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import dagger.hilt.android.lifecycle.HiltViewModel
import kotlinx.coroutines.async
import kotlinx.coroutines.awaitAll
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.combine
import kotlinx.coroutines.launch
import uk.hasali.zenith.LibraryScreen
import uk.hasali.zenith.Season
import uk.hasali.zenith.Show
import uk.hasali.zenith.ZenithApiClient
import uk.hasali.zenith.navigation.NavScreenProvider
import javax.inject.Inject

@HiltViewModel
class ShowDetailsViewModel @Inject constructor(
    screenProvider: NavScreenProvider,
    private val client: ZenithApiClient,
) : ViewModel() {
    val screen: LibraryScreen.ShowDetails by screenProvider

    private val _show = MutableStateFlow<Show?>(null)
    private val _seasons = MutableStateFlow<List<Season>?>(null)

    val state = combine(_show, _seasons) { show, seasons ->
        ShowDetailsViewState(
            show = show,
            seasons = seasons,
        )
    }

    fun refresh() {
        viewModelScope.launch {
            awaitAll(
                async { _show.value = client.getShow(screen.id) },
                async { _seasons.value = client.getSeasons(screen.id) },
            )
        }
    }

    fun refreshMetadata() {
        viewModelScope.launch {
            client.refreshMetadata(screen.id)
            refresh()
        }
    }
}
