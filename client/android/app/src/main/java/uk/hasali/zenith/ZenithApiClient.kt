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
    val overview: String,
    @SerialName("release_date")
    val releaseDate: Long,
)

@Serializable
data class Show(
    val id: Int,
    val name: String,
    val poster: String,
    val backdrop: String,
    val overview: String,
    @SerialName("start_date")
    val startDate: Long,
)

@Serializable
data class Season(
    val id: Int,
    val name: String,
    @SerialName("season_number")
    val seasonNumber: Int,
    val poster: String,
)

@Serializable
data class Episode(
    val id: Int,
    val name: String,
    @SerialName("episode_number")
    val episodeNumber: Int,
    val overview: String,
    val thumbnail: String,
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
data class SubtitleStreamInfo(
    val index: Int,
    val title: String?,
    val language: String?,
)

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

class ZenithApiClient(private val client: HttpClient) {
    suspend fun getMovies(): List<Movie> =
        client.get("https://zenith.hasali.uk/api/movies")

    suspend fun getRecentMovies(): List<Movie> =
        client.get("https://zenith.hasali.uk/api/movies/recent")

    suspend fun getShows(): List<Show> =
        client.get("https://zenith.hasali.uk/api/tv/shows")

    suspend fun getRecentShows(): List<Show> =
        client.get("https://zenith.hasali.uk/api/tv/shows/recent")

    suspend fun getSeasons(showId: Int): List<Season> =
        client.get("https://zenith.hasali.uk/api/tv/shows/$showId/seasons")

    suspend fun getEpisodes(seasonId: Int): List<Episode> =
        client.get("https://zenith.hasali.uk/api/tv/seasons/$seasonId/episodes")

    fun getVideoUrl(id: Int) = "https://zenith.hasali.uk/api/videos/$id"

    suspend fun getVideoInfo(id: Int): VideoInfo =
        client.get("https://zenith.hasali.uk/api/videos/$id/info")

    suspend fun updateProgress(videoId: Int, position: Long): Unit =
        client.post("https://zenith.hasali.uk/api/progress/$videoId?position=$position")

    suspend fun getTranscoderState(): TranscoderState =
        client.get("https://zenith.hasali.uk/api/transcoder")

    suspend fun startTranscode(videoId: Int): Unit =
        client.post("https://zenith.hasali.uk/api/transcoder?video_id=$videoId")

    suspend fun getImportQueue(): List<ImportQueueItem> =
        client.get("https://zenith.hasali.uk/api/import/queue")

    suspend fun importShow(show: ImportShowRequest): Unit =
        client.post("https://zenith.hasali.uk/api/tv/shows") {
            contentType(ContentType.Application.Json)
            body = show
        }

    suspend fun importEpisode(showId: Int, episode: ImportEpisodeRequest): Unit =
        client.post("https://zenith.hasali.uk/api/tv/shows/$showId/episodes") {
            contentType(ContentType.Application.Json)
            body = episode
        }
}
