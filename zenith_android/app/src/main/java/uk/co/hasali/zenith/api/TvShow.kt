package uk.co.hasali.zenith.api

import com.google.gson.annotations.SerializedName
import java.time.Instant
import java.time.ZoneOffset

data class TvShow(
    val id: Int,
    val name: String,
    @SerializedName("start_date")
    val startDate: Long?,
    @SerializedName("end_date")
    val endDate: Long?,
    val overview: String?,
    val poster: String?,
    val backdrop: String?,
) {
    val startYear: Int?
        get() = startDate?.let {
            Instant.ofEpochSecond(it)
                .atOffset(ZoneOffset.UTC)
                .year
        }

    val endYear: Int?
        get() = endDate?.let {
            Instant.ofEpochSecond(it)
                .atOffset(ZoneOffset.UTC)
                .year
        }
}
