package uk.hasali.zenith

import io.ktor.client.*
import io.ktor.client.request.*
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

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
data class VideoInfo(val path: String, val position: Double?)

@Serializable
data class TranscoderState(val current: Int?, val queue: List<Int>)

class ZenithApiClient(private val client: HttpClient) {
    suspend fun getShows(): List<Show> =
        client.get("https://zenith.hasali.uk/api/tv/shows")

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
}
