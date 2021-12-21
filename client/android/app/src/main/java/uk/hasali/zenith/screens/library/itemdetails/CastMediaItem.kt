package uk.hasali.zenith.screens.library.itemdetails

import kotlinx.serialization.Serializable
import uk.hasali.zenith.api.Episode
import uk.hasali.zenith.api.Movie
import uk.hasali.zenith.api.Season
import uk.hasali.zenith.api.Show

@Serializable
sealed class CastMediaItem {
    @Serializable
    data class MovieItem(val movie: Movie) : CastMediaItem()

    @Serializable
    data class ShowItem(val show: Show) : CastMediaItem()

    @Serializable
    data class SeasonItem(val show: Show, val season: Season) : CastMediaItem()

    @Serializable
    data class EpisodeItem(val show: Show, val season: Season, val episode: Episode) :
        CastMediaItem()
}

fun CastMediaItem(item: MediaItemDetails) = when (item) {
    is MovieDetails -> CastMediaItem.MovieItem(item.movie)
    is ShowDetails -> CastMediaItem.ShowItem(item.show)
    is SeasonDetails -> CastMediaItem.SeasonItem(item.show, item.season)
    is EpisodeDetails -> CastMediaItem.EpisodeItem(item.show, item.season, item.episode)
}
