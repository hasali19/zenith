package uk.hasali.zenith

import io.ktor.client.*
import io.ktor.client.request.*
import io.ktor.http.*
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.channels.SendChannel
import kotlinx.coroutines.channels.awaitClose
import kotlinx.coroutines.channels.onFailure
import kotlinx.coroutines.channels.trySendBlocking
import kotlinx.coroutines.flow.callbackFlow
import kotlinx.coroutines.flow.first
import kotlinx.serialization.*
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.JsonClassDiscriminator
import okhttp3.OkHttpClient
import okhttp3.Request
import okhttp3.Response
import okhttp3.sse.EventSource
import okhttp3.sse.EventSourceListener
import okhttp3.sse.EventSources
import java.util.concurrent.TimeUnit
import javax.inject.Inject

@Serializable
enum class MediaItemType {
    @SerialName("movie")
    Movie,

    @SerialName("show")
    Show,

    @SerialName("season")
    Season,

    @SerialName("episode")
    Episode,
}

@Serializable
sealed class MediaItem {
    abstract val id: Int
    abstract val type: MediaItemType
}

@Serializable
@SerialName("movie")
data class Movie(
    override val id: Int,
    val title: String,
    @SerialName("release_date")
    val releaseDate: Long?,
    val overview: String?,
    val poster: String?,
    val backdrop: String?,
    @SerialName("video_info")
    val videoInfo: VideoInfo,
    @SerialName("user_data")
    val userData: VideoUserData,
) : MediaItem() {
    override val type: MediaItemType
        get() = MediaItemType.Movie
}

@Serializable
@SerialName("show")
data class Show(
    override val id: Int,
    val name: String,
    @SerialName("start_date")
    val startDate: Long?,
    @SerialName("end_date")
    val endDate: Long?,
    val overview: String?,
    val poster: String?,
    val backdrop: String?,
    @SerialName("user_data")
    val userData: CollectionUserData,
) : MediaItem() {
    override val type: MediaItemType
        get() = MediaItemType.Show
}

@Serializable
@SerialName("season")
data class Season(
    override val id: Int,
    @SerialName("show_id")
    val showId: Int,
    @SerialName("season_number")
    val seasonNumber: Int,
    val name: String?,
    val overview: String?,
    val poster: String?,
    val backdrop: String?,
    @SerialName("user_data")
    val userData: CollectionUserData,
) : MediaItem() {
    override val type: MediaItemType
        get() = MediaItemType.Season
}

@Serializable
@SerialName("episode")
data class Episode(
    override val id: Int,
    @SerialName("show_id")
    val showId: Int,
    @SerialName("season_id")
    val seasonId: Int,
    @SerialName("season_number")
    val seasonNumber: Int,
    @SerialName("episode_number")
    val episodeNumber: Int,
    val name: String?,
    val overview: String?,
    val thumbnail: String?,
    @SerialName("video_info")
    val videoInfo: VideoInfo,
    @SerialName("user_data")
    val userData: VideoUserData,
) : MediaItem() {
    override val type: MediaItemType
        get() = MediaItemType.Episode
}

@Serializable
data class VideoInfo(
    val path: String,
    val duration: Double,
    val format: String? = null,
    val video: VideoStreamInfo? = null,
    val audio: List<AudioStreamInfo>? = null,
    val subtitles: List<SubtitleStreamInfo>? = null,
)

@Serializable
data class VideoUserData(
    @SerialName("is_watched")
    val isWatched: Boolean,
    val position: Double?,
)

@Serializable
data class VideoUserDataPatch(
    @SerialName("is_watched")
    val isWatched: Boolean? = null,
    val position: Double? = null,
)

@Serializable
data class CollectionUserData(
    val unwatched: Int,
)

@Serializable
data class VideoStreamInfo(
    val id: Int,
    val index: Int,
    val codec: String,
    val width: Long,
    val height: Long,
)

@Serializable
data class AudioStreamInfo(
    val id: Int,
    val index: Int,
    val codec: String,
    val language: String?,
)

@Serializable
sealed class SubtitleStreamInfo {
    abstract val id: Int
    abstract val title: String?
    abstract val language: String?

    @Serializable
    @SerialName("embedded")
    data class Embedded(
        override val id: Int,
        override val title: String?,
        override val language: String?,
        val index: Int,
    ) : SubtitleStreamInfo()

    @Serializable
    @SerialName("external")
    data class External(
        override val id: Int,
        override val title: String?,
        override val language: String?,
        val path: String,
    ) : SubtitleStreamInfo()
}

@OptIn(ExperimentalSerializationApi::class)
@Serializable
@JsonClassDiscriminator("state")
sealed class TranscoderJob {
    abstract val id: Int

    @Serializable
    @SerialName("queued")
    data class Queued(@SerialName("video_id") override val id: Int) : TranscoderJob()

    @Serializable
    @SerialName("processing")
    data class Processing(@SerialName("video_id") override val id: Int, val progress: Double) :
        TranscoderJob()
}

@Serializable
sealed class TranscoderEvent {
    @Serializable
    @SerialName("initial_state")
    data class InitialState(val queue: List<TranscoderJob>) : TranscoderEvent()

    @Serializable
    @SerialName("queued")
    data class Queued(val id: Int) : TranscoderEvent() {
        fun toJob() = TranscoderJob.Queued(id)
    }

    @Serializable
    @SerialName("started")
    data class Started(val id: Int) : TranscoderEvent() {
        fun toJob() = TranscoderJob.Processing(id, 0.0)
    }

