package uk.hasali.zenith

import com.squareup.moshi.Json
import com.squareup.moshi.Moshi
import com.squareup.moshi.adapters.PolymorphicJsonAdapterFactory
import com.squareup.moshi.kotlin.reflect.KotlinJsonAdapterFactory
import io.ktor.client.*
import io.ktor.client.request.*
import io.ktor.http.*
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.channels.SendChannel
import kotlinx.coroutines.channels.awaitClose
import kotlinx.coroutines.channels.onFailure
import kotlinx.coroutines.channels.trySendBlocking
import kotlinx.coroutines.flow.callbackFlow
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import okhttp3.OkHttpClient
import okhttp3.Request
import okhttp3.Response
import okhttp3.sse.EventSource
import okhttp3.sse.EventSourceListener
import okhttp3.sse.EventSources
import java.util.concurrent.TimeUnit

@Serializable
data class Movie(
    val id: Int,
    val title: String,
    val poster: String,
    val backdrop: String,
    val duration: Double,
    val overview: String,
    @SerialName("release_date")
    val releaseDate: Long,
    @SerialName("is_watched")
    val isWatched: Boolean,
)

@Serializable
data class Show(
    val id: Int,
    val name: String,
    val poster: String,
    val backdrop: String,
    val overview: String,
    @SerialName("start_date")
    val startDate: Long?,
    @SerialName("unwatched_episodes")
    val unwatchedEpisodes: Int,
)

@Serializable
data class Season(
    val id: Int,
    @SerialName("show_id")
    val showId: Int,
    @SerialName("season_number")
    val seasonNumber: Int,
    val name: String,
    val overview: String,
    val poster: String,
    val backdrop: String,
)

@Serializable
data class Episode(
    val id: Int,
    @SerialName("show_id")
    val showId: Int,
    @SerialName("season_id")
    val seasonId: Int,
    @SerialName("episode_number")
    val episodeNumber: Int,
    val name: String,
    val overview: String,
    val thumbnail: String?,
    val duration: Double,
    @SerialName("is_watched")
    val isWatched: Boolean,
)

@Serializable
data class VideoInfo(
    val path: String,
    val type: String,
    val format: String,
    val duration: Double,
    val position: Double?,
    val video: VideoStreamInfo,
    val audio: AudioStreamInfo,
    val subtitles: List<SubtitleStreamInfo>,
)

@Serializable
data class VideoStreamInfo(
    val codec: String,
    val profile: String,
    val width: Long,
    val height: Long,
)

