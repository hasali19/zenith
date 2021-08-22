package uk.hasali.zenith

import io.ktor.client.*
import io.ktor.client.request.*
import io.ktor.http.*
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

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

@Serializable
data class TranscoderState(val current: Int?, val queue: List<Int>)

@Serializable
data class ImportQueueItem(
    val name: String,
    val path: String,
)

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

class ZenithApiClient(private val client: HttpClient, private val baseUrl: String) {
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

    suspend fun getTranscoderState(): TranscoderState =
        client.get("$baseUrl/api/transcoder")

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
