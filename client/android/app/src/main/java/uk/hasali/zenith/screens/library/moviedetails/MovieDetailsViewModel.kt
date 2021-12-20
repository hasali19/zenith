package uk.hasali.zenith.screens.library.moviedetails

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

@HiltViewModel
class MovieDetailsViewModel @Inject constructor(
    screenProvider: NavScreenProvider,
    private val client: ZenithMediaService,
) : ViewModel() {
    private val screen: LibraryScreen.MovieDetails by screenProvider

    private val _movie = MutableStateFlow<Movie?>(null)
    val movie get() = _movie.asStateFlow()

    fun refresh() {
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
