package uk.hasali.zenith.media

enum class VideoItemType {
    Movie,
    TvShow,
}

data class VideoItem(
    val id: Int,
    val type: VideoItemType,
    val url: String,
    val title: String,
    val subtitle: String?,
    val backdrop: String?,
    val duration: Double,
    val subtitles: List<SubtitleTrack>,
)

sealed class SubtitleTrack {
    abstract val id: Int
    abstract val url: String?
    abstract val title: String?
    abstract val language: String?

    data class Embedded(
        val index: Int,
        override val url: String?,
        override val id: Int,
        override val title: String?,
        override val language: String?,
    ) : SubtitleTrack()

    data class External(
        override val url: String?,
        override val id: Int,
        override val title: String?,
        override val language: String?,
    ) : SubtitleTrack()
}
