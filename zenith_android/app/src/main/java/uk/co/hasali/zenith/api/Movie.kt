package uk.co.hasali.zenith.api

import com.google.gson.annotations.SerializedName
import java.time.Instant
import java.time.ZoneOffset

data class Movie(
    val id: Int,
    val title: String,
    @SerializedName("release_date")
    val releaseDate: Long?,
    val overview: String?,
    val poster: String?,
    val backdrop: String?,
    val duration: Double,
) {
    val releaseYear: Int?
        get() = releaseDate?.let {
            Instant.ofEpochSecond(it)
                .atOffset(ZoneOffset.UTC)
                .year
        }
}
