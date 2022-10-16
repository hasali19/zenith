package uk.hasali.zenith.screens.library.shows

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import dagger.hilt.android.lifecycle.HiltViewModel
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch
import uk.hasali.zenith.api.Show
import uk.hasali.zenith.api.ZenithMediaService
import javax.inject.Inject

@HiltViewModel
class ShowsViewModel @Inject constructor(
    private val client: ZenithMediaService,
) : ViewModel() {
    private val _shows = MutableStateFlow<List<Show>?>(null)
    val shows get() = _shows.asStateFlow()

    fun refresh() {
        viewModelScope.launch {
            _shows.value = client.getShows()
        }
    }
}