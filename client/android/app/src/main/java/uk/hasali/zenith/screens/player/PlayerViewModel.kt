package uk.hasali.zenith.screens.player

import android.util.Log
import androidx.lifecycle.SavedStateHandle
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import dagger.hilt.android.lifecycle.HiltViewModel
import io.ktor.client.features.*
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.combine
import kotlinx.coroutines.launch
import uk.hasali.zenith.*
import uk.hasali.zenith.navigation.NavScreenProvider
import javax.inject.Inject

@HiltViewModel
class PlayerViewModel @Inject constructor(
    screenProvider: NavScreenProvider,
    preferences: Preferences,
    mediaUrlProvider: MediaUrlProvider,
    private val client: ZenithApiClient,
) : ViewModel() {
    private val screen: TopLevelScreen.Player by screenProvider

    private val server = preferences.serverUrl
    private var _item = MutableStateFlow<MediaItem?>(null)

    val item = combine(server, _item) { server, item ->
        if (server == null || item == null) {
            return@combine null
        }

        val title: String?
        val backdrop: String?
        val videoInfo: VideoInfo?

        when (item) {
            is Movie -> {
                title = item.title
                backdrop = item.backdrop
                videoInfo = item.videoInfo
            }

            is Episode -> {
                title = item.name
                backdrop = item.thumbnail
                videoInfo = item.videoInfo
            }

            else -> return@combine null
        }

        VideoItem(
            type = screen.type,
            url = MediaUrlProvider().getVideoUrl(server, item.id),
            title = title ?: "Untitled",
            backdrop = backdrop,
            duration = videoInfo.duration,
            startPosition = screen.position ?: 0.0,
            subtitles = videoInfo.subtitles.orEmpty().map {
                when (it) {
                    is SubtitleStreamInfo.Embedded -> SubtitleTrack.Embedded(
                        index = it.index,
                        url = mediaUrlProvider.getSubtitleUrl(server, it.id),
                        id = it.id,
                        title = it.title,
                        language = it.language,
                    )
                    is SubtitleStreamInfo.External -> SubtitleTrack.External(
                        url = mediaUrlProvider.getSubtitleUrl(server, it.id),
                        id = it.id,
                        title = it.title,
                        language = it.language,
                    )
                }
            }
        )
    }

    init {
        viewModelScope.launch {
            _item.value = client.getItem(screen.id)
        }
    }

    fun updateProgress(position: Long) {
        if (!BuildConfig.DEBUG) viewModelScope.launch {
            try {
                client.updateProgress(screen.id, position)
            } catch (e: ServerResponseException) {
                Log.w(
                    "PlayerScreen",
                    "Failed to update progress on server: ${e.message}",
                )
            }
        }
    }

    override fun onCleared() {
        println("Viewmodel cleared")
    }
}