@Serializable
data class AudioStreamInfo(
    val codec: String,
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

sealed class TranscoderJob {
    abstract val id: Int

    data class Queued(@Json(name = "video_id") override val id: Int) : TranscoderJob()
    data class Processing(@Json(name = "video_id") override val id: Int, val progress: Double) :
        TranscoderJob()
}

sealed class TranscoderEvent {
    data class InitialState(val queue: List<TranscoderJob>) : TranscoderEvent()

    data class Queued(val id: Int) : TranscoderEvent() {
        fun toJob() = TranscoderJob.Queued(id)
    }

    data class Started(val id: Int) : TranscoderEvent() {
        fun toJob() = TranscoderJob.Processing(id, 0.0)
    }

    data class Progress(val id: Int, val progress: Double) : TranscoderEvent() {
        fun toJob() = TranscoderJob.Processing(id, progress)
    }

    data class Success(val id: Int) : TranscoderEvent()
    data class Failure(val id: Int) : TranscoderEvent()
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

private class EventSourceClient(private val moshi: Moshi) {
    private val client = OkHttpClient.Builder()
        .readTimeout(0, TimeUnit.SECONDS)
        .build()

    private val eventSourceFactory = EventSources.createFactory(client)

    private class Listener<T>(
        private val type: Class<T>,
        private val channel: SendChannel<T>,
        private val moshi: Moshi
    ) : EventSourceListener() {
        override fun onEvent(eventSource: EventSource, id: String?, type: String?, data: String) {
            channel.trySendBlocking(moshi.adapter(this.type).fromJson(data)!!)
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

        val source = eventSourceFactory.newEventSource(req, Listener(T::class.java, channel, moshi))

        awaitClose {
            source.cancel()
        }
    }
}

class ZenithApiClient(private val client: HttpClient, private val baseUrl: String) {
    private val moshi = Moshi.Builder()
        .add(
            PolymorphicJsonAdapterFactory.of(TranscoderJob::class.java, "state")
                .withSubtype(TranscoderJob.Queued::class.java, "queued")
                .withSubtype(TranscoderJob.Processing::class.java, "processing")
        )
        .add(
            PolymorphicJsonAdapterFactory.of(TranscoderEvent::class.java, "type")
                .withSubtype(TranscoderEvent.InitialState::class.java, "initial_state")
                .withSubtype(TranscoderEvent.Queued::class.java, "queued")
                .withSubtype(TranscoderEvent.Started::class.java, "started")
                .withSubtype(TranscoderEvent.Progress::class.java, "progress")
                .withSubtype(TranscoderEvent.Success::class.java, "success")
                .withSubtype(TranscoderEvent.Failure::class.java, "failure")
        )
        .addLast(KotlinJsonAdapterFactory())
        .build()

    private val sseClient = EventSourceClient(moshi)

    suspend fun getMovies(): List<Movie> =
        client.get("$baseUrl/api/movies")

    suspend fun getRecentMovies(): List<Movie> =
        client.get("$baseUrl/api/movies/recent")

    suspend fun getMovie(id: Int): Movie =
        client.get("$baseUrl/api/movies/$id")

    suspend fun getShows(): List<Show> =
        client.get("$baseUrl/api/tv/shows")

    suspend fun getRecentShows(): List<Show> =
        client.get("$baseUrl/api/tv/shows/recent")

    suspend fun getShow(id: Int): Show =
        client.get("$baseUrl/api/tv/shows/$id")

    suspend fun getSeasons(showId: Int): List<Season> =
        client.get("$baseUrl/api/tv/shows/$showId/seasons")

    suspend fun getSeason(id: Int): Season =
        client.get("$baseUrl/api/tv/seasons/$id")

    suspend fun getEpisodes(seasonId: Int): List<Episode> =
        client.get("$baseUrl/api/tv/seasons/$seasonId/episodes")

    suspend fun getEpisode(id: Int): Episode =
        client.get("$baseUrl/api/tv/episodes/$id")

    fun getVideoUrl(id: Int) = "$baseUrl/api/videos/$id"

    fun getSubtitleUrl(id: Int) = "$baseUrl/api/subtitles/$id"

    suspend fun getVideoInfo(id: Int): VideoInfo =
        client.get("$baseUrl/api/videos/$id/info")

    suspend fun updateProgress(videoId: Int, position: Long): Unit =
        client.post("$baseUrl/api/progress/$videoId?position=$position")

    fun getTranscoderEvents() = sseClient.get<TranscoderEvent>("$baseUrl/api/transcoder/events")

    suspend fun startTranscode(videoId: Int): Unit =
        client.post("$baseUrl/api/transcoder?video_id=$videoId")

    suspend fun getImportQueue(): List<ImportQueueItem> =
        client.get("$baseUrl/api/import/queue")

    suspend fun importShow(show: ImportShowRequest): Unit =
        client.post("$baseUrl/api/tv/shows") {
            contentType(ContentType.Application.Json)
            body = show
        }

    suspend fun importEpisode(showId: Int, episode: ImportEpisodeRequest): Unit =
        client.post("$baseUrl/api/tv/shows/$showId/episodes") {
            contentType(ContentType.Application.Json)
            body = episode
        }

    suspend fun importMovie(movie: ImportMovieRequest): Unit =
        client.post("$baseUrl/api/movies") {
            contentType(ContentType.Application.Json)
            body = movie
        }

    suspend fun refreshMetadata(id: Int): Unit =
        client.post("$baseUrl/api/metadata/$id/refresh")
}
