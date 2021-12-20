package uk.hasali.zenith.screens.library.episodedetails

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import dagger.hilt.android.lifecycle.HiltViewModel
import kotlinx.coroutines.async
import kotlinx.coroutines.awaitAll
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.combine
import kotlinx.coroutines.launch
import okhttp3.MediaType.Companion.toMediaType
import okhttp3.MultipartBody
import okhttp3.RequestBody.Companion.toRequestBody
import uk.hasali.zenith.LibraryScreen
import uk.hasali.zenith.api.*
import uk.hasali.zenith.navigation.NavScreenProvider
import javax.inject.Inject

@HiltViewModel
class EpisodeDetailsViewModel @Inject constructor(
    screenProvider: NavScreenProvider,
    private val client: ZenithMediaService,
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
            refresh()
        }
    }

    fun importSubtitle(filename: String, content: ByteArray) {
        viewModelScope.launch {
            val dataPart = ImportSubtitleRequest(
                source = ImportSource.Upload,
                videoId = screen.id,
                title = filename,
                language = null
            )

            val mime = when {
                filename.endsWith(".srt") -> "application/x-subrip"
                filename.endsWith(".vtt") -> "text/vtt"
                else -> throw IllegalArgumentException("Unsupported subtitle extension: $filename")
            }

            val contentType = mime.toMediaType()
            val fileBody = content.toRequestBody(contentType)

            client.importSubtitle(
                data = dataPart,
                file = MultipartBody.Part
                    .createFormData("file", filename, fileBody),
            )

            refresh()
        }
    }
}
