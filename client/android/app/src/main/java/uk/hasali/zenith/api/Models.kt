package uk.hasali.zenith.api

import kotlinx.datetime.Instant
import kotlinx.datetime.TimeZone
import kotlinx.datetime.toLocalDateTime
import kotlinx.serialization.ExperimentalSerializationApi
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.JsonClassDiscriminator
import uk.hasali.zenith.ui.twoDigitNumber

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
sealed interface MediaItem {
    val id: Int
    val type: MediaItemType
}

@Serializable
sealed interface VideoItem : MediaItem {
    val videoInfo: VideoInfo
    val userData: VideoUserData

    val shouldResume: Boolean
        get() {
            val position = userData.position ?: return false
            return position > 0.05 * videoInfo.duration && position < 0.9 * videoInfo.duration
        }
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
    override val videoInfo: VideoInfo,
    @SerialName("user_data")
    override val userData: VideoUserData,
) : MediaItem, VideoItem {
    override val type: MediaItemType
        get() = MediaItemType.Movie

    fun releaseYear() = releaseDate?.let {
        Instant.fromEpochSeconds(it)
            .toLocalDateTime(TimeZone.UTC)
            .year
    }
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
) : MediaItem {
    override val type: MediaItemType
        get() = MediaItemType.Show

    fun startYear() = startDate?.let {
        Instant.fromEpochSeconds(it)
            .toLocalDateTime(TimeZone.UTC)
            .year
    }
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
    @SerialName("show_name")
    val showName: String,
    val overview: String?,
    val poster: String?,
    val backdrop: String?,
    @SerialName("user_data")
    val userData: CollectionUserData,
) : MediaItem {
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
    @SerialName("show_name")
    val showName: String,
    val overview: String?,
    val poster: String?,
    val backdrop: String?,
    val thumbnail: String?,
    @SerialName("video_info")
    override val videoInfo: VideoInfo,
    @SerialName("user_data")
    override val userData: VideoUserData,
) : MediaItem, VideoItem {
    override val type: MediaItemType
        get() = MediaItemType.Episode

    fun seasonEpisodeString() = "S${twoDigitNumber(seasonNumber)}E${twoDigitNumber(episodeNumber)}"
}

@Serializable
data class VideoInfo(
    val path: String,
    val duration: Double,
    val format: String? = null,
    val video: VideoStreamInfo? = null,
    val audio: List<AudioStreamInfo>? = null,
    val subtitles: List<SubtitleInfo>? = null,
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
data class SubtitleInfo(
    val id: Int,
    @SerialName("video_id")
    val videoId: Int,
    @SerialName("stream_index")
    val streamIndex: Int?,
    val path: String?,
    val title: String?,
    val language: String?,
)

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
sealed class ImportSource {
    @Serializable
    @SerialName("Upload")
    object Upload : ImportSource()
}

@Serializable
data class ImportSubtitleRequest(
    val source: ImportSource,
    @SerialName("video_id")
    val videoId: Int,
    val title: String?,
    val language: String?,
)
