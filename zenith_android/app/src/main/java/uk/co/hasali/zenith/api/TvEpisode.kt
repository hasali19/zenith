package uk.co.hasali.zenith.api

import com.google.gson.annotations.SerializedName

class TvEpisode(
    val id: Int,
    @SerializedName("show_id")
    val showId: Int,
    @SerializedName("season_id")
    val seasonId: Int,
    @SerializedName("episode_number")
    val episodeNumber: Int,
    val name: String?,
    @SerializedName("air_date")
    val airDate: Long?,
    val overview: String?,
    val thumbnail: String?,
    val duration: Double,
    @SerializedName("is_watched")
    val isWatched: Boolean,
)