    @Serializable
    @SerialName("progress")
    data class Progress(val id: Int, val progress: Double) : TranscoderEvent() {
        fun toJob() = TranscoderJob.Processing(id, progress)
    }

    @Serializable
    @SerialName("success")
    data class Success(val id: Int) : TranscoderEvent()

    @Serializable
    @SerialName("error")
    data class Error(val id: Int) : TranscoderEvent()
}

@Serializable
data class ImportQueueItem(
    val name: String,
    val path: String,
    val info: ImportQueueItemInfo?,
)

@Serializable
sealed class ImportQueueItemInfo {
    @Serializable
    @SerialName("movie")
    data class Movie(val title: String, val year: Int?) : ImportQueueItemInfo()

    @Serializable
    @SerialName("episode")
    data class Episode(val name: String, val season: Int, val episode: Int) : ImportQueueItemInfo()
}

@Serializable
sealed class ImportSource {
    @Serializable
    @SerialName("Local")
    data class LocalImportSource(val path: String) : ImportSource()
}

@Serializable
data class ImportShowRequest(
    val name: String,
    val episodes: List<ImportEpisodeRequest>,
)

@Serializable
data class ImportEpisodeRequest(
    val source: ImportSource,
    @SerialName("season_number")
    val seasonNumber: Int,
    @SerialName("episode_number")
    val episodeNumber: Int,
)

@Serializable
data class ImportMovieRequest(
    val source: ImportSource,
    val title: String,
    val year: Int,
)

private class EventSourceClient {
    private val client = OkHttpClient.Builder()
        .readTimeout(0, TimeUnit.SECONDS)
        .build()

    private val eventSourceFactory = EventSources.createFactory(client)

    private class Listener<T>(
        private val deserializer: DeserializationStrategy<T>,
        private val channel: SendChannel<T>
    ) : EventSourceListener() {
        override fun onEvent(eventSource: EventSource, id: String?, type: String?, data: String) {
            channel.trySendBlocking(Json.decodeFromString(deserializer, data))
                .onFailure { channel.close(it) }
        }

        override fun onClosed(eventSource: EventSource) {
            channel.close()
        }

        override fun onFailure(
            eventSource: EventSource,
            t: Throwable?,
            response: Response?
        ) {
            channel.close(t)
        }
    }

    @OptIn(ExperimentalCoroutinesApi::class)
    inline fun <reified T> get(url: String) = callbackFlow<T> {
        val req = Request.Builder()
            .url(url)
            .build()

        val source = eventSourceFactory.newEventSource(req, Listener(serializer(), channel))

        awaitClose {
            source.cancel()
        }
    }
}

class ZenithApiClient @Inject constructor(
    private val client: HttpClient,
    private val preferences: Preferences,
) {
    private val sseClient = EventSourceClient()

    private suspend inline fun <reified T> withBaseUrl(block: (baseUrl: String?) -> T): T =
        preferences.serverUrl.first().let {
            if (it == null) {
                throw IllegalStateException("Server base url is unset")
            } else {
                block(it)
            }
        }

    private suspend fun url(path: String) = withBaseUrl {
        "$it/api/$path"
    }

    private suspend inline fun <reified T> get(path: String): T = client.get(url(path))

    suspend fun getItem(id: Int): MediaItem = get("items/$id")

    suspend fun updateUserData(id: Int, data: VideoUserDataPatch): Unit =
        client.patch(url("items/$id/user_data")) {
            contentType(ContentType.Application.Json)
            body = data
        }

    suspend fun getMovies(): List<Movie> = get("movies")

    suspend fun getRecentMovies(): List<Movie> = get("movies/recent")

    @Suppress("unused")
    suspend fun getMovie(id: Int): Movie = get("movies/$id")

    suspend fun getShows(): List<Show> = get("tv/shows")

    suspend fun getRecentShows(): List<Show> = get("tv/shows/recent")

    suspend fun getShow(id: Int): Show = get("tv/shows/$id")

    suspend fun getSeasons(showId: Int): List<Season> = get("tv/shows/$showId/seasons")

    suspend fun getSeason(id: Int): Season = get("tv/seasons/$id")

    suspend fun getEpisodes(seasonId: Int): List<Episode> = get("tv/seasons/$seasonId/episodes")

    @Suppress("unused")
    suspend fun getEpisode(id: Int): Episode = get("tv/episodes/$id")

    suspend fun updateProgress(videoId: Int, position: Long): Unit =
        client.post(url("progress/$videoId?position=$position"))

    suspend fun getTranscoderEvents() = sseClient.get<TranscoderEvent>(url("transcoder/events"))

    suspend fun startTranscode(videoId: Int): Unit =
        client.post(url("transcoder?video_id=$videoId"))

    suspend fun getImportQueue(): List<ImportQueueItem> = get("import/queue")

    suspend fun importShow(show: ImportShowRequest): Unit = client.post(url("tv/shows")) {
        contentType(ContentType.Application.Json)
        body = show
    }

    suspend fun importEpisode(showId: Int, episode: ImportEpisodeRequest): Unit =
        client.post(url("tv/shows/$showId/episodes")) {
            contentType(ContentType.Application.Json)
            body = episode
        }

    suspend fun importMovie(movie: ImportMovieRequest): Unit = client.post(url("movies")) {
        contentType(ContentType.Application.Json)
        body = movie
    }

    suspend fun refreshMetadata(id: Int): Unit = client.post(url("metadata/$id/refresh"))
}
