package uk.hasali.zenith.screens.library.episodedetails

import uk.hasali.zenith.api.Episode
import uk.hasali.zenith.api.Season
import uk.hasali.zenith.api.Show

data class EpisodeDetailsViewState(
    val show: Show? = null,
    val season: Season? = null,
    val episode: Episode? = null,
)
