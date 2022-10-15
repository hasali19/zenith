package uk.hasali.zenith.screens.library.itemdetails

import android.util.Log
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.google.android.gms.cast.framework.CastContext
import com.google.android.gms.cast.framework.CastState
import com.google.android.gms.cast.framework.CastStateListener
import dagger.hilt.android.lifecycle.HiltViewModel
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json
import okhttp3.MediaType.Companion.toMediaType
import okhttp3.MultipartBody
import okhttp3.RequestBody.Companion.toRequestBody
import uk.hasali.zenith.LibraryScreen
import uk.hasali.zenith.MediaUrlProvider
import uk.hasali.zenith.api.*
import uk.hasali.zenith.media.MediaSessionManager
import uk.hasali.zenith.media.SubtitleTrack
import uk.hasali.zenith.media.VideoItem
import uk.hasali.zenith.media.VideoItemType
import uk.hasali.zenith.navigation.NavScreenProvider
import javax.inject.Inject

sealed interface MediaItemDetails

data class MovieDetails(val movie: Movie) : MediaItemDetails

data class ShowDetails(val show: Show, val seasons: List<Season>) : MediaItemDetails

data class SeasonDetails(val season: Season, val episodes: List<Episode>) : MediaItemDetails

data class EpisodeDetails(val episode: Episode) : MediaItemDetails

@HiltViewModel
class ItemDetailsViewModel @Inject constructor(
    screenProvider: NavScreenProvider,
    private val zenith: ZenithMediaService,
    private val mediaUrlProvider: MediaUrlProvider,
    private val mediaSessionManager: MediaSessionManager,
) : ViewModel(), CastStateListener {
    private val screen: LibraryScreen.ItemDetails by screenProvider

    private val _item = MutableStateFlow<MediaItemDetails?>(null)
    val item
        get() = _item.asStateFlow()

    private val castContext = CastContext.getSharedInstance()

    fun enableCastNotifier() {
        castContext?.addCastStateListener(this)
    }

    fun disableCastNotifier() {
        castContext?.removeCastStateListener(this)
    }

    override fun onCastStateChanged(state: Int) {
        if (state == CastState.CONNECTED) {
            notifyCastSession()
        }
    }

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

    fun play(position: Double?) {
        mediaSessionManager.getOrCreatePlayer()
            .setItem(
                item = _item.value!!.toVideoItem(),
                startAt = (position ?: 0.0).toLong() * 1000,
            )
    }

    private fun MediaItemDetails.toVideoItem(): VideoItem {
        val id: Int
        val type: VideoItemType
        val title: String?
        val subtitle: String?
        val poster: String?
        val backdrop: String?
        val videoInfo: VideoInfo?

        when (this) {
            is MovieDetails -> {
                id = movie.id
                type = VideoItemType.Movie
                title = movie.title
                subtitle = movie.releaseYear()?.toString()
                poster = movie.poster
                backdrop = movie.backdrop
                videoInfo = movie.videoInfo
            }

            is EpisodeDetails -> {
                id = episode.id
                type = VideoItemType.TvShow
                title = episode.name
                subtitle = "${episode.showName}: ${episode.seasonEpisodeString()}"
                poster = episode.poster
                backdrop = episode.thumbnail ?: episode.backdrop
                videoInfo = episode.videoInfo
            }

            else -> throw IllegalArgumentException("MediaItem must be a video")
        }

        return VideoItem(
            id = id,
            type = type,
            url = mediaUrlProvider.getVideoUrl(id),
            title = title ?: "Untitled",
            subtitle = subtitle,
            poster = poster,
            backdrop = backdrop,
            duration = videoInfo.duration,
            subtitles = videoInfo.subtitles.orEmpty().map {
                if (it.streamIndex != null) {
                    SubtitleTrack.Embedded(
                        index = it.streamIndex,
                        url = when (it.path) {
                            null -> null
                            else -> mediaUrlProvider.getSubtitleUrl(it.id)
                        },
                        id = it.id,
                        title = it.title,
                        language = it.language,
                    )
                } else {
                    SubtitleTrack.External(
                        url = mediaUrlProvider.getSubtitleUrl(it.id),
                        id = it.id,
                        title = it.title,
                        language = it.language,
                    )
                }
            }
        )
    }

    private fun notifyCastSession() {
        val session = CastContext.getSharedInstance()
            ?.sessionManager
            ?.currentCastSession

        if (session != null) {
            _item.value?.let { item ->
                val namespace = "urn:x-cast:uk.hasali.zenith.cast"
                val mediaItem = when (item) {
                    is MovieDetails -> item.movie
                    is ShowDetails -> item.show
                    is SeasonDetails -> item.season
                    is EpisodeDetails -> item.episode
                }
                val message = Json.encodeToString(mediaItem)
                Log.i("ItemDetails", "Sending current item to cast session: $message")
                session.sendMessage(namespace, message)
            }
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
                season = item,
                episodes = zenith.getEpisodes(item.id),
            )
            is Episode -> EpisodeDetails(
                episode = item,
            )
        }
        notifyCastSession()
    }
}
