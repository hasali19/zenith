package uk.hasali.zenith.screens.library.itemdetails

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import dagger.hilt.android.lifecycle.HiltViewModel
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch
import okhttp3.MediaType.Companion.toMediaType
import okhttp3.MultipartBody
import okhttp3.RequestBody.Companion.toRequestBody
import uk.hasali.zenith.LibraryScreen
import uk.hasali.zenith.api.*
import uk.hasali.zenith.navigation.NavScreenProvider
import javax.inject.Inject

sealed interface MediaItemDetails

data class MovieDetails(val movie: Movie) : MediaItemDetails

data class ShowDetails(val show: Show, val seasons: List<Season>) : MediaItemDetails

data class SeasonDetails(val show: Show, val season: Season, val episodes: List<Episode>) :
    MediaItemDetails

data class EpisodeDetails(val show: Show, val season: Season, val episode: Episode) :
    MediaItemDetails

@HiltViewModel
class ItemDetailsViewModel @Inject constructor(
    screenProvider: NavScreenProvider,
    private val zenith: ZenithMediaService,
) : ViewModel() {
    private val screen: LibraryScreen.ItemDetails by screenProvider

    private val _item = MutableStateFlow<MediaItemDetails?>(null)
    val item
        get() = _item.asStateFlow()

    fun refresh() {
        viewModelScope.launch {
            refreshData()
        }
    }

    fun setWatched(isWatched: Boolean) {
        viewModelScope.launch {
            zenith.updateUserData(screen.id, VideoUserDataPatch(isWatched = isWatched))
        }
    }

    fun startTranscode() {
        viewModelScope.launch {
            zenith.startTranscode(screen.id)
        }
    }

    fun refreshMetadata() {
        viewModelScope.launch {
            zenith.refreshMetadata(screen.id)
            refreshData()
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

            zenith.importSubtitle(
                data = dataPart,
                file = MultipartBody.Part
                    .createFormData("file", filename, fileBody),
            )

            refreshData()
        }
    }

    private suspend fun refreshData() {
        _item.value = when (val item = zenith.getItem(screen.id)) {
            is Movie -> {
                MovieDetails(
                    movie = item,
                )
            }
            is Show -> {
                ShowDetails(
                    show = item,
                    seasons = zenith.getSeasons(item.id),
                )
            }
            is Season -> SeasonDetails(
                show = zenith.getShow(item.showId),
                season = item,
                episodes = zenith.getEpisodes(item.id),
            )
            is Episode -> EpisodeDetails(
                show = zenith.getShow(item.showId),
                season = zenith.getSeason(item.seasonId),
                episode = item,
            )
        }
    }
}
