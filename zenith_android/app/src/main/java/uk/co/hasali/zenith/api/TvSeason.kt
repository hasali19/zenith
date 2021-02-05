package uk.co.hasali.zenith.api

import com.google.gson.annotations.SerializedName

data class TvSeason(
    val id: Int,
    @SerializedName("show_id")
    val showId: Int,
    @SerializedName("season_number")
    val seasonNumber: Int,
    val name: String?,
    val overview: String?,
    val poster: String?,
    val backdrop: String?,
)
